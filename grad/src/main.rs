use std::ops::{Range, RangeInclusive};
use std::io::{Write, stdout};

use rayon::prelude::*;
use tracing::{instrument, trace};

use grad::time::{local, utc};
use grad::random::{self, WordList};
use grad::cli::command::*;

mod instrumentation;

fn write_out(buffer: &[u8]) -> std::io::Result<()> {
    let mut handle = stdout();

    handle.write_all(buffer)?;
    handle.flush()?;

    Ok(())
}

macro_rules! batch {
    ($iterations:expr, $function:expr) => {
        let mut stdout = stdout();

        if let Some(iterations) = $iterations {
            (0..iterations)
                .into_par_iter()
                .map(|index| {
                    let buffer: Vec<u8> = $function;

                    trace!(index = index, iterations = iterations - 1);

                    (index, buffer)
                })
                .try_for_each(|(index, mut buffer)| {
                    let mut handle = stdout.lock();

                    // Write the delimiter to the buffer if and only if
                    // the current message is not the final message.
                    if index < iterations { buffer.push(b'\n') }

                    handle.write_all(&buffer)
                })?;
        }
        else {
            stdout.write_all(&$function)?;
        }

        stdout.flush()?;
    };
}

#[instrument]
fn execute(command: Command) -> anyhow::Result<()> {
    match command {
        Command::Create { command } => match command {
            CreateCommand::Timestamp { command, format } => {
                let buffer = match command {
                    Some(TimestampCommand::Utc) => match format {
                        Some(TimestampFormat::ISO8601) => utc::now_iso8601(),
                        Some(TimestampFormat::RFC2822) => utc::now_rfc2822(),
                        Some(TimestampFormat::RFC3339) => utc::now_rfc3339(),
                        None => utc::now()
                    }
                    Some(TimestampCommand::Local) | None => match format {
                        Some(TimestampFormat::ISO8601) => local::now_iso8601(),
                        Some(TimestampFormat::RFC2822) => local::now_rfc2822(),
                        Some(TimestampFormat::RFC3339) => local::now_rfc3339(),
                        None => local::now()
                    }
                };

                write_out(&buffer)?;
            }
        }
        Command::Random { command } => match command {
            RandomCommand::Byte { length } => {
                let buffer = random::random_byte(length);

                write_out(&buffer)?;
            }
            RandomCommand::Hex { uppercase, length, batch } => {
                batch!(batch.iterations, random::random_hex(uppercase, length));
            }
            RandomCommand::Base64 { url, length, batch } => {
                batch!(batch.iterations, random::random_base64(url, length));
            }
            RandomCommand::Digit { length, batch } => {
                batch!(batch.iterations, random::random_digit(length));
            }
            RandomCommand::Integer { range, batch } => {
                trace!("{}", range.clone().either(|range| format!("{:?}", range), |range| format!("{:?}", range)));
                batch!(
                    batch.iterations,
                    range.clone().either(
                        |range| random::random_integer(Range::from(range.clone())),
                        |range| random::random_integer(RangeInclusive::from(range.clone()))
                    )
                );
            }
            RandomCommand::Password { include, length, batch } => {
                let chars: Vec<char> = include.iter()
                    .flat_map(Vec::from)
                    .collect();

                batch!(batch.iterations, random::random_password(&chars, length));
            }
            RandomCommand::Passphrase { separator, length, batch } => {
                let words = WordList::default();

                batch!(batch.iterations, random::random_passphrase(&words, &separator, length));
            }
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let arguments = grad::cli::parse()?;

    instrumentation::init_tracing(arguments.verbosity)?;

    execute(arguments.command)
}
