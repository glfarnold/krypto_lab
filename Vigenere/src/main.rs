use crate::vigenere::vigenere::*;
mod vigenere;

fn main() {
    // let a: String = read_from_file("src/Kryptotext_TAG.txt");
    // let a_raw = remove_chars(&a.chars().collect());
    // let a_raw_string = a_raw.iter().collect();
    // let n = find_key_length(&a);
    // let key = get_key(&a_raw_string, &n);
    // println!("{:?}", decrypt(&a, &key.chars().map(|c| c as u8).collect()));

    let b = read_from_file("src/Klartext_1.txt");
    let k = String::from("ABCDEFG");
    let _ = write_output_to_file("./output.txt", 
                            &encrypt(&b, &k.chars().map(|c| c as u8).collect()));

    
    let c = read_from_file("./output.txt");
    let a_raw = remove_chars(&c.chars().collect());
    let a_raw_string = a_raw.iter().collect();
    let n = find_key_length(&c);
    let key = get_key(&a_raw_string, &n);


    let _ = write_output_to_file("./output2.txt", 
    &decrypt(&c, &k.chars().map(|c| c as u8).collect()));
}
