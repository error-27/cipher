use clap::{Parser, Subcommand};
mod ciphers;

/// Terminal program to encrypt and decrypt text strings!
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Cipher to use
    #[command(subcommand)]
    cipher: Cipher,

    /// Decipher an input
    #[clap(long, short, action)]
    decipher: bool,
}

#[derive(Subcommand, Debug, Clone)]
enum Cipher {
    /// Use a Caesar cipher
    Caesar {
        /// Amount to shift the alphabet
        shift: usize,
        /// Text to encrypt
        input: String
    },
    /// Use a Vigenere cipher
    Vigenere {
        /// Keyword to encrypt by (only alphabetic characters are counted)
        key: String,
        /// Text to encrypt
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
