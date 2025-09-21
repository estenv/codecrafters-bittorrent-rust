use anyhow::{bail, Result};
use base64::prelude::*;
use serde_json::{Map, Number};
use std::env;

fn decode_bencoded_value(input: &[u8]) -> Result<(serde_json::Value, &[u8])> {
    match input[0] {
        b'0'..=b'9' => {
            let (value, rest) = decode_bytes(input)?;
            let json_value = match std::str::from_utf8(value) {
                Ok(s) => serde_json::Value::String(s.to_string()),
                Err(_) => serde_json::Value::String(BASE64_STANDARD.encode(value)),
            };
            Ok((json_value, rest))
        }
        b'i' => {
            let Some(end) = input.iter().position(|&b| b == b'e') else {
                bail!("Missing end of integer");
            };
            let Some(num) =
                Number::from_i128(std::str::from_utf8(&input[1..end])?.parse::<i128>()?)
            else {
                bail!("Failed to parse number");
            };
            Ok((serde_json::Value::Number(num), &input[end + 1..]))
        }
        b'l' => {
            let mut remaining = &input[1..];
            let mut items = vec![];
            while !remaining.starts_with(b"e") {
                let (value, rest) = decode_bencoded_value(remaining)?;
                items.push(value);
                remaining = rest;
            }
            Ok((serde_json::Value::Array(items), &remaining[1..]))
        }
        b'd' => {
            let mut map = Map::new();
            let mut remaining = &input[1..];
            while !remaining.starts_with(b"e") {
                let (key_bytes, rest) = decode_bytes(remaining)?;
                let key = match std::str::from_utf8(key_bytes) {
                    Ok(s) => s.to_string(),
                    Err(_) => BASE64_STANDARD.encode(key_bytes),
                };
                let (value, rest) = decode_bencoded_value(rest)?;
                map.insert(key, value);
                remaining = rest;
            }
            Ok((serde_json::Value::Object(map), &remaining[1..]))
        }
        _ => bail!("Decoding failed"),
    }
}

fn decode_bytes(input: &[u8]) -> Result<(&[u8], &[u8])> {
    let Some(colon_pos) = input.iter().position(|&b| b == b':') else {
        bail!("Failed to read string length");
    };
    let len = std::str::from_utf8(&input[..colon_pos])?.parse::<usize>()?;
    let start = colon_pos + 1;
    let end = start + len;
    Ok((&input[start..end], &input[end..]))
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    match command.as_str() {
        "decode" => {
            let encoded_value = &args[2];
            let (decoded_value, _) = decode_bencoded_value(encoded_value.as_bytes())?;
            println!("{}", decoded_value);
        }
        "info" => {
            let data = std::fs::read(&args[2])?;
            if let Some(lines) = print_torrent_file(&data) {
                lines.iter().for_each(|l| println!("{l}"));
            }
        }
        _ => println!("unknown command: {}", args[1]),
    }

    Ok(())
}

fn print_torrent_file(data: &[u8]) -> Option<Vec<String>> {
    let Ok((serde_json::Value::Object(decoded), _)) = decode_bencoded_value(data) else {
        return None;
    };
    let mut lines = vec![];
    lines.push(format!(
        "Tracker URL: {}",
        decoded.get("announce")?.as_str()?
    ));
    let serde_json::Value::Object(info) = decoded.get("info")? else {
        return None;
    };
    lines.push(format!("Length: {}", info.get("length")?));
    Some(lines)
}
