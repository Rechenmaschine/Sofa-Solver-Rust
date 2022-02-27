use std::f64::consts::{PI};
use crate::search_space::{X_RANGE};

pub trait Curve: Clone {
    //fn get_coefficients(&self) -> Vec<f64>;

    fn get_theta(&self, x: f64, y: f64) -> f64{
        let r = (x*x + y*y).sqrt();
        return (x/r).acos();
    }

    fn relevant_interval(&self, x_step_size: f64) -> Vec<[f64; 3]> {
        let mut relevant_points = vec![];
        let mut x = 0f64;
        loop {
            let y = self.f(x);

            if !(y >= 0f64) {
                break;
            }
            if !(x <= X_RANGE) {
                return vec![];
            }
            let theta = self.get_theta(x, y);
            relevant_points.push([theta, x, y]);
            relevant_points.push([PI-theta, -x, y]);
            x += x_step_size;
        }
        return relevant_points;
    }

    fn f(&self, x: f64) -> f64;
}

#[derive(Clone)]
pub struct Ellipse{
    pub coefficients:Vec<f64>
}
impl Curve for Ellipse {
    fn f(&self, x: f64) -> f64 {
        (self.coefficients[1] / self.coefficients[0]) * (self.coefficients[0]*self.coefficients[0] - x*x).sqrt()
    }
}


#[derive(Clone)]
pub struct Polynomial{
    pub coefficients:Vec<f64>,
    degree: u32
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

