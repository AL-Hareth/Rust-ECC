use rust_ecc::finite_field::FieldElement;
use rust_ecc::point::Point;

#[cfg(test)]
pub mod point_tests {
    use super::*;

    #[test]
    fn point_addition_test() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);
        let p1 = Point::new(
            Some(FieldElement::new(192, prime)),
            Some(FieldElement::new(105, prime)),
            a,
            b,
        );
        let p2 = Point::new(
            Some(FieldElement::new(17, prime)),
            Some(FieldElement::new(56, prime)),
            a,
            b,
        );

        assert_eq!((p1 + p2).x.unwrap().value, 170);
    }

    #[test]
    fn point_scalar_mul_test() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);
        let p1 = Point::new(
            Some(FieldElement::new(47, prime)),
            Some(FieldElement::new(71, prime)),
            a,
            b,
        );

        assert_eq!((p1 * 5 as i64).x.unwrap().value, 126);
        assert_eq!((p1 * 5 as i64).y.unwrap().value, 96);
    }
}
