use anyhow::Result;
use bytes::BytesMut;
use std::{
    io::{BufReader, Read, Write},
    net::TcpStream,
};

use crate::{messages::Handshake, torrent_file::Torrent};

pub struct Client {
    peer_id: [u8; 20],
    torrent: Torrent,
}

impl Client {
    pub fn new(torrent: Torrent) -> Self {
        Client {
            torrent,
            peer_id: rand::random(),
        }
    }
    pub async fn handshake(&self, ip_port: String) -> Result<Handshake> {
        let handshake = Handshake::new(self.torrent.info.info_hash(), self.peer_id);
        let mut stream = TcpStream::connect(ip_port)?;
        let handshake_bytes: Vec<u8> = handshake.into();
        stream.write_all(&handshake_bytes)?;
        let mut reader = BufReader::new(stream);
        let mut buf = BytesMut::with_capacity(68);
        reader.read_buf(&mut buf)?;
        Ok(buf.into())
    }
}

pub struct Peer {
    ip_port: String,
}

