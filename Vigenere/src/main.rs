use crate::vigenere::vigenere::*;
mod vigenere;

fn main() {
    let a: String = String::from("HELLO, WORLD!");
    println!("{:?}", find_key_length(&a));
}
