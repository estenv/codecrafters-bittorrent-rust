use serde_json::{Map, Number};
use std::env;

#[allow(dead_code)]
fn decode_bencoded_value(encoded_value: &str) -> Option<(serde_json::Value, &str)> {
    match encoded_value.chars().next().unwrap() {
        c if c.is_ascii_digit() => {
            let (value, rest) = decode_to_owned_string(encoded_value)?;
            Some((serde_json::Value::String(value), rest))
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
        'd' => {
            let mut map: Map<String, serde_json::Value> = Map::new();
            let mut remaining = &encoded_value[1..];
            while !remaining.starts_with("e") {
                let (key, rest) = decode_to_owned_string(remaining)?;
                let (value, rest) = decode_bencoded_value(rest)?;
                map.insert(key, value);
                remaining = rest;
            }
            Some((serde_json::Value::Object(map), &remaining[1..]))
        }
        _ => None,
    }
}

fn decode_to_owned_string(encoded_string: &str) -> Option<(String, &str)> {
    let (len, rest) = encoded_string.split_once(':')?;
    let len = len.parse::<usize>().ok()?;
    Some((rest[..len].into(), &rest[len..]))
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
