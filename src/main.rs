use std::env;
use std::fs;
use lib_skadi::parser::Torrent;

use std::collections::BTreeMap;

use bencode::util::ByteString;
use bencode::{Bencode, Encoder};

fn main() {
    let file_path = '../demo/WinRar.torrent';
    let torrent_file = fs::read_to_string(file_path)
        .expect("No such file {}", file_path);
    let torrent = Torrent();
    if file:
        torrent.parse_file();
    else if link:
        torrent.read_link();
    else:
        println!("Error in parsing the files.")
}
