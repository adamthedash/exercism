use std::fmt::{Display, Formatter};

#[derive(Eq, Debug)]
pub struct Clock {
    h: i32,
    m: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_mins = (hours * 60 + minutes).rem_euclid(24 * 60);

        Self {
            h: total_mins / 60,
            m: total_mins % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(
            self.h,
            self.m + minutes,
        )
    }
}

impl PartialEq<Self> for Clock {
    fn eq(&self, other: &Self) -> bool {
        return self.h == other.h && self.m == other.m;
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.h, self.m)
    }
}
