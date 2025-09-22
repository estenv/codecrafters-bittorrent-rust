use anyhow::Result;
use base64::prelude::*;
use bytes::{BufMut, Bytes, BytesMut};
use codecrafters_bittorrent::{
    torrent_file::Torrent,
    tracker::{get_peers, tracker_get},
};
use serde_json::{json, Map};
use std::{
    env,
    io::{BufReader, Read, Write},
    net::TcpStream,
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
            let response = tracker_get(&torrent)?;
            let peers = get_peers(response);
            peers.iter().for_each(|p| println!("{p}"))
        }
        "handshake" => {
            let data = std::fs::read(&args[2])?;
            let torrent: Torrent = serde_bencode::from_bytes(&data)?;
            let peer_id: [u8; 20] = rand::random();
            let handshake = Handshake::new(torrent.info.info_hash(), peer_id);
            let mut stream = TcpStream::connect(args[3].as_str())?;
            let handshake_bytes: Bytes = handshake.into();
            stream.write_all(&handshake_bytes)?;
            let mut reader = BufReader::new(stream);
            let mut buf = vec![];
            reader.read_to_end(&mut buf)?;
            let response_handshake: Handshake = buf.into();
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

#[derive(serde::Serialize)]
struct Handshake {
    pstr_len: u8,
    pstr: String,
    info_hash: Vec<u8>,
    peer_id: [u8; 20],
}

impl Handshake {
    fn new(info_hash: Vec<u8>, peer_id: [u8; 20]) -> Self {
        Handshake {
            pstr_len: 19,
            pstr: "BitTorrent protocol".to_string(),
            info_hash,
            peer_id,
        }
    }
}

impl From<Handshake> for Bytes {
    fn from(value: Handshake) -> Self {
        let mut bytes = BytesMut::with_capacity(68);
        bytes.put_u8(value.pstr_len);
        bytes.put_slice(value.pstr.as_bytes());
        bytes.put_bytes(0, 8);
        bytes.put_slice(&value.info_hash);
        bytes.put_slice(&value.peer_id);
        bytes.freeze()
    }
}

impl From<Vec<u8>> for Handshake {
    fn from(value: Vec<u8>) -> Self {
        if value.len() != 68 {
            panic!("Invalid handshake length: {}", value.len());
        }
        Self {
            pstr_len: value[0],
            pstr: String::from_utf8_lossy(&value[1..20]).to_string(),
            info_hash: value[28..48].to_vec(),
            peer_id: value[48..68].try_into().unwrap(),
        }
    }
}
