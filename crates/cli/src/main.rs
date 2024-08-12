use alloy::{
    network::EthereumWallet,
    primitives::{Address, FixedBytes},
    signers::local::{LocalSigner, PrivateKeySigner},
    transports::http::reqwest::Url,
};
use base64::Engine;
use clap::{command, Parser, Subcommand, ValueEnum};
use color_eyre::eyre::eyre;
use karak_contracts::registration::{BlsRegistration, OperatorRegistration};
use karak_sdk::{
    keypair::{
        bn254::{self, G2Pubkey},
        traits::Keypair,
    },
    keystore::{
        self,
        aws::AwsKeystoreParams,
        local::LocalEncryptedKeystore,
        traits::{AsyncEncryptedKeystore, EncryptedKeystore},
    },
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

    #[arg(short, long, required = false, default_value = "http://localhost:8545")]
    rpc_url: Url,
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

        /// Keystore to retrieve the keypair
        #[arg(long)]
        keystore: Keystore,

        /// Keypair to retrieve
        #[arg(short, long)]
        keypair: String,

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
    /// Aggregate BLS signatures
    SigAgg {
        #[arg(short, long)]
        signatures: Vec<String>,
    },
    /// Aggregate BLS BN254 G2 pubkeys
    KeyAgg {
        #[arg(short, long)]
        keys: Vec<String>,
    },
    /// Register an operator to a DSS
    RegisterOperator {
        #[arg(short, long)]
        dss: Address,

        #[arg(short, long)]
        message: String,

        #[arg(short, long)]
        message_encoding: Encoding,

        #[arg(short, long)]
        core_address: Address,
    },
}

