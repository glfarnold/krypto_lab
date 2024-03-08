use crate::{io_functions::io_functions::*, vigenere::vigenere::*};
mod vigenere;
mod io_functions;

fn main() {
    let (input, key, output_path, mode) = read_args();
    match mode {
        // encrypt
        true => {
            let keys: Vec<u8> = key.chars().map(|c| c as u8).collect();
            let _ = write_output_to_file(&output_path, &String::from(""), &encrypt(&input, &keys));
        },
        // attack
        false => {
            let n = find_key_length(&input);
            let k = get_key(&remove_chars(&input.chars().collect()).iter().collect(), &n);
            let output = decrypt(&input, &k.chars().map(|c| c as u8).collect());
            let _ = write_output_to_file(&output_path, &k, &output);
        }
    };
}
