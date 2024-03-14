pub mod io_functions {
    use std::{env, fs::File, io::{self, BufRead}};

    use num_bigint::BigUint;
    use num_traits::{zero, Zero};

    pub fn get_rc(path: &str) -> Vec<u64> {
        let file = File::open(path).expect("Pfad existiert nicht");
        let contents = io::BufReader::new(file);
        let mut result: Vec<u64> = Vec::new();
        
        for line in contents.lines() {
            match line {
                Ok(line_contents) =>  {
                    result.push(u64::from_str_radix(&line_contents[2..], 16).unwrap());
                }, 
                Err(_) => eprintln!("Inhalt der Datei konnte nicht gelesen werden")
            }
        }
        result
    }

    fn read_bigint(path: &str) -> BigUint {
        let file = File::open(path).expect("Pfad existiert nicht");
        let contents = io::BufReader::new(file);
        let mut result: BigUint = BigUint::zero();

        for line in contents.lines() {
            match line {
                Ok(mut line_contents) =>  {
                    if line_contents.len() % 2 != 0 {
                        line_contents.push('0');
                    }
                    let bytes = match hex::decode(line_contents) {
                        Ok(bytes) => bytes,
                        Err(_) => panic!("Zahl konnte nicht gelesen werden")
                    };
                    result = BigUint::from_bytes_le(&bytes);
                }, 
                Err(_) => eprintln!("Inhalt der Datei konnte nicht gelesen werden")
            }
        }
        result
    }

    pub fn read_args() -> (BigUint, String) {
        let args: Vec<String> = env::args().collect();
        let input_path = args[1].clone();
        let output_path = args[2].clone();

        let input = read_bigint(&input_path);
        (input, output_path)
    }

    pub fn print_state(state: &Vec<Vec<Vec<u8>>>) {
        for i in 0..5 {
            for j in 0..5 {
                for k in 0..8 {
                    for _ in 0..8 {
                        let byte = state[i][j][8*k..8*k+8].to_vec();
                        let mut byte_array = [0;1];
                        for (i, &bit) in byte.iter().enumerate() {
                            byte_array[0] |= bit << i;
                        }
                        let num: u8 = u8::from_le_bytes(byte_array);
                        print!(" {:02X} ", num);
                    }
                }
                println!("");
            }
        }
    }
}