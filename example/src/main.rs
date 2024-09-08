use eparser::book::parse_book;
use eparser::file::read_from_epub_url_str;
use eparser::file::read_from_url_str;
use eparser::file::Files;
use eparser::file::{read_from_dir, read_from_file, read_from_zip, ZipArchive};
use eparser::package::manifest::ResourceMap;
use std::fs::File;

async fn read1() {
    let file = File::open("./res/example.epub").unwrap();
    let mut zip = ZipArchive::new(file).unwrap();
    let mut files = read_from_zip(&mut zip).unwrap();
    let book = parse_book(&mut files).await.unwrap();
    println!("{:?}", book);
}

async fn read2() {
    let file = File::open("./res/example.epub").unwrap();
    let mut files = read_from_file(file).unwrap();
    let book = parse_book(&mut files).await.unwrap();

    let pkg = book.packages().first().unwrap();
    let sref = pkg.spine.get(12).unwrap();
    let res = pkg.get_res_by_ref(sref).unwrap();
    let data = files.get_by_res(&res).await.unwrap();

    let s = String::from_utf8(data.clone()).unwrap();
    let xhtml = eparser::xhtml::parse_xhtml(&s).unwrap();

    let body = xhtml.body_str();

    println!("{:?}", body);
}

async fn read3() {
    let dir = "./res/example";
    let mut files = read_from_dir(dir).unwrap();
    let book = parse_book(&mut files).await.unwrap();
    println!("{:?}", book);
}

async fn read4() {
    let url = "http://localhost:8000/example/";
    let mut files = read_from_url_str(url).await.unwrap();
    let book = parse_book(&mut files).await.unwrap();
    println!("{:?}", book);
}

async fn read5() {
    let url = "http://localhost:8000/example.epub";
    let mut files = read_from_epub_url_str(url).unwrap();
    let book = parse_book(&mut files).await.unwrap();
    println!("{:?}", book);
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            read5().await;
        })
}
