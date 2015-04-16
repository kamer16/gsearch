#![feature(core)]
#![feature(unicode)]
#![feature(path_ext)]
#![feature(fs_walk)]
#![feature(collections)]

extern crate rustc_serialize;

pub mod document;
pub mod tokenized_document;
use std::io;

fn main() {
    let indexer = tokenized_document::load("data.txt");
    if indexer.is_some() {
        let indexer = indexer.unwrap();
        let mut stdin = io::stdin();
        let mut word = String::new();
        while stdin.read_line(&mut word).unwrap() != 0 {
            println!("{:?}", tokenized_document::search(&indexer, &word));
            word.clear();
        }
    }
    else {
        let d = document::fetch("data/20news-bydate-train", true);
        let td = tokenized_document::analyze(d, tokenized_document::normalize);
        let indexer = tokenized_document::build(td);
        tokenized_document::save(indexer, "data.txt");
    }
}
