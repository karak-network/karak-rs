pub mod register;

use alloy::primitives::Address;
use clap::{Arg, ArgMatches, Command};
use register::{process_registration, RegistrationArgs};
use url::Url;

use crate::{
    common::{Encoding, Keystore},
    components::Component,
    CliError, Profile,
};

pub struct Operator;

impl Component for Operator {
    fn add_commands(cmd: Command) -> Command {
        cmd.subcommand(
            Command::new("register")
                .about("Register an operator")
                .args([
                    Arg::new("bn254_keypair_location").long("bn254_keypair_location"),
                    Arg::new("bn254_keystore")
                        .long("bn254_keystore")
                        .value_parser(clap::builder::EnumValueParser::<Keystore>::new()),
                    Arg::new("bn254_passphrase")
                        .long("bn254_passphrase")
                        .required(false),
                    Arg::new("secp256k1_keypair_location").long("secp256k1_keypair_location"),
                    Arg::new("secp256k1_keystore")
                        .long("secp256k1_keystore")
                        .value_parser(clap::builder::EnumValueParser::<Keystore>::new()),
                    Arg::new("secp256k1_passphrase")
                        .long("secp256k1_passphrase")
                        .required(false),
                    Arg::new("dss_address")
                        .long("dss_address")
                        .value_parser(clap::value_parser!(Address)),
                    Arg::new("message").long("message"),
                    Arg::new("messageEncoding")
                        .long("messageEncoding")
                        .value_parser(clap::builder::EnumValueParser::<Encoding>::new()),
                    Arg::new("rpc_url")
                        .long("rpc_url")
                        .value_parser(clap::value_parser!(Url)),
                ]),
        )
    }

    async fn run(args: &ArgMatches, profile: &Profile) -> Result<(), CliError> {
        match args.subcommand() {
            Some(("register", sub_args)) => {
                let reg_args = RegistrationArgs {
                    bn254_keypair_location: sub_args
                        .get_one::<String>("bn254_keypair_location")
                        .unwrap(),
                    bn254_keystore: sub_args.get_one::<Keystore>("bn254_keystore").unwrap(),
                    secp256k1_keystore: sub_args.get_one::<Keystore>("secp256k1_keystore").unwrap(),
                    secp256k1_passphrase: Some(
                        sub_args.get_one::<String>("secp256k1_passphrase").unwrap(),
                    ),
                    rpc_url: sub_args.get_one::<Url>("rpc_url").unwrap().to_owned(),
                    core_address: profile.karak_address,
                    dss_address: sub_args
                        .get_one::<Address>("dss_address")
                        .unwrap()
                        .to_owned(),
                    message: sub_args.get_one::<String>("message").unwrap(),
                    message_encoding: sub_args.get_one::<Encoding>("messageEncoding").unwrap(),
                    bn254_passphrase: Some(sub_args.get_one::<String>("bn254_passphrase").unwrap()),
                    secp256k1_keypair_location: sub_args
                        .get_one::<String>("secp256k1_keypair_location")
                        .unwrap(),
                };
                process_registration(reg_args)
                    .await
                    .map_err(|e| CliError::ComponentError(e.to_string()))
            }

            _ => Err(CliError::UnknownCommand),
        }
    }
}
