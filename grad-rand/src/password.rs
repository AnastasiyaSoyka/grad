use rand::prelude::*;
use rand::distr::slice::Choose;
use rand_chacha::ChaChaRng;

pub fn random_password(chars: &[char], length: usize) -> Vec<u8> {
    if length == 0 { return Vec::new() }

    let rng = ChaChaRng::from_rng(&mut rand::rng());
    let distribution = Choose::new(chars).unwrap();

    rng.sample_iter(distribution).take(length).collect::<String>().into_bytes()
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_generate_empty_password() {
        let chars: Vec<char> = "!@*-_.0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        let bytes = super::random_password(&chars, 0);

        assert_eq!(bytes.len(), 0)
    }

    #[test]
    fn can_generate_ten_thousand_character_password() {
        let chars: Vec<char> = "!@*-_.0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
        let bytes = super::random_password(&chars, 10000);
        let string = std::str::from_utf8(&bytes).unwrap();

        assert_eq!(string.chars().count(), 10000)
    }
}
