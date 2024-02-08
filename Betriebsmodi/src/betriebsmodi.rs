pub mod betriebsmodi {
    use crate::functions::functions;

    pub fn code_block_chiffre(_ini: &mut Vec<u8>, t: u8, plaintext: &mut Vec<u8>, keys: &Vec<Vec<u8>>) -> Vec<u8> {
        let mut crypttext = Vec::new();
        let blocks: Vec<Vec<u8>> = divide_into_blocks(plaintext, t);
        let mut ini = vec![0;16];
    
        for i in 0..blocks.len() {
            let mut tmp: Vec<u8> = blocks[i].clone();
            tmp = tmp.iter().zip(ini.iter()).map(|(&x, &y)| x^y).collect();
            tmp = functions::aes(&mut tmp, keys);
            if i != 0 {
                ini = tmp.clone();
            }
            crypttext.extend(tmp);
        }
        crypttext
    }

    pub fn output_feedback(ini: &mut Vec<u8>, t: u8, plaintext: &mut Vec<u8>) -> Vec<u8> {
        let mut crypttext = Vec::new();
        let mut blocks = divide_into_blocks(plaintext, t);
        for i in 0..blocks.len() {
            let mut tmp: Vec<u8> = blocks[i].clone();
            // ini = encrypt(ini);
            tmp = tmp.iter().zip(ini.iter()).map(|(&x, &y)| x^y).collect();
            crypttext.extend(&tmp);
        }
        crypttext
    }

    pub fn counter(nonce: &mut Vec<u8>, t: u8, plaintext: &mut Vec<u8>) -> Vec<u8> {
        let mut crypttext = Vec::new();
        let mut blocks = divide_into_blocks(plaintext, t);
        for i in 0..blocks.len() {
            let mut tmp: Vec<u8> = blocks[i].clone();
            // nonce = encrypt(nonce);
            tmp = tmp.iter().zip(nonce.iter()).map(|(&x, &y)| x^y).collect();
            crypttext.extend(&tmp);
        }
        crypttext
    }

    pub fn divide_into_blocks(plaintext: &mut Vec<u8>, t: u8) -> Vec<Vec<u8>> {
        // pad mit 0
        while plaintext.len() as u8 % t != 0 {
            plaintext.push(0);
        }
    
        let mut blocks: Vec<Vec<u8>> = Vec::new();
        let num_blocks: u8 = plaintext.len() as u8 / t;
    
        // erstelle Matrix mit plaintextblöcken der Länge t
        for i in 0..num_blocks {
            let mut start = i * t;
            let mut end = (i + 1) * t;
            let mut tmp: Vec<u8> = Vec::new();
            for j in 0..t {
                let mut idx = start + j;
                tmp.push(plaintext[idx as usize])
            }
    
            blocks.push(tmp);
        }
        blocks
    }
}