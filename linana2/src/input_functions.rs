pub mod input_funtions {
    use std::{env, fs::File, io::{self, BufRead}};

    // Funktion, die Hexadezimalzahlen aus einem Input File einliest und als Vec<Vec<u8>> zurückgibt 
    // Jede Zeile ist ein Vec<u8>, wobei jedes von einem Leerzeichen getrennte Zeichen als u8 interpretiert wird
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

    // Einlesen der über die Kommandozeile angegebenen Pfade und auslesen der an diesen Pfaden liegenden Inputs
    pub fn read_args() -> (Vec<u8>, Vec<Vec<u8>>) {
        let args: Vec<String> = env::args().collect();
        let sbox_path = args[1].clone();
        let approx_path = args[2].clone();

        let sbox = hex_input(&sbox_path)[0].clone();
        let approx = hex_input(&approx_path);
        (sbox, approx)
    }
}