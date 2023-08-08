use clap::Parser;
use cryptoys::historical::atbash;

#[derive(Parser)]
struct Args{

    #[arg(short, long="atbash")]
    atbash:bool,

    #[arg(short, long="plaintext")]
    plaintext:String,

    #[arg(short, long="key")]
    key:Option<String>
}

fn main() {
    let args = Args::parse();

    if args.atbash{
        let ciphertext = atbash::encrypt(&args.plaintext).to_string();
        println!("{ciphertext}");
    }
}
