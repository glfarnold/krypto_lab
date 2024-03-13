pub mod sha_functions {
    use num_bigint::{BigInt, BigUint};
    use num_traits::{One, Zero};
    use std::ops::BitAnd;

    pub fn pad(m: &BigUint, r: &u64) -> BigUint {
        let mut result = m.clone();
        let j = r - ((m.bits() + 2) % r);
        result <<= 1; result += BigUint::one();
        result <<= j; 
        result <<= 1; result += BigUint::one();
        result
    }

    pub fn divide_into_blocks(m: &BigUint, r: &u64) -> Vec<BigUint> {
        let mut message = m.clone();
        let length = m.bits();
        assert_eq!(length % r, 0);
        let num_blocks = length / *r;
        let mut result: Vec<BigUint> = Vec::new();
        let mut tmp = BigUint::zero();

        for i in 0..*r {
            tmp ^= &BigUint::one() << i;
        }

        for _ in 0..num_blocks {
            let block = message.clone() & tmp.clone();
            result.insert(0, block);
            message >>= r;
        }

        result
    }

    pub fn string_to_state(s: &Vec<u8>) -> Vec<Vec<Vec<u8>>> {
        let mut state: Vec<Vec<Vec<u8>>> = Vec::new();
        for y in 0..5 {
            for x in 0..5 {
                for z in 0..64 {
                    state[x][y][z] = s[64*(5*y + x) + z];
                }
            }
        }
        state
    }

    pub fn state_to_string(state: &Vec<Vec<Vec<u8>>>) -> Vec<u8> {
        let mut lanes: [[u8; 64]; 25] = [[0; 64]; 25];
        for i in 0..5 {
            for j in 0..5 {
                for k in 0..64 {
                    lanes[5*i + j]. 
                }
                
            }
        }
    }
}