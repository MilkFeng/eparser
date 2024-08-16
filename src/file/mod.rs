use std::fmt::Debug;
use std::io::Read;
use std::ops::{Deref, DerefMut};

use url::Url;

pub trait Files {
    /// Get the root URL of the files.
    fn root_url(&self) -> &Url;

    /// Get the content of a file by its URL.
    fn get(&mut self, url: &Url) -> Option<&Vec<u8>>;
}

#[cfg(feature = "local")]
pub mod local;
mod remote;