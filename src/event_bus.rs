#[derive(Debug)]
pub enum Event {
    OrbitCompleted,
    EnteredSun,
    EnteredEclipse,
    BatteryLow,
    BatteryCritical, 
    BatteryFull,
    BatteryDegraded,
    GroundStationContactStarted,
    GroundStationContactEnded,
    TransmissionSuccess,
    TransmissionFailed,
    TemperatureTooHigh,
    TemperatureTooLow,
}

pub struct EventBus {
    pub events: Vec<Event>,
}

impl EventBus {
    pub fn new() -> Self {
        Self { events: Vec::new() }
    }

    pub fn emit(&mut self, event: Event) {
        self.events.push(event);
    }

    pub fn process(&mut self) {
        for event in &self.events {
            println!("EVENT: {:?}\n", event);
        }

        self.events.clear();
    }
}
