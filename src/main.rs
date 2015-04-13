#![feature(fs)]
#![feature(path)]
#![feature(io)]
#![feature(collections)]

pub mod document;

fn main() {
    //let d = document::fetch("data/20news-bydate-train", true);
    //let _ = document::fetch("data/20news-bydate-test/alt.atheism", false);
    let d = document::fetch("data/20news-bydate-train/sci.crypt/15672", false);
}
