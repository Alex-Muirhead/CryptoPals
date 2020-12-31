use std::env;

use cryptopals::EncodedData;

fn main() {
    let args: Vec<String> = env::args().collect();

    // The program name is the 0th value
    let input_one = EncodedData::from_hex(&args[1]);
    let input_two = EncodedData::from_hex(&args[2]);

    let crossed_data = input_one ^ input_two;

    println!("The XOR'd string is {}", (crossed_data.to_hex()));
}
