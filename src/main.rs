use std::fs::File;
use std::path::Path;
use ::url::ParseError::RelativeUrlWithoutBase;
use ::url::Url;

mod media_type;
mod property;
mod manifest;
mod url;

fn main() {

    let url = Url::parse("http://www.a.com#243234234").unwrap();
    let url = url.join("/").unwrap();

    println!("{}", url);

    let epub_url = Url::parse("epub:/content.opf").unwrap();
    println!("{}", epub_url);

    let epub_url = epub_url.join("images/cover.jpg").unwrap();
    println!("{}", epub_url);

    let path = Path::new("src/main.rs");
    let path = path.join("src/media_type.rs");

    println!("{}", path.display());

}
