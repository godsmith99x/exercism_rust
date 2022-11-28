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

pub trait Planet {
    const EARTH_ORBITAL_PERIOD: f64 = 31557600.0;
    const ORBITAL_RATIO: f64;

    fn orbital_period() -> f64 {
        Self::EARTH_ORBITAL_PERIOD * Self::ORBITAL_RATIO
    }

    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / Self::orbital_period()
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {const ORBITAL_RATIO: f64 =  0.2408467;}
impl Planet for Venus {const ORBITAL_RATIO: f64 =  0.61519726;}
impl Planet for Earth {const ORBITAL_RATIO: f64 =  1.0;}
impl Planet for Mars {const ORBITAL_RATIO: f64 = 1.8808158;}
impl Planet for Jupiter {const ORBITAL_RATIO: f64 =  11.862615;}
impl Planet for Saturn {const ORBITAL_RATIO: f64 =  29.447498;}
impl Planet for Uranus {const ORBITAL_RATIO: f64 =  84.016846;}
impl Planet for Neptune {const ORBITAL_RATIO: f64 =  164.79132;}
