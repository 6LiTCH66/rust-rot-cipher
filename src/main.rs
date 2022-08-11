use std::io::{stdin};

use rust_rot_cipher::{Cipher, CipherMethod};

fn main() {
    let mut cipher = Cipher::new();

    let mut choise = String::new();

    loop {
        println!("1 - Encrypt a message.");
        println!("2 - Decrypt a message.");
        println!("3 - Exit.");
        stdin().read_line(&mut choise).expect("Cannot read proper line!");

        match choise.trim().parse::<i32>() {
            Ok(number) => {
                choise.clear();
                match number {
                    1 => cipher.cipher_option(CipherMethod::Encrypt),
                    2 => cipher.cipher_option(CipherMethod::Decrypt),
                    3 => break,
                    _ => println!("Please try again"),
                }
            }
            Err(err) => println!("Error {}", err)
        }
    }

}
