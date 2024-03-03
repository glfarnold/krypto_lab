use crate::add::add::*;
mod add;

fn main() {
    let input_path = read_args().0;
    let output_path = read_args().2;
    let key = read_args().1;
    let mode = read_args().3;

    let _ = match mode {
        0 => {
            let plaintext = read_from_file(&input_path);
            let crypttext = encrypt(&plaintext, &key);
            let _ = write_output_to_file(&output_path, &crypttext);
        },
        1 => {
            let crypttext = read_from_file(&input_path);
            let plaintext = decrypt_with_key(&crypttext, &key);
            let _ = write_output_to_file(&output_path, &plaintext);
        }, 
        2 => {
            let crypttext = read_from_file(&input_path);
            let plaintext = decrypt(&crypttext).0;
            let k = decrypt(&crypttext).1.to_string();
            let output = format!("{}\n{}", k, plaintext);
            let _ = write_output_to_file(&output_path, &output);
        }
        _ => {
            panic!("Modus als Zahl zwischen 0 und 2 angeben")
        },
    };

    
}
