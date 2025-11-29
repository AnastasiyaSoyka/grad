use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::LazyLock;

use regex::Regex;
use thiserror::Error;
use derive_more::{Constructor, Debug, From, Into};

#[derive(std::fmt::Debug, Error)]
pub enum ParseRangeError {
    #[error("The haystack '{0}' did not match the pattern '{1}'.")]
    NoMatch(String, String),

    #[error(transparent)]
    ParseInt(#[from] ParseIntError)
}

#[derive(Clone, Default, PartialEq, Eq, Hash, Debug, Constructor, From, Into)]
pub struct Range<Idx>(std::ops::Range<Idx>);

#[derive(Clone, PartialEq, Eq, Hash, Debug, Constructor, From, Into)]
pub struct RangeInclusive<Idx>(std::ops::RangeInclusive<Idx>);

macro_rules! from_str_impl {
    ($($type:ident),*) => {
        $(
            impl FromStr for Range<$type> {
                type Err = ParseRangeError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?<start>-?\d+)\.\.(?<end>-?\d+)").unwrap());

                    let haystack = s.trim();
                    let captures = RE.captures(haystack).ok_or(ParseRangeError::NoMatch(haystack.to_string(), RE.to_string()))?;
                    let start = $type::from_str(&captures["start"])?;
                    let end = $type::from_str(&captures["end"])?;
                    let instance = Range::new(start..end);

                    Ok(instance)
                }
            }

            impl FromStr for RangeInclusive<$type> {
                type Err = ParseRangeError;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    static RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?<start>-?\d+)\.\.=(?<end>-?\d+)").unwrap());

                    let haystack = s.trim();
                    let captures = RE.captures(haystack).ok_or(ParseRangeError::NoMatch(haystack.to_string(), RE.to_string()))?;
                    let start = $type::from_str(&captures["start"])?;
                    let end = $type::from_str(&captures["end"])?;

                    let instance = RangeInclusive::new(start..=end);

                    Ok(instance)
                }
            }
        )*
    }
}

from_str_impl!(usize, isize, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use paste::paste;

    macro_rules! can_parse_impl {
        ($($type:ident),*) => {
            $(
                paste! {
                    #[test]
                    fn [<can_parse_range_ $type>]() {
                        let string = format!("{0}..{1}", $type::MIN, $type::MAX);
                        let range = $type::MIN..$type::MAX;

                        assert_eq!(range, std::ops::Range::<$type>::from(super::Range::<$type>::from_str(&string).unwrap()));
                    }

                    #[test]
                    fn [<can_parse_range_inclusive_ $type>]() {
                        let string = format!("{0}..={1}", $type::MIN, $type::MAX);
                        let range = $type::MIN..=$type::MAX;

                        assert_eq!(range, std::ops::RangeInclusive::<$type>::from(super::RangeInclusive::<$type>::from_str(&string).unwrap()));
                    }

                    #[test]
                    #[should_panic]
                    fn [<panics_on_range_ $type>]() {
                        super::Range::<$type>::from_str("FAIL").unwrap();
                    }

                    #[test]
                    #[should_panic]
                    fn [<panics_on_range_inclusive_ $type>]() {
                        super::RangeInclusive::<$type>::from_str("FAIL").unwrap();
                    }
                }
            )*
        };
    }

    can_parse_impl!(usize, isize, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128);
}
