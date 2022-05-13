#[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Velocity {
    meters_on_second: f32,
}

impl Velocity {
    pub const ZERO: Self = Self::from_meters_on_second(0.0);

    pub const fn from_meters_on_second(val: f32) -> Self {
        Self {
            meters_on_second: val,
        }
    }

    pub const fn as_meters_on_second(&self) -> f32 {
        self.meters_on_second
    }

    pub fn min(&self, max: Self) -> Self {
        Self::from_meters_on_second(self.meters_on_second.min(max.meters_on_second))
    }

    pub fn max(&self, max: Self) -> Self {
        Self::from_meters_on_second(self.meters_on_second.max(max.meters_on_second))
    }

    pub fn abs(&self) -> Self {
        Self::from_meters_on_second(self.meters_on_second.abs())
    }

    pub fn signum(&self) -> f32 {
        self.meters_on_second.signum()
    }
}

mod std_math_implementations {
    use std::time::Duration;

    use super::super::Length;
    use super::Velocity;

    impl std::ops::Mul<Velocity> for Duration {
        type Output = Length;

        fn mul(self, rhs: Velocity) -> Self::Output {
            Length::from_meters(self.as_secs_f32() * rhs.meters_on_second)
        }
    }

    impl std::ops::Mul<Duration> for Velocity {
        type Output = Length;

        fn mul(self, rhs: Duration) -> Self::Output {
            rhs * self
        }
    }

    impl std::ops::Add for Velocity {
        type Output = Velocity;

        fn add(self, rhs: Velocity) -> Self::Output {
            Velocity::from_meters_on_second(self.meters_on_second + rhs.meters_on_second)
        }
    }

    impl std::ops::AddAssign for Velocity {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }

    impl std::ops::Sub for Velocity {
        type Output = Velocity;

        fn sub(self, rhs: Velocity) -> Self::Output {
            Velocity::from_meters_on_second(self.meters_on_second - rhs.meters_on_second)
        }
    }

    impl std::ops::Neg for Velocity {
        type Output = Velocity;

        fn neg(self) -> Self::Output {
            Velocity::from_meters_on_second(-self.meters_on_second)
        }
    }

    impl std::ops::Mul<Velocity> for f32 {
        type Output = Velocity;

        fn mul(self, rhs: Velocity) -> Self::Output {
            Velocity::from_meters_on_second(self * rhs.meters_on_second)
        }
    }

    impl std::ops::Div<f32> for Velocity {
        type Output = Velocity;

        fn div(self, rhs: f32) -> Self::Output {
            Self::from_meters_on_second(self.meters_on_second / rhs)
        }
    }
}
