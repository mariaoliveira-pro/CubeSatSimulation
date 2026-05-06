use crate::event_bus::{Event, EventBus};
use crate::orbital;

#[derive(Debug)]
pub struct EnergyModel {
    pub battery_level: f32,
    pub solar_panel_output: f32,
    pub consumption_rate: f32,
    pub degradation_rate: f32,
    pub max_capacity: f32,
}

impl EnergyModel {
    pub fn new() -> Self {
        Self {
            battery_level: 100.0,
            solar_panel_output: 0.0,
            consumption_rate: 1.0,
            degradation_rate: 0.01,
            max_capacity: 100.0,
        }
    }

    pub fn update(&mut self, orbital_model: &orbital::OrbitalModel, event_bus: &mut EventBus) {

        let previous_battery_level = self.battery_level;

        if orbital_model.orbit_completed() {
            self.max_capacity -= self.degradation_rate;
            self.max_capacity = self.max_capacity.max(0.0);
        }

        match orbital_model.phase {
            orbital::OrbitalPhase::SunPhase => {
                self.solar_panel_output = 4.0;
                self.battery_level =
                    self.battery_level + self.solar_panel_output - self.consumption_rate;
            }
            orbital::OrbitalPhase::EclipsePhase => {
                self.solar_panel_output = 0.0;
                self.battery_level = self.battery_level - self.consumption_rate;
            }
        }

        self.battery_level = self.battery_level.clamp(0.0, self.max_capacity);

        if previous_battery_level > 10.0 && self.battery_level <= 10.0 {
            event_bus.emit(Event::BatteryCritical);
        } else

        if previous_battery_level > 30.0 && self.battery_level <= 30.0 {
            event_bus.emit(Event::BatteryLow);
        }

        if previous_battery_level < self.max_capacity && self.battery_level >= self.max_capacity {
            event_bus.emit(Event::BatteryFull);
        }

    }
}
