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
    pub fn evaluate(
        &self,
        battery_level: f32,
        battery_capacity: f32,
        anomaly: bool,
    ) -> MissionState {
        if battery_capacity < 10.0 {
            return MissionState::EndOfLife;
        }

        if anomaly {
            return MissionState::SafeMode;
        }

        if matches!(self, MissionState::SafeMode) {
            if battery_level >= 50.0 {
                return MissionState::Nominal;
            } else {
                //anomaly é false
                return MissionState::LowPower;
            }
        }

        if matches!(self, MissionState::LowPower) && battery_level >= 50.0 {
            return MissionState::Nominal;
        }

        if battery_level < 25.0 {
            return MissionState::LowPower;
        }

        if matches!(self, MissionState::Leop) {
            return MissionState::Commissioning;
        }

        if matches!(self, MissionState::Commissioning) {
            return MissionState::Nominal;
        }

        MissionState::Nominal
    }
}
