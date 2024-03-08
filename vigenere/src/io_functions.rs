pub mod io_functions {
    use std::{env, fs::{self, File}, io::Write};

    pub fn read_args() -> (String, String, String, bool) {
        let args: Vec<String> = env::args().collect();
        if !(args.len() == 4 || args.len() == 5) {
            panic!("Inkorrekte Angabe der Kommandozeilenargumente")
        }
        let mode: bool = args[args.len()-1].parse().unwrap();
        let input_path = args[1].clone();
        let input = read_from_file(&input_path);
        let mut key = String::new();
        let output_path = args[args.len()-2].clone();
        if mode {
            let key_path = args[2].clone();
            key = read_from_file(&key_path);
        }

        (input, key, output_path, mode)
    }

    pub fn read_from_file(file_path: &str) -> String {
        let file_contents = match fs::read_to_string(file_path) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("Fehler beim Lesen der Inputdatei: {}", err);
                String::new()
            }
        };
        file_contents
    }

    pub fn write_output_to_file(file_path: &str, key: &String, output: &String) -> Result<(), std::io::Error> {
        let mut file = File::create(file_path)?; 
        if key.is_empty() {
            file.write_all(output.as_bytes())?;
        }
        else {
            writeln!(file, "{}", key)?;
            writeln!(file, "{}", output)?;
        }
    
        Ok(())
    }
}