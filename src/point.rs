// file for the point on a eq: y^2 = x^3 + ax + b
use crate::finite_field::FieldElement;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: Option<FieldElement>,
    pub y: Option<FieldElement>,
    pub a: FieldElement,
    pub b: FieldElement,
}

impl Point {
    pub fn new(
        x: Option<FieldElement>,
        y: Option<FieldElement>,
        a: FieldElement,
        b: FieldElement,
    ) -> Point {
        match (&x, &y) {
            (Some(el_x), Some(el_y)) => {
                if el_y.pow(2) != el_x.pow(3) + (a * el_x) + b {
                    panic!(
                        "This point ({}, {}) is not on the curve",
                        el_x.value, el_y.value
                    );
                }
                Point {
                    x: x,
                    y: y,
                    a: a,
                    b: b,
                }
            }
            (None, None) => Point {
                x: None,
                y: None,
                a: a,
                b: b,
            },
            _ => panic!("Unknown values"),
        }
    }
}

impl ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        if self.a != rhs.a || self.b != rhs.b {
            panic!("These two points are of different graphs");
        }

        match (self.x, self.y, rhs.x, rhs.y) {
            (None, None, _, _) => rhs,
            (_, _, None, None) => self,
            (Some(x1), Some(_y1), Some(x2), Some(_y2))
                if x1.value == x2.value && _y1.value != _y2.value =>
            {
                Point::new(None, None, self.a, self.b)
            }
            (Some(x1), Some(y1), Some(x2), Some(y2)) if x1.value != x2.value => {
                let slope = (y2 - y1) / (x2 - x1);
                let x3 = slope.pow(2) - x1 - x2;
                let y3 = (slope * (x1 - x3)) - y1;

                Point::new(Some(x3), Some(y3), self.a, self.b)
            }
            (Some(x1), Some(y1), Some(x2), Some(_y2)) if x1.value == x2.value && y1.value == 0 => {
                Point::new(None, None, self.a, self.b)
            }
            (Some(x1), Some(y1), Some(x2), Some(y2))
                if x1.value == x2.value && y1.value == y2.value =>
            {
                let slope = (x2.pow(2) * 3 + self.a) / (y1 * 2);
                let x3 = slope.pow(2) - x1 * 2;
                let y3 = slope * (x1 - x3) - y1;

                Point::new(Some(x3), Some(y3), self.a, self.b)
            }
            _ => panic!("unhandled case"),
        }
    }
}

impl ops::Mul<i128> for Point {
    type Output = Point;

    fn mul(self, rhs: i128) -> Self::Output {
        let mut coef = rhs;
        let mut current = self;
        let mut result = Point::new(None, None, self.a, self.b);

        while coef > 0 {
            if coef & 1 == 1 {
                result = result + current;
            }

            current = current + current;
            coef >>= 1;
        }

        result
    }
}
