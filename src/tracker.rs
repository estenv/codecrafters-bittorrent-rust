use std::{
    collections::HashMap,
    net::{Ipv4Addr, SocketAddrV4},
};

use anyhow::{Context, Result};
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use reqwest::blocking::Client;

use crate::torrent_file::Torrent;

pub fn tracker_get(torrent: &Torrent) -> Result<HashMap<String, serde_bencode::value::Value>> {
    let client = Client::new();
    let info_hash = percent_encode(&torrent.info.info_hash(), NON_ALPHANUMERIC).to_string();
    let url = format!(
        "{}?info_hash={}&peer_id={}&port={}&uploaded={}&downloaded={}&left={}&compact=1",
        torrent.announce, info_hash, "-PC0001-123456789012", "6881", "0", "0", torrent.info.length,
    );

    let response = client
        .get(url)
        .send()
        .context("Tracker GET request failed")?;
    let body = response.bytes().context("Failed to read response body")?;
    dbg!(&body);
    dbg!(String::from_utf8_lossy(&body));
    serde_bencode::from_bytes(&body).context("Failed to deserialize response body")
}

pub fn get_peers(response: HashMap<String, serde_bencode::value::Value>) -> Vec<SocketAddrV4> {
    let Some(serde_bencode::value::Value::Bytes(bytes)) = response.get("peers") else {
        return vec![];
    };
    bytes
        .chunks_exact(6)
        .map(|x| {
            let ip = Ipv4Addr::new(x[0], x[1], x[2], x[3]);
            let port = u16::from_be_bytes([x[4], x[5]]);
            SocketAddrV4::new(ip, port)
        })
        .collect()
}
