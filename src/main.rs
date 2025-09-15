use serde_json::Number;
use std::env;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> Option<serde_json::Value> {
    match encoded_value.chars().next().unwrap() {
        c if c.is_ascii_digit() => {
            let (len, rest) = encoded_value.split_once(':')?;
            let len = len.parse::<usize>().ok()?;
            Some(serde_json::Value::String(rest[..len].into()))
        }
        'i' => {
            let end = encoded_value.find('e')?;
            let num = encoded_value[1..end].parse::<i128>().ok()?;
            Some(serde_json::Value::Number(Number::from_i128(num)?))
        }
        _ => None,
    }
}

// Usage: your_program.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        // You can use print statements as follows for debugging, they'll be visible when running tests.
        eprintln!("Logs from your program will appear here!");

        let encoded_value = &args[2];
        let decoded_value = decode_bencoded_value(encoded_value).unwrap();
        println!("{}", decoded_value);
    } else {
        println!("unknown command: {}", args[1])
    }
}
