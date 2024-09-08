use std::fmt::Debug;
use std::io::Read;
use std::ops::{Deref, DerefMut};

use url::Url;

pub trait Files {
    /// Get the root URL of the files.
    fn root_url(&self) -> &Url;

    /// Get the content of a file by its URL.
    async fn get(&mut self, url: &Url) -> Option<&Vec<u8>>;
}

#[cfg(not(target_arch = "wasm32"))]
mod local;
#[cfg(not(target_arch = "wasm32"))]
pub use local::*;

#[cfg(not(target_arch = "wasm32"))]
mod remote_epub;
#[cfg(not(target_arch = "wasm32"))]
pub use remote_epub::*;

mod remote;
pub use remote::*;
