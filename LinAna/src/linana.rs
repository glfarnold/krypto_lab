pub mod linana {
    extern crate SPN;
    use SPN::spn::*;
    use rand::{Rng, SeedableRng};

    pub const SBOX_INV: [u8; 16] = [0xe, 0x3, 0x4, 0x8, 0x1, 0xc, 0xa,
                                    0xf, 0x7, 0xd, 0x9, 0x6, 0xb, 0x2, 0x0, 0x5];

    pub fn bits(num: &u8) -> [u8; 8] {
        (0..8).map(|i| num >> i & 0x1).rev().collect::<Vec<_>>().try_into().unwrap()
    }

    pub fn create_n_pairs(n: &i32) -> Vec<(u16, u16)> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0x4242);
        let mut pairs: Vec<(u16, u16)> = Vec::with_capacity(*n as usize);
        for _ in 0..*n as usize {
            let random_value: u16 = rng.gen(); 
            let encrypted_value: u16 = spn()
            pairs.push((random_value, encrypted_value));
        }
        pairs
    }

    pub fn my_function(m: Vec<(u16, u16)>) -> (u8, u8) {
        let tmp = m.clone();
        let mut maxkey: (u8, u8) = (0,0);
        let mut a: Vec<Vec<i32>> = vec![vec![0;16];16];

        for (x,y) in tmp {
            for l1 in 0..16 {
                for l2 in 0..16 {
                    let y2 = (y >> 8) & 0xf;
                    let y4 = y & 0xf; 

                    let v2 = l1 ^ y2; 
                    let v4 = l2 ^ y4; 

                    let u2 = SBOX_INV[v2 as usize];
                    let u4 = SBOX_INV[v4 as usize];

                    let x1: u8 = ((x >> 8) & 0xff) as u8;

                    let x_bits = bits(&x1);
                    let u2_bits = bits(&u2);
                    let u4_bits = bits(&u4);

                    if bits(&x1)[4] ^ bits(&x1)[6] ^ bits(&x1)[7] ^ 
                       bits(&u2)[5] ^ bits(&u2)[7] ^ bits(&u4)[5] ^ bits(&u4)[7] == 0 {
                        a[l1 as usize][l2 as usize] += 1;
                    }
                }
            }
        }

        let mut max = -1;
        let t: i32 = m.len() as i32; 
        let mut b: Vec<Vec<i32>> = vec![vec![0;16];16];

        for l1 in 0..16 {
            for l2 in 0..16 {
                b[l1][l2] = (a[l1][l2] - (t/2)).abs(); 
                if b[l1][l2] > max {
                    max = b[l1][l2];
                    maxkey = (l1 as u8, l2 as u8);
                }
            }
        }

        maxkey
    }
}