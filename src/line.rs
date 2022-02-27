#[derive(Copy, Clone, Debug)]
pub struct Line {
    pub min: f64,
    pub max: f64,
}

impl Line {
    pub fn length(&self) -> f64 {
        self.max - self.min
    }

    pub fn length_abs(&self) -> f64 {
        self.length().abs()
    }

    pub fn area_between(&self, rhs: &Self, dx: f64) -> f64 {
        ((self.length_abs() + rhs.length()) / 2f64) * dx
    }

    pub fn shrink(&mut self, rhs: &Self) {
        if self.min < rhs.min {
            self.min = rhs.min;
        }
        if self.max > rhs.max {
            self.max = rhs.max;
        }
    }
}
