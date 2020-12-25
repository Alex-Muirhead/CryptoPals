#[allow(dead_code)]
fn hex_to_base64(hex_str: &str) -> String {
    let bin_repr: Vec<u8> = hex_str.to_lowercase()
        .as_bytes()
        .chunks(6)
        .flat_map(process_chunk)
        .collect();
    return String::from_utf8(bin_repr).unwrap()
}

#[allow(dead_code)]
fn process_chunk(chunk: &[u8]) -> impl Iterator<Item=u8> {
    let dec: u32 = chunk.iter()
        .map(utf8_to_hex)
        .fold(0, |acc, n| 16*acc + n as u32);
    (0..4).rev().map(move |i| -> u8 {
        base64_to_utf8((dec >> 6*i) as u8 % 64)
    })
}

#[allow(dead_code)]
fn utf8_to_hex(utf8_byte: &u8) -> u8 {
    match utf8_byte {
        n @ 97..=102 => n - 87,  // a-f
        n @ 48..=57  => n - 48,  // 0-9
        _ => panic!("Invalid hex character!")
    }
}

#[allow(dead_code)]
fn base64_to_utf8(base64_byte: u8) -> u8 {
    match base64_byte {
        n @ 0..=25  => n + 65,  // A-Z
        n @ 26..=51 => n + 71,  // a-z
        n @ 52..=61 => n - 4,   // 0-9
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
            input.as_bytes()
                .iter()
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