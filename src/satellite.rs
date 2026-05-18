use crate::orbital::OrbitalModel;
use crate::energy::EnergyModel;
use crate::mission_state::MissionState;

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
}

impl Satellite {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            orbital_model: OrbitalModel::new(10, 5),
            energy_model: EnergyModel::new(),
            mission_state: MissionState::Leop,
            operational_mode: OperationalMode::Idle,
        }
    }

    pub fn update(&mut self) {
        self.update_models();
        self.update_mission_state();
        self.update_operational_mode();
    }


    fn update_models(&mut self) {
        self.orbital_model.update();
        self.energy_model.update(&self.orbital_model);
    }

    fn update_mission_state(&mut self) {
        self.mission_state = self.mission_state.evaluate(self.energy_model.battery_level, self.energy_model.max_capacity);
    }

    fn update_operational_mode(&mut self) {
        self.operational_mode = match self.mission_state {
            MissionState::Leop | MissionState::Commissioning => OperationalMode::Idle,
            MissionState::Nominal => OperationalMode::Transmitting,
            MissionState::LowPower => OperationalMode::Charging,
            MissionState::SafeMode | MissionState::EndOfLife => OperationalMode::Idle,
        }
    }
}
