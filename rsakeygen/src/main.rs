use io_functions::io_functions::{read_user_input, write_output_to_file};
use keygen::keygen::{generate_prime, rsa_keygen};

mod keygen;
mod io_functions;
fn main() {
    let (num_bits, priv_path, pub_path, primes_path) = read_user_input();

    let a = generate_prime(&(num_bits-5)); 
    let b = generate_prime(&(num_bits-5));
    let primes = vec![a.clone(),b.clone()];

    let pub_key = vec![rsa_keygen(&a, &b).0.0, rsa_keygen(&a, &b).0.1];
    let priv_key = vec![rsa_keygen(&a, &b).1.0, rsa_keygen(&a, &b).1.1];

    write_output_to_file(&priv_path, &priv_key).expect("Schreiben von Output fehlgeschlagen");
    write_output_to_file(&pub_path, &pub_key).expect("Schreiben von Output fehlgeschlagen");
    write_output_to_file(&primes_path, &primes).expect("Schreiben von Output fehlgeschlagen");
}
