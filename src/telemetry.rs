pub struct Telemetry {
    battery_soc_pct: f32,
    battery_capacity: f32,
    solar_power: f32,
    mission_state: u32,
    operational_mode: u32,
}

impl Telemetry {
    pub fn new() -> Self {
        // inicializa ligação ao collector
    }

    pub fn record(&self, satellite: &Satellite) {
        // atualiza os valores a cada tick
    }
}