use std::fs::File;

use eparser::book::parse_book;
use eparser::file::local::{read_from_dir, read_from_zip, ZipArchive};

fn read1() {
    let file = File::open("../res/example.epub").unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    let files = read_from_zip(&mut zip).unwrap();
    println!("{:?}", files);

    let book = parse_book(files).unwrap();
    println!("{:?}", book);
}

fn read2() {
    let dir = "../res/example";
    let files = read_from_dir(dir).unwrap();
    println!("{:?}", files);

    let book = parse_book(files).unwrap();
    println!("{:?}", book);
}

fn main() {
    read2()
}
