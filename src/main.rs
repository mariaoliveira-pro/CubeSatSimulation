use crate::simulation::Simulation;

mod energy;
mod event_bus;
mod ground_station;
mod mission_state;
mod network;
mod orbital;
mod satellite;
mod simulation;
mod telemetry;

#[tokio::main]
async fn main() {
    let mut sim = Simulation::new(1, 6000);
    sim.run();
}
