use std::ptr::read;

use io_functions::io_functions::read_args;
use num_bigint::BigUint;
use num_traits::Zero;
use sha_functions::sha_functions::{divide_into_blocks, keccak, pad, read_as_le, rnd};

use crate::{io_functions::io_functions::get_rc, sha_functions::sha_functions::{bigint_to_vec_le, state_to_string, string_to_state, vec_le_to_bigint}};

mod sha_functions;
mod io_functions;
fn main() {
    let (m, output_path) = read_args();
    let m_padded = pad(&m, &1152);
    let message = read_as_le(&m_padded);
    let blocks = divide_into_blocks(&message, &1152);
    let mut s = BigUint::zero();
    for i in 0..blocks.len() {
        let mut p: BigUint = blocks[i].clone();
        p <<= 448;
        let tmp: BigUint = s.clone() ^ p;
        let state: Vec<u8> = bigint_to_vec_le(&tmp);   
        let round_constants = get_rc("data/roundConstants.txt");
        let result = keccak(&state, &round_constants);  
        s = vec_le_to_bigint(&result);
    }
}
