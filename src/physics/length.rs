pub struct Length {
    meters: f32,
}

impl Length {
    pub const fn from_meters(meters: f32) -> Self {
        Self { meters }
    }

    pub const fn as_meters(&self) -> f32 {
        self.meters
    }

    pub fn as_cm(&self) -> f32 {
        100.0 * self.meters
    }
}
