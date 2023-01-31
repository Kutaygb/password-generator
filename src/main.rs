use rand::prelude::*;
use rand::distributions::Alphanumeric;
use std::io;

fn generate_password(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let password: String = std::iter::repeat(())
        .map(|()| (rng.sample(Alphanumeric) as char))
        .take(length)
        .collect();
    password
}

fn main() {
    println!("How long should the password be? (default: 16)");

    let mut password_length = String::new();
    io::stdin().read_line(&mut password_length).unwrap();

    let password_length = match password_length.trim().parse::<usize>() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Using default length of 16.");
            16
        }
    };

    println!("Include special characters? (y/n)");

    let mut _include_special_characters = String::new();
    io::stdin().read_line(&mut _include_special_characters).unwrap();

    let _include_special_characters = _include_special_characters.trim().to_lowercase() == "y";

    let password = generate_password(password_length);
    println!("Generated password: {}", password);
}
