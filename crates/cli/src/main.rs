use base64::Engine;
use clap::{command, Parser, Subcommand, ValueEnum};
use color_eyre::eyre::eyre;
use karak_sdk::{
    keypair::{
        bn254::{self, G2Pubkey},
        traits::Keypair,
    },
    keystore::{self},
    signer::{
        bls::{self, keypair_signer::verify_signature, signature::Signature},
        traits::Signer,
    },
};
use sha3::{Digest, Keccak256};
use std::{fs, path::PathBuf, str::FromStr};

#[derive(Parser)]
#[command(version, about = "Karak CLI", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage keypairs
    Keypair {
        #[command(subcommand)]
        subcommand: KeypairSubcommands,
    },
    /// Sign messages
    Sign {
        #[arg(short, long, value_enum)]
        scheme: Scheme,

        #[arg(short, long)]
        message: String,

        #[arg(short = 'e', long, value_enum)]
        message_encoding: Encoding,

        /// Path to BN254 keypair
        #[arg(short, long)]
        keypair: PathBuf,

        /// Passphrase to decrypt keypair
        #[arg(short, long)]
        passphrase: Option<String>,
    },
    /// Verify signatures
    SigVerify {
        #[arg(short, long)]
        message: String,

        #[arg(short = 'e', long, value_enum)]
        message_encoding: Encoding,

        #[arg(short, long)]
        signature: String,

        #[arg(short = 'k', long)]
        public_key: String,
    },
}

#[derive(Clone, ValueEnum, Debug)]
enum Curve {
    /// BN254 (also known as alt_bn128) is the curve used in Ethereum for BLS aggregation
    Bn254,
}

#[derive(Clone, ValueEnum, Debug)]
enum Scheme {
    /// Boneh–Lynn–Shacham (BLS) signature scheme using BN254
    Bls,
}

#[derive(Clone, ValueEnum, Debug)]
enum Encoding {
    Utf8,
    Hex,
    Base64,
    Base64URL,
    Base58,
}

