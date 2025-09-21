use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

#[derive(Deserialize, Serialize, Debug)]
pub struct Torrent {
    announce: String,
    info: Info,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Info {
    length: i64,
    name: String,
    #[serde(rename = "piece length")]
    piece_length: i64,
    #[serde(with = "serde_bytes")]
    pieces: Vec<u8>,
}

impl Info {
    pub fn info_hash(&self) -> Vec<u8> {
        let encoded = serde_bencode::to_bytes(self).unwrap();
        let mut hasher = Sha1::new();
        hasher.update(encoded);
        hasher.finalize().to_vec()
    }
}

impl Torrent {
    pub fn print(&self) {
        println!("Tracker URL: {}", self.announce);
        println!("Length: {}", self.info.length);
        let hash = hex::encode(self.info.info_hash());
        println!("Info Hash: {}", hash);
    }
}
