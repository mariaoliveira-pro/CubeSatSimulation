use crate::event_bus::{Event, EventBus};

#[derive(Debug, Clone, Copy)]
pub enum OrbitalPhase {
    SunPhase,
    EclipsePhase,
}

#[derive(Debug)]
pub struct OrbitalModel {
    pub phase: OrbitalPhase,
    pub sun_time: u32,
    pub eclipse_time: u32,
    pub cycle_time: u32,
    pub orbit_number: u32,
    pub altitude_km: f32,
    pub orbital_period_minutes: f32,
}

impl OrbitalModel {
    pub fn new(sun_time: u32, eclipse_time: u32) -> Self {
        Self {
            phase: OrbitalPhase::SunPhase,
            sun_time: sun_time,
            eclipse_time: eclipse_time,
            cycle_time: 0,
            orbit_number: 1,
            altitude_km: 500.0,
            orbital_period_minutes: 94.6,
        }
    }

    pub fn update(&mut self, event_bus: &mut EventBus) {
        self.cycle_time += 1;

        if self.cycle_time > self.sun_time + self.eclipse_time {
            self.cycle_time = 1;
            self.orbit_number += 1;
            event_bus.emit(Event::OrbitCompleted);
        }

        if self.cycle_time <= self.sun_time {
            self.phase = OrbitalPhase::SunPhase;
            if self.cycle_time == 1 {
                event_bus.emit(Event::EnteredSun);
            }
        } else {
            self.phase = OrbitalPhase::EclipsePhase;
            if self.cycle_time == self.sun_time + 1 {
                event_bus.emit(Event::EnteredEclipse);
            }
        }
    }

    pub fn orbit_completed(&self) -> bool {
        self.cycle_time == self.sun_time + self.eclipse_time
    }
}
