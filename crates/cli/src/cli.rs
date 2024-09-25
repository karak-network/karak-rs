use std::path::Path;

use clap::{command, Arg, ArgMatches, Command};
use color_eyre::eyre::Result;

use crate::{
    components::{Component, Dss, Operator},
    CliError, Config, Profile,
};

pub struct Cli {
    pub cmd: Command,
    pub config_path: String,
    pub runner: Runner,
}

impl Cli {
    pub fn new(runner: Runner, about: &'static str, config_path: Option<String>) -> Self {
        let config_default = concat!(env!("HOME"), "/.karak");
        Self {
            cmd: command!(runner.name())
                .about(about)
                .flatten_help(true)
                .next_line_help(true)
                .arg_required_else_help(true)
                .propagate_version(true)
                .subcommand_required(true)
                .arg(
                    Arg::new("profile")
                        .short('p')
                        .long("profile")
                        .value_name("PROFILE")
                        .global(true)
                        .required(false)
                        .default_value("default")
                        .help("Selects the config profile to be used"),
                )
                .arg(
                    Arg::new("config")
                        .short('c')
                        .long("config")
                        .value_name("CONFIG")
                        .global(true)
                        .required(false)
                        .default_value(config_default)
                        .help(format!(
                            "Changes the default config file path. Default: {}",
                            config_default
                        )),
                ),
            runner,
            config_path: config_path.unwrap_or(config_default.to_string()),
        }
    }

    pub fn with_configure(mut self) -> Self {
        self.cmd = Config::add_commands(self.cmd);
        self
    }

    pub fn with_completions(mut self) -> Self {
        self.cmd = self.cmd.subcommand(command!());
        self
    }

    pub fn with_component<T: Component>(mut self, parent: Option<&'static str>) -> Self {
        match parent {
            Some(name) => {
                let mut parent = Command::new(name);
                parent = T::add_commands(parent);
                self.cmd = self.cmd.subcommand(parent);
                self
            }
            None => {
                self.cmd = T::add_commands(self.cmd);
                self
            }
        }
    }

    pub async fn run(&self) -> Result<(), CliError> {
        let matches = self.cmd.clone().get_matches();

        match matches.subcommand() {
            Some(("configure", sub_matches)) => Config::run(sub_matches),
            Some(("completions", _)) => {
                println!("Generating completions");
                Ok(())
            }
            _ => match self.runner {
                Runner::Karak => self.karak_matcher(matches).await,
                Runner::Operator => self.operator_matcher(matches).await,
                Runner::Dss => self.dss_matcher(matches).await,
            },
        }
    }

    fn pre_run(&self, matches: &ArgMatches) -> Result<Profile, CliError> {
        let config = Config::read_config(Path::new(matches.get_one::<String>("config").unwrap()))?
            .get_profile(matches.get_one::<String>("profile").unwrap())?;

        Ok(config)
    }

    async fn karak_matcher(&self, args: ArgMatches) -> Result<(), CliError> {
        let config = self.pre_run(&args)?;

        match args.subcommand() {
            Some(("operator", sub_matches)) => {
                Operator::run(sub_matches, &config).await?;
                Ok(())
            }
            Some(("dss", sub_matches)) => {
                Dss::run(sub_matches, &config).await?;
                Ok(())
            }
            _ => {
                println!("Unknown command");
                Err(CliError::UnknownCommand)
            }
        }
    }

    async fn operator_matcher(&self, args: ArgMatches) -> Result<(), CliError> {
        let config = self.pre_run(&args)?;

        Operator::run(&args, &config).await?;
        Ok(())
    }

    async fn dss_matcher(&self, args: ArgMatches) -> Result<(), CliError> {
        let config = self.pre_run(&args)?;

        Dss::run(&args, &config).await?;
        Ok(())
    }
}

pub enum Runner {
    Karak,
    Operator,
    Dss,
}

impl Runner {
    fn name(&self) -> &'static str {
        match self {
            Runner::Karak => "karak",
            Runner::Operator => "operator",
            Runner::Dss => "dss",
        }
    }
}
