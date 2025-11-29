use std::fmt::Display;

use num::Integer;
use rand::distr::uniform::{SampleRange, SampleUniform};
use rand::prelude::*;
use rand_chacha::ChaChaRng;

pub fn random_integer<T, R>(range: R) -> Vec<u8>
where
    T: SampleUniform + Display + Integer,
    R: SampleRange<T>
{
    let mut rng = ChaChaRng::from_rng(&mut rand::rng());

    rng.random_range(range).to_string().into_bytes()
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_generate_integer() {
        let bytes = super::random_integer(0..=1024);
        let string = std::str::from_utf8(&bytes).unwrap();
        let number = u64::from_str_radix(string, 10).unwrap();

        assert!(number < 1024, "{} >= {}", number, 1024)
    }
}
