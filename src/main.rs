use std::thread;
use std::time::Duration;

mod energy;
mod event_bus;
mod ground_station;
mod mission_state;
mod orbital;
mod satellite;

use event_bus::EventBus;
use satellite::Satellite;

fn main() {
    let mut satellite = Satellite::new(1);
    let mut event_bus = EventBus::new();

    for i in 0..20 {
        satellite.update(&mut event_bus);

        println!("\nPasso: {} | Satellite ID: {}", i, satellite.id);

        println!(
            "Orbit: {} | Cycle time: {} | Phase: {:?}",
            satellite.orbital_model.orbit_number,
            satellite.orbital_model.cycle_time,
            satellite.orbital_model.phase
        );

        println!(
            "Battery: {:.2}% | Capacity: {:.2}% | Solar output: {:.2}",
            satellite.energy_model.battery_level,
            satellite.energy_model.max_capacity,
            satellite.energy_model.solar_panel_output
        );

        println!(
            "Ground contact: {} | Mission state: {:?} | Operational mode: {:?}",
            satellite.ground_station.contact_active,
            satellite.mission_state,
            satellite.operational_mode
        );

        event_bus.process();

        thread::sleep(Duration::from_secs(1));
    }
}
