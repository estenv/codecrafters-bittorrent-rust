use serde_json::Number;
use std::env;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> Option<(serde_json::Value, &str)> {
    match encoded_value.chars().next().unwrap() {
        c if c.is_ascii_digit() => {
            let (len, rest) = encoded_value.split_once(':')?;
            let len = len.parse::<usize>().ok()?;
            Some((serde_json::Value::String(rest[..len].into()), &rest[len..]))
        }
        'i' => {
            let end = encoded_value.find('e')?;
            let num = encoded_value[1..end].parse::<i128>().ok()?;
            Some((
                serde_json::Value::Number(Number::from_i128(num)?),
                &encoded_value[end + 1..],
            ))
        }
        'l' => {
            let mut remaining = &encoded_value[1..];
            let mut items = vec![];
            while !remaining.starts_with("e") {
                let (value, rest) = decode_bencoded_value(remaining)?;
                items.push(value);
                remaining = rest;
            }
            Some((serde_json::Value::Array(items), &remaining[1..]))
        }
        _ => None,
    }
}

// Usage: your_program.sh decode "<encoded_value>"
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        // let decoded_value = serde_bencode::from_str::<serde_json::Value>(encoded_value).unwrap();
        let (decoded_value, _) = decode_bencoded_value(encoded_value).unwrap();
        println!("{}", decoded_value);
    } else {
        println!("unknown command: {}", args[1])
    }
}
