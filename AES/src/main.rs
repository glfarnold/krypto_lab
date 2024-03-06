use aes::aes::aes;
use io_functions::io_functions::{hex_input, print_beauty};

use crate::aes::aes::{add_round_key, get_sbox, ini_aes, mix_columns, shift_rows, sub_bytes};

mod aes;
mod io_functions;

fn main() {
    // let mut c = vec![0x00, 0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xa0, 0xb0, 0xc0, 0xd0, 0xe0, 0xf0];
    // let mut a: Vec<u8> = vec![0xdb, 0x13, 0x53, 0x45, 0xf2, 0x0a, 0x22, 0x5c, 0x01, 0x01, 0x01, 0x01, 0xC6, 0xc6, 0xc6, 0xc6];
    // let b = ini_aes(&c);
    // println!("{:?}", b);
    // let m = mix_columns(&b, &false);
    // let sbox = get_sbox(&true);
    // let m = sub_bytes(&b, &sbox);


    // for i in 0..4{
    //     for j in 0..4 {
    //         print!(" {:02X} ", m[j][i]);
    //     }
    // println!("");
// }
    let pt = hex_input("data/Beispiel_1_Klartext.txt")[0].clone();
    let keys = hex_input("data/Beispiel_key.txt");
    let ct = aes(&ini_aes(&pt), &keys, &true);
    print_beauty(&ct);
    
}


