use crate::file::Files;
use std::collections::BTreeMap;
use std::error::Error;
use std::io::{Cursor, Read};
use url::Url;
use zip::ZipArchive;

#[derive(Debug)]
pub struct RemoteEpub {
    original_url: Url,
    logical_root_url: Url,
    client: reqwest::Client,

    has_fetched_zip: bool,
    fetch_zip_error: bool,
    files: BTreeMap<Url, Vec<u8>>,
}

impl RemoteEpub {
    async fn fetch_zip(&mut self) -> Result<(), Box<dyn Error>> {
        // fetch zip file from original_url and extract files
        let response = self.client.get(self.original_url.clone()).send().await?;
        let stream = response.bytes().await?;
        let mut reader = Cursor::new(stream);
        let mut zip = ZipArchive::new(&mut reader)?;
        for i in 0..zip.len() {
            let mut file = zip.by_index(i)?;
            let mut content = Vec::new();
            file.read_to_end(&mut content)?;
            let url = self.logical_root_url.join(file.name())?;
            self.files.insert(url, content);
        }
        Ok(())
    }
}

impl Files for RemoteEpub {
    fn root_url(&self) -> &Url {
        &self.logical_root_url
    }

    async fn get(&mut self, url: &Url) -> Option<&Vec<u8>> {
        // if `has_fetched_zip` is false, fetch zip file from original_url and extract files
        if !self.has_fetched_zip {
            if self.fetch_zip_error {
                return None;
            }
            if self.fetch_zip().await.is_err() {
                self.fetch_zip_error = true;
            } else {
                self.has_fetched_zip = true;
            }
            if self.fetch_zip_error {
                return None;
            }
        }
        self.files.get(url)
    }
}

/// Read files from an EPUB URL.
pub fn read_from_epub_url(url: Url) -> RemoteEpub {
    RemoteEpub {
        original_url: url.clone(),
        logical_root_url: url,
        client: reqwest::Client::builder().build().unwrap(),
        has_fetched_zip: false,
        fetch_zip_error: false,
        files: BTreeMap::new(),
    }
}

/// Read files from an EPUB URL string.
pub fn read_from_epub_url_str(url: &str) -> Result<RemoteEpub, url::ParseError> {
    let url = Url::parse(url)?;
    Ok(read_from_epub_url(url))
}
