pub mod rsa_key {
    use num_bigint::*;
    use num_traits::{ops::checked, One, Zero};
    use rand::Rng;

    pub fn miller_rabin(n: &BigInt) -> bool {
        //determine k
        let n_minus_1: BigInt = n.clone() - BigInt::from(1);
        let k = n_minus_1.trailing_zeros().unwrap();

        // determine m
        let tmp = BigInt::from(1) << k;
        let m: BigInt = (n-1) / tmp;
        assert_eq!(m.clone() << k, n-1);

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
            if b.clone() == n_minus_1 {
                return true
            }
            b = (&b * &b) % n;
        }
        return false;
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
        let z = gen_random(123);
        let primes: Vec<BigInt> = vec![BigInt::from(1), BigInt::from(7), BigInt::from(11), BigInt::from(13),
                                       BigInt::from(17), BigInt::from(19), BigInt::from(23), BigInt::from(29)];
        
        let mut p = BigInt::from(30) * &z;
        let mut i = 1;

        loop {
            let mut check_prime: bool = true;
            for prime in &primes {
                let p_tmp: BigInt = &p + prime;

                // check 5 times with miller rabin if p is a prime number
                for j in 0..5 {
                    check_prime = miller_rabin(&p_tmp);
                    if !check_prime {
                        break;
                    }
                }
                if check_prime {
                    p = p_tmp;
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
        let n = p*q;
        let phi_n: BigInt = (p - BigInt::from(1)) * (q - BigInt::from(1));
        // erzeuge e zufällig, check if euklid(e, phi_n) == 1 
        let e: BigInt = (BigInt::one() << 16) + BigInt::one();
        if euclid(&e, &phi_n) != One::one() {
            // erzeuge 16 bit primzahl e
            println!("e phi nicht teilerfremd")
        }
        // euklidex d berechnen 
        let d = euclidex(&phi_n, &e).2;
        ((e,n.clone()), (d,n.clone()))
    }

    pub fn euclid(a: &BigInt, b: &BigInt) -> BigInt {
        let mut a = a.clone(); let mut b = b.clone();
        if a == Zero::zero() {
            return b.clone();
        }
        loop {
            if b.clone() == Zero::zero() {
                break;
            }
            let h = a % &b;
            a = b;
            b = h;
        }
        return a;
    }

    pub fn euclidex(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
        assert!(a > b);
        let mut r_0 = a.clone();
        let mut r_1 = b.clone(); 
        let mut r_2 = &r_0 % &r_1;
        let mut s_0: BigInt = One::one(); let mut s_1: BigInt = Zero::zero(); let mut s: BigInt = Zero::zero(); 
        let mut t_0: BigInt = Zero::zero(); let mut t_1: BigInt = One::one(); let mut t: BigInt = Zero::zero();

        loop {
            if r_2 == Zero::zero() {
                break;
            }
            let d = &r_0 / &r_1;
            r_0 = r_1; r_1 = r_2.clone(); 
            r_2 = &r_0 % &r_1;
            s = &s_0 - &d * &s_1;
            t = &t_0 - &d * &t_1;
            s_0 = s_1.clone(); s_1 = s.clone(); t_0 = t_1.clone(); t_1 = t.clone();
        }    
        let ggt = &s*a + &t*b;
        (ggt, s, t)
    }

    fn gen_random (num_bits: u64) -> BigInt {
        let mut rng = rand::thread_rng();
        let mut z = rng.gen_bigint(num_bits);
        while z.sign() == Sign::Minus {
            z = rng.gen_bigint(num_bits);
        }
        z
    }
}