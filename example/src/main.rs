use std::fs::File;

use eparser::book::parse_book;
use eparser::file::Files;
use eparser::file::local::{read_from_dir, read_from_file, read_from_zip, ZipArchive};
use eparser::package::manifest::ResourceMap;

fn read1() {
    let file = File::open("./res/example.epub").unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    let mut files = read_from_zip(&mut zip).unwrap();
    let book = parse_book(&mut files).unwrap();
    println!("{:?}", book);
}

fn read2() {
    let file = File::open("./res/example.epub").unwrap();
    let mut files = read_from_file(file).unwrap();
    let book = parse_book(&mut files).unwrap();

    let pkg = book.packages().first().unwrap();
    let sref = pkg.spine.get(12).unwrap();
    let res = pkg.get_res_by_ref(sref).unwrap();
    let data = files.get_by_res(&res).unwrap();

    let s = String::from_utf8(data.clone()).unwrap();
    let xhtml = eparser::xhtml::parse_xhtml(&s).unwrap();

    let body = xhtml.body_str();

    println!("{:?}", body);
}

fn read3() {
    let dir = "./res/example";
    let mut files = read_from_dir(dir).unwrap();
    let book = parse_book(&mut files).unwrap();
    println!("{:?}", book);
}

fn main() {
    read2()
}
