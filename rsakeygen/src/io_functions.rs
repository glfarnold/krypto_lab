pub mod io_functions {
    use std::{env, fs::File};
    use std::io::{self, BufRead};
    use std::path::Path;
    use std::io::Write;

    use num_bigint::BigInt;

    pub fn write_output_to_file(file_path: &str, output: &Vec<BigInt>) -> Result<(), std::io::Error> {
        let mut file = File::create(file_path)?; 
        for i in 0..output.len() {
            writeln!(file, "{:?}", output[i])?;
        }
    
        Ok(())
    }

    pub fn read_user_input() -> (u64, String, String, String) {
        let mut args: Vec<String> = env::args().collect();
        let num_bits: u64 = args[1].clone().parse().unwrap();
        let priv_key_path = args[2].clone();
        let pub_key_path = args[3].clone();
        let primes_key_path = args[4].clone();

        (num_bits, priv_key_path, pub_key_path, primes_key_path)
    }
}