use core::num;
use std::{ops::BitXor, fs};
mod functions;
use crate::functions::functions::*;
use crate::keygen::keygen::generate_key;
use ndarray::{linalg::Dot, *};
use numrs::matrix::{Matrix, self};
pub mod betriebsmodi;
// pub mod functions;
// // use crate::functions::*;
use crate::betriebsmodi::betriebsmodi::*;
// extern crate numrs;
// use numrs::matrix;
// use numrs::matrix::Matrix;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
mod keygen;


fn main_aes() {
    let mut plaintext: Vec<Vec<u8>> = hex_input("/home/garnold/Uni/Lab/Betriebsmodi/src/pt.txt");
    let keys = hex_input("/home/garnold/Uni/Lab/Betriebsmodi/src/key.txt");

    let mut tmp: Vec<u8> = Vec::new();
    for i in 0..plaintext.len() {
        tmp.extend(plaintext[i].clone());
    }
    let mut matrix = ini_aes(&mut tmp);
    

    let c = code_block_chiffre(&mut vec![0;16], 16, &mut tmp, &keys);
}









// pub fn replace_col(matrix: &mut Array2<u8>, column: &Array1<u8>, index: usize) -> Array2<u8> {
//     let mut new_matrix: Array2<u8> = matrix.clone();
//     let mut new_col = new_matrix.index_axis_mut(Axis(1), index);
//     new_col.assign(column.as_slice().unwrap());
//     new_matrix
// }

pub fn replace_row(matrix: &mut Array2<u8>, row: &Array1<u8>, index: usize) -> Array2<u8> {
    let new_matrix: Array2<u8> = matrix.clone();
    let mut new_row = matrix.index_axis_mut(Axis(0), index);
    new_row.assign(row);
    new_matrix
}

 

pub fn add_round_key(matrix: &mut Array2<u8>, keys: &Vec<Vec<u8>>, round: usize) -> Array2<u8> {
    let mut encrypted_matrix: Array2<u8> = Array2::default((4,4));
    let key = &keys[round];
    let mut help_matrix = matrix.clone();

    for (i, mut col) in matrix.columns_mut().into_iter().enumerate() {
        for j in 0..4 {
            col[j] ^= key[i + 4 * j];
        }
    }
    encrypted_matrix
}
