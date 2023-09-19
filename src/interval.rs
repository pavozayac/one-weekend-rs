pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min, max }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn universe() -> Self {
        Interval {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && self.max > x
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && self.max >= x
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}

impl Default for Interval {
    fn default() -> Self {
        Interval {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
}
