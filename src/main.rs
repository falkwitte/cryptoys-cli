use clap::{Parser, Subcommand};
use cryptoys::historical::{affine, atbash};
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
        }
    }
}
