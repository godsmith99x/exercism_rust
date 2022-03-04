use std::fmt;

const HOURS_IN_DAY: i32 = 24;
const MINS_IN_HOUR: i32 = 60;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let m: i32 = minutes.rem_euclid(MINS_IN_HOUR);
        let m_roll_over: f64 = minutes as f64 / MINS_IN_HOUR as f64;

        let h: i32 = (hours as f64 + m_roll_over).rem_euclid(HOURS_IN_DAY as f64) as i32;

        Self {
            hours: h,
            minutes: m,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
}
