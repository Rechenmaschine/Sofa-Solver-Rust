#![allow(unused)]

use crate::curves::{Curve, Polynomial, Ellipse};
use crate::sofa::{Corridor, Sofa, Interval};
use std::alloc::System;
use std::time::SystemTime;

mod curves;
mod line;
mod sofa;

fn main() {
    let curve = Polynomial::new(vec![0.6449, -0.8044, -0.1755,-0.9479,-2.9591,-4.4303,-6.4655,-8.5473]);
    //let curve = Ellipse::new(vec![0.6301, 0.6450]);

    let mut sofa = Sofa::new(curve, 0.00001);
    sofa.cutout(0.0001);
    println!("{}", sofa.calculate_area());
}
