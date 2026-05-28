use crate::satellite::Satellite;
use crate::event_bus::EventBus;
use std::thread;
use std::time::Duration;

pub struct Simulation {
    pub satellite: Satellite,
    pub event_bus: EventBus,
    pub tick: u32,
    pub total_ticks: u32,
}

impl Simulation {
    pub fn new(tick: u32, total_ticks: u32) -> Self {
        Self {
            satellite: Satellite::new(1) ,
            event_bus: EventBus::new(),
            tick,
            total_ticks,
            }

        }

    pub fn run(&mut self) {
        self.satellite.print_mission_info();
        while self.tick < self.total_ticks {
            self.step();
        }
    }

    pub fn step(&mut self ) {
        self.satellite.update(&mut self.event_bus, self.tick);
        self.satellite.network.receive(self.tick, &mut self.event_bus);
        self.satellite.print_state(self.tick);
        self.event_bus.process();
        self.tick += 1;

        thread::sleep(Duration::from_millis(300));
    }
}
