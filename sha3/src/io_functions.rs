pub mod io_functions {
    use std::{env, fs::File, io::{self, BufRead, Read, Write}};

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

    fn read_num(path: &str) -> io::Result<BigUint> {
        let mut file = File::open(path)?;
    
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let content = content.trim();
    
        let number = BigUint::parse_bytes(content.as_bytes(), 16)
            .ok_or(io::Error::new(io::ErrorKind::InvalidData, "Eingabe kann nicht gelesen werden"))?;
    
        Ok(number)
    }
    

    pub fn read_args() -> (BigUint, String) {
        let args: Vec<String> = env::args().collect();
        let input_path = args[1].clone();
        let output_path = args[2].clone();

        let input = read_num(&input_path).unwrap();
        (input, output_path)
    }

    pub fn print_state(state: &Vec<Vec<Vec<u8>>>) {
        for i in 0..5 {
            for j in 0..5 {
                for k in 0..8 {
                    let mut num = 0;
                    let byte = state[j][i][8*k..8*k+8].to_vec();
                    let mut byte_array = [0;1];
                    for (i, &bit) in byte.iter().enumerate() {
                        byte_array[0] |= bit << i;
                    }
                    byte_array[0] = byte_array[0].reverse_bits();
                    num = u8::from_be_bytes(byte_array);
                    print!(" {:02X} ", num);
                }
                println!("");
            }
        }
    }

    pub fn write_output_to_file(path: &str, output: &Vec<u8>) -> Result<(), std::io::Error> {
        let mut file = File::create(path)?; 
        for i in 0..output.len() {
            write!(file, "{:02X}", output[i])?;
        }
        writeln!(file, "")?;
    
        Ok(())
    }
}