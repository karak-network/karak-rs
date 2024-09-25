mod bls;
mod keypair;

use bls::{process_aggregate, process_sign, process_verify, AggregateParams, MessageArgs};
use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::common::{Curve, Encoding, Keystore};
use crate::components::dss::keypair::{
    process_generate, process_pubkey, KeypairArgs, KeypairLocationArgs,
};
use crate::components::Component;
use crate::{CliError, Profile};

pub struct Dss;

impl Component for Dss {
    fn add_commands(cmd: Command) -> Command {
        let keypair_cmd = Command::new("keypair")
            .subcommand(Command::new("generate"))
            .subcommand(Command::new("pubkey").args([Arg::new("keypair").long("keypair")]));

        let bls_cmd = Command::new("bls")
            .subcommand(
                Command::new("sign").args([
                    Arg::new("keypair").long("keypair"),
                    Arg::new("message").long("message"),
                    Arg::new("encoding")
                        .long("encoding")
                        .short('e')
                        .value_parser(clap::builder::EnumValueParser::<Encoding>::new()),
                ]),
            )
            .subcommand(
                Command::new("verify").args([
                    Arg::new("pubkey").long("pubkey"),
                    Arg::new("signature").long("signature"),
                    Arg::new("message").long("message"),
                    Arg::new("encoding")
                        .long("encoding")
                        .short('e')
                        .value_parser(clap::builder::EnumValueParser::<Encoding>::new()),
                ]),
            )
            .subcommand(
                Command::new("aggregate")
                    .subcommand(
                        Command::new("signatures").arg(
                            Arg::new("signatures")
                                .long("signatures")
                                .action(ArgAction::Append),
                        ),
                    )
                    .subcommand(
                        Command::new("pubkeys").arg(
                            Arg::new("pubkeys")
                                .long("pubkeys")
                                .action(ArgAction::Append),
                        ),
                    ),
            );

        cmd.subcommand(keypair_cmd).subcommand(bls_cmd).args([
            Arg::new("keystore")
                .long("keystore")
                .value_parser(clap::builder::EnumValueParser::<Keystore>::new())
                .global(true),
            Arg::new("passphrase")
                .long("passphrase")
                .required(false)
                .global(true),
            Arg::new("curve")
                .long("curve")
                .value_parser(clap::builder::EnumValueParser::<Curve>::new())
                .global(true),
        ])
    }

    async fn run(args: &ArgMatches, _: &Profile) -> Result<(), CliError> {
        match args.subcommand() {
            Some(("keypair", sub_args)) => match sub_args.subcommand() {
                Some(("generate", sub_args)) => {
                    let keypair_args = KeypairArgs {
                        keystore: sub_args.get_one::<Keystore>("keystore").unwrap().to_owned(),
                        passphrase: sub_args.get_one::<String>("passphrase").cloned(),
                    };

                    let curve = sub_args.get_one::<Curve>("curve").unwrap().to_owned();

                    process_generate(keypair_args, curve)
                        .await
                        .map_err(|e| CliError::ComponentError(e.to_string()))
                }
                Some(("pubkey", sub_args)) => {
                    let keypair_args = KeypairArgs {
                        keystore: sub_args.get_one::<Keystore>("keystore").unwrap().to_owned(),
                        passphrase: Some(
                            sub_args.get_one::<String>("passphrase").unwrap().to_owned(),
                        ),
                    };

                    let keypair_location_args = KeypairLocationArgs {
                        keypair: sub_args.get_one::<String>("keypair").unwrap().to_owned(),
                    };

                    let curve = sub_args.get_one::<Curve>("curve").unwrap().to_owned();

                    process_pubkey(keypair_args, keypair_location_args, curve)
                        .await
                        .map_err(|e| CliError::ComponentError(e.to_string()))
                }
                _ => Err(CliError::UnknownCommand),
            },
            Some(("bls", sub_args)) => match sub_args.subcommand() {
                Some(("sign", sub_args)) => {
                    let keypair_location_args = KeypairLocationArgs {
                        keypair: sub_args.get_one::<String>("keypair").unwrap().to_owned(),
                    };

                    let keypair_args = KeypairArgs {
                        keystore: sub_args.get_one::<Keystore>("keystore").unwrap().to_owned(),
                        passphrase: Some(
                            sub_args.get_one::<String>("passphrase").unwrap().to_owned(),
                        ),
                    };

                    let message_args = MessageArgs {
                        message: sub_args.get_one::<String>("message").unwrap().to_owned(),
                        message_encoding: sub_args
                            .get_one::<Encoding>("encoding")
                            .unwrap()
                            .to_owned(),
                    };

                    process_sign(keypair_location_args, keypair_args, message_args)
                        .await
                        .map_err(|e| CliError::ComponentError(e.to_string()))
                }

                Some(("verify", sub_args)) => {
                    let message_args = MessageArgs {
                        message: sub_args.get_one::<String>("message").unwrap().to_owned(),
                        message_encoding: sub_args
                            .get_one::<Encoding>("encoding")
                            .unwrap()
                            .to_owned(),
                    };

                    let pubkey = sub_args.get_one::<String>("pubkey").unwrap().to_owned();

                    let signature = sub_args.get_one::<String>("signature").unwrap().to_owned();

                    process_verify(message_args, pubkey, signature)
                        .map_err(|e| CliError::ComponentError(e.to_string()))
                }

                Some(("bls", sub_args)) => match sub_args.subcommand() {
                    Some(("signatures", sub_args)) => {
                        let signatures = sub_args
                            .get_many::<String>("signatures")
                            .unwrap_or_default()
                            .map(|v| v.to_owned())
                            .collect::<Vec<_>>();

                        let params = AggregateParams::Signatures(signatures);

                        process_aggregate(params)
                            .map_err(|e| CliError::ComponentError(e.to_string()))
                    }

                    Some(("pubkeys", sub_args)) => {
                        let pubkeys = sub_args
                            .get_many::<String>("pubkeys")
                            .unwrap_or_default()
                            .map(|v| v.to_owned())
                            .collect::<Vec<_>>();

                        let params = AggregateParams::Pubkeys(pubkeys);

                        process_aggregate(params)
                            .map_err(|e| CliError::ComponentError(e.to_string()))
                    }
                    _ => Err(CliError::UnknownCommand),
                },

                _ => Err(CliError::UnknownCommand),
            },
            _ => Err(CliError::UnknownCommand),
        }
    }
}
