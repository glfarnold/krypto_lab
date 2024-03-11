pub mod io_functions {
    use std::env;
    use std::fmt::write;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    use std::io::Write;


    pub fn hex_input(path: &str) -> Vec<Vec<u8>> {
        let file = File::open(path).expect("Pfad existiert nicht");
        let contents = io::BufReader::new(file);
    
        let mut vec: Vec<Vec<u8>> = Vec::new();
        for line in contents.lines() {
            match line {
                Ok(line_contents) => {
                    let vals: Vec<&str> = line_contents.split_whitespace().collect();
                    vec.push(vals.iter().map(|&val| u8::from_str_radix(val, 16).unwrap()).collect());
                },
                Err(_) => eprintln!("Inhalt der Datei kann nicht gelesen werden")
            };
        }
        vec
    }

    pub fn print_beauty(a: &Vec<Vec<u8>>) {
        for i in 0..4 {
            for j in 0..4 {
                print!(" {:02X} ", a[i][j]);
            }
        }
        println!("");
    } 

    pub fn write_output_to_file(file_path: &str, output: &Vec<u8>) -> Result<(), std::io::Error> {
        let mut file = File::create(file_path)?;
        for i in 0..output.len() {
            write!(file, "{:02X} ", output[i])?;
        }
    
        Ok(())
    }

    pub fn write_key_to_file(key_path: &str, keys: &Vec<Vec<u8>>) -> Result<(), std::io::Error> {
        let mut file = File::create(key_path)?;
        
        for key in keys {
            for val in key {
                write!(file, "{:02X} ", val)?;
            }
            writeln!(file, "")?;
        }
    
        Ok(())
    }

    pub fn read_args() -> (String, String, String, String, bool, String){
        let args: Vec<String> = env::args().collect();
        if !(args.len() == 6 || args.len() == 7) {
            panic!("Programm ausführen mit: cargo run -- [Betriebsmodus] [input path] [key path] [output path] mode [IV path (optional)]")
        }
        let betriebsmodus: String = args[1].to_ascii_lowercase();
        let input_path: String = args[2].clone();
        let key_path: String = args[3].clone();
        let output_path: String = args[4].clone();
        let mode: bool = args[5].parse().unwrap();
        let mut iv_path: String = String::new();
        if args.len() == 7 {
            iv_path = args[6].clone();
        }
        (betriebsmodus, input_path, key_path, output_path, mode, iv_path)

    }
}