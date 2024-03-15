pub mod sha_functions {
    use num_bigint::{BigInt, BigUint};
    use num_traits::{One, Zero};
    use std::{ops::BitAnd, result, vec};

    use crate::io_functions::io_functions::print_state;

    // paddet die Message auf eine Länge von k*r Bits für eine natürliche Zahl k 
    // Es wird zunächst 01 angehangen, dann eine 1, 0 bis r-1 0en und abschließend eine 1
    pub fn pad(m: &BigUint, r: &u64) -> BigUint {
        let mut result = m.clone();
        result = add_01(&result);
        let j = r - ((result.bits() + 2) % r);
        result <<= 1; result += BigUint::one();
        result <<= j; 
        result <<= 1; result += BigUint::one();
        result
    }

    // hängt 01 an die gegebene Nachricht an
    pub fn add_01(m: &BigUint) -> BigUint {
        let mut result = m.clone();
        result <<= 2; 
        result + BigUint::one()
    }

    // Einlesen der Bytes als little endian
    pub fn read_as_le(m: &BigUint) -> BigUint {
        let mut m_padded_bytes = m.to_bytes_be();
        for i in 0..m_padded_bytes.len() {
        m_padded_bytes[i] = m_padded_bytes[i].reverse_bits();
        }
        let m_le = BigUint::from_bytes_be(&m_padded_bytes);
        m_le
    }

    // unterteilt die Nachricht in Blöcke mit r Bits
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

    // die Funktion f arbeitet auf Vec<Vec<Vec<u8>>>, daher muss der Input, der als BigUint 
    // interpretiert wurde transformiert werden
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
        result.reverse();
        result
    }

    // Input der Rundenfunktion ist ein 5x5x64 Array 
    // mit dieser Funktion wird das gegebene 1600 bit Array in ein 5x5x64 Array transformiert
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

    // für die Ausgabe wird das 5x5x64 Array zurück in ein Vec<u8> transformiert
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

    // theta funktion 
    pub fn theta(state: &Vec<Vec<Vec<u8>>>) -> Vec<Vec<Vec<u8>>> {
        let mut result: Vec<Vec<Vec<u8>>> = vec![vec![vec![0;64];5];5]; 

        // C[x,z] enthält die Parität der Spalte [x,z]
        let mut c: Vec<Vec<u8>> = vec![vec![0;64];5];
        for i in 0..5 {
            let x = (i+3) %5;
            for z in 0..64 {
                let mut tmp: u8 = 0;
                for j in 0..5 {
                    let y = (j+3) %5;
                    tmp ^= state[x][y][z];
                }
                c[x][z] = tmp;
            }
        }

        // D[x,z] setzt sich aus der Parität der Spalte [x-1,z] und [x+1,z-1] zusammen
        let mut d: Vec<Vec<u8>> = vec![vec![0;64];5];
        for i in 0..5 {
            let x = (i+3)%5;
            for z in 0..64 {
                d[x][z] = c[(x+4) % 5][z] ^ c[(x+1) % 5][(z+63) % 64];
            }
        }

        // Ergebnis der theta Funktion ist state[x][y][z] ^ D[x][z]
        for i in 0..5 {
            let x = (i+3)%5;
            for z in 0..64 {
                for j in 0..5 {
                    let y = (j+3)%5;
                    result[x][y][z] = result[x][y][z] ^ d[x][z];
                }
            }
        }
        result

    }

    // rho Funktion
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

    // pi funktion
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

    // chi funktion
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

    // iota funktion
    pub fn iota(state: &Vec<Vec<Vec<u8>>>, rc: &u64) -> Vec<Vec<Vec<u8>>> {
        let mut result: Vec<Vec<Vec<u8>>> = state.clone();
        for z in 0..64 {
            let tmp = (rc >> (63-z) & 1) as u8;
            result[0][0][z] ^= tmp;
        }
        result
    }

    // Rundenfunktion rnd (f) 
    pub fn rnd(state: &Vec<Vec<Vec<u8>>>, rc: &u64) -> Vec<Vec<Vec<u8>>> {
        let mut result = state.clone();
        result = theta(&result);
        result = rho(&result);
        result = pi(&result);
        result = chi(&result);
        iota(&result, &rc)
    }

    // sha-3 main Funktion, führt Rundenfunktion 24x aus
    pub fn keccak(s: &Vec<u8>, round_constants: &Vec<u64>) -> Vec<u8> {
        let mut state = string_to_state(s);
        for i in 0..24 {
            state = rnd(&state, &round_constants[i]);
        }
        state_to_string(&state)
    }
}