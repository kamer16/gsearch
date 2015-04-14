use std::ascii::AsciiExt;
use std::collections::HashSet;
use std::collections::HashMap;
use std::io::Write;
use std::io::Read;
use rustc_serialize::json;
use std::fs::File;
use std::fs::PathExt;
use std::path::Path;

use document;

pub type Index = HashMap<String, HashSet<String>>;

#[derive(RustcEncodable,RustcDecodable)]
pub struct Indexer {
    pub index: Index,
}

pub struct TokenizedDocument {
    pub words: Vec<String>,
    pub url: String,
}

// Function used to normalize our strings
pub fn normalize(s: &str) -> String {
    s.nfkd_chars().map(|c| c.to_ascii_lowercase())
        .filter(|&c| c.is_alphabetic()).collect()
}

// Split word at each character that is not an alphabetic character
pub fn analyze(docs: Vec<document::Document>,
               process: fn(&str) -> String) -> Vec<TokenizedDocument> {
    let mut res = Vec::new();
    for doc in docs {
        // We separate words each time we find an non alphabetical character
        let cut: String = doc.text.chars()
            .map(|c| if c.is_alphabetic() {c} else {' '}).collect();
        let token: Vec<String> = cut.split(" ").filter(|x| x.len() != 0)
            .map(|s| process(s)).collect();
        res.push(TokenizedDocument { words: token, url: doc.url } );
    }
    res
}

pub fn build(td: Vec<TokenizedDocument>) -> Indexer {
    let mut res: Indexer = Indexer { index: HashMap::new() };
    for doc in td {
        for word in doc.words {
            if res.index.contains_key(&word) {
                res.index.get_mut(&word).unwrap().insert(doc.url.clone());
            }
            else {
                let mut hs = HashSet::new();
                hs.insert(doc.url.clone());
                res.index.insert(word, hs);
            }
        }
    }
    res
}

pub fn save(indexer: Indexer, path: &str) {
    let encoded = json::encode(&indexer).unwrap();
    let mut file = File::create(path).ok().expect("Write file Err");
    file.write_all(encoded.as_bytes());
}

pub fn load(path: &str) -> Option<Indexer> {
    let path = Path::new(path);
    if (*path).is_file() {
        let mut file = File::open(&path).ok().expect("Open file Err");
        let mut encoded = String::new();
        file.read_to_string(&mut encoded);
        let decoded: Indexer = json::decode(&encoded).ok().expect("decode err");
        return Some(decoded)
    }
    else {
        return None;
    }
}
