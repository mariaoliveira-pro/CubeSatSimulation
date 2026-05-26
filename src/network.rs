use crate::satellite::{Satellite, OperationalMode};
use crate::mission_state::MissionState;
use rand::Rng;

use crate::event_bus::{Event, EventBus};

pub struct Message {
    battery_level: f32,
    battery_capacity: f32,
    orbit_number: u32,
    mission_state: MissionState,
    operational_mode: OperationalMode,
    solar_panel_output: f32,
    tick_chegada: u32,
}

pub struct Network {
    messages: Vec<Message>,
}

impl Network {
    fn new() -> Self{
        Self {
            messages: Vec::new(),
        }
    }
    fn send(&mut self, satellite: &Satellite, tick_atual: u32) {
        let message = Message {
            battery_level: satellite.energy_model.battery_level,
            battery_capacity: satellite.energy_model.max_capacity,
            orbit_number: satellite.orbital_model.orbit_number,
            mission_state: satellite.mission_state,
            operational_mode: satellite.operational_mode,
            solar_panel_output: satellite.energy_model.solar_panel_output,
            tick_chegada: tick_atual + 5,
        };
        
        self.messages.push(message);
 
    }

    fn receive(&mut self, tick_atual: u32, event_bus: &mut EventBus) {
        for message in &self.messages {
            let rand_val = rand::thread_rng().gen_range(0.0..1.0);
            if tick_atual >= message.tick_chegada {
                if rand_val < 0.05 {
                    event_bus.emit(Event::TransmisionFailed);
                } else {
                    event_bus.emit(Event::TransmissionSuccess);
                }
            }
        }
        self.messages.retain(|m| tick_atual < m.tick_chegada);
    }
}