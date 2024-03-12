use std::env;

use dhk::dhk::{dhk, generate_p};

mod dhk;

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_bits: u64 = args[1].parse().unwrap();
    
    let p = generate_p(&num_bits);
    dhk(&p);
}
