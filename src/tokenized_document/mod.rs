use rustc_serialize::json;

use std::ascii::AsciiExt;
use std::collections::HashMap;
use std::io::{Write, Read};
use std::fs::{File, PathExt};
use std::path::Path;

use document;

pub type Index = HashMap<String, Vec<String>>;

#[derive(RustcEncodable,RustcDecodable)]
pub struct Indexer {
    pub index: Index,
}

pub struct TokenizedDocument {
    pub words: Vec<String>,
    pub url: String,
}

pub struct MultiIndex {
    generations: Vec<Index>,
    // Given an URL say which generation to use
    support: HashMap<String, usize>,
}

// Rewrite new indexes of capacity 256 and then remove all the old indexes
impl MultiIndex {
    fn update(&mut self) {
        let prev_last = self.generations.len();
        let mut generation = Index::new();
        for (url, val) in self.support.iter() {
            if generation.len() == 256 {
                self.generations.push(generation);
                generation = Index::new();
            }
            generation.insert(url.clone(), self.generations[*val][url].clone());
        }
        if generation.len() > 0 {
            self.generations.push(generation);
        }
        self.generations = self.generations.iter().cloned().skip(prev_last).collect();
    }
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
                let v = &mut res.index.get_mut(&word).unwrap();
                match v.binary_search(&doc.url) {
                    Ok(_) => (),
                    Err(pos) => v.insert(pos, doc.url.clone())
                }
            }
            else {
                let mut inverted_list = Vec::new();
                inverted_list.push(doc.url.clone());
                res.index.insert(word, inverted_list);
            }
        }
    }
    res
}

pub fn save(indexer: Indexer, path: &str) {
    let encoded = json::encode(&indexer).unwrap();
    let mut file = File::create(path).ok().expect("Write file Err");
    file.write_all(encoded.as_bytes()).ok().expect("Can't save");
}

pub fn load(path: &str) -> Option<Indexer> {
    let path = Path::new(path);
    if (*path).is_file() {
        let mut file = File::open(&path).ok().expect("Open file Err");
        let mut encoded = String::new();
        file.read_to_string(&mut encoded).ok().expect("Cant read to String");
        let decoded: Indexer = json::decode(&encoded).ok().expect("decode err");
        return Some(decoded)
    }
    else {
        return None;
    }
}

// Index the hashmap to find inverted list
pub fn search<'a>(indexer: &'a Indexer, word: &str)-> Option<Vec<&'a String>> {
    // Create a new string without \n
    let normed = word.replace("\n", "");
    // Only look at words that are in index
    // Returns an Vec<Peekable<iterator of &String>> over &'a str
    let mut i_list: Vec<_> = normed.split(" ").map(|w| normalize(w))
        // discard useless &'a str
        .filter(|w| w.len() != 0 && indexer.index.contains_key(w))
        // map each sub &'a str to a ref peekable iterator on the inverted list
        .map(|w| indexer.index[&w].iter().peekable())
        // collect our list of peekable iterators
        .collect();

    let mut res = Vec::new();
    while i_list.len() != 0 {
        // Consume i_list i-e consume the iterator and not the data
        let min = i_list.iter_mut()
            .map(|p| *p.peek().unwrap()).min().unwrap();
        if i_list.iter_mut().fold(true, |b, p| if *p.peek().unwrap() == min {
                                         p.next();
                                         b && true
                                     } else {
                                         b && false } ) {
            res.push(min);
        }
        if i_list.iter_mut().any(|p| p.is_empty()) {
            break;
        }
    }
    if res.len() > 0 {
        Some(res)
    }
    else {
        None
    }
}
