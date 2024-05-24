use std::collections::BTreeMap;

use bencode::util::ByteString;
use bencode::{Bencode, Encoder};

pub struct Torrent {
    announce: String,
    announce_list: Vec<String>,
    name: String,
    comment: String,
    multi_file: bool,
    piece_length: i32,
    length: i64,
    creation_date: String,
    total_size: i32,
}

fn main() {
    print!("Work in progress ...")

}
