use rust_ecc::finite_field::FieldElement;

#[cfg(test)]
pub mod finite_field_tests {
    use super::*;

    #[test]
    fn test_add() {
        let el1 = FieldElement::new(10, 11);
        let el2 = FieldElement::new(3, 11);

        assert_eq!((el1 + el2).value, 2);
    }

    #[test]
    fn test_mul() {
        let el1 = FieldElement::new(10, 11);
        let el2 = FieldElement::new(3, 11);

        assert_eq!((el1 * el2).value, 8);
    }

    #[test]
    fn test_coef() {
        let el1 = FieldElement::new(3, 11);

        assert_eq!((el1 * 4).value, 1);
    }

    #[test]
    fn test_pow() {
        let el1 = FieldElement::new(4, 11);

        assert_eq!(el1.pow(2).value, 5);
    }

    #[test]
    fn test_div() {
        let el1 = FieldElement::new(10, 11);
        let el2 = FieldElement::new(3, 11);

        assert_eq!((el1 / el2).value, 7)
    }
}
