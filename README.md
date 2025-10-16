# Elliptic Curve Cryptography
This is [Elliptic Curve Cryptography](https://en.wikipedia.org/wiki/Elliptic-curve_cryptography) done from scratch in Rust.

If you are willing to see this app in action do the following:

## Running tests from the test/ directory
```bash
cargo test
```

## Running the actual private/public key generation
First go to main and set you parameters
The variables are:
`secret` for your secret key which is entered as command args
`gx, gy` for your generator points
`p` for your modulus
`a, b` for your elliptic curve coefficients (y^2 = x^3 + ax + b)

running the main:
```bash
cargo run <secret_key_integer>
```

Thanks for checking out my app!
