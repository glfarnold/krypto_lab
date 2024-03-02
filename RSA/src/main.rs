mod rsa_key;
use num_bigint::BigInt;
use crate::rsa_key::rsa_key::*;

fn main() {
    let a = BigInt::from(693); 
    let b = BigInt::from(147);
    println!("{}", generate_prime());
    // println!("{}", miller_rabin(&BigInt::from(31)));
    // println!("{:?}", rsa_keygen(&generate_prime(),&generate_prime()));
}