#[derive(Clone, ValueEnum, Debug)]
enum Curve {
    /// BN254 (also known as alt_bn128) is the curve used in Ethereum for BLS aggregation
    Bn254,
    /// Secp256k1 is the curve used in Ethereum for ECDSA signatures
    Secp256k1,
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

#[derive(Clone, ValueEnum, Debug)]
enum Keystore {
    Local,
    Aws,
}

#[derive(Subcommand)]
enum KeypairSubcommands {
    /// Generate a new keypair
    Generate {
        /// Curve to use for key generation
        #[arg(short, long, value_enum)]
        curve: Curve,

        /// Keystore to save the keypair
        #[arg(short, long)]
        keystore: Keystore,

        /// Passphrase to encrypt keypair
        #[arg(short, long)]
        passphrase: Option<String>,
    },
    /// View public key
    Pubkey {
        /// Curve to use for key generation
        #[arg(short, long, value_enum)]
        curve: Curve,

        /// Keystore to retrieve the keypair
        #[arg(short = 's', long)]
        keystore: Keystore,

        /// Keypair to retrieve
        #[arg(short, long)]
        keypair: Option<String>,

        /// Passphrase to decrypt keypair
        #[arg(short, long)]
        passphrase: Option<String>,
    },
}

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let cli = Cli::parse();
    match cli.command {
        Commands::Keypair { subcommand } => match subcommand {
            KeypairSubcommands::Generate {
                curve,
                keystore,
                passphrase,
            } => {
                println!("Generating new keypair for curve: {:?}", curve);
                match curve {
                    Curve::Bn254 => {
                        let keypair = bn254::Keypair::generate();
                        println!("Generated BN254 keypair with public key: {keypair}");

                        let passphrase = match passphrase {
                            Some(passphrase) => passphrase,
                            None => rpassword::prompt_password("Enter keypair passphrase: ")?,
                        };

                        match keystore {
                            Keystore::Local => {
                                let output = PathBuf::from(format!("{keypair}.bls"));

                                if let Some(parent) = output.parent() {
                                    fs::create_dir_all(parent)?;
                                }

                                fs::File::create(&output)?;

                                let local_keystore =
                                    keystore::local::LocalEncryptedKeystore::new(output.clone());
                                local_keystore.store(&keypair, &passphrase)?;

                                let resolved_path = output.canonicalize()?;
                                let resolved_path_str =
                                    resolved_path.to_str().ok_or(eyre!("Path is invalid"))?;
                                println!("Saved keypair to {resolved_path_str}");
                            }
                            Keystore::Aws => {
                                let config = aws_config::load_from_env().await;
                                let aws_keystore =
                                    keystore::aws::AwsEncryptedKeystore::new(&config);

                                let secret_name = format!("{keypair}.bls");

                                aws_keystore
                                    .store(
                                        &keypair,
                                        &passphrase,
                                        &AwsKeystoreParams {
                                            secret_name: secret_name.clone(),
                                        },
                                    )
                                    .await?;

                                println!("Saved keypair to {secret_name} in AWS Secrets Manager");
                            }
                        }
                    }
                    Curve::Secp256k1 => match keystore {
                        Keystore::Local => {
                            let password = match passphrase {
                                Some(passphrase) => passphrase,
                                None => rpassword::prompt_password("Enter keypair passphrase: ")?,
                            };
                            let mut rng = rand::thread_rng();
                            let private_key = PrivateKeySigner::random();
                            println!(
                                "Generated secp256k1 keypair with address: {}",
                                private_key.address()
                            );

                            let keypath = dirs_next::home_dir()
                                .ok_or(eyre!("Could not find home directory"))?
                                .join(".config")
                                .join("karak");
                            fs::create_dir_all(&keypath)?;
                            let filename = "secp256k1.json";
                            let fullpath = keypath.join(filename);
                            LocalSigner::encrypt_keystore(
                                keypath,
                                &mut rng,
                                private_key.to_bytes(),
                                password,
                                Some(filename),
                            )?;
                            let resolved_path = fullpath.canonicalize()?;
                            let resolved_path_str =
                                resolved_path.to_str().ok_or(eyre!("Path is invalid"))?;
                            println!("Saved keypair to {resolved_path_str}");
                        }
                        Keystore::Aws => {
                            // TODO: Implement
                            unimplemented!();
                        }
                    },
                }
            }
            KeypairSubcommands::Pubkey {
                curve,
                keystore,
                keypair,
                passphrase,
            } => match curve {
                Curve::Bn254 => {
                    let passphrase = match passphrase {
                        Some(passphrase) => passphrase,
                        None => rpassword::prompt_password("Enter keypair passphrase: ")?,
                    };

                    match keystore {
                        Keystore::Local => {
                            let keypair = match keypair {
                                Some(keypair) => PathBuf::from_str(&keypair)?,
                                None => dirs_next::home_dir()
                                    .ok_or(eyre!("Could not find home directory"))?
                                    .join(".config")
                                    .join("karak")
                                    .join("keypair.bls"),
                            };

                            let local_keystore =
                                keystore::local::LocalEncryptedKeystore::new(keypair);

                            let keypair: bn254::Keypair = local_keystore.retrieve(&passphrase)?;

                            println!("Public Key (retrieved from local keystore): {keypair}");
                        }
                        Keystore::Aws => {
                            let config = aws_config::load_from_env().await;
                            let aws_keystore = keystore::aws::AwsEncryptedKeystore::new(&config);

                            let keypair = keypair.ok_or(eyre!("Keypair name is required"))?;
                            let secret_name = format!("{keypair}.bls");

                            let keypair: bn254::Keypair = aws_keystore
                                .retrieve(&passphrase, &AwsKeystoreParams { secret_name })
                                .await?;

                            println!("Public Key (retrieved from AWS Secrets Manager): {keypair}");
                        }
                    }
                }
                Curve::Secp256k1 => match keystore {
                    Keystore::Local => {
                        let password = match passphrase {
                            Some(passphrase) => passphrase,
                            None => rpassword::prompt_password("Enter keypair passphrase: ")?,
                        };
                        let keypath = match keypair {
                            Some(keypair) => PathBuf::from_str(&keypair)?,
                            None => dirs_next::home_dir()
                                .ok_or(eyre!("Could not find home directory"))?
                                .join(".config")
                                .join("karak")
                                .join("secp256k1.json"),
                        };

                        let private_key = LocalSigner::decrypt_keystore(keypath, password)?;
                        println!(
                            "Public Key (retrieved from local keystore): {}",
                            private_key.address()
                        );
                    }
                    Keystore::Aws => {
                        // TODO: Implement
                        unimplemented!();
                    }
                },
            },
        },
        Commands::Sign {
            scheme,
            message,
            message_encoding,
            keystore,
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

                let passphrase = match passphrase {
                    Some(passphrase) => passphrase,
                    None => rpassword::prompt_password("Enter keypair passphrase: ")?,
                };

                let keypair: bn254::Keypair = {
                    match keystore {
                        Keystore::Local => {
                            let local_keystore = keystore::local::LocalEncryptedKeystore::new(
                                PathBuf::from(keypair),
                            );
                            local_keystore.retrieve(&passphrase)?
                        }
                        Keystore::Aws => {
                            let config = aws_config::load_from_env().await;
                            let aws_keystore = keystore::aws::AwsEncryptedKeystore::new(&config);
                            let secret_name = format!("{keypair}.bls");
                            aws_keystore
                                .retrieve(&passphrase, &AwsKeystoreParams { secret_name })
                                .await?
                        }
                    }
                };

                println!("Signing with BN254 keypair: {keypair}");

                let signer = bls::keypair_signer::KeypairSigner::from(keypair);

                let signature = signer.sign_message(&hash_buffer)?;
                println!("Signature: {signature}");
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

            match verify_signature(&public_key, &signature, hash_buffer) {
                Ok(_) => println!("Signature is valid"),
                _ => println!("Signature verification failed!"),
            }
        }
        Commands::SigAgg { signatures } => {
            let signatures: Vec<Signature> = signatures
                .iter()
                .map(|signature| Signature::from_str(signature).unwrap())
                .collect();

            // TODO: Does this work? I.e. does adding default to existing signatures mess it up?
            let agg_signature: Signature = signatures.iter().sum();

            println!("Aggregated signature: {agg_signature}");
        }
        Commands::KeyAgg { keys } => {
            let keys: Vec<G2Pubkey> = keys
                .iter()
                .map(|key| G2Pubkey::from_str(key).unwrap())
                .collect();

            // TODO: Does this work? I.e. does adding default to existing keys mess it up?
            let agg_key: G2Pubkey = keys.iter().sum();

            println!("Aggregated key: {agg_key}");
        }
        Commands::RegisterOperator {
            dss,
            message,
            message_encoding,
            core_address,
        } => {
            let bn254_keypair_path = dirs_next::home_dir()
                .ok_or(eyre!("Could not find home directory"))?
                .join(".config")
                .join("karak")
                .join("bn254.json");
            let secp256k1_keypair_path = dirs_next::home_dir()
                .ok_or(eyre!("Could not find home directory"))?
                .join(".config")
                .join("karak")
                .join("secp256k1.json");
            let passphrase =
                rpassword::prompt_password("Enter keypair passphrase for your BN254 keypair: ")?;
            let bn254_keypair: bn254::Keypair =
                LocalEncryptedKeystore::new(bn254_keypair_path).retrieve(&passphrase)?;
            let passphrase = rpassword::prompt_password(
                "Enter keypair passphrase for your secp256k1 keypair: ",
            )?;
            let secp256k1_keypair =
                LocalSigner::decrypt_keystore(secp256k1_keypair_path, passphrase)?;

            let message_bytes = match message_encoding {
                Encoding::Utf8 => message.as_bytes().to_vec(),
                Encoding::Hex => hex::decode(message)?,
                Encoding::Base64 => base64::engine::general_purpose::STANDARD.decode(message)?,
                Encoding::Base64URL => base64::engine::general_purpose::URL_SAFE.decode(message)?,
                Encoding::Base58 => bs58::decode(message).into_vec()?,
            };
            let msg_hash = FixedBytes::<32>::from_slice(&message_bytes);
            let signer = bls::keypair_signer::KeypairSigner::from(bn254_keypair.clone());
            let signature = signer.sign_message(msg_hash)?;

            let registration = BlsRegistration {
                g1_pubkey: bn254_keypair.public_key().g1,
                g2_pubkey: bn254_keypair.public_key().g2,
                msg_hash,
                signature,
            };

            let wallet = EthereumWallet::from(secp256k1_keypair);
            let provider = alloy::providers::ProviderBuilder::new()
                .with_recommended_fillers()
                .wallet(wallet)
                .on_http(cli.rpc_url.clone());
            let core = karak_contracts::Core::CoreInstance::new(core_address, provider);
            core.register_operator_to_dss_with_bls(dss, &registration)
                .await?;
        }
    }

    Ok(())
}
