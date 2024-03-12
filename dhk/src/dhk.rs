pub mod dhk {
    use num_bigint::{BigInt, RandBigInt, Sign};
    use num_traits::{One, Zero};
    use rand::Rng;

    // Ablauf des Diffie-Hellmann Key Exchange
    pub fn dhk(p: &BigInt) {
        let m: BigInt = p - BigInt::from(1);
        let g: BigInt = get_g(p);
        let a: BigInt = get_secret_num(p);
        let b: BigInt = get_secret_num(p);

        let A = square_and_multiply(&g, &a, &m);
        let B = square_and_multiply(&g, &b, &m);

        let s0 = square_and_multiply(&A, &b, &m);
        let s1 = square_and_multiply(&B, &a, &m);
        assert_eq!(s0, s1);
        print_nums(&p, &g, &A, &B, &s0);
    }

    // Ausgabe
    fn print_nums(p: &BigInt, g: &BigInt, A: &BigInt, B: &BigInt, s: &BigInt) {
        println!("{}", p);
        println!("{}", g);
        println!("{}", A);
        println!("{}", B);
        println!("{}", s);
    }

    // Es werden solange Primzahlen q erzeugt, bis eine die Eigenschaft hat, dass p = 2q+1 eine Primzahl ist
    // dann wird p zurückgegeben
    pub fn generate_p(num_bits: &u64) -> BigInt {
        let mut z = gen_random(&(num_bits-5));
        let mut q = generate_prime(&z);
        while !check_p(&q) {
            z += BigInt::from(1);
            q = generate_prime(&z);
        }
        2*q + BigInt::from(1)
    }

    // für eine Eingabe q, die eine Primzahl ist, wird als bool zurückgegeben, ob 2q+1 eine Primzahl ist
    pub fn check_p(q: &BigInt) -> bool {
        let mut is_prime: bool = true;
        for _ in 0..5 {
            is_prime = miller_rabin(&(2*q + BigInt::from(1)));
            if !is_prime {
                break;
            }
        }
        is_prime
    }

    // da q mit p = 2q+1 eine Primzahl ist, kann man jedes Element aus Z_p als Erzeuger wählen
    // deswegen wird hier eine zufällige Zahl aus dieser Gruppe zurückgegeben
    pub fn get_g(p: &BigInt) -> BigInt {
        let m = p - BigInt::from(1);
        let mut rng = rand::thread_rng();
        let mut g: BigInt = rng.gen_bigint_range(&BigInt::from(2), p);
        let prime_factors = vec![BigInt::from(2), m.clone()];
        let mut is_generator: bool = false;
        while !is_generator {
            for prime_factor in &prime_factors {
                let x = &m / prime_factor; 
                is_generator = true;
                if square_and_multiply(&g, &x, &m) == BigInt::from(1) {
                    g = rng.gen_bigint_range(&BigInt::from(2), p);
                    is_generator = false;
                    break;
                }
            }
        }
        g        
    }

    // Alice und Bob nehmen beide zufällig eine Zahl x mit 1 < x < p
    fn get_secret_num(p: &BigInt) -> BigInt {
        let mut rng = rand::thread_rng();
        rng.gen_range(BigInt::from(2)..p.clone())
    }

    // Funktion, die eine Primzahl mit einer gegebenen Bitlänge erzeugt
    // dafür wird für die übergebene Zahl z einer bestimmten Länge aufsteigend 30*z addiert mit den Werten im primes 
    // Vektor mit dem Miller Rabin Test überprüft
    pub fn generate_prime(z: &BigInt) -> BigInt {
        let primes: Vec<BigInt> = vec![BigInt::from(1), BigInt::from(7), BigInt::from(11), BigInt::from(13),
                                       BigInt::from(17), BigInt::from(19), BigInt::from(23), BigInt::from(29)];
        
        let mut p = BigInt::from(30) * z.clone();
        let mut i = 1;

        loop {
            let mut check_prime: bool = true;
            for prime in &primes {
                let p_tmp: BigInt = &p + prime;
                for _ in 0..5 {
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
                p = BigInt::from((30+i)* z.clone());
                i += 1;
            }
        }
        p
    }

    // Miller Rabin Test, um zu überprüfen, ob eine Zahl eine Primzahl ist
    // wird meist 5x ausgeführt, um ein sicheres Ergebnis zu erreichen, 
    // da es sich um einen Monte Carlo Algorithmus handelt
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
        for _ in 1..k+1 {
            if b.clone() == n_minus_1 {
                return true
            }
            b = square_and_multiply(&b, &BigInt::from(2), n);
        }
        return false;
    }

    // Erzeugt eine zufällige Zahl einer bestimmten Bitlänge 
    fn gen_random (num_bits: &u64) -> BigInt {
        let mut rng = rand::thread_rng();
        let mut z = rng.gen_bigint(*num_bits);
        while z.sign() == Sign::Minus {
            z = rng.gen_bigint(*num_bits);
        }
        z.set_bit(num_bits-1, true);
        z
    }

    // Implementierung des Quadrieren und Multiplizieren Algorithmus
    // gibt x^m mod n zurück
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
}