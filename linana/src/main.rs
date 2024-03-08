use io_functions::io_functions::*;

use crate::linana_functions::linana_functions::*;
mod io_functions;
mod linana_functions;
mod spn_functions;
fn main() {
    let (pairs, output_path) = read_args();
    let _ = write_key_to_file(&output_path, &lin_attack(&pairs)); 
}
