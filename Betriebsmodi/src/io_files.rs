pub mod io_files {
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    use std::io::Write;


    pub fn hex_input(path: &str) -> Vec<Vec<u8>> {
        let file = File::open(path).expect("path does not exist");
        let contents = io::BufReader::new(file);
    
        let mut vec: Vec<Vec<u8>> = Vec::new();
        for line in contents.lines() {
            match line {
                Ok(line_contents) => {
                    let vals: Vec<&str> = line_contents.split_whitespace().collect();
                    vec.push(vals.iter().map(|&val| u8::from_str_radix(val, 16).unwrap()).collect());
                },
                Err(_) => eprintln!("could not read contents of file")
            };
        }
        vec
    }

    // // for the generation of the round key we need the 
    // pub fn key_input(path: &str) -> Vec<u32> {
    //     let file = File::open(path).expect("path does not exist");
    //     let contents = io::BufReader::new(file);    
    //     let mut vec: Vec<u32> = Vec::new();
    //     for line in contents.lines() {
    //         match line {
    //             Ok(line_contents) => {
    //                 let vals: Vec<&str> = line_contents.split_whitespace().collect();
    //                 let bytes: Vec<u8> = vals.iter().map(|&val| u8::from_str_radix(val, 16).unwrap()).collect();
    //                 for i in 0..4 {
    //                     let mut tmp: u32 = 0;
    //                     for j in 0..4 {
    //                         let (byte = bytes[4*i + j] as u32); 

    //                         tmp = (bytes[4*i + j] as u32) << (3-j)*8;
    //                     }
    //                     vec.push(tmp);
    //                 }
    //             },
    //             Err(_) => eprintln!("could not read contents of file")
    //         };
    //     }
    //     vec
    // } 

    pub fn write_output_to_file(file_path: &str, output: &Vec<u32>) -> Result<(), std::io::Error> {
        let mut file = File::create(file_path)?;
        
        writeln!(file, "{:?}", output)?;
    
        Ok(())
    }
}