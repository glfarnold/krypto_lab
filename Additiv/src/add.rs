pub mod add {
    pub fn encrypt(plaintext: &String, k: &u8) -> String {
        let mut ascii_vec: Vec<u8> = plaintext.chars().map(|c| c as u8).collect();
        for i in 0..ascii_vec.len() {
            if 65 <= ascii_vec[i] && ascii_vec[i] < 91 {
                ascii_vec[i] -= 65;
                ascii_vec[i] = (ascii_vec[i] + k) % 26;
                ascii_vec[i] += 65;
            }
        }
        let crypttext: String = ascii_vec.into_iter().map(|c| c as char).collect();
        crypttext
    }

    pub fn decrypt(crypttext: &String) -> String {
        let max_index: u8 = find_max(crypttext) as u8;
        let key: u8 = (max_index - 4) % 26;
        let mut ascii_vec: Vec<u8> = crypttext.chars().map(|c| c as u8).collect();
        for i in 0..ascii_vec.len() {
            if 65 <= ascii_vec[i] && ascii_vec[i] < 91 {
                ascii_vec[i] -= 65;
                ascii_vec[i] = (ascii_vec[i] - key) % 26;
                ascii_vec[i] += 65;
            }
        }
        let crypttext: String = ascii_vec.into_iter().map(|c| c as char).collect();
        crypttext
    }

    fn find_max(crypttext: &String) -> usize {
        let mut counter: Vec<i32> = vec![0;26];
        let mut chars: Vec<u8> = crypttext.chars().map(|c| c as u8).collect();
        for i in 0..chars.len() {
            if 65 <= chars[i] && chars[i] < 91 {
                chars[i] -= 65;
                counter[chars[i] as usize] += 1;
            }
        }
        println!("{:?}", counter);
        
        let max_index = match counter.iter().enumerate().max_by_key(|(_, &v)| v) {
            Some((index, _)) => index,
            None => {  
                panic!("Crypttext ist leer");
            }
        };
        max_index
            }
}