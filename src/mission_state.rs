pub enum MissionState {
    Leop,
    Commissioning,
    Nominal,
    LowPower,
    SafeMode,
    EndOfLife,
}

impl MissionState {
    pub fn evaluate(&self, battery_level: f32) -> MissionState {
        match self {

        }
    }
}