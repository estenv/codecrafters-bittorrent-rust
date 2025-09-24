use anyhow::Result;
use base64::prelude::*;
use codecrafters_bittorrent::{
    messages::Handshake,
    torrent_file::Torrent,
    tracker::{get_peers, tracker_get},
};
use serde_json::{json, Map};
use std::{
    env,
    io::{BufReader, Read, Write},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];

    if command == "decode" {
        let encoded_value = &args[2];
        let decoded_value: serde_bencode::value::Value =
            serde_bencode::from_bytes(encoded_value.as_bytes())?;
        let decoded_json = to_json(&decoded_value);
        println!("{}", decoded_json);
        std::process::exit(0);
    }
    let data = std::fs::read(&args[2])?;
    let torrent: Torrent = serde_bencode::from_bytes(&data)?;

    match command.as_str() {
        "info" => {
            torrent.print();
        }
        "peers" => {
            let response = tracker_get(&torrent)?;
            let peers = get_peers(response);
            peers.iter().for_each(|p| println!("{p}"))
        }
        "handshake" => {
            println!("Peer ID: {}", hex::encode(response_handshake.peer_id));
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
