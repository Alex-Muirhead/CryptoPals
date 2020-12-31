use std::ops::Deref;

pub struct EncodedData {
    bytes: Vec<u8>
}

impl Deref for EncodedData {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}

impl EncodedData {
    fn from_hex(utf8_str: &str) -> Self {
        let bytes = utf8_str.to_lowercase()  // Ensure a-z
            .bytes()
            .map(utf8_to_hex)
            .collect::<Vec<u8>>()
            .chunks_exact(2)
            .map(|pair| pair[0] * 16 + pair[1])
            .collect();
        EncodedData { bytes }
    }
}

#[allow(dead_code)]
pub fn hex_to_base64(hex_str: &str) -> String {
    let utf8_bytes: EncodedData = EncodedData::from_hex(hex_str)
        .chunks(3)
        .flat_map(process_chunk)
        .collect();
    return String::from_utf8(utf8_bytes).unwrap()
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
fn utf8_to_hex(utf8_byte: u8) -> u8 {
    match utf8_byte {
        97..=102 => utf8_byte - 87,  // a-f
        48..=57  => utf8_byte - 48,  // 0-9
        _ => panic!("Invalid hex character!")
    }
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
mod test {
    use super::{utf8_to_hex, hex_to_base64};

    #[test]
    fn read_hex() {
        let input = "0123456789abcdef";
        assert_eq!(
            input.bytes()
                .map(utf8_to_hex)
                .collect::<Vec<u8>>(),
            (0..16).collect::<Vec<u8>>()
        );
    }

    #[test]
    fn initial_run() {
        let input = "49276d206b69";
        assert_eq!(hex_to_base64(input), String::from("SSdtIGtp"))
    }
}