pub mod vigenere {
    use num::integer::gcd;

    // Verschlüsselung des Vigenereverfahrens 
    // Zeichen, die keine Großbuchstaben sind, werden übergangen
    pub fn encrypt(plaintext: &String, keys: &Vec<u8>) -> String {
        let n = keys.len();
        let k: Vec<u8> = keys.iter().map(|c| c - 65).collect();
        let mut counter = 0;
        let mut ascii_vec: Vec<u8> = plaintext.chars().map(|c| c as u8).collect(); 
        for i in 0..ascii_vec.len() {
            if 65 <= ascii_vec[i] && ascii_vec[i] < 91 {
                ascii_vec[i] -= 65;
                ascii_vec[i] = (ascii_vec[i] + k[counter % n]) % 26;
                counter += 1;
                ascii_vec[i] += 65;
            }
        }
        let crypttext: String = ascii_vec.into_iter().map(|c| c as char).collect();
        crypttext
    }


    // Entschlüsselung mit bekanntem Schlüssel 
    // Zeichen, die keine Großbuchstaben sind, werden übergangen
    pub fn decrypt(crypttext: &String, key: &Vec<u8>) -> String {
        let n = key.len();
        let k: Vec<u8> = key.iter().map(|c| c - 65).collect();
        let mut counter = 0;
        let mut ascii_vec: Vec<u8> = crypttext.chars().map(|c| c as u8).collect(); 
        for i in 0..ascii_vec.len() {
            if 65 <= ascii_vec[i] && ascii_vec[i] < 91 {
                ascii_vec[i] -= 65;
                let mut tmp = (ascii_vec[i] as i32 - k[counter % n] as i32) % 26;
                if tmp < 0 {
                    tmp = 26 + tmp;
                }
                ascii_vec[i] = tmp as u8;
                counter += 1;
                ascii_vec[i] += 65;
            }
        }
        let crypttext: String = ascii_vec.into_iter().map(|c| c as char).collect();
        crypttext
    }

    // Häufigkeitsanalyse bei bekannter Blocklänge n
    // der Kryptotext wird in n Blöcke unterteilt, in diesen wird jeweils Häufigkeitsanalyse angewendet
    pub fn get_key(crypttext: &String, n: &i32) -> String {
        let texts: Vec<String> = divide_into_blocks(crypttext, n);
        let mut key: String = String::new();
        for i in 0..texts.len() {
            let max = find_max(&texts[i]) as u8;
            let mut key_char: i32 = (max as i32 - 4) % 26;
            if key_char < 0 {
                key_char = 26 + key_char;
            }
            key.push((key_char as u8 + 65) as char );
        }
        key
    }

    // Funktion, um den am häufigsten vorkommenden Buchstaben in einem String zu finden
    fn find_max(crypttext: &String) -> usize {
        let mut counter: Vec<i32> = vec![0;26];
        let mut chars: Vec<u8> = crypttext.chars().map(|c| c as u8).collect();
        for i in 0..chars.len() {
            if 65 <= chars[i] && chars[i] < 91 {
                chars[i] -= 65;
                counter[chars[i] as usize] += 1;
            }
        }
        
        let max_index = match counter.iter().enumerate().max_by_key(|(_, &v)| v) {
            Some((index, _)) => index,
            None => {  
                panic!("Crypttext ist leer");
            }
        };
        max_index
    }

    // Funktion, die den Koinzidenzindex für einen gegebenen String berechnet
    pub fn get_coincidence_index(crypttext: &String) -> f64 {
        let mut h: Vec<i32> = vec![0;26];
        let tmp: Vec<char> = remove_chars(&crypttext.chars().collect());
        let mut chars: Vec<u8> = tmp.iter().map(|&c| c as u8).collect();
        let n = chars.len() as i32;
        for i in 0..chars.len() {
            if 65 <= chars[i] && chars[i] < 91 {
                chars[i] -= 65;
                h[chars[i] as usize] += 1;
            }
        }
        let mut ic: f64 = 0.0;
        for i in 0..h.len() {
            ic = ic + (h[i] * (h[i]-1)) as f64 ;
        }
        ic /(n * (n-1)) as f64
    }

    // Funktion, die einen String in i Blöcke unterteilt
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

    // für jede Blockanzahl wird der Mittelwert der ICs der Blöcke berechnet 
    // von den drei Blockanzahlen mit den größten Werten wird der größte gemeinsame Teiler bestimmt, dies ist 
    // mit hoher Wahrscheinlichkeit die Länge des Schlüssels
    pub fn find_key_length(crypttext: &String) -> i32 {
        let mut ic = vec![0.0;100];
        for i in 1..101 {
            let texts = divide_into_blocks(crypttext, &i);
            let mut ic_tmp = 0.0;
            for j in 0..i {
                ic_tmp += get_coincidence_index(&texts[j as usize]);
            }
            ic[i as usize -1] = ic_tmp / i as f64;
        }
        let mut indexed_ic: Vec<(usize, &f64)> = ic.iter().enumerate().collect();
        indexed_ic.sort_by(|a,b| a.1.partial_cmp(b.1).unwrap());
        let sorted_indices: Vec<i32> = indexed_ic.iter().map(|&(index, _)| (index + 1) as i32).collect();
        // take the 3 indices of the lengths with the biggest ICs and calculate their greatest common divisor
        let indices_of_greatest_ics: Vec<i32> = sorted_indices[sorted_indices.len()-4..sorted_indices.len()].to_vec();
        get_gcd(&indices_of_greatest_ics)
    }

    // Funktion, die den größten gemeinsamen Teiler der Einträge eines Vektors zurückgibt
    fn get_gcd(vec: &Vec<i32>) -> i32 {
        let mut result = vec[0];
        for &val in &vec[1..] {
            result = gcd(result, val);
        } 
        result
    }
}