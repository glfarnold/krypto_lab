use input_functions::input_funtions::read_args;
use linana2::linana2::*;

use crate::linana2::linana2::quality;
mod input_functions;
mod linana2;

fn main() {
    let (sbox, approx) = read_args();
    let active_boxes = get_active_boxes(&approx);
    let mut qualities: Vec<f64> = Vec::new();
    for activ_box in active_boxes {
        qualities.push(bias(&(activ_box.0, activ_box.1), &sbox));
    }
    println!("{}", quality(&qualities));
}
