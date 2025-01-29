use std::io::{self, Read};
use std::i32;

fn is_prime(x: i32) -> bool {
    for i in 2..=x.sqrt() {
        if x % i == 0 {
            println!("The number is not prime");
            false
        }
    }
    println!("The number is prime");
    true

}


fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read");

    let input: i32 = input.trim().parse();



}