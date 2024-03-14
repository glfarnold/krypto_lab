pub mod sha_functions {
    use num_bigint::{BigInt, BigUint};
    use num_traits::{One, Zero};
    use std::{ops::BitAnd, vec};

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

    pub fn bigint_to_vec_le(m: &BigUint) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        if *m == BigUint::zero() {
            return vec![0];
        }

        let mut tmp = m.clone();
        while !tmp.is_zero() {
            let bit = tmp.bit(0);
            result.push(bit as u8);
            tmp >>= 1;
        }

        result
    }

    pub fn vec_le_to_bigint(vec: &Vec<u8>) -> BigUint {
        let mut result = BigUint::zero();
        println!("{:?}", vec);
        for (i, &bit) in vec.iter().enumerate() {
            if bit != 0 {
                result |= BigUint::one() << i;
            }
        }
        result
    }

    pub fn string_to_state(s: &Vec<u8>) -> Vec<Vec<Vec<u8>>> {
        let mut state: Vec<Vec<Vec<u8>>> = vec![vec![vec![0;64];5];5];
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
        let mut result: Vec<u8> = Vec::new();
        for i in 0..5 {
            for j in 0..5 {
                for k in 0..64 {
                    result.push(state[j][i][k]);
                }
            }
        }
        result
    }

    pub fn theta(state: &Vec<Vec<Vec<u8>>>) -> Vec<Vec<Vec<u8>>> {
        let mut result: Vec<Vec<Vec<u8>>> = vec![vec![vec![0;64];5];5]; 
        let mut c: Vec<Vec<u8>> = vec![vec![0;64];5];
        for x in 0..5 {
            for z in 0..64 {
                let mut tmp: u8 = 0;
                for y in 0..5 {
                    tmp ^= state[x][y][z];
                }
                c[x][z] = tmp;
            }
        }

        let mut d: Vec<Vec<u8>> = vec![vec![0;64];5];
        for x in 0..5 {
            for z in 0..64 {
                d[x][z] = c[(x+4) % 5][z] ^ c[(x+1) % 5][(z+63) % 64];
            }
        }
        
        for x in 0..5 {
            for z in 0..64 {
                for y in 0..5 {
                    result[x][y][z] = state[x][y][z] ^ d[x][z];
                }
            }
        }
        
        result

    }

    pub fn rho(state: &Vec<Vec<Vec<u8>>>) -> Vec<Vec<Vec<u8>>> {
        let mut result: Vec<Vec<Vec<u8>>> = vec![vec![vec![0;64];5];5]; 
        for z in 0..64 {
            result[0][0][z] = state[0][0][z];
        }
        let (mut x,mut y) = (1,0);
        for t in 0..24 {
            for z in 0..64 {
                result[x][y][z] = state[x][y][(10*64 + z - (t+1)*(t+2)/2) % 64];
                (x, y) = (y, (2*x+3*y) % 5);
            }
        }
        result
    }

    pub fn pi(state: &Vec<Vec<Vec<u8>>>) -> Vec<Vec<Vec<u8>>> {
        let mut result: Vec<Vec<Vec<u8>>> = vec![vec![vec![0;64];5];5]; 
        for x in 0..5 {
            for y in 0..5 {
                for z in 0..64 {
                    result[x][y][z] = state[(x+3*y)%5][x][z];
                }
            }
        }
        result
    }

    pub fn chi(state: &Vec<Vec<Vec<u8>>>) -> Vec<Vec<Vec<u8>>> {
        let mut result: Vec<Vec<Vec<u8>>> = vec![vec![vec![0;64];5];5]; 
        for x in 0..5 {
            for y in 0..5 {
                for z in 0..64 {
                    result[x][y][z] = state[x][y][z] ^ ((state[(x+1)%5][y][z] ^ 1) & state[(x+2)%5][y][z]);
                }
            }
        }
        result
    }

    pub fn iota(state: &Vec<Vec<Vec<u8>>>, round_constants: &Vec<u64>, round: &usize) -> Vec<Vec<Vec<u8>>> {
        let mut result: Vec<Vec<Vec<u8>>> = state.clone();
        let rc = round_constants[*round];
        for z in 0..64 {
            let tmp = (rc >> (63-z) & 1) as u8;
            result[0][0][z] ^= tmp;
        }
        result
    }
}