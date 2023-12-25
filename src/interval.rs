#[derive(Debug, Copy, Clone)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

// Constants
pub const EMPTY_INTERVAL: Interval = Interval {
    min: f64::INFINITY,
    max: f64::NEG_INFINITY,
};

pub const UNIVERSE_INTERVAL: Interval = Interval {
    min: f64::NEG_INFINITY,
    max: f64::INFINITY,
};
