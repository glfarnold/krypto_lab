pub mod keygen {
    use crate::{functions::functions, get_sub_matrix};

    pub fn generate_key(k: &Vec<u32>) -> Vec<u32> {
        assert_eq!(k.len(), 4);
        let mut w: Vec<u32> = Vec::new();
        for i in 0..44 {
            if i < 4 {
                w.push(k[i]);
            }
            else if i % 4 == 0 {
                assert!(i > 3);
                w.push(w[i-4] ^ rcon(&((i as u32)/4)) ^ sub_word(&rot_word(&w[i-1])));
            } 
            else {
                w.push(w[i-4] ^ w[i-1]);
            }
        }
        w
    }

    fn rcon(i: &u32) -> u32 {
        let rc: Vec<u32> = vec![0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];
        rc[*i as usize -1] << 24
    }

    fn sub_word(w: &u32) -> u32 {
        let mut result: u32 = 0;
        let sbox = get_sub_matrix(true);
        let tmp = vec![0xff000000, 0x00ff0000, 0x0000ff00, 0x000000ff];
        for i in 0..4 {
            let val = w & tmp[i] >> (3-i)*8;
            let m = val >> 4;
            let n = val &0xf;
            result ^= (sbox[m as usize][n as usize] as u32) << (3-i)*8;
        }
        result
    }

    fn rot_word(w: &u32) -> u32 {
        let tmp = w & 0xff000000;
        (w << 8) ^ (tmp >> 24)
    }
}