use std::io;

use cryptopals::EncodedData;

fn main() {
    let mut arg_one = String::new();
    let mut arg_two = String::new();

    println!("Input first data string:");
    io::stdin()
        .read_line(&mut arg_one)
        .expect("Failed to read line");

    println!("Input second data string:");
    io::stdin()
        .read_line(&mut arg_two)
        .expect("Failed to read line");

    // The program name is the 0th value
    let input_one = EncodedData::from_hex(&arg_one.trim_end());
    let input_two = EncodedData::from_hex(&arg_two.trim_end());

    let crossed_data = input_one ^ input_two;

    println!("The XOR'd string is {}", (crossed_data.to_hex()));
}
