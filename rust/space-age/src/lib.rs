// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

impl Duration {
    pub fn as_secs(&self) -> u64 {
        self.seconds
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

const EARTH_YEAR_SECONDS: f64 = 365.25 * 24. * 3600.;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        // Mercury orbital period is approximately 0.2408467 in Earth years
        d.as_secs() as f64 / (0.2408467 * EARTH_YEAR_SECONDS)
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        // Venus orbital period is approximately 0.61519726 in Earth years
        d.as_secs() as f64 / (0.61519726 * EARTH_YEAR_SECONDS)
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        // Earth year is approximately 365 days
        d.as_secs() as f64 / (1. * EARTH_YEAR_SECONDS)
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        d.as_secs() as f64 / (1.8808158 * EARTH_YEAR_SECONDS)
    }
}
impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        d.as_secs() as f64 / (11.862615 * EARTH_YEAR_SECONDS)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        d.as_secs() as f64 / (29.447498 * EARTH_YEAR_SECONDS)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        d.as_secs() as f64 / (84.016846 * EARTH_YEAR_SECONDS)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        d.as_secs() as f64 / (164.79132 * EARTH_YEAR_SECONDS)
    }
}
