pub mod linana2 {
    fn dot_product(a: &u8, b: &u8) -> u8 {
        let mut tmp = a & b;
        let mut result = 0;
        while tmp != 0 {
            result ^= tmp & 0b1;
            tmp >>= 1;
        }
        
        result
    }

    pub fn bias((a,b): &(u8, u8), sbox: &Vec<u8>) -> f64 {
        let mut e = 0;
        for u in 0..16 {
            let tmp = dot_product(a,&u) ^ dot_product(b, &sbox[u as usize]);
            if tmp == 0 {
                e += 1;
            }
        }
        e -= 8;
        (e as f64 / 16.0).abs()
    }

    pub fn quality(biases: &Vec<f64>) -> f64 {
        biases.iter().fold(1.0, |acc, e| acc*e)
    }

    pub fn get_active_boxes(approx: &Vec<Vec<u8>>) -> Vec<(u8, u8)> {
        let mut active_boxes: Vec<(u8, u8)> = Vec::new();
        for i in 0..approx.len() {
            for j in 0..approx[0].len() {
                if approx[i][j] != 0 {
                    let input = approx[i][j] >> 4;
                    let output = approx[i][j] & 0xf;
                    active_boxes.push((input, output));
                }
            }
        }
        active_boxes
    }
}