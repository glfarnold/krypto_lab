use num_bigint::BigUint;

use crate::sha_functions::sha_functions::{bigint_to_vec_le, state_to_string, string_to_state};

mod sha_functions;
fn main() {
    let m = BigUint::from(256u32);
    println!("{:?}", bigint_to_vec_le(&m));
}
