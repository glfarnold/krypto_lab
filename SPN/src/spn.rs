pub mod spn {
    use core::panic;
    use std::env;
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read, Write};

    pub const SBOX: [u8; 16] = [0xe, 0x4, 0xd, 0x1, 
                        0x2, 0xf, 0xb, 0x8, 
                        0x3, 0xa, 0x6, 0xc, 
                        0x5, 0x9, 0x0, 0x7 ];

    pub fn spn(plaintext: &u16, keys: &Vec<u16>, sbox: &[u8; 16]) -> u16 {
        let mut text: u16 = *plaintext;
        for i in 0..3 {
            text = add_key(&text, &keys[i]);
            text = substitute(&text, sbox);
            text = permutate(&text);
        }
        text = add_key(&text, &keys[3]);
        text = substitute(&text, sbox);
        text = add_key(&text, &keys[4]);

        text
    }

    pub fn add_key(val: &u16, key: &u16) -> u16 {
        val ^ key
    }

    pub fn substitute(plaintext: &u16, sbox: &[u8; 16]) -> u16 {
        let mut vec: Vec<u8> = Vec::new();
        for i in 0..4 {
            let tmp: u8 = (plaintext >> 4*i) as u8 & 0xf;
            vec.push(tmp);
        }

        let mut result: u16 = 0;
        for (i, val) in vec.iter().rev().enumerate() {
            let mut tmp = sbox[(*val) as usize] as u16;
            tmp = tmp << 4*(3-i);
            result ^= tmp;
        }
        result
    }

    pub fn permutate(plaintext: &u16) -> u16 {
        let p: Vec<usize> = vec![1,5,9,13,2,6,10,14,3,7,11,15,4,8,12,16];
        let mut vec: Vec<u16> = vec![0;16];
        let mut result: u16 = 0;

        for i in 0..p.len() {
            let tmp = plaintext >> (15-i) & 0x1;
            vec[p[i]-1] = tmp;
        }
        for i in 0..vec.len() {
            let mut tmp = vec[i];
            tmp <<= 15-i;
            result ^= tmp;
        }

        result
    }
}