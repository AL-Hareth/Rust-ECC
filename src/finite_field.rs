use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FieldElement {
    pub value: i128,
    pub prime: i128,
}

impl FieldElement {
    pub fn new(num: i128, prime: i128) -> FieldElement {
        if num >= prime {
            panic!("Field Element: {} cannot be greater than {}", num, prime);
        }

        FieldElement {
            value: num,
            prime: prime,
        }
    }

    // for exponentiation
    pub fn pow(self, mut exponent: i128) -> FieldElement {
        let mut base = self.value;
        if self.prime == 1 {
            return FieldElement::new(0, self.prime);
        }

        let mut result = 1;
        base = base.rem_euclid(self.prime);

        while exponent > 0 {
            // If exponent is odd, multiply base with result
            if exponent % 2 == 1 {
                result = (result * base).rem_euclid(self.prime);
            }

            // Exponent must be even now
            exponent >>= 1; // Equivalent to exponent = exponent / 2
            base = (base * base).rem_euclid(self.prime);
        }

        FieldElement::new(result, self.prime)
    }
}

impl ops::Add for FieldElement {
    type Output = FieldElement;

    fn add(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot add two numbers from different Fields");
        }

        let result_value = (self.value + rhs.value).rem_euclid(self.prime);
        FieldElement::new(result_value, self.prime)
    }
}

impl ops::Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot add two numbers from different Fields");
        }

        let result_value = (self.value - rhs.value).rem_euclid(self.prime);
        FieldElement::new(result_value, self.prime)
    }
}

impl ops::Mul<FieldElement> for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot add two numbers from different Fields");
        }

        let result_value = (self.value * rhs.value).rem_euclid(self.prime);
        FieldElement::new(result_value, self.prime)
    }
}
impl ops::Mul<&FieldElement> for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: &Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot add two numbers from different Fields");
        }

        let result_value = (self.value * rhs.value).rem_euclid(self.prime);
        FieldElement::new(result_value, self.prime)
    }
}

// For multiplying with a coeffecient
impl ops::Mul<i128> for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: i128) -> FieldElement {
        let result_value = (self.value * rhs).rem_euclid(self.prime);
        FieldElement::new(result_value, self.prime)
    }
}

impl ops::Mul<i128> for &FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: i128) -> FieldElement {
        let result_value = (self.value * rhs).rem_euclid(self.prime);
        FieldElement::new(result_value, self.prime)
    }
}

impl ops::Div for FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: Self) -> Self::Output {
        if self.prime != rhs.prime {
            panic!("Cannot add two numbers from different Fields");
        }
        // getting the inverse of divisor using Fermat's theorem
        let inv = rhs.pow(self.prime - 2);

        let result_value = (self * inv).value.rem_euclid(self.prime);
        FieldElement::new(result_value, self.prime)
    }
}
