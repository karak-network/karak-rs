pub mod dss;
pub mod operator;

pub use dss::Dss;
pub use operator::Operator;

use crate::{CliError, Profile};
use clap::{ArgMatches, Command};

pub trait Component {
    fn add_commands(cmd: Command) -> Command;

    async fn run(args: &ArgMatches, profile: &Profile) -> Result<(), CliError>;
}
