#![allow(unused)]

use crate::curves::{Curve, Ellipse, Polynomial, HyperbolicCosine, Secant};
use crate::sofa::{Corridor, Interval, Sofa};
use std::alloc::System;
use std::time::SystemTime;

mod curves;
mod line;
mod sofa;

fn main() {
    //let curve = Polynomial::new(vec![0.6449, -0.8044, -0.1755, -0.9479, -2.9591, -4.4303, -6.4655, -8.5473, ]);
    //let curve = Ellipse::new(vec![0.6301, 0.6450]);
    //let curve = HyperbolicCosine::new(vec![-0.7065, 2.2219, -1.9119, 1.2886]);
    let curve = Secant::new(vec![-0.4192, 1.7840, 1.0605]);

    let mut sofa = Sofa::new(curve, 0.00001);
    sofa.cutout(0.00001);
    println!("{}", sofa.calculate_area());
}
