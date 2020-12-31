use std::{iter::FromIterator, ops::BitXor};
use std::ops::Deref;

pub mod hex;

pub struct EncodedData {
    pub bytes: Vec<u8>
}

impl Deref for EncodedData {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl FromIterator<u8> for EncodedData {
    fn from_iter<I: IntoIterator<Item=u8>>(iter: I) -> Self {
        EncodedData { bytes: Vec::from_iter(iter) }
    }
}

impl BitXor for EncodedData {
    type Output = EncodedData;

    fn bitxor(self, other: EncodedData) -> Self::Output {
        self.iter().zip(other.iter())
            .map(|(a, b)| a ^ b)
            .collect()
    }
}

#[allow(dead_code)]
pub fn hex_to_base64(hex_str: &str) -> String {
    let utf8_bytes: EncodedData = EncodedData::from_hex(hex_str)
        .chunks(3)
        .flat_map(process_chunk)
        .collect();
    return String::from_utf8(utf8_bytes.bytes).unwrap()
}

#[allow(dead_code)]
fn process_chunk(chunk: &[u8]) -> impl Iterator<Item=u8> {
    let bits: u32 = chunk.iter()
        .fold(0, |acc, &n| acc * 256 + n as u32);
    (0..4).rev().map(move |i| -> u8 {
        base64_to_utf8((bits >> 6*i) as u8 % 64)
    })
}

#[allow(dead_code)]
fn base64_to_utf8(base64_byte: u8) -> u8 {
    match base64_byte {
        0..=25  => base64_byte + 65,  // A-Z
        26..=51 => base64_byte + 71,  // a-z
        52..=61 => base64_byte - 4,   // 0-9
        62 => 43,               // +
        63 => 47,               // /
        _  => panic!("Invalid base64 number!")
    }
}

#[cfg(test)]
mod encoder_test {
    use super::hex_to_base64;

    #[test]
    fn initial_run() {
        let input = "49276d206b69";
        assert_eq!(hex_to_base64(input), String::from("SSdtIGtp"))
    }
}