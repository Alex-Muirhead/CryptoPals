use crate::EncodedData;

impl EncodedData {
    /// Implement constructor from a string of hex characters.
    pub fn from_hex(utf8_str: &str) -> Self {
        let bytes = utf8_str.to_lowercase()  // Ensure a-z
            .bytes()
            .map(utf8_to_hex)
            .collect::<Vec<u8>>()
            .chunks_exact(2)
            .map(|pair| pair[0] * 16 + pair[1])
            .collect();
        EncodedData { bytes }
    }

    pub fn to_hex(&self) -> String {
        let utf8_bytes: Vec<u8> = self.bytes
            .iter()
            .flat_map(|&val| vec![val / 16, val % 16].into_iter())
            .map(hex_to_utf8)
            .collect();
        String::from_utf8(utf8_bytes).unwrap()
    }
}

/// Converts utf8 encoded hex characters to their respective values
#[allow(dead_code)]
fn utf8_to_hex(utf8_byte: u8) -> u8 {
    match utf8_byte {
        97..=102 => utf8_byte - 87,  // a-f
        48..=57  => utf8_byte - 48,  // 0-9
        _ => panic!(format!("Invalid hex character {:#x}!", utf8_byte))
    }
}

/// Converts hex values to their respective utf8 encoded characters
#[allow(dead_code)]
fn hex_to_utf8(hex_value: u8) -> u8 {
    match hex_value {
        10..=15 => hex_value + 87,  // a-f
         0..=9  => hex_value + 48,  // 0-9
        _ => panic!("Invalid hex value!")
    }
}

#[cfg(test)]
mod test {
    use super::{hex_to_utf8, utf8_to_hex};

    #[test]
    fn decode_hex() {
        let input = "0123456789abcdef";
        assert_eq!(
            input.bytes()
                .map(utf8_to_hex)
                .collect::<Vec<u8>>(),
            (0..16).collect::<Vec<u8>>()
        );
    }

    #[test]
    fn encode_hex() {
        let output = "0123456789abcdef";
        assert_eq!(
            output.bytes()
                .collect::<Vec<u8>>(),
            (0..16).map(hex_to_utf8)
                .collect::<Vec<u8>>()
        );
    }
}