use crate::mission_state::MissionState;
use crate::satellite::OperationalMode;
use rand::Rng;

use crate::event_bus::{Event, EventBus};

#[derive(Debug)]
struct Message {
    battery_level: f32,
    battery_capacity: f32,
    orbit_number: u32,
    mission_state: MissionState,
    operational_mode: OperationalMode,
    solar_panel_output: f32,
    tick_chegada: u32,
}

#[derive(Debug)]
pub struct Network {
    messages: Vec<Message>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn send_data(
        &mut self,
        battery_level: f32,
        battery_capacity: f32,
        orbit_number: u32,
        mission_state: MissionState,
        operational_mode: OperationalMode,
        solar_panel_output: f32,
        tick_atual: u32,
    ) {
        let message = Message {
            battery_level,
            battery_capacity,
            orbit_number,
            mission_state,
            operational_mode,
            solar_panel_output,
            tick_chegada: tick_atual + 5,
        };

        self.messages.push(message);
    }

    pub fn receive(&mut self, tick_atual: u32, event_bus: &mut EventBus) {
        for message in &self.messages {
            let rand_val = rand::thread_rng().gen_range(0.0..1.0);
            if tick_atual >= message.tick_chegada {
                if rand_val < 0.05 {
                    event_bus.emit(Event::TransmissionFailed);
                } else {
                    println!(
                        "Received telemetry | Battery: {:.2}% | Capacity: {:.2}% | Orbit: {} | Mission: {:?} | Mode: {:?} | Solar: {:.2}",
                        message.battery_level,
                        message.battery_capacity,
                        message.orbit_number,
                        message.mission_state,
                        message.operational_mode,
                        message.solar_panel_output
                    );
                    event_bus.emit(Event::TransmissionSuccess);
                }
            }
        }
        self.messages.retain(|m| tick_atual < m.tick_chegada);
    }
}
