use std::fs::File;

use eparser::book::parse_book;
use eparser::file::local::{read_from_dir, read_from_file, read_from_zip, ZipArchive};

fn read1() {
    let file = File::open("./res/example.epub").unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    let files = read_from_zip(&mut zip).unwrap();
    let book = parse_book(files).unwrap();
    println!("{:?}", book);
}

fn read2() {
    let file = File::open("./res/example.epub").unwrap();
    let files = read_from_file(file).unwrap();
    let book = parse_book(files).unwrap();
    println!("{:?}", book);
}

fn read3() {
    let dir = "./res/example";
    let files = read_from_dir(dir).unwrap();
    let book = parse_book(files).unwrap();
    println!("{:?}", book);
}

fn main() {
    read2()
}
