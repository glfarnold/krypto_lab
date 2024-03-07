pub mod aes_old {
    use ndarray::*;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;
    use crate::io_functions::io_functions::*;


    pub fn add_round_key(matrix: &mut Array2<u8>, keys: &Vec<Vec<u8>>, round: usize) -> Array2<u8> {
        let mut encrypted_matrix: Array2<u8> = matrix.clone();
        let key = &keys[round];    
        for (i, mut col) in encrypted_matrix.columns_mut().into_iter().enumerate() {
            for j in 0..4 {
                col[j] ^= key[i + 4 * j];
            }
        }
        encrypted_matrix
    }

    // initializes Matrix for AES, parameter is one block of 128 bits, equal to 16 u8
    pub fn ini_aes(block: &mut Vec<u8>) -> Array2<u8> {
        let rows = 4;
        let cols = 4;
        assert_eq!(block.len(), rows * cols);
    
        let mut state = Array2::default((rows, cols));
        for row in 0..rows {
            for col in 0..cols {
                state[(row, col)] = block[row + 4 * col].clone();
            }
        }
        state
    }

    pub fn shift_rows(state: &mut Array2<u8>) -> Array2<u8> {
        let mut help_matrix =  state.clone();
        for (i, mut row) in help_matrix.rows_mut().into_iter().enumerate() {
            // copy row to tmp
            let mut tmp: Vec<u8> = Vec::new();
            for j in 0..4 {
                tmp.push(row[j]);
            }
            // perform the shift op
            for k in 0..4 {
                row[k] = tmp[(k + i) % 4];
            }
        }
        help_matrix
    }

    pub fn replace_col(matrix: &mut Array2<u8>, column: &Array1<u8>, index: usize) -> Array2<u8> {
        let new_matrix: Array2<u8> = matrix.clone();
        let mut new_col = matrix.index_axis_mut(Axis(1), index);
        new_col.assign(column);
        new_matrix
    }
    
    pub fn replace_row(matrix: &mut Array2<u8>, row: &Array1<u8>, index: usize) -> Array2<u8> {
        let new_matrix: Array2<u8> = matrix.clone();
        let mut new_row = matrix.index_axis_mut(Axis(0), index);
        new_row.assign(row);
        new_matrix
    }

    pub fn mix_columns(state: &mut Array2<u8>) -> Array2<u8> {
        let a = vec![[2, 3, 1, 1], [1, 2, 3, 1], [1, 1, 2, 3], [3, 1, 1, 2]];
        let mut tmp: u8 = 0;
        let mut help_matrix = state.clone();

        // iterate over cols 
        for (i, mut col) in help_matrix.columns_mut().into_iter().enumerate() {
            let mut b: Vec<u8> = Vec::new();
            // iterate over entries of col 
            for j in 0..4 {
                let x: Vec<u8> = a[j].to_vec();
                // dot product
                for k in 0..4 {
                    let val = russian_mult(&col[k], &x[k]);
                    tmp ^= val;
                }
                b.push(tmp);
                tmp = 0;
            }
            for m in 0..4 {
                col[m] = b[m];
            }
        }
        help_matrix
    }

    pub fn xtime(a: &u8) -> u8 {
        let mut t = a << 1;
        if a.leading_ones() != 0 {
            t ^= 0x1b;
        }
        t
    }

    pub fn russian_mult(a: &u8, b: &u8) -> u8 {
        let mut a = a.clone();
        let mut b = b.clone();

        let mut result: u8 = 0;
        for counter in 0..8 {
            if (a & 0x01) != 0 {
                result ^= b;
            }
            b = xtime(&b);
            a >>= 1;
        }
        result
    }

    pub fn sub_bytes(matrix: &Array2<u8>, sub_matrix: &Vec<Vec<u8>>) -> Array2<u8> {
        let mut help_matrix = matrix.clone();
        for (_i, mut col) in help_matrix.columns_mut().into_iter().enumerate() {
            for j in 0..4 {
                let m: u8 = col[j] >> 4;
                let n: u8 = col[j] & 0x0f;
                col[j] = sub_matrix[m as usize][n as usize];
            }
        }
        help_matrix
    }

    // if a true encrypt, if false decrypt
    pub fn get_sub_matrix(a: bool) -> Vec<Vec<u8>> {
        if a {
            hex_input("/home/garnold/Uni/Lab/Betriebsmodi/src/SBox.txt")
        } else {
            hex_input("./SBoxinvers.txt")
        }
    }

    pub fn aes(block: &mut Vec<u8>, keys: &Vec<Vec<u8>>) -> Vec<u8> {
        let mut crypttext: Vec<u8> = Vec::new();
        let rounds = keys.len();
        let mut matrix: Array2<u8> = ini_aes(block);
        matrix = add_round_key(&mut matrix, keys, 0);
        println!("{:?}", matrix);
        for round in 1..rounds-1 {
            matrix = sub_bytes(&matrix, &get_sub_matrix(true));
            println!("{:?}", matrix);
            matrix = shift_rows(&mut matrix);
            println!("{:?}", matrix);
            matrix = mix_columns(&mut matrix);
            println!("{:?}", matrix);
            matrix = add_round_key(&mut matrix, keys, round);
            println!("{:?}", matrix);
        }
        matrix = sub_bytes(&matrix, &get_sub_matrix(true));
        println!("{:?}", matrix);
        matrix = shift_rows(&mut matrix);
        println!("{:?}", matrix);
        matrix = add_round_key(&mut matrix, keys, rounds-1);
        println!("{:?}", matrix);
    
        // transform matrix back into vector as it is the return type
        for col in matrix.columns() {
            let tmp = col.to_vec();
            crypttext.extend(tmp);
        }
        crypttext
    }    
}