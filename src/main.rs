use decoder::WordDocument;
use serde_derive::Deserialize;
use std::{
    fs::{self, File},
    io,
    path::Path,
};

mod decoder;
mod subreader;

fn main() -> io::Result<()> {
    println!("This is a rust program that is supposed to compary .doc and .docx files to see if they have maybe been converted from each other");

    let config = Config::new();
    let file_path = config.input.old_doc_file_name;

    let file = File::open(file_path)?;
    let word_doc = WordDocument::read_file(file)?;
    let _ = word_doc.get_logical_structures();
    let _ = word_doc.get_physical_sructures();

    // let something = word_doc.to_json();
    // println!("{}", something.pretty(2));

    // let fib = &word_doc.fib;
    // println!("{:?}", fib);
    // let stylesheet = &word_doc.stylesheet;
    // println!("{:#?}", stylesheet);

    word_doc.print_cfb_structure();
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Input {
    old_doc_file_name: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    input: Input,
}

impl Config {
    fn new() -> Self {
        let path = Path::new("doc_anal_config.toml");
        let content = fs::read_to_string(path).expect("Unable to locate 'doc_anal_config.toml'");
        toml::from_str(&content).unwrap()
    }
}
