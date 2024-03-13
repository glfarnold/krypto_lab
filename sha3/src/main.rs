use crate::sha_functions::sha_functions::string_to_state;

mod sha_functions;
fn main() {
    let s: [u8; 1600] = [5;1600];
    println!("{:?}", string_to_state(&s))
}
