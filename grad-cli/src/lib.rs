use clap::{Parser, Error};

pub mod verbosity;
pub mod command;

use verbosity::Verbosity;
use command::Command;

/// Defines the command-line arguments for the program.
#[derive(Parser, Debug, Clone)]
#[command(name = "grad", author, version, about, long_about, arg_required_else_help = true, propagate_version = true)]
pub struct Arguments {
    #[command(subcommand)]
    pub command: Command,

    #[command(flatten)]
    pub verbosity: Verbosity
}

/// Attempt to parse all program arguments.
pub fn parse() -> Result<Arguments, Error> {
    Arguments::try_parse()
}
