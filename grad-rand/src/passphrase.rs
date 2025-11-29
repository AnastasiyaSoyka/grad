use std::borrow::Borrow;

use itertools::Itertools;
use rand::distr::slice::Choose;
use rand::prelude::*;
use rand_chacha::ChaChaRng;
use tracing::{info, instrument};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WordList {
    inner: Vec<String>
}

impl WordList {
    pub fn inner(&self) -> &[String] {
        &self.inner
    }
}

impl Default for WordList {
    #[instrument]
    fn default() -> Self {
        let inner = grad_io::decompress_lines(include_bytes!("wordlist.zst")).unwrap();

        info!("Initialized default word list with {} words.", inner.len());

        Self { inner }
    }
}

pub fn random_passphrase<W, S>(words: W, separator: S, length: usize) -> Vec<u8>
where
    W: Borrow<WordList>, S: AsRef<str>
{
    if length == 0 {
        return Vec::new();
    }

    let rng = ChaChaRng::from_rng(&mut rand::rng());
    let distribution = Choose::new(words.borrow().inner()).unwrap();

    rng.sample_iter(distribution)
        .take(length)
        .join(separator.as_ref())
        .into_bytes()
}

#[cfg(test)]
mod tests {
    fn word_count(buffer: &Vec<u8>) -> usize {
        std::str::from_utf8(buffer)
            .unwrap()
            .split(' ')
            .count()
    }

    #[test]
    fn can_generate_ten_thousand_word_passphrase() {
        let words = super::WordList::default();
        let bytes = super::random_passphrase(&words, " ", 10000);
        let count = word_count(&bytes);

        assert_eq!(count, 10000)
    }

    #[test]
    fn can_generate_hundred_thousand_word_passphrase() {
        let words = super::WordList::default();
        let bytes = super::random_passphrase(&words, " ", 100000);
        let count = word_count(&bytes);

        assert_eq!(count, 100000)
    }

    #[test]
    fn can_generate_empty_passphrase() {
        let words = super::WordList::default();
        let bytes = super::random_passphrase(&words, " ", 0);

        assert_eq!(bytes.len(), 0)
    }

    #[test]
    #[should_panic]
    fn empty_wordlist_panics() {
        let words = super::WordList { inner: Vec::new() };

        super::random_passphrase(&words, " ", 1);
    }
}
