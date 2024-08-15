pub mod book;
pub mod package;
pub mod file;
pub mod utils;
pub mod oebps;

#[cfg(test)]
mod test {
    use std::fs::File;

    use zip::ZipArchive;

    use crate::book::parse_book;
    use crate::file::read_zip;

    #[test]
    fn test_parse() {
        let file = File::open("res/example.epub").unwrap();
        let mut zip = ZipArchive::new(file).unwrap();
        let files = read_zip(&mut zip);
        println!("{:?}", files);

        let book = parse_book(files).unwrap();
        println!("{:?}", book)
    }
}
