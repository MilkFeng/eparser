This is epub parse library for rust.

Now it is in development.

## Usage
```rust
use std::fs::File;

use eparser::book::parse_book;
use eparser::file::local::read_from_file;

fn main() {
    let file = File::open("./res/example.epub").unwrap();
    let files = read_from_file(file).unwrap();
    let book = parse_book(files).unwrap();
    println!("{:?}", book);
}
```