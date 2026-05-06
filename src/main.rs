use std::thread;
use std::time::Duration;

mod orbital;
mod energy;

fn main() {
    let mut model = orbital::OrbitalModel::new(10, 5);
    let mut energy_model = energy::EnergyModel::new();

    for i in 0..20 {
        model.update();
        energy_model.update(&model);

        println!("Passo: {} | Current phase: {:?} | Cycle time: {}\n", i, model.phase, model.cycle_time);
        println!("Battery level: {:.2}% | battery capacity: {:.2}%\n", energy_model.battery_level, energy_model.max_capacity);

        thread::sleep(Duration::from_secs(1));
    }
}
