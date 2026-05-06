use std::thread;
use std::time::Duration;

mod orbital;

fn main() {
    let mut model = orbital::OrbitalModel::new(10, 5);
    
    for i in 0..20 {
        model.update();

        println!("Passo: {} | Current phase: {:?} | Cycle time: {}", i, model.phase, model.cycle_time);

        thread::sleep(Duration::from_secs(1));
    }
}
