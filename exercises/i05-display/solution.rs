use std::fmt;

struct Time {
    hours: u32,
    minutes: u32,
}

impl Time {
    fn new(hours: u32, minutes: u32) -> Time {
        Time { hours, minutes }
    }

    fn add_minutes(&self, extra: u32) -> Time {
        let total = (self.hours * 60 + self.minutes + extra) % (24 * 60);
        Time::new(total / 60, total % 60)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

fn main() {
    let t = Time::new(9, 5);
    println!("now: {t}");
    println!("in 3 hours: {}", t.add_minutes(180));
}
