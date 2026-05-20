    use crate::event_bus::{Event, EventBus};

    #[derive(Debug)]
    pub struct GroundStation {
        pub contact_active: bool,
        pub contact_start_tick: u32,
        pub contact_duration: u32,
        pub pass_every_n_orbits: u32,
    }


    impl GroundStation {
        pub fn new(pass_every_n_orbits: u32, contact_duration: u32) -> Self {
            Self {
                contact_active: false,
                contact_start_tick: 3, //manter 3 para testes, depois mudar para 20
                contact_duration: contact_duration, //recebe do satellite
                pass_every_n_orbits: pass_every_n_orbits, //também recebe do satellite
            }
        }

        pub fn update(&mut self, orbital_model: &crate::orbital::OrbitalModel, event_bus: &mut EventBus) {
            let contact_antes = self.contact_active;

            let orbit_passes = orbital_model.orbit_number % self.pass_every_n_orbits == 0;

            let contact_window_start = self.contact_start_tick;
            let contact_window_end = self.contact_start_tick + self.contact_duration;

            if orbit_passes && orbital_model.cycle_time >= contact_window_start && orbital_model.cycle_time < contact_window_end {
                self.contact_active = true;
                if contact_antes != self.contact_active {
                    event_bus.emit(Event::GroundStationContactStarted);
                }
            } else {
                self.contact_active = false;
                if contact_antes != self.contact_active {
                    event_bus.emit(Event::GroundStationContactEnded);
                }
            }
        }
    }
