use crate::simulation::Simulation;

mod energy;
mod event_bus;
mod ground_station;
mod mission_state;
mod orbital;
mod satellite;
mod simulation;
mod network;

fn main() {
    let mut sim = Simulation::new(1, 100);
    sim.run();
   
}
