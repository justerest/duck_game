use std::time::Duration;

use super::velocity::Velocity;

pub struct Acceleration {
    meters_on_second_on_second: f32,
}

impl Acceleration {
    pub const fn from_meters_on_second_on_second(val: f32) -> Self {
        Self {
            meters_on_second_on_second: val,
        }
    }

    pub const fn as_meters_on_second_on_second(&self) -> f32 {
        self.meters_on_second_on_second
    }
}

impl std::ops::Mul<Acceleration> for Duration {
    type Output = Velocity;

    fn mul(self, rhs: Acceleration) -> Self::Output {
        Velocity::from_meters_on_second(self.as_secs_f32() * rhs.meters_on_second_on_second)
    }
}

impl std::ops::Mul<Duration> for Acceleration {
    type Output = Velocity;

    fn mul(self, rhs: Duration) -> Self::Output {
        rhs * self
    }
}

impl std::ops::Mul<Acceleration> for f32 {
    type Output = Acceleration;

    fn mul(self, rhs: Acceleration) -> Self::Output {
        Acceleration::from_meters_on_second_on_second(self * rhs.meters_on_second_on_second)
    }
}
