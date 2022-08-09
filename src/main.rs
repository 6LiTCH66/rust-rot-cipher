use std::io::{stdin, stdout, Write};

fn encrypt_message(message: &[u8], shift: u8) -> String{
    let encrypt_array_of_bytes = message.iter().map(|byte|{
        match byte {
            97..=122 => { // for lowercase's bytes
                if byte + shift > 122 {
                    (byte + shift) - 26
                }
                else {
                    byte + shift
                }
            }
            65..=90 => { // for uppercase's bytes
                if byte + shift > 90 {
                    (byte + shift) - 26
                }else {
                    byte + shift
                }

            },
            _ => *byte, // for others chars
        }


    }).collect::<Vec<_>>();
    String::from_utf8_lossy(&encrypt_array_of_bytes).to_string()
}



fn main() {
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
                    1 => encrypt(),
                    2 => println!("Decrypt"),
                    3 => break,
                    _ => println!("Please try again"),
                }
            }
            Err(err) => println!("Error {}", err)
        }
    }


}

fn encrypt(){
    let mut message = String::new();
    let mut shift = String::new();
    println!("Please enter a message you'd like to encrypt:");
    stdin().read_line(&mut message).expect("Cannot read proper line!");
    println!("Please choose encrypting method from 1 to 25:");
    stdin().read_line(& mut shift).expect("Cannot read proper line!");

    match shift.trim().parse::<u8>() {
        Ok(shift_num) => {
            match shift_num {
                1..=25 => {
                    let message_to_bytes = message.trim().as_bytes();
                    let encrypted_message = encrypt_message(message_to_bytes, shift_num);
                    println!("{}", encrypted_message);

                },
                _ => println!("Try again!"),
            }
        },
        Err(err) => println!("Error {}", err)
    }
}
