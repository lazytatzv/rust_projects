use std::io;

fn main() {
    let mut str = String::new();

    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read");

    let str = str.trim();

    let reversed: String = str.chars().rev().collect();

    if str == reversed {
        println!("Yes");
    } else {
        println!("No");
    }

}