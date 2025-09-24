use bytes::{BufMut, BytesMut};

pub struct PeerMessage {
    pub length: u32,
    pub id: MessageId,
    pub payload: Vec<u8>,
}

pub enum MessageId {
    Choke = 0,
    Unchoke = 1,
    Interested = 2,
    NotInterested = 3,
    Have = 4,
    Bitfield = 5,
    Request = 6,
    Piece = 7,
    Cancel = 8,
    Port = 9,
}

pub struct Handshake {
    pub pstr_len: u8,
    pub pstr: String,
    pub info_hash: Vec<u8>,
    pub peer_id: [u8; 20],
}

impl Handshake {
    pub fn new(info_hash: Vec<u8>, peer_id: [u8; 20]) -> Self {
        Handshake {
            pstr_len: 19,
            pstr: "BitTorrent protocol".to_string(),
            info_hash,
            peer_id,
        }
    }
}

impl From<Handshake> for Vec<u8> {
    fn from(value: Handshake) -> Self {
        let mut bytes = BytesMut::with_capacity(68);
        bytes.put_u8(value.pstr_len);
        bytes.put_slice(value.pstr.as_bytes());
        bytes.put_bytes(0, 8);
        bytes.put_slice(&value.info_hash);
        bytes.put_slice(&value.peer_id);
        bytes.freeze().to_vec()
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
