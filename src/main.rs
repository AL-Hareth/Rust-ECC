mod finite_field;
mod point;
use finite_field::FieldElement;
use point::Point;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let secret_str: &String = &args[1];

    const P: i64 = (1 << 61) - 1;
    const A: i64 = 0;
    const B: i64 = 8;

    let a = FieldElement::new(A as i128, P as i128);
    let b = FieldElement::new(B as i128, P as i128);
    let (gx, gy): (i64, i64) = (2, 4);
    let gfx = FieldElement::new(gx as i128, P as i128);
    let gfy = FieldElement::new(gy as i128, P as i128);

    let generator_point = Point::new(Some(gfx), Some(gfy), a, b);

    let secret: i128 = secret_str.parse().unwrap();

    let public = generator_point * secret;

    println!(
        "Private key: {} \nPublic key: {}",
        secret,
        public
            .x
            .unwrap_or(FieldElement::new(1 as i128, P as i128))
            .value
    );
}
