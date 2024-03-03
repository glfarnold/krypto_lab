pub mod vigenere {
    use std::ascii;

    pub fn encrypt(plaintext: &String, keys: &Vec<u8>) -> String {
        let n = keys.len();
        let mut counter = 0;
        let mut ascii_vec: Vec<u8> = plaintext.chars().map(|c| c as u8).collect(); 
        for i in 0..ascii_vec.len() {
            if 65 < ascii_vec[i] && ascii_vec[i] < 91 {
                ascii_vec[i] -= 65;
                ascii_vec[i] += keys[counter % n];
                counter += 1;
                ascii_vec[i] += 65;
            }
        }
        let crypttext: String = ascii_vec.into_iter().map(|c| c as char).collect();
        crypttext
    }

    fn get_coincidence_index(crypttext: &String) -> f64 {
        let n = crypttext.len() as i32;
        let mut h: Vec<i32> = vec![0;26];
        let mut chars: Vec<u8> = crypttext.chars().map(|c| c as u8).collect();
        for i in 0..chars.len() {
            if 65 <= chars[i] && chars[i] < 91 {
                chars[i] -= 65;
                h[chars[i] as usize] += 1;
            }
        }

        let mut ic: f64 = 0.0;
        for i in 0..h.len() {
            ic += (h[i] * (h[i]-1)) as f64 /(n * (n-1)) as f64;
        }
        ic
    }

    pub fn divide_into_blocks(crypttext: &String, i: &i32) -> Vec<String> {
        let chars: Vec<char> = remove_chars(&crypttext.chars().collect());
        let mut texts: Vec<String> = vec![String::new(); *i as usize];
        for j in 0..chars.len() {
            texts[(j as i32 % i) as usize].push(chars[j]);
        }
        texts
    }

    // Vigenere Chiffre ignoriert alle Characters, die keine lateinischen Großbuchstaben sind
    // deswegen werden diese mit dieser Funktion entfernt, bevor der Crypttext in i Blöcke geteilt wird
    pub fn remove_chars(chars: &Vec<char>) -> Vec<char> {
        let mut tmp = chars.clone();
        tmp.retain(|&c| 65 <= c as u8 && (c as u8) < 91);
        tmp
    }

    pub fn find_key_length(crypttext: &String) -> i32 {
        let mut ic = vec![0.0;100];
        for i in 1..3 {
            let texts = divide_into_blocks(crypttext, &i);
            let mut ic_tmp = 0.0;
            for j in 0..i {
                ic_tmp += get_coincidence_index(&texts[j as usize]);
            }
            ic[i as usize] = ic_tmp / i as f64;
        }
        ic.sort_by(|a,b| a.partial_cmp(b).unwrap());
        let biggest_ic: Vec<f64> = ic.iter().rev().take(3).cloned().collect();
        println!("{:?}", biggest_ic);
        0
    }
}