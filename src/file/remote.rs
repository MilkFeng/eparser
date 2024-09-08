use crate::file::Files;
use std::collections::BTreeMap;
use std::fmt::Debug;
use thiserror::Error;
use url::Url;

#[cfg(target_arch = "wasm32")]
use reqwest_wasm as reqwest;

#[derive(Clone, Debug)]
pub struct RemoteFiles {
    url: Url,
    client: reqwest::Client,
    cache: BTreeMap<Url, Vec<u8>>,
}

impl Files for RemoteFiles {
    fn root_url(&self) -> &Url {
        &self.url
    }

    async fn get(&mut self, url: &Url) -> Option<&Vec<u8>> {
        if !self.cache.contains_key(url) {
            // fetch the file from the remote server
            let response = self.client.get(url.clone()).send().await;
            if let Ok(response) = response {
                let data = response.bytes().await;
                if let Ok(data) = data {
                    self.cache.insert(url.clone(), data.to_vec());
                }
            }
        }
        self.cache.get(url)
    }
}

impl RemoteFiles {
    pub fn new(url: Url) -> Self {
        RemoteFiles {
            url,
            cache: BTreeMap::new(),
            client: reqwest::Client::builder().build().unwrap(),
        }
    }

    pub fn new_with_client(url: Url, client: reqwest::Client) -> Self {
        RemoteFiles {
            url,
            cache: BTreeMap::new(),
            client,
        }
    }
}

#[derive(Debug, Error)]
pub enum RemoteError {
    #[error("Failed to parse URL")]
    UrlParseError(#[from] url::ParseError),
}

/// Read files from a remote URL.
pub async fn read_from_url_str(url: &str) -> Result<RemoteFiles, RemoteError> {
    let url = Url::parse(url).map_err(RemoteError::UrlParseError)?;
    Ok(RemoteFiles::new(url))
}

/// Read files from a remote URL.
pub async fn read_from_url(url: Url) -> RemoteFiles {
    RemoteFiles::new(url)
}
