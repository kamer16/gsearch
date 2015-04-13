use std::path::Path;
use std::io::Read;
use std::fs::{self, File, PathExt};

#[derive(Clone)]
pub struct Document {
    pub text : String,
    pub url : String,
}

impl Document {
    fn new(text: String, url: &str) -> Document {
        Document {
            text : text,
            url : String::from_str(url),
        }
    }
}

pub fn fetch(path: &str, recursive: bool) -> Vec<Document>{
    let mut res = Vec::new();
    let path = Path::new(path);
    if recursive && path.is_dir() {
        for path in fs::walk_dir(&path).ok().expect("Bad path found") {
            add_file(&path.unwrap().path(), &mut res);
        }
    }
    else if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            if entry.path().is_file() {
                add_file(&entry.path(), &mut res);
            }
        }
    }
    else if path.is_file() {
        add_file(&path, &mut res);
    }
    res
}

fn add_file(path: &Path, res: &mut Vec<Document>) {
    match File::open(&path) {
        Err(ref mut e) => println!("{:?} not a file", e),
        Ok(ref mut file) => {
            let mut s = Vec::new();
            file.read_to_end(&mut s).unwrap();
            let tmp: String = s.iter().map(|&c| c as char).collect();
            for a in tmp.chars() {
                print!("{}", a);
            }
            println!("");
            for a in &s {
                print!("{} ", a);
            }
            println!("{} string length", tmp.len() as isize);
            println!("{} vec length", s.len() as isize);
            res.push(Document::new(tmp, path.to_str().unwrap()));
            //res.push(Document::new(String::from_utf8_lossy(& s).into_owned(), path.to_str().unwrap()));
        }
    }
}