#[derive(Subcommand)]
enum KeypairSubcommands {
    /// Generate a new keypair
    Generate {
        /// Curve to use for key generation
        #[arg(short, long, value_enum)]
        curve: Curve,

        /// File path to save the keypair
        #[arg(short, long)]
        output: PathBuf,

        /// Passphrase to encrypt keypair
        #[arg(short, long)]
        passphrase: Option<String>,
    },
    /// View public key
    Pubkey {
        /// Curve to use for key generation
        #[arg(short, long, value_enum)]
        curve: Curve,

        /// File path to load the keypair
        #[arg(short, long)]
        keypair: PathBuf,

        /// Passphrase to decrypt keypair
        #[arg(short, long)]
        passphrase: Option<String>,
    },
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    match cli.command {
        Commands::Keypair { subcommand } => match subcommand {
            KeypairSubcommands::Generate {
                curve,
                output,
                passphrase,
            } => {
                println!("Generating new keypair for curve: {:?}", curve);
                match curve {
                    Curve::Bn254 => {
                        let keypair = bn254::Keypair::generate();
                        println!("Generated BN254 keypair with public key: {keypair}");

                        if let Some(parent) = output.parent() {
                            fs::create_dir_all(parent)?;
                        }

                        fs::File::create(&output)?;

                        let passphrase = match passphrase {
                            Some(passphrase) => passphrase,
                            None => rpassword::prompt_password("Enter keypair passphrase: ")?,
                        };

                        let sk_bytes: Vec<u8> = keypair.try_into()?;

                        let passphrase_bytes = passphrase.as_bytes();

                        println!("Encrypting keypair...");

                        // TODO: I kept the scrypt_log_n parameter to 15 here but this is orders of magnitude less secure than the geth keystore value of 18
                        //       This is in the interest of testing speed for now but we should update it later once we finalize the params
                        let encrypted_keypair =
                            keystore::encrypt_data_v3(&sk_bytes, passphrase_bytes, 14, 1)?;

                        keystore::save_to_file(&encrypted_keypair, &output)?;

                        let resolved_path = output.canonicalize()?;
                        let resolved_path_str =
                            resolved_path.to_str().ok_or(eyre!("Path is invalid"))?;
                        println!("Saved keypair to {resolved_path_str}");
                    }
                }
            }
            KeypairSubcommands::Pubkey {
                curve,
                keypair,
                passphrase,
            } => match curve {
                Curve::Bn254 => {
                    let encrypted_keypair = keystore::load_from_file(&keypair)?;

                    let passphrase = match passphrase {
                        Some(passphrase) => passphrase,
                        None => rpassword::prompt_password("Enter keypair passphrase: ")?,
                    };

                    let sk_bytes =
                        keystore::decrypt_data_v3(&encrypted_keypair, passphrase.as_bytes())?;

                    let keypair = bn254::Keypair::try_from(sk_bytes.as_slice())?;
                    println!("Public Key: {keypair}");
                }
            },
        },
        Commands::Sign {
            scheme,
            message,
            message_encoding,
            keypair,
            passphrase,
        } => match scheme {
            Scheme::Bls => {
                let message_bytes = match message_encoding {
                    Encoding::Utf8 => message.as_bytes().to_vec(),
                    Encoding::Hex => hex::decode(message)?,
                    Encoding::Base64 => {
                        base64::engine::general_purpose::STANDARD.decode(message)?
                    }
                    Encoding::Base64URL => {
                        base64::engine::general_purpose::URL_SAFE.decode(message)?
                    }
                    Encoding::Base58 => bs58::decode(message).into_vec()?,
                };

                // We Keccak256 hash the message to a 32 bytes hash

                let mut hasher = Keccak256::new();
                hasher.update(message_bytes);
                let result = hasher.finalize();

                let mut hash_buffer = [0u8; 32];
                hash_buffer.copy_from_slice(&result);

                let encrypted_keypair = keystore::load_from_file(&keypair)?;

                let passphrase = match passphrase {
                    Some(passphrase) => passphrase,
                    None => rpassword::prompt_password("Enter keypair passphrase: ")?,
                };

                let sk_bytes =
                    keystore::decrypt_data_v3(&encrypted_keypair, passphrase.as_bytes())?;

                let keypair: bn254::Keypair = sk_bytes.as_slice().try_into()?;
                println!("Signing with BN254 keypair: {keypair}");

                let signer = bls::keypair_signer::KeypairSigner::from(keypair);

                let signature = signer.sign_message(&hash_buffer)?;
                let bs58_encoded_signature = bs58::encode(signature).into_string();
                println!("Signature: {bs58_encoded_signature}");
            }
        },
        Commands::SigVerify {
            signature,
            message,
            message_encoding,
            public_key,
        } => {
            let message_bytes = match message_encoding {
                Encoding::Utf8 => message.as_bytes().to_vec(),
                Encoding::Hex => hex::decode(message)?,
                Encoding::Base64 => base64::engine::general_purpose::STANDARD.decode(message)?,
                Encoding::Base64URL => base64::engine::general_purpose::URL_SAFE.decode(message)?,
                Encoding::Base58 => bs58::decode(message).into_vec()?,
            };

            // We Keccak256 hash the message to a 32 bytes hash

            let mut hasher = Keccak256::new();
            hasher.update(message_bytes);
            let result = hasher.finalize();

            let mut hash_buffer = [0u8; 32];
            hash_buffer.copy_from_slice(&result);

            let public_key = G2Pubkey::from_str(&public_key)?;
            let signature = Signature::from_str(&signature)?;

            match verify_signature(&public_key, &signature, &hash_buffer) {
                Ok(_) => println!("Signature is valid"),
                _ => println!("Signature verification failed!"),
            }
        }
    }

    Ok(())
}
