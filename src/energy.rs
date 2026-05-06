use crate::orbital;

#[derive(Debug)]
pub struct EnergyModel {
    pub battery_level: f32,
    pub solar_panel_output: f32,
    pub consumption_rate: f32,
}

impl EnergyModel {
    pub fn new () -> Self {
        Self {
            battery_level: 100.0,
            solar_panel_output: 0.0,
            consumption_rate: 1.0,
        }
    }

    pub fn update(&mut self, orbital_phase: &orbital::OrbitalPhase) {
        match orbital_phase {
            orbital::OrbitalPhase::SunPhase => {
                self.solar_panel_output = 4.0;
                self.battery_level = self.battery_level + self.solar_panel_output - self.consumption_rate;
            }
            orbital::OrbitalPhase::EclipsePhase => {
                self.solar_panel_output = 0.0;
                self.battery_level = self.battery_level - self.consumption_rate;
            }
        }

        self.battery_level = self.battery_level.clamp(0.0, 100.0);
    }

}