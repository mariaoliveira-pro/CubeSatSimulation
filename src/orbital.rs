#[derive(Debug)]
pub enum OrbitalPhase {
    SunPhase,
    EclipsePhase,
}

#[derive(Debug)]
pub struct OrbitalModel {
    pub phase: OrbitalPhase,
    pub sun_time: u32,
    pub eclipse_time: u32,
    pub cycle_time: u32,
}


impl OrbitalModel {
    pub fn new(sun_time: u32, eclipse_time: u32) -> Self {
        Self {
            phase: OrbitalPhase::SunPhase,
            sun_time,
            eclipse_time,
            cycle_time: 0,
        }
    }

    pub fn update (&mut self) {
        self.cycle_time += 1;

        if self.cycle_time <= self.sun_time {
            self.phase = OrbitalPhase::SunPhase;
        } else {
            self.phase = OrbitalPhase::EclipsePhase;
        }

        if self.cycle_time >= self.sun_time + self.eclipse_time {
            self.cycle_time = 0;
        }
    }
}
