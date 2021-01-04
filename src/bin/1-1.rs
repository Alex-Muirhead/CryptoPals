use std::env;

use cryptopals::hex_to_base64;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = &args[1];  // The program name is the 0th value
    println!("The encoded string is {}", hex_to_base64(&input));
}