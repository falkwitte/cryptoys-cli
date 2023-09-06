use clap::{Parser, Subcommand};
use cryptoys::historical::{affine, atbash, caesar, playfair, rot13};
use cryptoys::key::otp;
use std::fs::{read_to_string, write};
use std::path::PathBuf;

#[derive(Subcommand)]
enum Commands {
    /// atbash cipher
    Atbash {
        /// encrypts plaintext with the atbash cipher
        #[arg(short = 'e', long)]
        encrypt: Option<String>,

        /// decrypts ciphertext encrypted with the atbash cipher
        #[arg(short = 'd', long)]
        decrypt: Option<String>,
    },

    /// affine cipher
    Affine {
        /// encrypts plaintext with the affine cipher
        #[arg(short = 'e', long)]
        encrypt: Option<String>,

        /// decrypts ciphertext encrypted with the affine cipher
        #[arg(short = 'd', long)]
        decrypt: Option<String>,

        /// 'a' in the formula, a and b have to be coprime
        #[arg(short = 'a', required = true)]
        a: i32,

        /// 'b' in the formula, a and b have to be coprime
        #[arg(short = 'b', required = true)]
        b: i32,
    },

    /// caesar cipher
    Caesar {
        /// encrypts plaintext with the caesar cipher
        #[arg(short = 'e', long)]
        encrypt: Option<String>,

        /// decrypts ciphertext encrypted with the caesar cipher
        #[arg(short = 'd', long)]
        decrypt: Option<String>,

        /// shift
        #[arg(required = true, short = 's', long)]
        shift: u8,
    },

    /// rot13 cipher
    Rot13 {
        /// encrypts plaintext with the caesar cipher
        #[arg(short = 'e', long)]
        encrypt: Option<String>,

        /// decrypts ciphertext encrypted with the casear cipher
        #[arg(short = 'd', long)]
        decrypt: Option<String>,
    },

    /// playfair cipher
    Playfair {
        /// encrypts plaintext with the playfair cipher
        #[arg(short = 'e', long)]
        encrypt: Option<String>,

        /// decrypts ciphertext encrypted with the playfair cipher
        #[arg(short = 'd', long)]
        decrypt: Option<String>,

        /// key
        #[arg(short = 'k', long, required = true)]
        key: String,
    },

    /// one-time pad
    OTP {
        /// encrypts plaintext using the one-time pad and a given key
        #[arg(short = 'e', long)]
        encrypt: Option<String>,

        /// decrypts ciphertext that was encrypted witht the one-time pad cipher with the given pad(key)
        #[arg(short = 'd', long)]
        decrypt: Option<String>,

        /// pad, works with ',' seperated list, requires numbers
        #[arg(required = true)]
        pad: Vec<u8>,
    },
}

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Atbash { encrypt, decrypt } => {
            if encrypt.is_some() {
                let encrypt = extract_file_content(encrypt.unwrap());
                let ciphertext = atbash::encrypt(&encrypt).to_string();
                write_to_output_file(ciphertext);
            }
            if decrypt.is_some() {
                let decrypt = extract_file_content(decrypt.unwrap());
                let plaintext = atbash::decrypt(&decrypt);
                write_to_output_file(plaintext);
            }
        }

        Commands::OTP {
            encrypt,
            decrypt,
            pad,
        } => {
            if encrypt.is_some() {
                let encrypt = extract_file_content(encrypt.unwrap());
                let ciphertext = otp::encrypt(pad.clone(), &encrypt).to_string();
                write_to_output_file(ciphertext);
            }
            if decrypt.is_some() {
                let decrypt = extract_file_content(decrypt.unwrap());
                let plaintext = otp::decrypt(pad, &decrypt);
                write_to_output_file(plaintext);
            }
        }

        Commands::Affine {
            encrypt,
            decrypt,
            a,
            b,
        } => {
            if encrypt.is_some() {
                let encrypt = extract_file_content(encrypt.unwrap());
                let ciphertext = affine::encrypt(a, b, &encrypt).to_string();
                write_to_output_file(ciphertext);
            }
            if decrypt.is_some() {
                let decrypt = extract_file_content(decrypt.unwrap());
                let plaintext = affine::decrypt(a, b, &decrypt);
                write_to_output_file(plaintext);
            }
        }

        Commands::Caesar {
            encrypt,
            decrypt,
            shift,
        } => {
            if encrypt.is_some() {
                let encrypt = extract_file_content(encrypt.unwrap());
                let ciphertext = caesar::encrypt(&encrypt, shift).to_string();
                write_to_output_file(ciphertext);
            }
            if decrypt.is_some() {
                let decrypt = extract_file_content(decrypt.unwrap());
                let plaintext = caesar::decrypt(&decrypt, shift);
                write_to_output_file(plaintext);
            }
        }

        Commands::Rot13 { encrypt, decrypt } => {
            if encrypt.is_some() {
                let encrypt = extract_file_content(encrypt.unwrap());
                let ciphertext = rot13::encrypt(&encrypt).to_string();
                write_to_output_file(ciphertext);
            }
            if decrypt.is_some() {
                let decrypt = extract_file_content(decrypt.unwrap());
                let plaintext = rot13::decrypt(&decrypt);
                write_to_output_file(plaintext);
            }
        }

        Commands::Playfair {
            encrypt,
            decrypt,
            key,
        } => {
            if encrypt.is_some() {
                let encrypt = extract_file_content(encrypt.unwrap());
                let ciphertext = playfair::encrypt(&encrypt, key.as_str());
                write_to_output_file(ciphertext);
            }
            if decrypt.is_some() {
                let decrypt = extract_file_content(decrypt.unwrap());
                let plaintext = playfair::decrypt(&decrypt, key.as_str());
                write_to_output_file(plaintext);
            }
        }
    }
}

fn extract_file_content(file: String) -> String {
    let file_path = PathBuf::from(file);
    read_to_string(file_path).unwrap()
}

fn write_to_output_file(content: String) {
    write(PathBuf::from("output.txt"), content).unwrap();
}
