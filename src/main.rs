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

fn decrypt_message(message: &[u8], shift: u8) -> String{
    let decrypted_array_of_bytes = message.iter().map(|byte|{
        match byte {
            97..=122 => { // for lowercase's letters
                if byte - shift < 97 {
                    (byte - shift) + 26
                }else {
                    byte - shift
                }
            },
            65..=90 => { // for uppercase's letter
                if byte - shift < 65 {
                    (byte - shift) + 26
                }else{
                    byte - shift
                }
            },
            _ => *byte // from other chars
        }
    }).collect::<Vec<_>>();
    println!("{:?}", &decrypted_array_of_bytes);

    String::from_utf8_lossy(&decrypted_array_of_bytes).to_string()
}



fn main() {
    let test = "d";
    println!("{:?}", test.as_bytes());
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
                    1 => encrypting_decrypting(number, "encrypt".to_string()),
                    2 => encrypting_decrypting(number, "decrypt".to_string()),
                    3 => break,
                    _ => println!("Please try again"),
                }
            }
            Err(err) => println!("Error {}", err)
        }
    }

}

fn encrypting_decrypting(choice: i32, option: String){
    let mut message = String::new();
    let mut shift = String::new();
    println!("Please enter a message you'd like to {}:", option);
    stdin().read_line(&mut message).expect("Cannot read proper line!");
    println!("Please choose encrypting method from 1 to 25:");
    stdin().read_line(& mut shift).expect("Cannot read proper line!");

    match shift.trim().parse::<u8>() {
        Ok(shift_num) => {
            match shift_num {
                1..=25 => {
                    match choice {
                        1 => {
                            let message_to_bytes = message.trim().as_bytes();
                            let encrypted_message = encrypt_message(message_to_bytes, shift_num);
                            println!("{}", encrypted_message);
                        },
                        2 => {
                            let message_to_bytes = message.trim().as_bytes();
                            let decrypted_message = decrypt_message(message_to_bytes, shift_num);
                            println!("{}", decrypted_message);
                        },
                        _ => println!("No such option!")
                    }

                },
                _ => println!("Try again!"),
            }
        },
        Err(err) => println!("Error {}", err)
    }
}
