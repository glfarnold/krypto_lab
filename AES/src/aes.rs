pub mod aes {
    use std::vec;
    use crate::io_functions::io_functions::{hex_input, print_beauty};

    // transforms a Array of u8 into a column major 2D Array, pt is 128 Bits Block
    pub fn ini_aes(pt: &Vec<u8>) -> Vec<Vec<u8>> {
        assert_eq!(pt.len(), 16);
        let mut vec = pt.clone();
        let mut matrix: Vec<Vec<u8>> = vec![vec![0;4];4];
        for i in  0..matrix.len() {
            for j in 0..matrix[0].len() {
                matrix[i][j] = vec[4*i + j];
            }
        }
        matrix
    } 

    // Ergebnis der AES Verschlüsselung ist ein 2D Array, mit dieser Funktion 
    // Umwandlung zu 16 Byte Block
    pub fn finish_aes(ct: &Vec<Vec<u8>>) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        for i in 0..4 {
            for j in 0..4 {
                result.push(ct[i][j]);
            }
        }
        result
    }

    pub fn add_round_key(state: &Vec<Vec<u8>>, key: &Vec<u8>) -> Vec<Vec<u8>> {
        let mut result = state.clone();
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = result[i][j] ^ key[4*i + j];
            }
        }
        result
    }

    fn get_round_key(round: &usize, keys: &Vec<Vec<u8>>) -> Vec<u8> {
        keys[*round].clone()
    }

    pub fn mix_columns(state: &Vec<Vec<u8>>, encrypt: &bool) -> Vec<Vec<u8>> {
        let mut result = state.clone();
        // encrypt
        if *encrypt {
            let help_matrix: Vec<Vec<u8>> = vec![vec![2,3,1,1], vec![1,2,3,1], vec![1,1,2,3], vec![3,1,1,2]];
            for i in 0..4 {
                let col = result[i].clone();
                for j in 0..4 {
                    let mut vals: Vec<u8> = vec![0;4];
                    for k in 0..4 {
                        vals[k] = russian_mult(&help_matrix[j][k],&col[k]);
                    }
                    result[i][j] = vals[0] ^ vals[1] ^ vals[2] ^ vals[3]; 
                }
            }
        }
        // decrypt
        else {
            let help_matrix: Vec<Vec<u8>> = vec![vec![0xE, 0xB, 0xD, 0x9], vec![0x9, 0xE, 0xB, 0xD], vec![0xD, 0x9, 0xE, 0xB],
                                                 vec![0xB, 0xD, 0x9, 0xE]];
            for i in 0..4 {
                let col = result[i].clone();
                for j in 0..4 {
                    let mut vals: Vec<u8> = vec![0;4];
                    for k in 0..4 {
                        vals[k] = russian_mult(&help_matrix[j][k],&col[k]);
                    }
                    result[i][j] = vals[0] ^ vals[1] ^ vals[2] ^ vals[3]; 
                }
            }
        }
        result
    }

    fn xtime(a: &u8) -> u8 {
        let mut t = a << 1;
        if a >> 7 == 1 {
            t ^= 0x1b;
        }
        t
    }

    fn russian_mult(a: &u8, b: &u8) -> u8 {
        let mut a = a.clone();
        let mut b = b.clone();

        let mut result: u8 = 0;
        for _ in 0..8 {
            if (a & 0x01) != 0 {
                result ^= b;
            }
            b = xtime(&b);
            a >>= 1;
        }
        result
    }

    pub fn shift_rows(state: &Vec<Vec<u8>>, encrypt: &bool) -> Vec<Vec<u8>> {
        let mut result = state.clone();
        for i in 0..4 {
            let mut row = Vec::new();
            for j in 0..4 {
                row.push(result[j][i])
            }
            if *encrypt {
                row.rotate_left(i);
            }
            else {
                row.rotate_right(i);
            }
            for j in 0..4 {
                result[j][i] = row[j];
            }
        }
        result
    }

    pub fn sub_bytes(state: &Vec<Vec<u8>>, sbox: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut result = state.clone();
        for i in 0..4 {
            for j in 0..4 {
                let row = result[i][j] >> 4;
                let col = result[i][j] & 0x0f;
                result[i][j] = sbox[row as usize][col as usize];
            }
        }
        result
    }

    pub fn get_sbox(encrypt: &bool) -> Vec<Vec<u8>> {
        if *encrypt {
            let path = "data/SBox.txt";
            return hex_input(path);
        }
        else {
            let path = "data/SBoxInvers.txt";
            return hex_input(path);
        }
    }

    pub fn aes(state: &Vec<Vec<u8>>, keys: &Vec<Vec<u8>>, encrypt: &bool) -> Vec<Vec<u8>> {
        if *encrypt {
            aes_encrypt(state, keys)
        }
        else {
            aes_decrypt(state, keys)
        }
    }

    fn aes_encrypt(block: &Vec<Vec<u8>>, keys: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut state = block.clone();
        let sbox = get_sbox(&true);
        let mut round_key = get_round_key(&0, keys);
        state = add_round_key(&state, &round_key);
        for i in 1..10 {
            state = sub_bytes(&state, &sbox);
            state = shift_rows(&state, &true);
            state = mix_columns(&state, &true);
            round_key = get_round_key(&i, keys);
            state = add_round_key(&state, &round_key);       
        }
        state = sub_bytes(&state, &sbox);
        state = shift_rows(&state, &true);
        round_key = get_round_key(&10, keys);
        state = add_round_key(&state, &round_key);


        state
    }

    fn aes_decrypt(state: &Vec<Vec<u8>>, keys: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut state = state.clone();
        let sbox = get_sbox(&false);
        let mut round_key = get_round_key(&10, keys);
        state = add_round_key(&state, &round_key);
        state = shift_rows(&state, &false);
        state = sub_bytes(&state, &sbox);
        for i in 1..10 {
            round_key = get_round_key(&(10-i), keys);
            state = add_round_key(&state, &round_key);
            state = mix_columns(&state, &false);
            state = shift_rows(&state, &false);
            state = sub_bytes(&state, &sbox);
        }
        round_key = get_round_key(&0, keys);
        state = add_round_key(&state, &round_key);

        state
    }
}