use clap::{Args, Subcommand, ValueEnum};
use either::Either;

use grad_parse::range::{Range, RangeInclusive};

mod parse;

/// Defines the commands for the program.
#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    /// Create a specific type of data with the specified properties.
    Create {
        /// The sub-command to execute.
        #[command(subcommand)]
        command: CreateCommand
    },

    /// Generate random data with the specified properties.
    Random {
        /// The sub-command to execute.
        #[command(subcommand)]
        command: RandomCommand
    }
}

#[derive(ValueEnum, Debug, Clone)]
pub enum TimestampFormat {
    ISO8601,
    RFC2822,
    RFC3339
}

#[derive(ValueEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CharacterSet {
    /// Include characters from `[a-z]`.
    LowerCase,

    /// Include characters from `[A-Z]`.
    UpperCase,

    /// Include characters from `[0-9]`.
    Digit,

    /// Include characters from `[!@*-_.]`.
    Symbol
}

const LOWERCASE: &[char; 26] = &[
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm',
    'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
];

const UPPERCASE: &[char; 26] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
];

const DIGIT: &[char; 10] = &[
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'
];

const SYMBOL: &[char; 6] = &['!', '@', '*', '-', '_', '.'];

impl From<CharacterSet> for Vec<char> {
    fn from(value: CharacterSet) -> Self {
        match value {
            CharacterSet::LowerCase => Vec::from(LOWERCASE),
            CharacterSet::UpperCase => Vec::from(UPPERCASE),
            CharacterSet::Digit => Vec::from(DIGIT),
            CharacterSet::Symbol => Vec::from(SYMBOL)
        }
    }
}

impl From<&CharacterSet> for Vec<char> {
    fn from(value: &CharacterSet) -> Self {
        match value {
            CharacterSet::LowerCase => Vec::from(LOWERCASE),
            CharacterSet::UpperCase => Vec::from(UPPERCASE),
            CharacterSet::Digit => Vec::from(DIGIT),
            CharacterSet::Symbol => Vec::from(SYMBOL)
        }
    }
}

#[derive(Subcommand, Debug, Clone)]
pub enum CreateCommand {
    /// Get the current time in the specified format.
    Timestamp {
        #[command(subcommand)]
        command: Option<TimestampCommand>,

        /// The format to use when creating the timestamp.
        #[arg(value_enum, short, long, global = true)]
        format: Option<TimestampFormat>
    },
}

/// Log verbosity configuration.
#[derive(Args, Debug, Clone)]
#[group()]
pub struct Batch {
    /// The humber of batch operations to perform.
    pub iterations: Option<usize>,

    /// The sequence used to separate batched outputs.
    #[arg(short = 'D', long = "delimiter", default_value_t = String::from("\n"), requires = "iterations")]
    pub delimiter: String
}

#[derive(Subcommand, Debug, Clone)]
pub enum RandomCommand {
    /// Generate random bytes.
    Byte {
        /// The number of bytes to generate.
        length: usize
    },
    /// Generate random bytes and encode them as a hexadecimal string.
    Hex {
        /// Print hexadecimal digits in uppercase.
        #[arg(short = 'u', long = "uppercase")]
        uppercase: bool,

        /// The length of the hexadecimal string in bytes.
        length: usize,

        /// Options for batch processing.
        #[command(flatten)]
        batch: Batch
    },
    /// Generate random bytes and encode them as a Base64 string.
    Base64 {
        /// Use an RFC4648 conformant encoding.
        #[arg(short = 'u', long = "urlsafe")]
        url: bool,

        /// The length of the Base64 string in bytes.
        length: usize,

        /// Options for batch processing.
        #[command(flatten)]
        batch: Batch
    },
    /// Generate a random sequence of digits.
    Digit {
        /// The number of digits to generate.
        length: usize,

        /// Options for batch processing.
        #[command(flatten)]
        batch: Batch
    },
    /// Generate a random number from the set of the integers.
    Integer {
        /// A bounded range of integers. Acceptable formats are:
        ///
        /// 1. A half-open interval in the form of start..end (e.g. 0..10).
        ///
        /// 2. A closed interval in the form of start..=end (e.g. 0..=10).
        #[arg(value_parser = parse::parse_range)]
        range: Either<Range<i128>, RangeInclusive<i128>>,

        /// Options for batch processing.
        #[command(flatten)]
        batch: Batch
    },
    /// Generate a random password with a configurable character set.
    Password {
        /// Which characters to include.
        #[arg(short = 'i', long = "include", value_enum, default_values_t = vec![CharacterSet::LowerCase, CharacterSet::UpperCase, CharacterSet::Digit])]
        include: Vec<CharacterSet>,

        /// The length of the password.
        length: usize,

        /// Options for batch processing.
        #[command(flatten)]
        batch: Batch
    },
    /// Generate a passphrase composed of words chosen at random from a word list.
    Passphrase {
        /// A string used to separate words in the passphrase
        #[arg(short = 'S', long = "separator", default_value = " ")]
        separator: String,

        /// The number of words to generate
        length: usize,

        /// Options for batch processing.
        #[command(flatten)]
        batch: Batch
    }
}

#[derive(Subcommand, Debug, Copy, Clone)]
pub enum TimestampCommand {
    /// Create a timestamp using the UTC timezone.
    Utc,

    /// Create a timestamp using the local timezone.
    Local
}
