use crate::linana::linana::*;
use crate::spn::spn::*;
use std::env;
use std::fs::File;
use std::io::Write;

mod spn;
mod linana;

fn main() {
    // let mut args: Vec<String> = env::args().collect();
    // assert!(args.len() > 2);
    // args.remove(0);

    // let plaintext_string: Vec<String> = args.iter().take(args.len()-2).cloned().collect();
    // let key_string = args[args.len()-2].clone();
    // let opath = args[args.len()-1].clone();

    // let keys = get_key(key_string);
    // let plaintext = get_plaintext(plaintext_string);

    // let mut crypttext: Vec<u16> = Vec::new();
    // for i in 0..plaintext.len() {
    //     crypttext.push(spn(&plaintext[i], &keys, &SBOX));
    // }


    // if let Err(err) = write_output_to_file(&opath, &crypttext) {
    //     eprintln!("Error writing to file: {}", err);
    //     std::process::exit(1);
    // } else {
    //     println!("Output written to file: {}", opath);
    // }
    let key = 0xaaaa;
    let keys = vec![key; 5];
    
    let m: Vec<(u16, u16)> = (0..8000).map(|i| (i, spn(&i, &keys, &SBOX))).collect();
    println!("{:?}", my_function(m));
}




