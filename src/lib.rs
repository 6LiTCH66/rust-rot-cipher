use std::io::{stdin};
use std::num::ParseIntError;
use std::fmt::{Debug};


#[derive(Debug)]
pub enum CipherMethod{
    Encrypt,
    Decrypt,
    None,
}

#[derive(Debug)]
pub struct Cipher{
    pub message: String,
    pub shift: u8

}

impl Cipher{
    pub fn new() -> Self{
        Self{
            message: String::new(),
            shift: 0,
        }
    }


    pub fn encrypt_message(&mut self){
        let message_to_bytes = self.message.trim().as_bytes();
        let encrypt_array_of_bytes = message_to_bytes.iter().map(|byte|{
            match byte {
                97..=122 => { // for lowercase's bytes
                    if byte + self.shift > 122 {
                        (byte + self.shift) - 26
                    }
                    else {
                        byte + self.shift
                    }
                }
                65..=90 => { // for uppercase's bytes
                    if byte + self.shift > 90 {
                        (byte + self.shift) - 26
                    }else {
                        byte + self.shift
                    }

                },
                _ => *byte, // for others chars
            }

        }).collect::<Vec<_>>();
        self.message = String::from_utf8_lossy(&encrypt_array_of_bytes).to_string()

    }

    pub fn decrypt_message(&mut self){
        let message_to_bytes = self.message.trim().as_bytes();
        let decrypted_array_of_bytes = message_to_bytes.iter().map(|byte|{
            match byte {
                97..=122 => { // for lowercase's letters
                    if byte - self.shift < 97 {
                        (byte - self.shift) + 26
                    }else {
                        byte - self.shift
                    }
                },
                65..=90 => { // for uppercase's letter
                    if byte - self.shift < 65 {
                        (byte - self.shift) + 26
                    }else{
                        byte - self.shift
                    }
                },
                _ => *byte // for other chars
            }
        }).collect::<Vec<_>>();
        self.message = String::from_utf8_lossy(&decrypted_array_of_bytes).to_string()

    }

    fn validate_shift(&mut self, shift: String) -> Result<u8, ParseIntError>{
        match shift.trim().parse::<u8>() {
            Ok(shift) => {
                self.shift = shift;
                Ok(shift)
            },
            Err(err) => {
                self.clean_message();
                Err(err)
            },
        }
    }

    fn clean_message(&mut self){
        self.message.clear()
    }


    pub fn cipher_option(&mut self, option: CipherMethod){
        let mut shift = String::new();
        println!("Please enter a message you'd like to {:?}:", option);
        stdin().read_line(&mut self.message).expect("Cannot read proper line!");
        println!("Please choose encrypting method from 1 to 25:");

        stdin().read_line(&mut shift).expect("Cannot read proper line!");


        match self.validate_shift(shift) {
            Ok(_) => {
                match self.shift {
                    1..=25 => {
                        match option {
                            CipherMethod::Encrypt => {
                                self.encrypt_message();
                                println!("{}", self.message);
                                self.clean_message();
                            },
                            CipherMethod::Decrypt => {
                                self.decrypt_message();
                                println!("{}", self.message);
                                self.clean_message();
                            },
                            _ => println!("No such option!"),
                        }
                    },
                    _ => println!("Try again!"),
                }
            },
            Err(err) => println!("Error {}", err)
        }

    }
}