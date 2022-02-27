use crate::sofa::Interval;

pub trait Curve: Clone {
    fn f(&self, x: f64) -> f64;

    fn nullstelle(&self, mut interval: Interval, toleranz: f64) ->f64{
        while interval.lower <= interval.upper {
            let x = (interval.lower + interval.upper) / 2f64;

            let f_of_x = self.f(x);

            if (interval.upper-interval.lower).abs() <= toleranz {
                return interval.lower;
            } else if f_of_x < 0.0 {
                interval.upper = x;
            } else /* f_of_x > 0.0 */{
                interval.lower = x;
            }
        }
        panic!("Failed to calculate theta")
    }
}

#[derive(Clone)]
pub struct Linear {
    pub m: f64,
    pub c: f64,
}

impl Linear {
    pub fn new(m: f64, c: f64) -> Self {
        Self { m, c }
    }
}
impl Curve for Linear {
    fn f(&self, x: f64) -> f64 {
        self.m * x + self.c
    }
}

#[derive(Clone)]
pub struct Ellipse {
    pub coefficients: Vec<f64>,
}

impl Ellipse {
    pub fn new(coefficients: Vec<f64>) -> Self {
        Self {
            coefficients,
        }
    }}
impl Curve for Ellipse {
    fn f(&self, x: f64) -> f64 {
        (self.coefficients[1] / self.coefficients[0])
            * (self.coefficients[0] * self.coefficients[0] - x * x).sqrt()
    }
    fn nullstelle(&self, interval: Interval, toleranz: f64) -> f64 {
        self.coefficients[0]
    }
}

#[derive(Clone)]
pub struct Polynomial {
    pub coefficients: Vec<f64>,
    degree: u32,
}
impl Polynomial {
    pub fn new(coefficients: Vec<f64>) -> Self {
        let degree = coefficients.len() as u32;
        Self {
            coefficients,
            degree,
        }
    }
}
impl Curve for Polynomial {
    fn f(&self, x: f64) -> f64 {
        let mut y = 0.0;
        for i in 0..self.degree as usize {
            y += self.coefficients[i] * x.powi((2 * i) as i32)
        }
        return y;
    }
}
