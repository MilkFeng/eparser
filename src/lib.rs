pub mod book;
pub mod package;
pub mod file;
pub mod utils;
pub mod oebps;

#[cfg(test)]
mod test {
    use crate::book::parse_book;
    use crate::file::read_zip;
    use std::fs::File;
    use zip::ZipArchive;

    #[test]
    fn test() {
        let mut zip = ZipArchive::new(File::open("res/example.epub").unwrap()).unwrap();
        let files = read_zip(&mut zip);

        let book = parse_book(files).unwrap();
        println!("{:?}", book)
    }
}
