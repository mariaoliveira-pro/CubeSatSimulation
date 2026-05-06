use std::thread;
use std::time::Duration;

mod orbital;
mod energy;
mod event_bus;

use event_bus::EventBus;
use orbital::OrbitalModel;
use energy::EnergyModel;

fn main() {
    let mut model = OrbitalModel::new(10, 5);
    let mut energy_model = EnergyModel::new();
    let mut event_bus = EventBus::new();

    for i in 0..20 {
        model.update(&mut event_bus);
        energy_model.update(&model, &mut event_bus);

        println!("Passo: {} | Current phase: {:?} | Cycle time: {}\n", i, model.phase, model.cycle_time);
        println!("Battery level: {:.2}% | battery capacity: {:.2}%\n", energy_model.battery_level, energy_model.max_capacity);

        event_bus.process();

        thread::sleep(Duration::from_secs(1));
    }
}

