#![allow(unused)]

use crate::curves::{Curve, Ellipse, Polynomial, HyperbolicCosine, Secant};
use crate::sofa::{Corridor, Interval, Sofa};
use std::alloc::System;
use std::time::SystemTime;
use rand::Rng;

mod curves;
mod line;
mod sofa;

fn main() {
    let mut coeff = vec![0.6484759236132569, -0.9243198264108918, 0.4554706187391541, -2.78770057052311, 1.013198698934143, -5.6469242652612825, -9.29635020433, -14.29877124747172, -16.709676163756598, -17.75436667251395, -18.00787183104618, -18.67690149320014, -18.788623534478944, -20.03434314370808, -20.85292016861066, -21.38432373690313, -21.710739738344035, -21.88676770450287, -21.49400627509508, -21.70304822195018, -21.603776256522583];
    loop {
        let mut area = 0.0;
        let mut i = 0;
        while i < coeff.len() {
            let random_step_size = rand::thread_rng().gen_range(0.0001..0.01);
            // Positive direction
            coeff[i] += random_step_size;
            let curve = Polynomial::new(coeff.to_vec());
            // let mut curve = Ellipse::new(coeff.to_vec());
            // let curve = HyperbolicCosine::new(coeff.to_vec());
            // let curve = Secant::new(coeff.to_vec());
            let mut sofa = Sofa::new(curve, 0.00185);
            sofa.cutout(0.0002);
            let new_plus_area = sofa.calculate_area();

            // Negative direction
            coeff[i] -= 2.0 * random_step_size;
            let curve = Polynomial::new(coeff.to_vec());
            // curve = Ellipse::new(coeff.to_vec());
            // let curve   = HyperbolicCosine::new(coeff.to_vec());
            // let curve = Secant::new(coeff.to_vec());
            let mut sofa = Sofa::new(curve, 0.00185);
            sofa.cutout(0.0002);
            let new_minus_area = sofa.calculate_area();
            // Area is bigger in plus-direction

            if new_plus_area > new_minus_area {
                coeff[i] += 2.0 * random_step_size;    // Reset coefficients because that is the "bigger" direction
                area = new_plus_area;
            } else {    // Area is bigger in minus-direction
                // Here we don't need to reset the coefficient
                area = new_minus_area;
            }
            i += 1;
        }
        println!("{:?}; {}", coeff, area);
    }
}