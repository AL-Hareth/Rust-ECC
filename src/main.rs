mod finite_field;
mod point;
use finite_field::FieldElement;
use point::Point;

fn main() {
    const P: i64 = (1 << 61) - 1;
    const A: i64 = 0;
    const B: i64 = 1;

    let a = FieldElement::new(A as i128, P as i128);
    let b = FieldElement::new(B as i128, P as i128);
    let (gx, gy): (i64, i64) = (2, 3);
    let gfx = FieldElement::new(gx as i128, P as i128);
    let gfy = FieldElement::new(gy as i128, P as i128);

    let generator_point = Point::new(Some(gfx), Some(gfy), a, b);

    let secret: i128 = 5001;

    let public = generator_point * secret;

    println!(
        "The public key derived from the private key {} is: {}",
        secret,
        public.x.unwrap().value
    );
}
