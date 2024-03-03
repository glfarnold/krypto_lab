use crate::add::add::{decrypt, encrypt};

mod add;

fn main() {
    let a = String::from("ICH GEHE HEUTE BADEN");
    let crypttext = encrypt(&a, &5);
    println!("{}", encrypt(&a, &5));
    println!("{}", decrypt(&crypttext));
}
