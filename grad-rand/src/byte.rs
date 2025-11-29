use rand::prelude::*;
use rand_chacha::ChaChaRng;
use data_encoding::{HEXLOWER, HEXUPPER, BASE64, BASE64URL};

pub fn random_byte(length: usize)-> Vec<u8> {
    if length == 0 { return Vec::new() }

    let rng = ChaChaRng::from_rng(&mut rand::rng());

    rng.random_iter().take(length).collect()
}

pub fn random_hex(uppercase: bool, length: usize) -> Vec<u8> {
    if length == 0 { return Vec::new() }

    let rng = ChaChaRng::from_rng(&mut rand::rng());
    let buffer: Vec<u8> = rng.random_iter().take(length).collect();

    if uppercase { HEXUPPER.encode(&buffer).into_bytes() }
    else { HEXLOWER.encode(&buffer).into_bytes() }
}

pub fn random_base64(url: bool, length: usize) -> Vec<u8> {
    if length == 0 { return Vec::new() }

    let rng = ChaChaRng::from_rng(&mut rand::rng());
    let buffer: Vec<u8> = rng.random_iter().take(length).collect();

    if url { BASE64URL.encode(&buffer).into_bytes() }
    else { BASE64.encode(&buffer).into_bytes() }
}

#[cfg(test)]
mod tests {
    #[test]
    fn can_generate_one_kilobyte() {
        let bytes = super::random_byte(1024);

        assert_eq!(bytes.len(), 1024)
    }

    #[test]
    fn can_generate_zero_bytes() {
        let bytes = super::random_byte(0);

        assert_eq!(bytes.len(), 0)
    }

    #[test]
    fn can_generate_zero_bytes_as_hex() {
        let bytes = super::random_hex(false, 0);

        assert_eq!(bytes.len(), 0)
    }

    #[test]
    fn can_generate_one_kilobyte_as_hex() {
        let length: usize = 1024;
        let bytes = super::random_hex(false, length);
        let string = std::str::from_utf8(&bytes).unwrap();

        assert_eq!(string.chars().count(), length * 2)
    }

    #[test]
    fn can_generate_zero_bytes_as_base64() {
        let bytes = super::random_base64(false, 0);

        assert_eq!(bytes.len(), 0)
    }

    #[test]
    fn can_generate_one_kilobyte_as_base64() {
        let bytes = super::random_base64(false, 1024);
        let string = std::str::from_utf8(&bytes).unwrap();

        assert_eq!(string.chars().count(), 1368)
    }
}
