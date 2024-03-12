pub mod io_functions {
    use std::{env, fs::File};
    use std::io::{self, BufRead, BufReader};
    use std::path::Path;
    use std::io::Write;

    use num_bigint::BigInt;

    pub fn write_output_to_file(file_path: &str, output: &BigInt) -> Result<(), std::io::Error> {
        let mut file = File::create(file_path)?; 
        writeln!(file, "{:?}", output)?;
    
        Ok(())
    }

    fn read_file(path: &str) -> Vec<BigInt> {
        let file = File::open(path).expect("Pfad existiert nicht");
        let contents = io::BufReader::new(file);
        let mut result: Vec<BigInt> = Vec::new();
    
        for line in contents.lines() {
            match line {
                Ok(line_contents) =>  {
                    result.push(line_contents.parse().unwrap());
                }, 
                Err(_) => eprintln!("Inhalt der Datei konnte nicht gelesen werden")
            }
        }
        result
    }

    pub fn read_user_input() -> (BigInt, Vec<BigInt>, String, bool) {
        let mut args: Vec<String> = env::args().collect();
        let message_path = args[1].clone();
        let key_path = args[2].clone();
        let output_path = args[3].clone();
        let mode: bool = args[4].clone().parse().unwrap();

        let message = read_file(&message_path)[0].clone();
        let key = read_file(&key_path);
        assert_eq!(key.len(), 2);

        (message, key, output_path, mode)
    }
}