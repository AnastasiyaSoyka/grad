use std::sync::LazyLock;

use rand::prelude::*;
use rand::distr::slice::Choose;
use rand_chacha::ChaChaRng;

const CHARS: &[char; 10] = &['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

static DISTR: LazyLock<Choose<char>> = LazyLock::new(|| Choose::new(CHARS).unwrap());

pub fn random_digit(length: usize) -> Vec<u8> {
    if length == 0 { return Vec::new() }

    let rng = ChaChaRng::from_rng(&mut rand::rng());

    rng.sample_iter(*DISTR).take(length).collect::<String>().into_bytes()
}

#[cfg(test)]
mod tests {
    #[test]
    fn generates_zero_digits() {
        let bytes = super::random_digit(0);

        assert_eq!(bytes.len(), 0)
    }

    #[test]
    fn generates_ten_thousand_digits() {
        let bytes = super::random_digit(10000);
        let string = std::str::from_utf8(&bytes).unwrap();

        assert_eq!(string.chars().count(), 10000)
    }
}
