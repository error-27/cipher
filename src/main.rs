use clap::{Parser, Subcommand};
mod ciphers;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cipher: Cipher,

    #[clap(long, short, action)]
    decipher: bool,
}

#[derive(Subcommand, Debug, Clone)]
enum Cipher {
    Caesar {
        shift: usize,
        input: String
    },
    Vigenere {
        key: String,
        input: String
    }
}

fn main() {
    let args = Args::parse();
    
    let output: String;
    if !args.decipher {
        output = match args.cipher {
            Cipher::Caesar {input, shift} => {
                ciphers::encode_caesar(input, shift)
            },
            Cipher::Vigenere {key, input} => {
                ciphers::encode_vigenere(input, key)
            }
        }
    } else {
        output = match args.cipher {
            Cipher::Caesar {input, shift} => {
                ciphers::decode_caesar(input, shift)
            },
            Cipher::Vigenere {key, input} => {
                ciphers::decode_vigenere(input, key)
            }
        }
    }
    
    println!("{}", output)
}
