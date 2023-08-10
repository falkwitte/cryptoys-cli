use clap::{Parser, Subcommand};
use cryptoys::historical::{affine, atbash, caesar, rot13, playfair};
use cryptoys::key::otp;

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
    Caesar{
        /// encrypts plaintext with the caesar cipher
        #[arg(short='e', long)]
        encrypt: Option<String>,

        /// decrypts ciphertext encrypted with the caesar cipher
        #[arg(short='d', long)]
        decrypt: Option<String>,

        /// shift
        #[arg(required=true, short='s', long)]
        shift: u8,
    },

    /// rot13 cipher
    Rot13{
        /// encrypts plaintext with the caesar cipher
        #[arg(short='e', long)]
        encrypt: Option<String>,

        /// decrypts ciphertext encrypted with the casear cipher
        #[arg(short='d', long)]
        decrypt: Option<String>,
    },

    /// playfair cipher
    Playfair{
        /// encrypts plaintext with the playfair cipher
        #[arg(short='e', long)]
        encrypt: Option<String>,

        /// decrypts ciphertext encrypted with the playfair cipher
        #[arg(short='d', long)]
        decrypt: Option<String>,

        /// key
        #[arg(short='k', long, required=true)]
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
                let ciphertext = atbash::encrypt(&encrypt.unwrap()).to_string();
                println!("{ciphertext}");
            }
            if decrypt.is_some() {
                let plaintext = atbash::decrypt(&decrypt.unwrap());
                println!("{plaintext}");
            }
        }

        Commands::OTP {
            encrypt,
            decrypt,
            pad,
        } => {
            if encrypt.is_some() {
                let ciphertext = otp::encrypt(pad.clone(), &encrypt.unwrap()).to_string();
                println!("{ciphertext}");
            }
            if decrypt.is_some() {
                let plaintext = otp::decrypt(pad, &decrypt.unwrap());
                println!("{plaintext}");
            }
        }

        Commands::Affine {
            encrypt,
            decrypt,
            a,
            b,
        } => {
            if encrypt.is_some() {
                let ciphertext = affine::encrypt(a, b, &encrypt.unwrap()).to_string();
                println!("{ciphertext}");
            }
            if decrypt.is_some() {
                let plaintext = affine::decrypt(a, b, &decrypt.unwrap());
                println!("{plaintext}");
            }
        },
        
        Commands::Caesar {
            encrypt, 
            decrypt,
            shift,
        } => {
            if encrypt.is_some() {
                let ciphertext = caesar::encrypt(&encrypt.unwrap(), shift).to_string();
                println!("{ciphertext}");
            }
            if decrypt.is_some() {
                let plaintext = caesar::decrypt(&decrypt.unwrap(), shift);
                println!("{plaintext}");
            }
        },

        Commands::Rot13 {
            encrypt,
            decrypt 
        } => {
            if encrypt.is_some() {
                let ciphertext = rot13::encrypt(&encrypt.unwrap()).to_string();
                println!("{ciphertext}");
            }
            if decrypt.is_some() {
                let plaintext = rot13::decrypt(&decrypt.unwrap());
                println!("{plaintext}");
            }
        },

        Commands::Playfair {
            encrypt,
            decrypt,
            key,
        } => {
            if encrypt.is_some() {
                let ciphertext = playfair::encrypt(&encrypt.unwrap(), key.as_str()).to_string();
                println!("{ciphertext}");
            }
            if decrypt.is_some() {
                let plaintext = playfair::decrypt(&decrypt.unwrap(), key.as_str());
                println!("{plaintext}");
            }
        }
    }
}
