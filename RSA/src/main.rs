mod rsa_key;
mod rsa;
mod io_functions;
use std::process::Output;

use num_bigint::BigInt;
use crate::io_functions::io_functions::{read_user_input, write_output_to_file};
use crate::rsa_key::rsa_key::*;
use crate::rsa::rsa::*;

fn main() {
    let (m, key, output_path, mode) = read_user_input();

    if mode {
        let public_key = (key[0].clone(), key[1].clone());
        let output = rsa_encrypt(&m, &public_key);
        let _ = write_output_to_file(&output_path, &output);
    }
    else {
        let private_key = (key[0].clone(), key[1].clone());
        let output = rsa_decrypt(&m, &private_key);
        let _ = write_output_to_file(&output_path, &output);
    }
}
