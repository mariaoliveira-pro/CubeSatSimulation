use crate::{
    mission_state::MissionState,
    satellite::{OperationalMode, Satellite},
};
use opentelemetry::metrics::Gauge;
use opentelemetry::metrics::MeterProvider;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::runtime;
use std::time::Duration;

pub struct Telemetry {
    battery_soc_pct: Gauge<f64>,
    battery_capacity: Gauge<f64>,
    solar_power: Gauge<f64>,
    mission_state: Gauge<f64>,
    operational_mode: Gauge<f64>,
}

impl Telemetry {
    pub fn new() -> Self {
        let exporter = opentelemetry_otlp::MetricExporter::builder()
            .with_tonic()
            .with_endpoint("http://localhost:4317")
            .build()
            .unwrap();

        let reader = opentelemetry_sdk::metrics::PeriodicReader::builder(exporter, runtime::Tokio)
            .with_interval(Duration::from_secs(1))
            .build();

        let provider = opentelemetry_sdk::metrics::SdkMeterProvider::builder()
            .with_reader(reader)
            .build();

        opentelemetry::global::set_meter_provider(provider.clone());

        let meter = provider.meter("cubesat-sim");

        Self {
            battery_soc_pct: meter
                .f64_gauge("battery_soc_pct")
                .with_description("current battery percentage")
                .with_unit("%")
                .build(),
            battery_capacity: meter
                .f64_gauge("battery_capacity")
                .with_description("current battery capacity")
                .with_unit("%")
                .build(),
            solar_power: meter
                .f64_gauge("solar_power")
                .with_description("current solar power received")
                .build(),
            mission_state: meter.f64_gauge("mission_state").build(),
            operational_mode: meter.f64_gauge("operational_mode").build(),
        }
    }

    pub fn record(&self, satellite: &Satellite) {
        // atualiza os valores a cada tick
        self.battery_soc_pct
            .record(satellite.energy_model.battery_level.into(), &[]);
        self.battery_capacity
            .record(satellite.energy_model.max_capacity.into(), &[]);
        self.solar_power
            .record(satellite.energy_model.solar_panel_output.into(), &[]);

        let state_value = match satellite.mission_state {
            MissionState::Leop => 0.0,
            MissionState::Commissioning => 1.0,
            MissionState::Nominal => 2.0,
            MissionState::LowPower => 3.0,
            MissionState::SafeMode => 4.0,
            MissionState::EndOfLife => 5.0,
        };

        self.mission_state.record(state_value, &[]);

        let mode_value = match satellite.operational_mode {
            OperationalMode::Charging => 0.0,
            OperationalMode::Idle => 1.0,
            OperationalMode::Transmitting => 2.0,
        };

        self.operational_mode.record(mode_value, &[]);
    }
}
