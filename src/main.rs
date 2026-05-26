use crate::simulation::Simulation;

mod energy;
mod event_bus;
mod ground_station;
mod mission_state;
mod orbital;
mod satellite;
mod simulation;
mod network;

#[tokio::main]
async fn main() {
    let mut sim = Simulation::new(1, 200);
    sim.run();
   
}
