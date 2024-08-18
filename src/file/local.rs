use std::collections::BTreeMap;
use std::fmt::Debug;
use std::fs::{File, read_dir};
use std::io::{Read, Seek};
use std::path::{Path, PathBuf};

use thiserror::Error;
use url::Url;
pub use zip::result::ZipError;
pub use zip::ZipArchive;

use crate::file::Files;

#[derive(PartialEq, Clone)]
pub struct LocalFiles {
    files: BTreeMap<Url, Vec<u8>>,
    root_url: Url,
}

impl Files for LocalFiles {
    fn root_url(&self) -> &Url {
        &self.root_url
    }

    fn get(&mut self, url: &Url) -> Option<&Vec<u8>> {
        // remove the fragment from the URL
        return if url.path_segments().is_none() {
            self.files.get(url)
        } else {
            self.files.get(&url.join("").unwrap())
        };
    }
}

impl Debug for LocalFiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Files")
            .field("files", &self.files.keys().collect::<Vec<_>>())
            .field("root_url", &self.root_url)
            .finish()
    }
}

impl LocalFiles {
    pub fn empty() -> Self {
        LocalFiles {
            files: BTreeMap::new(),
            root_url: Url::parse("epub:/").unwrap(),
        }
    }
}

#[derive(Debug)]
pub enum LazyFile<R: Read> {
    NotLoaded(R),
    Loaded(Vec<u8>),
}

impl<R: Read> LazyFile<R> {
    pub fn file(&self) -> Option<&R> {
        match self {
            LazyFile::NotLoaded(file) => Some(file),
            LazyFile::Loaded(_) => None,
        }
    }

    pub fn file_mut(&mut self) -> Option<&mut R> {
        match self {
            LazyFile::NotLoaded(file) => Some(file),
            LazyFile::Loaded(_) => None,
        }
    }

    pub fn bytes(&self) -> Option<&Vec<u8>> {
        match self {
            LazyFile::NotLoaded(_) => None,
            LazyFile::Loaded(bytes) => Some(bytes),
        }
    }

    pub fn bytes_mut(&mut self) -> Option<&mut Vec<u8>> {
        match self {
            LazyFile::NotLoaded(_) => None,
            LazyFile::Loaded(bytes) => Some(bytes),
        }
    }
}

#[derive(Debug)]
pub struct LazyLocalFiles<R: Read> {
    root_url: Url,
    files: BTreeMap<Url, LazyFile<R>>,
}

impl<R: Read> Files for LazyLocalFiles<R> {
    fn root_url(&self) -> &Url {
        &self.root_url
    }

    fn get(&mut self, url: &Url) -> Option<&Vec<u8>> {
        let LazyLocalFiles { files, .. } = self;

        // remove the fragment from the URL
        let lazy_file = if url.path_segments().is_none() {
            files.get_mut(url)
        } else {
            files.get_mut(&url.join("").unwrap())
        };

        if lazy_file.is_none() {
            return None;
        }

        let lazy_file = lazy_file.unwrap();

        return if let LazyFile::Loaded(bytes) = lazy_file {
            // if the file is already loaded, return the bytes
            Some(bytes)
        } else {
            // if not loaded, load and store the bytes
            // get file
            let file = lazy_file.file_mut().unwrap();

            // read the file into memory
            let mut content = Vec::new();
            file.read_to_end(&mut content).unwrap();
            *lazy_file = LazyFile::Loaded(content);

            // return the bytes
            Some(lazy_file.bytes().unwrap())
        };
    }
}

#[derive(Debug, Error)]
pub enum LocalFilesError {
    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Invalid archive")]
    Zip(#[from] ZipError),
}

/// Read files from a ZIP archive.
pub fn read_from_zip<R: Read + Seek>(zip: &mut ZipArchive<R>) -> Result<LocalFiles, LocalFilesError> {
    let mut files = LocalFiles::empty();
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let mut content = Vec::new();
        let url_str = format!("epub:/{}", file.name());
        let url = Url::options().parse(&url_str).unwrap();
        file.read_to_end(&mut content).unwrap();
        files.files.insert(url, content);
    }
    Ok(files)
}

/// Read files from a Reader, which targets a ZIP archive.
pub fn read_from_reader<R: Read + Seek>(reader: R) -> Result<LocalFiles, LocalFilesError> {
    Ok(read_from_zip(&mut ZipArchive::new(reader)?)?)
}

/// Recursively read files from a directory.
fn recurse_files(path: impl AsRef<Path>) -> std::io::Result<Vec<PathBuf>> {
    let mut buf = vec![];
    let entries = read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        let meta = entry.metadata()?;

        if meta.is_dir() {
            let mut subdir = recurse_files(entry.path())?;
            buf.append(&mut subdir);
        }

        if meta.is_file() {
            buf.push(entry.path());
        }
    }

    Ok(buf)
}

/// Read files from a directory.
///
/// It will recursively read all files from the directory.
pub fn read_from_dir(path: impl AsRef<Path>) -> Result<LocalFiles, LocalFilesError> {
    let mut files = LocalFiles::empty();
    let paths = recurse_files(&path)?;
    for file_path in paths {
        let rel_path = file_path.strip_prefix(&path).unwrap();
        let rel_path_str = rel_path.to_str().unwrap().replace("\\", "/");
        let url = Url::parse(&format!("epub:/{}", rel_path_str)).unwrap();
        let content = std::fs::read(&file_path)?;
        files.files.insert(url, content);
    }
    Ok(files)
}

/// Read files from a directory lazily.
///
/// It will recursively get all files' metadata from the directory.
/// When use `get` method, it will read the file into memory.
pub fn lazy_read_from_dir(path: impl AsRef<Path>) -> Result<LazyLocalFiles<File>, LocalFilesError> {
    let mut files = LazyLocalFiles {
        root_url: Url::parse("epub:/").unwrap(),
        files: BTreeMap::new(),
    };
    let paths = recurse_files(&path)?;
    for file_path in paths {
        let rel_path = file_path.strip_prefix(&path).unwrap();
        let rel_path_str = rel_path.to_str().unwrap().replace("\\", "/");
        let url = Url::parse(&format!("epub:/{}", rel_path_str)).unwrap();
        files.files.insert(url, LazyFile::NotLoaded(File::open(&file_path)?));
    }
    Ok(files)
}

/// Read files from a ZIP file.
pub fn read_from_file(file: File) -> Result<LocalFiles, LocalFilesError> {
    Ok(read_from_zip(&mut ZipArchive::new(file)?)?)
}