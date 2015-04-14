#![feature(unicode)]
#![feature(path_ext)]
#![feature(fs_walk)]
#![feature(collections)]

extern crate rustc_serialize;

pub mod document;
pub mod tokenized_document;

fn main() {
    let indexer = tokenized_document::load("data.txt");
    if indexer.is_some() {
        let indexer = indexer.unwrap();
        println!("Indexing holy ...");
        for a in vec!("one", "two", "three", "four", "holy") {
            println!("{:?}", indexer.index[a]);
        }
    }
    else {
        let d = document::fetch("data/20news-bydate-train", true);
        let td = tokenized_document::analyze(d, tokenized_document::normalize);
        let indexer = tokenized_document::build(td);
        tokenized_document::save(indexer, "data.txt");
    }
}
