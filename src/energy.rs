use crate::event_bus::{Event, EventBus};
use crate::orbital;
use crate::satellite::OperationalMode;

#[derive(Debug)]
pub struct EnergyModel {
    pub battery_level: f32,
    pub solar_panel_output: f32,
    pub degradation_rate: f32,
    pub max_capacity: f32,
}

impl EnergyModel {
    pub fn new() -> Self {
        Self {
            battery_level: 100.0,
            solar_panel_output: 0.0,
            degradation_rate: 0.5,
            max_capacity: 100.0,
        }
    }

    pub fn update(
        &mut self,
        orbital_model: &orbital::OrbitalModel,
        operational_mode: &OperationalMode,
        event_bus: &mut EventBus,
    ) {
        let previous_battery_level = self.battery_level;
        let previous_max_capacity = self.max_capacity;

        if orbital_model.orbit_completed() {
            self.max_capacity -= self.degradation_rate;
            self.max_capacity = self.max_capacity.max(0.0);
        }

        if previous_max_capacity > 30.0 && self.max_capacity <= 30.0 {
            event_bus.emit(Event::BatteryDegraded);
        }

        let mode_consumption = match operational_mode {
            OperationalMode::Idle => 0.6,
            OperationalMode::Charging => 0.5,
            OperationalMode::Transmitting => 1.0,
        };

        match orbital_model.phase {
            orbital::OrbitalPhase::SunPhase => {
                self.solar_panel_output = 0.8;
                self.battery_level = self.battery_level + self.solar_panel_output - mode_consumption;
            }
            orbital::OrbitalPhase::EclipsePhase => {
                self.solar_panel_output = 0.0;
                self.battery_level = self.battery_level - mode_consumption;
            }
        }

        self.battery_level = self.battery_level.clamp(0.0, self.max_capacity);

        if previous_battery_level > 10.0 && self.battery_level <= 10.0 {
            event_bus.emit(Event::BatteryCritical);
        } else if previous_battery_level > 30.0 && self.battery_level <= 30.0 {
            event_bus.emit(Event::BatteryLow);
        }

        if previous_battery_level < self.max_capacity && self.battery_level >= self.max_capacity {
            event_bus.emit(Event::BatteryFull);
        }
    }
}
