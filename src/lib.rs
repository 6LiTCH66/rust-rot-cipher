use std::io::{stdin};
use std::num::ParseIntError;
use std::fmt::{Debug};


#[derive(Debug)]
pub enum CipherMethod{
    Encrypt,
    Decrypt,
    None,
}

#[derive(Debug, PartialEq)]
pub struct Cipher{
    message: String,
    shift: u8

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

#[cfg(test)]
mod tests{
    use crate::{CipherMethod, Cipher};

    #[test]
    fn creating_empty_entity_test(){
        let cipher = Cipher::new();
        assert_eq!(
            Cipher{
                message: String::new(),
                shift: 0,
            },
            cipher)
    }

    #[test]
    fn encrypt_with_only_letters_test(){
        let mut cipher = Cipher::new();
        cipher.message = "helloworld".to_string();
        cipher.shift = 15;
        cipher.encrypt_message();
        assert_eq!("wtaadldgas".to_string(), cipher.message)

    }

    #[test]
    fn encrypt_with_letters_and_space_test(){
        let mut cipher = Cipher::new();
        cipher.message = "hello world how is your day".to_string();
        cipher.shift = 23;
        cipher.encrypt_message();
        assert_eq!("ebiil tloia elt fp vlro axv".to_string(), cipher.message)
    }

    #[test]
    fn encrypt_with_letters_numbers_and_space_test(){
        let mut cipher = Cipher::new();
        cipher.message = "h3l1o w0rld1123".to_string();
        cipher.shift = 12;
        cipher.encrypt_message();
        assert_eq!("t3x1a i0dxp1123".to_string(), cipher.message)
    }

    #[test]
    fn encrypt_with_letters_numbers_space_and_chars_test(){
        let mut cipher = Cipher::new();
        cipher.message = "JJfjdsfjs JHJFfsd878&^*@($Of;df;[],.;[^hjfshdJKJFDS7373hjd^@)$#@#$2163$$32".to_string();
        cipher.shift = 25;
        cipher.encrypt_message();
        assert_eq!("IIeicreir IGIEerc878&^*@($Ne;ce;[],.;[^giergcIJIECR7373gic^@)$#@#$2163$$32".to_string(), cipher.message)
    }

    #[test]
    fn decrypt_with_only_letters_test(){
        let mut cipher = Cipher::new();
        cipher.message = "dahhksknhz".to_string();
        cipher.shift = 22;
        cipher.decrypt_message();
        assert_eq!("helloworld".to_string(), cipher.message)

    }

    #[test]
    fn decrypt_with_letters_and_space_test(){
        let mut cipher = Cipher::new();
        cipher.message = "zwddg zgo ak qgmj vsq ygafy".to_string();
        cipher.shift = 18;
        cipher.decrypt_message();
        assert_eq!("hello how is your day going".to_string(), cipher.message)
    }

    #[test]
    fn decrypt_with_letters_numbers_and_space_test(){
        let mut cipher = Cipher::new();
        cipher.message = "q3uu011q0f rb12h0da mj29p01wp".to_string();
        cipher.shift = 9;
        cipher.decrypt_message();
        assert_eq!("h3ll011h0w is12y0ur da29g01ng".to_string(), cipher.message)
    }

    #[test]
    fn decrypt_with_letters_numbers_space_and_chars_test(){
        let mut cipher = Cipher::new();
        cipher.message = "w3AA0!<>w0L Xh&@(N d12#@g_sp n@U}{shp?V)1c6^".to_string();
        cipher.shift = 15;
        cipher.decrypt_message();
        assert_eq!("h3LL0!<>h0W Is&@(Y o12#@r_da y@F}{dsa?G)1n6^".to_string(), cipher.message)
    }

}