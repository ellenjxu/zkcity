use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::constants::RISTRETTO_BASEPOINT_POINT;
use rand::rngs::OsRng;

// struct Location<F: Field> {
//     state: Option<F>,
//     city: Option<F>,
//     radius: F // radius of detection
// }

fn main() {
    let a = Scalar::from(10u32); // random [1,50] state
    let b = Scalar::from(10u32);
    
    // fix group G of prime order q
    let mut rng = OsRng;

    // A computes (A1, A1, A2) = (g^x, g^y, g^(xy+a))
    let x = Scalar::random(&mut rng);
    let y = Scalar::random(&mut rng);

    let a0 = RISTRETTO_BASEPOINT_POINT * x;
    let a1 = RISTRETTO_BASEPOINT_POINT * y;
    let a2 = RISTRETTO_BASEPOINT_POINT * (x * y + a);

    // B computes (B1, B2) = (A1^r*g^s, (A2/g^b)^r*A0^s)
    let r = Scalar::random(&mut rng);
    let s = Scalar::random(&mut rng);

    let b1 = a1 * r + RISTRETTO_BASEPOINT_POINT * s;
    let b2 = (a2 - RISTRETTO_BASEPOINT_POINT * b) * r + a0 * s;

    // A checks if B1^x = B2
    let check = b1 * x == b2;
    println!("Matched: {}", check);
}
