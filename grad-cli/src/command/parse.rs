use std::str::FromStr;

use either::Either;
use thiserror::Error;

use grad_parse::range::{ParseRangeError, Range, RangeInclusive};

#[derive(Debug, Error)]
#[error("The following errors occurred during parsing:\n\t{0}\n\t{1}")]
pub struct ParseError(ParseRangeError, ParseRangeError);

pub fn parse_range(value: &str) -> Result<Either<Range<i128>, RangeInclusive<i128>>, ParseError> {
    match (Range::<i128>::from_str(value), RangeInclusive::from_str(value)) {
        (Ok(range), ..) => Ok(Either::Left(range)),
        (.., Ok(range)) => Ok(Either::Right(range)),
        (Err(left), Err(right)) => Err(ParseError(left, right)),
    }
}
