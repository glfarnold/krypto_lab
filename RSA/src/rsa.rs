pub mod rsa {
    use num_bigint::*;
    use num_traits::{Zero, One};

    pub fn square_and_multiply (x: &BigInt, m: &BigInt, n: &BigInt) -> BigInt {
        let mut y: BigInt = One::one();
        let mut val: BigInt = x.clone();
        let mut exp: BigInt = m.clone();

        loop {
            if exp == Zero::zero() {
                break;
            }
            if exp.clone() % 2 != Zero::zero() {
                y = y*&val % n;
            }
            val = (&val*&val) % n;
            exp = &exp / 2;
        }
        y
    }

    pub fn rsa_encrypt(x: &BigInt, public_key: &(BigInt, BigInt)) -> BigInt {
        square_and_multiply(x, &public_key.0, &public_key.1)
    }

    pub fn rsa_decrypt(y: &BigInt, private_key: &(BigInt, BigInt)) -> BigInt {
        square_and_multiply(y, &private_key.0, &private_key.1)
    }
}