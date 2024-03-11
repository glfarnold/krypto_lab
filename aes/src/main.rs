use aes::aes::*;
use aes_keygen::aes_keygen::*;
use betriebsmodi::betriebsmodi::ecb::*;
use betriebsmodi::betriebsmodi::cbc::*;
use betriebsmodi::betriebsmodi::ofb::*;
use betriebsmodi::betriebsmodi::ctr::*;
use io_functions::io_functions::*;

mod aes;
mod io_functions;
mod betriebsmodi;
mod aes_keygen;

fn main() {
    let (betriebsmodus, input_path, key_path, output_path, mode, iv_path) = read_args();
    let pt = hex_input(&input_path)[0].clone();
    let key = convert_to_u32(&hex_input(&key_path)[0].clone());
    // Als erstes wird die Key Expansion durchgeführt
    let round_keys: Vec<Vec<u8>> = convert_to_u8(&generate_key(&key));
    let _ = write_key_to_file(&key_path, &round_keys);

    match betriebsmodus.as_str() {
        "ecb" => {
            let mut ct: Vec<u8> = Vec::new();
            if mode {
                ct = ecb_encrypt(&pt, &16, &round_keys);
            }
            else {
                ct = ecb_decrypt(&pt, &16, &round_keys);
            }
            let _ = write_output_to_file(&output_path, &ct);
        },
        "cbc" => {
            let mut ct: Vec<u8> = Vec::new();
            if mode {
                ct = cbc_encrypt(&pt, &16, &round_keys);
            }
            else {
                ct = cbc_decrypt(&pt, &16, &round_keys);
            }
            let _ = write_output_to_file(&output_path, &ct);
        },
        "ofb" => {
            let ini = hex_input(&iv_path)[0].clone();
            let ct = ofb_encrypt(&pt, &16, &round_keys, &ini);            
            let _ = write_output_to_file(&output_path, &ct);
        },
        "ctr" => {
            let mut ct: Vec<u8> = Vec::new();
            if mode {
                ct = cbc_encrypt(&pt, &16, &round_keys);
            }
            else {
                ct = cbc_decrypt(&pt, &16, &round_keys);
            }
            let _ = write_output_to_file(&output_path, &ct);

        },
        _ => {panic!("Folgende Betriebsmodi sind implementiert: ECB, CBC, OFB, CTR")}
    }
}


