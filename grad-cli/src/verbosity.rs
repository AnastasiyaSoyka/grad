use clap::Args;
use tracing::level_filters::LevelFilter;

/// Log verbosity configuration.
#[derive(Args, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[group(multiple = false)]
pub struct Verbosity {
    /// Suppress all messages, including errors.
    #[arg(short = 's', long = "silent", global = true)]
    silent: bool,

    /// Suppress informational messages and warnings.
    #[arg(short = 'q', long = "quiet", global = true)]
    quiet: bool,

    /// Emit informational messages, as well as warnings and errors.
    #[arg(short = 'v', long = "verbose", global = true)]
    verbose: bool,

    /// Emit debugging messages.
    #[arg(short = 'd', long = "debug", global = true)]
    debug: bool,

    /// Emit tracing messages.
    #[arg(short = 't', long = "trace", global = true)]
    trace: bool
}

impl From<Verbosity> for LevelFilter {
    fn from(value: Verbosity) -> Self {
        match value {
            Verbosity { silent: true, .. } => LevelFilter::OFF,
            Verbosity { quiet: true, .. } => LevelFilter::ERROR,
            Verbosity { silent: false, quiet: false, verbose: false, debug: false, trace: false } => LevelFilter::WARN,
            Verbosity { verbose: true, .. } => LevelFilter::INFO,
            Verbosity { debug: true, .. } => LevelFilter::DEBUG,
            Verbosity { trace: true, .. } => LevelFilter::TRACE
        }
    }
}

impl From<&Verbosity> for LevelFilter {
    fn from(value: &Verbosity) -> Self {
        match value {
            Verbosity { silent: true, .. } => LevelFilter::OFF,
            Verbosity { quiet: true, .. } => LevelFilter::ERROR,
            Verbosity { silent: false, quiet: false, verbose: false, debug: false, trace: false } => LevelFilter::WARN,
            Verbosity { verbose: true, .. } => LevelFilter::INFO,
            Verbosity { debug: true, .. } => LevelFilter::DEBUG,
            Verbosity { trace: true, .. } => LevelFilter::TRACE
        }
    }
}
