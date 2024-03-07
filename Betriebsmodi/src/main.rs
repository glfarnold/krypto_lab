use core::num;
use std::{ops::BitXor, fs};
mod functions;
use crate::functions::functions::*;
use crate::io_files::io_files::*;
use crate::keygen::keygen::generate_key;
use ndarray::{linalg::Dot, *};
// use numrs::matrix::{Matrix, self};
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
mod io_files; 


fn main() {
    let mut plaintext: Vec<Vec<u8>> = hex_input("/home/garnold/Uni/Lab/Betriebsmodi/src/pt.txt");
    // let keys = key_input("/home/garnold/Uni/Lab/Betriebsmodi/src/key.txt");
    // let _= write_output_to_file("/home/garnold/Uni/Lab/Betriebsmodi/src/key.txt", &keys);
    // println!("{:?}", keys);
}












 

