use crate::energy::EnergyModel;
use crate::event_bus::{Event, EventBus};
use crate::ground_station::GroundStation;
use crate::mission_state::MissionState;
use crate::network::Network;
use crate::orbital::{OrbitalModel, OrbitalPhase};

#[derive(Debug, Clone, Copy)]
pub enum OperationalMode {
    Idle,
    Charging,
    Transmitting,
}

#[derive(Debug)]
pub struct Satellite {
    pub id: u32,
    pub orbital_model: OrbitalModel,
    pub energy_model: EnergyModel,
    pub mission_state: MissionState,
    pub operational_mode: OperationalMode,
    pub ground_station: GroundStation,
    pub network: Network,
    pub temperature_celsius: f32,
    pub anomaly_active: bool,
}

impl Satellite {
    pub fn new(id: u32) -> Self {
        Self {
            id: id,
            orbital_model: OrbitalModel::new(62, 33),
            energy_model: EnergyModel::new(),
            mission_state: MissionState::Leop,
            operational_mode: OperationalMode::Idle,
            ground_station: GroundStation::new(5, 3), //contacto=3 para testes, depois mudar para 8
            network: Network::new(),
            temperature_celsius: 20.0,
            anomaly_active: false,
        }
    }

    pub fn update(&mut self, event_bus: &mut EventBus, tick: u32) {
        //atualiza ambiente
        self.update_orbital_model(event_bus);
        self.update_ground_station(event_bus);

        //escolhe modo com base no ambiente
        self.update_mission_state();
        self.update_operational_mode();

        //aplicar os efeitos do modo em que está
        self.update_energy_model(event_bus);
        let anomaly_changed = self.update_temperature(event_bus);

        if anomaly_changed {
            // verificar se os efeitos causaram problemas (anomalias) ou mudar de estado
            self.update_mission_state();
            self.update_operational_mode(); //duas vezes para garantir que o modo operacional se ajusta ao estado da missão atualizado
        }

        self.send_data(tick);
    }

    fn update_orbital_model(&mut self, event_bus: &mut EventBus) {
        self.orbital_model.update(event_bus);
    }

    fn update_energy_model(&mut self, event_bus: &mut EventBus) {
        self.energy_model
            .update(&self.orbital_model, &self.operational_mode, event_bus);
    }

    fn update_mission_state(&mut self) {
        self.mission_state = self.mission_state.evaluate(
            self.energy_model.battery_level,
            self.energy_model.max_capacity,
            self.anomaly_active,
        );
    }

    fn update_operational_mode(&mut self) {
        match self.mission_state {
            MissionState::Leop => {
                self.operational_mode = OperationalMode::Idle;
            }

            MissionState::Commissioning => {
                self.operational_mode = OperationalMode::Idle;
            }

            MissionState::SafeMode => {
                self.operational_mode = OperationalMode::Idle;
            }

            MissionState::EndOfLife => {
                self.operational_mode = OperationalMode::Idle;
            }

            MissionState::LowPower => {
                if matches!(self.orbital_model.phase, OrbitalPhase::SunPhase) {
                    self.operational_mode = OperationalMode::Charging;
                } else {
                    self.operational_mode = OperationalMode::Idle;
                }
            }

            MissionState::Nominal => {
                if self.ground_station.contact_active {
                    self.operational_mode = OperationalMode::Transmitting;
                } else {
                    if matches!(self.orbital_model.phase, OrbitalPhase::SunPhase) {
                        self.operational_mode = OperationalMode::Charging;
                    } else if matches!(self.orbital_model.phase, OrbitalPhase::EclipsePhase) {
                        self.operational_mode = OperationalMode::Idle;
                    }
                }
            }
        }
    }

    fn update_ground_station(&mut self, event_bus: &mut EventBus) {
        self.ground_station.update(&self.orbital_model, event_bus);
    }

    fn update_temperature(&mut self, event_bus: &mut EventBus) -> bool {
        let previous_anomaly = self.anomaly_active;

        match self.orbital_model.phase {
            OrbitalPhase::SunPhase => {
                if matches!(self.mission_state, MissionState::SafeMode) {
                    self.temperature_celsius += 0.05;
                } else {
                    self.temperature_celsius += 0.20;
                }
            }

            OrbitalPhase::EclipsePhase => {
                if matches!(self.mission_state, MissionState::SafeMode) {
                    self.temperature_celsius -= 0.35;
                } else {
                    self.temperature_celsius -= 0.15;
                }
            }
        }

        if matches!(self.operational_mode, OperationalMode::Charging) {
            self.temperature_celsius += 0.03;
        }

        if matches!(self.operational_mode, OperationalMode::Transmitting) {
            self.temperature_celsius += 0.09;
        }

        self.temperature_celsius = self.temperature_celsius.clamp(-40.0, 85.0);

        if self.temperature_celsius >= 65.0 {
            self.anomaly_active = true;
        }

        if self.temperature_celsius <= 60.0 {
            self.anomaly_active = false;
        }

        if !previous_anomaly && self.anomaly_active {
            event_bus.emit(Event::TemperatureCritical);
        }

        previous_anomaly != self.anomaly_active
    }

    fn send_data(&mut self, tick: u32) {
        if matches!(self.operational_mode, OperationalMode::Transmitting) {
            let battery_level = self.energy_model.battery_level;
            let battery_capacity = self.energy_model.max_capacity;
            let orbit_number = self.orbital_model.orbit_number;
            let mission_state = self.mission_state;
            let operational_mode = self.operational_mode;
            let solar_panel_output = self.energy_model.solar_panel_output;
            self.network.send_data(
                battery_level,
                battery_capacity,
                orbit_number,
                mission_state,
                operational_mode,
                solar_panel_output,
                tick,
            );
        }
    }

    pub fn print_mission_info(&self) {
        println!(
            "CubeSat Simulation | Altitude: {:.1} km | Period: {:.1} min",
            self.orbital_model.altitude_km, self.orbital_model.orbital_period_minutes
        );
    }

    pub fn print_state(&self, tick: u32) {
        println!("\nTick: {} | Satellite ID: {}", tick, self.id);
        println!(
            "Orbit: {} | Cycle time: {} | Phase: {:?}",
            self.orbital_model.orbit_number,
            self.orbital_model.cycle_time,
            self.orbital_model.phase
        );
        println!(
            "Battery: {:.2}% | Capacity: {:.2}% | Solar output: {:.2}",
            self.energy_model.battery_level,
            self.energy_model.max_capacity,
            self.energy_model.solar_panel_output
        );
        println!(
            "Ground contact: {} | Mission state: {:?} | Operational mode: {:?}",
            self.ground_station.contact_active, self.mission_state, self.operational_mode
        );
        println!(
            "Temperature: {:.2}°C | Anomaly active: {}",
            self.temperature_celsius, self.anomaly_active
        );
    }
}
