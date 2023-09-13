use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
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
