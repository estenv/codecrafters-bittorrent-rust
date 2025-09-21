use serde::{Deserialize, Serialize};
use sha1::{Digest, Sha1};

#[derive(Deserialize, Serialize, Debug)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Info {
    pub length: i64,
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

    pub fn piece_hashes(&self) -> Vec<String> {
        (0..self.pieces.len())
            .step_by(20)
            .map(|offset| hex::encode(&self.pieces[offset..offset + 20]))
            .collect()
    }
}

impl Torrent {
    pub fn print(&self) {
        println!("Tracker URL: {}", self.announce);
        println!("Length: {}", self.info.length);
        println!("Info Hash: {}", hex::encode(self.info.info_hash()));
        println!("Piece Length: {}", self.info.piece_length);
        println!("Piece Hashes:");
        for hash in self.info.piece_hashes() {
            println!("{hash}");
        }
    }
}
