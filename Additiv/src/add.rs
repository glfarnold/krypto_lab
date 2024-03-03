pub mod add {
    use std::fs;
    use std::{env, fs::File};
    use std::io::Write;

    pub fn encrypt(plaintext: &String, k: &i32) -> String {
        add_key(plaintext, k)
    }

    pub fn decrypt_with_key(plaintext: &String, k: &i32) -> String {
        add_key(plaintext, &-k)
    }

    fn add_key(plaintext: &String, k: &i32) -> String {
        let mut key: u8 = 0;
        if *k < 0 {
            key = (26 + k) as u8;
        }
        else {
            key = *k as u8;
        }
        let mut ascii_vec: Vec<u8> = plaintext.chars().map(|c| c as u8).collect();
        for i in 0..ascii_vec.len() {
            if 65 <= ascii_vec[i] && ascii_vec[i] < 91 {
                ascii_vec[i] -= 65;
                ascii_vec[i] = (ascii_vec[i] + key) % 26;
                ascii_vec[i] += 65;
            }
        }
        let crypttext: String = ascii_vec.into_iter().map(|c| c as char).collect();
        crypttext
    }

    pub fn decrypt(crypttext: &String) -> (String, i32) {
        let max_index: u8 = find_max(crypttext) as u8;
        let key: i32 = (max_index as i32 - 4) % 26;
        (add_key(crypttext, &(-key)), key)
    }

    fn find_max(crypttext: &String) -> usize {
        let mut counter: Vec<i32> = vec![0;26];
        let mut chars: Vec<u8> = crypttext.chars().map(|c| c as u8).collect();
        for i in 0..chars.len() {
            if 65 <= chars[i] && chars[i] < 91 {
                chars[i] -= 65;
                counter[chars[i] as usize] += 1;
            }
        }
        
        let max_index = match counter.iter().enumerate().max_by_key(|(_, &v)| v) {
            Some((index, _)) => index,
            None => {  
                panic!("Crypttext ist leer");
            }
        };
        max_index
    }

    pub fn read_args() -> (String, i32, String, i32){
        let args: Vec<String> = env::args().collect();
        if args.len() != 5 {
            panic!("Programm ausführen mit: cargo run -- [input path] key [output path] mode")
        }
        let input_path = args[1].clone();
        let key: i32 = args[2].parse().unwrap();
        if key < 0 || key > 25 {
            panic!("Key als Zahl zwischen 0 und 25 angeben");
        }
        let output_path = args[3].clone();
        let mode: i32 = args[4].parse().unwrap();
        (input_path, key, output_path, mode)
    }

    pub fn write_output_to_file(file_path: &str, output: &String) -> Result<(), std::io::Error> {
        let mut file = File::create(file_path)?; 
        file.write_all(output.as_bytes())?;
    
        Ok(())
    }

    pub fn read_from_file(file_path: &str) -> String {
        let file_contents = match fs::read_to_string(file_path) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("Error reading file: {}", err);
                String::new()
            }
        };
        file_contents
    }
}