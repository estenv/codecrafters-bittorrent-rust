use anyhow::Result;
use base64::prelude::*;
use codecrafters_bittorrent::{torrent_file::Torrent, tracker::tracker_get};
use serde_json::{json, Map};
use std::{
    env,
    net::{Ipv4Addr, SocketAddrV4},
};

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
        "peers" => {
            let data = std::fs::read(&args[2])?;
            let torrent: Torrent = serde_bencode::from_bytes(&data)?;
            let response = tracker_get(torrent)?;
            if let Some(serde_bencode::value::Value::Bytes(bytes)) = response.get("peers") {
                bytes
                    .chunks_exact(6)
                    .map(|x| {
                        let ip = Ipv4Addr::new(x[0], x[1], x[2], x[3]);
                        let port = u16::from_be_bytes([x[4], x[5]]);
                        SocketAddrV4::new(ip, port)
                    })
                    .for_each(|a| println!("{a}"))
            }
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
