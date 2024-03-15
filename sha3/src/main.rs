use std::ptr::read;

use io_functions::io_functions::{read_args, write_output_to_file};
use num_bigint::BigUint;
use num_traits::Zero;
use sha_functions::sha_functions::{divide_into_blocks, keccak, pad, read_as_le, rnd};

use crate::{io_functions::io_functions::get_rc, sha_functions::sha_functions::{bigint_to_vec_le, state_to_string, string_to_state}};

mod sha_functions;
mod io_functions;
fn main() {
    let (m, output_path) = read_args();
    let m_padded = pad(&m, &1152);
    let message = read_as_le(&m_padded);
    let blocks = divide_into_blocks(&message, &1152);
    let mut s = BigUint::zero();
    let mut result: Vec<u8> = Vec::new();
    for i in 0..blocks.len() {
        let mut p: BigUint = blocks[i].clone();
        p <<= 448;
        s = s.clone() ^ p;
        let state: Vec<u8> = bigint_to_vec_le(&s);   
        let round_constants = get_rc("data/roundConstants.txt");
        result = keccak(&state, &round_constants);  
    }
    result = result[0..224].to_vec();
    let mut output: Vec<u8> = Vec::new();
    for i in 0..224/8 {
        let mut num: u8 = 0;
        for j in 0..8 {
            let tmp = result[8*i+j];
            num <<= 1; num = num ^ tmp;
        }
        output.push(num);
    }
    
    let _ = write_output_to_file(&output_path, &output);
}
