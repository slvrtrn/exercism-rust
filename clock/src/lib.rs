use std::fmt::Formatter;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock::_new(hours, minutes)
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::_new(self.hours, self.minutes + minutes)
    }

    fn _new(hours: i32, minutes: i32) -> Self {
        let hours_roll_over = minutes / 60;
        let mut normalized_minutes = minutes % 60;
        let mut normalized_hours = (hours + hours_roll_over) % 24;
        if normalized_minutes < 0 {
            normalized_hours -= 1;
            normalized_minutes = 60 + normalized_minutes;
        }
        if normalized_hours < 0 {
            normalized_hours = 24 + normalized_hours;
        }
        Clock { hours: normalized_hours, minutes: normalized_minutes }
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // https://docs.rs/pad/0.1.6/pad/#padding-in-the-stdlib
        write!(f, "{h:0>2}:{m:0>2}", h = self.hours, m = self.minutes)
    }
}
