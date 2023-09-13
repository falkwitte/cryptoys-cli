use crate::logic::{affine, atbash, caesar, otp, playfair, rot13};
use crate::subcommands::Commands;
use crate::utils::{extract_file_content, write_to_output_file};
use clap::Parser;

mod logic;
mod stdin;
mod subcommands;
mod utils;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Atbash { encrypt, decrypt } => {
            atbash(encrypt, decrypt);
        }

        Commands::OTP {
            encrypt,
            decrypt,
            pad,
        } => {
            otp(encrypt, decrypt, pad);
        }

        Commands::Affine {
            encrypt,
            decrypt,
            a,
            b,
        } => {
            affine(encrypt, decrypt, a, b);
        }

        Commands::Caesar {
            encrypt,
            decrypt,
            shift,
        } => {
            caesar(encrypt, decrypt, shift);
        }

        Commands::Rot13 { encrypt, decrypt } => {
            rot13(encrypt, decrypt);
        }

        Commands::Playfair {
            encrypt,
            decrypt,
            key,
        } => {
            playfair(encrypt, decrypt, key);
        }
    }
}
