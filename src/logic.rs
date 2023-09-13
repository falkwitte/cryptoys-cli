use crate::extract_file_content;
use crate::stdin::stdin_read_to_string;
use crate::write_to_output_file;
use cryptoys::historical::{affine, atbash, caesar, playfair, rot13};
use cryptoys::key::otp;

/// This is just a test function remove if necessary
pub fn atbash(encrypt_flag: Option<String>, decrypt_flag: Option<String>) {
    if encrypt_flag.is_some() {
        let encrypt = extract_file_content(encrypt_flag.unwrap());
        let ciphertext = atbash::encrypt(&encrypt).to_string();
        write_to_output_file(ciphertext);
    } else if decrypt_flag.is_some() {
        let decrypt = extract_file_content(decrypt_flag.unwrap());
        let plaintext = atbash::decrypt(&decrypt);
        write_to_output_file(plaintext);
    } else if decrypt_flag.is_some() && encrypt_flag.is_some() {
        println!("Error: two opposite flags");
    } else if decrypt_flag.is_none() && encrypt_flag.is_none() {
        let ciphertext = atbash::encrypt(&stdin_read_to_string());
        println!("{}", ciphertext.to_string());
        //write_to_output_file(ciphertext.to_string());
    }
}

pub fn affine(encrypt_flag: Option<String>, decrypt_flag: Option<String>, a: i32, b: i32) {
    if encrypt_flag.is_some() && decrypt_flag.is_some() {
        println!("Error: two opposite flags")
    } else if encrypt_flag.is_some() {
        let encrypt = extract_file_content(encrypt_flag.unwrap());
        let ciphertext = affine::encrypt(a, b, &encrypt).to_string();
        write_to_output_file(ciphertext);
    } else if decrypt_flag.is_some() {
        let decrypt = extract_file_content(decrypt_flag.unwrap());
        let plaintext = affine::decrypt(a, b, &decrypt);
        write_to_output_file(plaintext);
    } else if encrypt_flag.is_none() && decrypt_flag.is_none() {
        let ciphertext = affine::encrypt(a, b, &stdin_read_to_string());
        println!("{}", ciphertext.to_string());
        //write_to_output_file(ciphertext.to_string());
    }
}

pub fn caesar(encrypt_flag: Option<String>, decrypt_flag: Option<String>, shift: u8) {
    if encrypt_flag.is_some() && decrypt_flag.is_some() {
        println!("Error: two opposite flags")
    } else if encrypt_flag.is_some() {
        let encrypt = extract_file_content(encrypt_flag.unwrap());
        let ciphertext = caesar::encrypt(&encrypt, shift).to_string();
        write_to_output_file(ciphertext);
    } else if decrypt_flag.is_some() {
        let decrypt = extract_file_content(decrypt_flag.unwrap());
        let plaintext = caesar::decrypt(&decrypt, shift);
        write_to_output_file(plaintext);
    } else if encrypt_flag.is_none() && decrypt_flag.is_none() {
        let ciphertext = caesar::encrypt(&stdin_read_to_string(), shift);
        println!("{}", ciphertext.to_string());
        //write_to_output_file(ciphertext.to_string());
    }
}

pub fn playfair(encrypt_flag: Option<String>, decrypt_flag: Option<String>, key: String) {
    if encrypt_flag.is_some() && decrypt_flag.is_some() {
        println!("Error: two opposite flags")
    } else if encrypt_flag.is_some() {
        let encrypt = extract_file_content(encrypt_flag.unwrap());
        let ciphertext = playfair::encrypt(&encrypt, &key).to_string();
        write_to_output_file(ciphertext);
    } else if decrypt_flag.is_some() {
        let decrypt = extract_file_content(decrypt_flag.unwrap());
        let plaintext = playfair::decrypt(&decrypt, &key);
        write_to_output_file(plaintext);
    } else if encrypt_flag.is_none() && decrypt_flag.is_none() {
        let ciphertext = playfair::encrypt(&stdin_read_to_string(), &key);
        println!("{}", ciphertext.to_string());
        //write_to_output_file(ciphertext.to_string());
    }
}

pub fn rot13(encrypt_flag: Option<String>, decrypt_flag: Option<String>) {
    if encrypt_flag.is_some() && decrypt_flag.is_some() {
        println!("Error: two opposite flags")
    } else if encrypt_flag.is_some() {
        let encrypt = extract_file_content(encrypt_flag.unwrap());
        let ciphertext = rot13::encrypt(&encrypt).to_string();
        write_to_output_file(ciphertext);
    } else if decrypt_flag.is_some() {
        let decrypt = extract_file_content(decrypt_flag.unwrap());
        let plaintext = rot13::decrypt(&decrypt);
        write_to_output_file(plaintext);
    } else if encrypt_flag.is_none() && decrypt_flag.is_none() {
        let ciphertext = rot13::encrypt(&stdin_read_to_string());
        println!("{}", ciphertext.to_string());
        //write_to_output_file(ciphertext.to_string());
    }
}

pub fn otp(encrypt_flag: Option<String>, decrypt_flag: Option<String>, pad: Vec<u8>) {
    if encrypt_flag.is_some() && decrypt_flag.is_some() {
        println!("Error: two opposite flags")
    } else if encrypt_flag.is_some() {
        let encrypt = extract_file_content(encrypt_flag.unwrap());
        let ciphertext = otp::encrypt(pad.clone(), &encrypt).to_string();
        write_to_output_file(ciphertext);
    } else if decrypt_flag.is_some() {
        let decrypt = extract_file_content(decrypt_flag.unwrap());
        let plaintext = otp::decrypt(pad, &decrypt);
        write_to_output_file(plaintext);
    } else if encrypt_flag.is_none() && decrypt_flag.is_none() {
        let ciphertext = atbash::encrypt(&stdin_read_to_string());
        println!("{}", ciphertext.to_string());
        //write_to_output_file(ciphertext.to_string());
    }
}
