pub mod betriebsmodi {
    use std::vec;

    pub fn divide_into_blocks(pt: &Vec<u8>, t: &i32) -> Vec<Vec<u8>> {
        let mut result: Vec<Vec<u8>> = Vec::new();
        let mut plaintext = pt.clone();
        while plaintext.len() as i32 % t != 0 {
            plaintext.push(0);
        }

        for i in 0..(pt.len() as i32 / t) + 1 {
            let start = i**t;
            let end = (i+1)**t;
            let vec = &plaintext[start as usize..end as usize];
            result.push(vec.to_vec());
        }
        result
    }

    pub fn add_blocks(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
        assert_eq!(a.len(), b.len());
        a.iter().zip(b.iter()).map(|(&x, &y)| x ^ y).collect()
    }

    pub mod cbc {
        use crate::aes::aes::*;
        use super::*;

        pub fn cbc_encrypt(pt: &Vec<u8>, t: &i32, keys: &Vec<Vec<u8>>) -> Vec<u8> {
            let blocks = divide_into_blocks(pt, t);
            let mut ini: Vec<u8> = vec![0;*t as usize];
            let mut result: Vec<u8> = Vec::new();

            for i in 0..blocks.len() {
                ini = add_blocks(&ini, &blocks[i]);
                // encrypt with aes
                let tmp = aes(&ini_aes(&ini), keys, &true);
                ini = finish_aes(&tmp);
                result.extend(ini.iter());
            }
            result
        }

        pub fn cbc_decrypt(ct: &Vec<u8>, t: &i32, keys: &Vec<Vec<u8>>) -> Vec<u8> {
            let blocks = divide_into_blocks(&ct, t);
            let mut ini: Vec<u8> = vec![0;*t as usize];
            let mut result: Vec<u8> = Vec::new();
            for i in 0..blocks.len() {
                // decrypt with aes
                let decrypted = finish_aes(&aes(&ini_aes(&blocks[i]), keys, &false));
                let tmp: Vec<u8> = add_blocks(&decrypted, &ini);
                ini = blocks[i].clone();
                result.extend(tmp.iter());
            }
            result
        }
    }
    
}