use crate::curves::{Curve, Linear};
use crate::line::Line;
use std::cmp::max;
use std::f64::consts::{PI, FRAC_PI_2};
use std::time::SystemTime;

pub struct Sofa<E: Curve> {
    pub curve: E,
    pub best_lines: Vec<Line>,
    pub curve_interval: Interval,
    pub dx: f64,
}

impl<E: Curve> Sofa<E> {
    pub fn new(curve: E, dx: f64) -> Self {
        let nullstelle = curve.nullstelle(Interval { lower: 0.0, upper: 1.0 }, 0.00000001f64);
        let curve_interval = Interval{ lower: -nullstelle, upper: nullstelle };

        let corridor = Corridor::new(&curve, curve_interval.lower, &curve_interval);
        let mut best_lines = corridor.get_lines(dx);

        Self{
            curve,
            best_lines,
            curve_interval,
            dx,
        }
    }

    pub fn calculate_area(&self) -> f64 {
        let lines = &self.best_lines;
        let mut total_area = 0.0;
        for i in 0..max(lines.len() as isize - 1, 0) as usize {
            total_area += lines[i].area_between(&lines[i + 1], self.dx);
        }
        total_area * 2f64
    }

    pub fn cutout(&mut self, r_dx: f64) {
        let start = SystemTime::now();
        let mut x = self.curve_interval.lower;
        while x < self.curve_interval.upper {
            let corridor = Corridor::new(&self.curve, x, &self.curve_interval);
            self.cutout_corridor(&corridor);
            x += r_dx;
        }
        let time_taken = SystemTime::now().duration_since(start).unwrap();
        println!("Calculation of Sofa took: {:?}", time_taken);
    }

    fn cutout_corridor(&mut self, corridor: &Corridor) {
        let cutout = &corridor.get_lines(self.dx);
        if cutout.len() != self.best_lines.len() {
            println!("Cutout: {}, best: {}", cutout.len(), self.best_lines.len());
            panic!("Different amount of lines in cutout and best");
            return;
        }

        for i in 0..self.best_lines.len() {
            self.best_lines[i].shrink(&cutout[i]);
        }
    }
}

pub struct Corridor {
    pub aussenwand: Envelope,
    pub innenwand: Envelope,
    pub intervals: [Interval; 4],
}

impl Corridor {
    pub fn new<E: Curve>(curve: &E, x: f64, curve_interval: &Interval ) -> Self {
        let theta = Self::calculate_theta(curve, x, 0.00000001f64);
        let m = Point { x, y: theta.tan() * x, };

        let tan_left = (theta / 2f64).tan();
        let tan_right = ((theta + PI) / 2f64).tan();

        let wi_left = Linear::new(tan_left, -tan_left * m.x + m.y);
        let wi_right = Linear::new(tan_right, -tan_right * m.x + m.y);

        let wa_left = Linear::new(
            tan_left,
            tan_left * m.x + tan_left * (theta / 2f64).sin() + m.y + (theta / 2f64).cos(),
        );
        let wa_right = Linear::new(
            tan_right,
            -tan_right * m.x - tan_right * ((theta + PI) / 2f64).sin() + m.y
                - ((theta + PI) / 2f64).cos(),
        );

        let nullstelle_innen = -(wi_right.c) / wi_right.m;
        let nullstelle_aussen = -(wa_right.c) / wa_right.m;

        let m_innen = m.clone();
        let m_aussen = Point{ x: m.x + theta.cos(), y: m.y + theta.sin() };

        let i1 = Interval{ lower: 0.0, upper: m_innen.x };
        let i2 = Interval{ lower: m_innen.x, upper: nullstelle_innen };
        let i3 = Interval{ lower: nullstelle_innen, upper: m_aussen.x };
        let i4 = Interval { lower: m_aussen.x, upper: 1.0 + curve_interval.upper};

        Self{
            aussenwand: Envelope { left: wa_left, right: wa_right },
            innenwand: Envelope { left: wi_left, right: wi_right },
            intervals: [i1, i2, i3, i4]
        }

    }

    pub fn calculate_theta<E: Curve>(curve: &E, mut x: f64, toleranz: f64) -> f64 {
        let curve_y = curve.f(x);
        //println!("Searching for y={}", curve_y);

        let is_negative = x.is_sign_negative();
        x = x.abs();

        let r = |x: f64, theta: f64| theta.tan() * x;

        let mut interval = Interval {
            lower: 0.0,
            upper: FRAC_PI_2,
        };

        while interval.lower <= interval.upper {
            let theta = (interval.lower + interval.upper) / 2f64;

            let r_of_x = r(x, theta);

            if (interval.upper-interval.lower).abs() <= toleranz {
                return if !is_negative { theta } else { PI - theta }
            } else if curve_y < r_of_x {
                interval.upper = theta;
            } else {
                interval.lower = theta;
            }
        }
        panic!("Failed to calculate theta")
    }

    fn get_lines(&self, dx: f64) -> Vec<Line> {
        let mut lines = vec![];

        let x_aussen = self.intervals[2].upper;
        let nullstelle = self.intervals[1].upper;

        let i1 = self.intervals[0];
        let mut x = i1.lower;
        while x < i1.upper {
            lines.push(Line {
                min: self.innenwand.left.f(x),
                max: self.aussenwand.left.f(x),
            });
            x += dx;
        }

        if x_aussen >= nullstelle {
            let i2 = self.intervals[1];
            while x < i2.upper {
                lines.push(Line {
                    min: self.innenwand.right.f(x),
                    max: self.aussenwand.left.f(x),
                });
                x += dx;
            }

            let i3 = self.intervals[2];
            while x < i3.upper {
                lines.push(Line {
                    min: 0.0,
                    max: self.aussenwand.left.f(x),
                });
                x += dx;
            }
        }
        else { //Wenn Mittelpunkt über Nullstelle herausragt
            let i2 = self.intervals[1];
            while x < x_aussen {
                lines.push(Line {
                    min: self.innenwand.right.f(x),
                    max: self.aussenwand.left.f(x),
                });
                x += dx;
            }

            let i3 = self.intervals[2];
            while x < nullstelle {
                lines.push(Line {
                    min: self.innenwand.right.f(x),
                    max: self.aussenwand.right.f(x),
                });
                x += dx;
            }
        }

        let i4 = self.intervals[3];
        while x < i4.upper {
            lines.push(Line {
                min: 0.0,
                max: self.aussenwand.right.f(x),
            });
            x += dx;
        }

        return lines;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

pub struct Envelope {
    pub left: Linear,
    pub right: Linear,
}

#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub lower: f64,
    pub upper: f64,
}
