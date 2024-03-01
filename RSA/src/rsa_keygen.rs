pub mod rsa_keygen {
    use num_bigint::*;
    use num_traits::{ops::checked, One, Zero};
    use rand::Rng;

    pub fn miller_rabin(n: &BigInt) -> bool {
        //determine k
        let k = n.trailing_zeros().unwrap();

        // determine m
        let mut temp: BigInt = n.clone() - 1;
        loop {
            if temp.clone() % 2 != Zero::zero()  {
                break;
            }
            temp = &temp / 2;
        }
        let m = (n-1) / temp;

        // generate random number a with 2 <= a < n
        let mut rng = rand::thread_rng();
        let low = BigInt::from(2);
        let high = n;
        let a = rng.gen_bigint_range(&low, &high);

        let mut b = square_and_multiply(&a, &m, &n);
        if b == One::one() {
            return true;
        }
        for i in 1..k+1 {
            if -b.clone() == One::one() {
                return true
            }
            b = (&b * &b) % n;
        }
        false
    }

    pub fn square_and_multiply (x: &BigInt, m: &BigInt, n: &BigInt) -> BigInt {
        let mut y: BigInt = One::one();
        let mut val: BigInt = x.clone();
        let mut exp: BigInt = m.clone();

        loop {
            if exp == Zero::zero() {
                break;
            }
            if exp.clone() % 2 != Zero::zero() {
                y = y*&val % n;
            }
            val = (&val*&val) % n;
            exp = &exp / 2;
        }
        y
    }

    pub fn generate_prime() -> BigInt {
        // generate random number z
        let mut rng = rand::thread_rng();
        let low = BigInt::from(1000000_i64);
        let high = BigInt::from(2000000_i64);
        let z = rng.gen_bigint_range(&low, &high);

        let primes: Vec<BigInt> = vec![BigInt::from(1), BigInt::from(7), BigInt::from(11), BigInt::from(13),
                                       BigInt::from(17), BigInt::from(19), BigInt::from(23), BigInt::from(29)];
        
        let mut p = BigInt::from(30*&z);

        loop {
            let mut check_prime: bool = true;
            let mut i = 1;
            for prime in &primes {
                p += prime;
                

                // check 40 times with miller rabin if p is a prime number
                for _ in 0..40 {
                    check_prime = miller_rabin(&p);
                    if !check_prime {
                        break;
                    }
                }
                if check_prime {
                    break;
                }
            }
            if check_prime {
                break;
            }
            else {
                p = BigInt::from((30+i)*&z);
                i += 1;
            }
        }
        p
    }

    pub fn rsa_keygen(p: &BigInt, q: &BigInt) ->((BigInt, BigInt), (BigInt, BigInt)) {
        let phi_n = (*p - BigInt::from(1)) * (*q - BigInt::from(1));
        // erzeuge e zufällig 

        // euklidex d berechnen 
    }
}