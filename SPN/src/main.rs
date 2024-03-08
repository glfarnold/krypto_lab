use io_functions::io_functions::{read_args, write_output_to_file};

use crate::linana::linana::*;
use crate::spn::spn::*;
use std::env;
use std::fs::File;
use std::io::Write;

mod spn;
mod linana;
mod io_functions;

fn main() {
    let (pt, keys, output_path) = read_args();
    let mut ct: Vec<u16> = Vec::new();
    for i in 0..pt.len() {
        ct.push(spn(&pt[i], &keys, &SBOX))
    }

    let _ = write_output_to_file(&output_path, &ct);
}




