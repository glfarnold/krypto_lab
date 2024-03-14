use num_bigint::BigUint;

use crate::sha_functions::sha_functions::{bigint_to_vec_le, state_to_string, string_to_state, vec_le_to_bigint};

mod sha_functions;
fn main() {
    let m = BigUint::from(256u32);
    let tmp = bigint_to_vec_le(&m);
    println!("{:?}", vec_le_to_bigint(&tmp));
}
