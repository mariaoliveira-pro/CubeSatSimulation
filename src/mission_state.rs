#[derive(Debug, Clone, Copy)]
pub enum MissionState {
    Leop,
    Commissioning,
    Nominal,
    LowPower,
    SafeMode,
    EndOfLife,
}

impl MissionState {
    pub fn evaluate(&self, battery_level: f32, battery_capacity: f32) -> MissionState {
        match self {
            _ if battery_capacity < 10.0 => MissionState::EndOfLife,
            MissionState::SafeMode => MissionState::SafeMode,
            //_ if anomalia=> MissionState::SafeMode,
            MissionState::LowPower if battery_level >= 50.0 => MissionState::Nominal,
            _ if battery_level < 25.0 => MissionState::LowPower,
            MissionState::Commissioning => MissionState::Nominal,
            MissionState::Leop => MissionState::Commissioning,
            _ => MissionState::Nominal,
        }
    }
}
