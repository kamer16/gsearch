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

// Convert file to an UTF8 string and loose information when not UTF8 encoding
fn add_file(path: &Path, res: &mut Vec<Document>) {
    if !path.is_file() {
        return
    }

    match File::open(&path) {
        Err(ref mut e) => println!("{:?} not a file", e),
        Ok(ref mut file) => {
            let mut s = Vec::new();
            match file.read_to_end(&mut s) {
                Ok(_) => (),
                Err(e) => println!("{}, {}", e, path.to_str().unwrap())
            }
            res.push(Document::new(String::from_utf8_lossy(&s).into_owned(),
                     path.to_str().unwrap()));
        }
    }
}
