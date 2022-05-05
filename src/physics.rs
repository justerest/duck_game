pub use self::acceleration::*;
pub use self::length::*;
pub use self::speed::*;

pub const EARTH_G: Acceleration = Acceleration::from_meters_on_second_on_second(9.8);

mod acceleration {
    use std::time::Duration;

    use super::speed::Speed;

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
        type Output = Speed;

        fn mul(self, rhs: Acceleration) -> Self::Output {
            Speed::from_meters_on_second(self.as_secs_f32() * rhs.meters_on_second_on_second)
        }
    }

    impl std::ops::Mul<Duration> for Acceleration {
        type Output = Speed;

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
}

mod speed {
    #[derive(Default, Clone, Copy, PartialEq, PartialOrd)]
    pub struct Speed {
        meters_on_second: f32,
    }

    impl Speed {
        pub const ZERO: Self = Self::from_meters_on_second(0.0);

        pub const fn from_meters_on_second(val: f32) -> Self {
            Self {
                meters_on_second: val,
            }
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
        use super::Speed;

        impl std::ops::Mul<Speed> for Duration {
            type Output = Length;

            fn mul(self, rhs: Speed) -> Self::Output {
                Length::from_meters(self.as_secs_f32() * rhs.meters_on_second)
            }
        }

        impl std::ops::Mul<Duration> for Speed {
            type Output = Length;

            fn mul(self, rhs: Duration) -> Self::Output {
                rhs * self
            }
        }

        impl std::ops::Add for Speed {
            type Output = Speed;

            fn add(self, rhs: Speed) -> Self::Output {
                Speed::from_meters_on_second(self.meters_on_second + rhs.meters_on_second)
            }
        }

        impl std::ops::Sub for Speed {
            type Output = Speed;

            fn sub(self, rhs: Speed) -> Self::Output {
                Speed::from_meters_on_second(self.meters_on_second - rhs.meters_on_second)
            }
        }

        impl std::ops::Neg for Speed {
            type Output = Speed;

            fn neg(self) -> Self::Output {
                Speed::from_meters_on_second(-self.meters_on_second)
            }
        }

        impl std::ops::Mul<Speed> for f32 {
            type Output = Speed;

            fn mul(self, rhs: Speed) -> Self::Output {
                Speed::from_meters_on_second(self * rhs.meters_on_second)
            }
        }

        impl std::ops::Div<f32> for Speed {
            type Output = Speed;

            fn div(self, rhs: f32) -> Self::Output {
                Self::from_meters_on_second(self.meters_on_second / rhs)
            }
        }
    }
}

mod length {
    pub struct Length {
        meters: f32,
    }

    impl Length {
        pub const fn from_meters(meters: f32) -> Self {
            Self { meters }
        }

        pub fn as_meters(&self) -> f32 {
            self.meters
        }

        pub fn as_cm(&self) -> f32 {
            100. * self.meters
        }
    }
}
