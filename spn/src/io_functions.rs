pub mod io_functions {
    use std::{env, fs::File, io::{BufRead, BufReader, Write}};

    pub fn read_args() -> (Vec<u16>, Vec<u16>, String){
        let args: Vec<String> = env::args().collect();
        if !(args.len() == 4) {
            panic!("Programm ausführen mit cargo run -- [input path] [key path] [output path]")
        }
        let pt: Vec<u16> = read_file(args[1].as_str());
        let key: Vec<u16> = vec![read_file(args[2].as_str())[0]; 5];
        let output_path = args[3].clone();
        (pt, key, output_path)
    }

    pub fn read_file(path: &str) -> Vec<u16> {
        let file = File::open(path).expect("Pfad existiert nicht");
        let reader = BufReader::new(file);
        let mut hex_values = Vec::new();

        for line in reader.lines() {
            let line = line.expect("Zeile konnte nicht gelesen werden");
            if line.len() != 4 {
                panic!("Jede Zeile soll exakt 4 Hex digits enthalten");
            }
            let hex_value = u16::from_str_radix(&line, 16).expect("Hex Wert konnte nicht geparst werden");
            hex_values.push(hex_value);
        }

        hex_values
    }

    pub fn write_output_to_file(file_path: &str, crypttext: &[u16]) -> Result<(), std::io::Error> {
        let mut file = File::create(file_path)?;
        
        for ct in crypttext {
            writeln!(file, "{:04X}", ct)?;
        }
    
        Ok(())
    }
}