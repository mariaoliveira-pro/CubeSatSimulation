use crate::energy::EnergyModel;
use crate::event_bus::EventBus;
use crate::ground_station::GroundStation;
use crate::mission_state::MissionState;
use crate::orbital::{OrbitalModel, OrbitalPhase};
use crate::network::Network;

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
        }
    }

    pub fn update(&mut self, event_bus: &mut EventBus, tick: u32) {
        self.update_orbital_model(event_bus);
        self.update_ground_station(event_bus);
        self.update_operational_mode(tick);
        self.update_energy_model(event_bus);
        self.update_mission_state();

    }

    fn update_orbital_model(&mut self, event_bus: &mut EventBus) {
        self.orbital_model.update(event_bus);
    }

    fn update_energy_model(&mut self, event_bus: &mut EventBus) {
        self.energy_model.update(&self.orbital_model, &self.operational_mode, event_bus);
    }

    fn update_mission_state(&mut self) {
        self.mission_state = self.mission_state.evaluate(
            self.energy_model.battery_level,
            self.energy_model.max_capacity,
            false
        );
    }

    fn update_operational_mode(&mut self, tick: u32) {
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
                } else  {
                    self.operational_mode = OperationalMode::Idle;
                }
            }

            MissionState::Nominal => {
                if self.ground_station.contact_active {
                    self.operational_mode = OperationalMode::Transmitting;
                    let battery_level = self.energy_model.battery_level;
                    let battery_capacity = self.energy_model.max_capacity;
                    let orbit_number = self.orbital_model.orbit_number;
                    let mission_state = self.mission_state;
                    let operational_mode = self.operational_mode;
                    let solar_panel_output = self.energy_model.solar_panel_output;
                    self.network.send_data(battery_level, battery_capacity, orbit_number, mission_state, operational_mode, solar_panel_output, tick);
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
            self.ground_station.contact_active,
            self.mission_state,
            self.operational_mode
        );
    }
}
