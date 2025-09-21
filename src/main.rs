use anyhow::Result;
use base64::prelude::*;
use codecrafters_bittorrent::torrent_file::Torrent;
use serde_json::{json, Map};
use std::env;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    match command.as_str() {
        "decode" => {
            let encoded_value = &args[2];
            let decoded_value: serde_bencode::value::Value =
                serde_bencode::from_bytes(encoded_value.as_bytes())?;
            let decoded_json = to_json(&decoded_value);
            println!("{}", decoded_json);
        }
        "info" => {
            let data = std::fs::read(&args[2])?;
            let torrent: Torrent = serde_bencode::from_bytes(&data)?;
            torrent.print();
        }
        _ => println!("unknown command: {}", args[1]),
    }

    Ok(())
}

fn to_json(bvalue: &serde_bencode::value::Value) -> serde_json::Value {
    match bvalue {
        serde_bencode::value::Value::Bytes(b) => match std::str::from_utf8(b) {
            Ok(s) => json!(s),
            Err(_) => json!(BASE64_STANDARD.encode(b)),
        },
        serde_bencode::value::Value::Int(i) => json!(i),
        serde_bencode::value::Value::List(l) => json!(l.iter().map(to_json).collect::<Vec<_>>()),
        serde_bencode::value::Value::Dict(d) => {
            let mut m = Map::new();
            for (k, v) in d {
                let key = match std::str::from_utf8(k) {
                    Ok(s) => s.to_string(),
                    Err(_) => BASE64_STANDARD.encode(k),
                };
                m.insert(key, to_json(v));
            }
            serde_json::Value::Object(m)
        }
    }
}
