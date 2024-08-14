use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Read;
use std::ops::{Deref, DerefMut};
use url::Url;

#[derive(PartialEq, Clone)]
pub struct Files {
    files: HashMap<Url, Vec<u8>>,
    pub root_url: Url,
}

impl Debug for Files {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Files")
            .field("files", &self.files.keys().collect::<Vec<_>>())
            .field("root_url", &self.root_url)
            .finish()
    }
}


impl Files {
    pub fn empty() -> Self {
        Files {
            files: HashMap::new(),
            root_url: Url::parse("epub:/").unwrap(),
        }
    }
}

impl Deref for Files {
    type Target = HashMap<Url, Vec<u8>>;

    fn deref(&self) -> &Self::Target {
        &self.files
    }
}

impl DerefMut for Files {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.files
    }
}

pub fn read_zip(zip: &mut zip::ZipArchive<std::fs::File>) -> Files {
    let mut files = Files::empty();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i).unwrap();
        let mut content = Vec::new();
        let url_str = format!("epub:/{}", file.name());
        let url = Url::options().parse(&url_str).unwrap();
        file.read_to_end(&mut content).unwrap();
        files.insert(url, content);
    }
    files
}