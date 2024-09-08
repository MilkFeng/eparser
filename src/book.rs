use crate::file::Files;
use crate::oebps::{parse_container, ContainerError};
use crate::package::parser::{PackageError, PackageParseOptions, PackageParser};
use crate::package::prefix::Prefixes;
use crate::package::Package;
use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};
use thiserror::Error;

#[derive(Debug)]
pub struct EpubBook(Vec<Package>);

/// An EPUB book. It is A collection of packages.
///
/// This is the main entry point for working with EPUB books.
impl EpubBook {
    /// Get all the packages in the book.
    pub fn packages(&self) -> &Vec<Package> {
        &self.0
    }
}

impl Deref for EpubBook {
    type Target = Vec<Package>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for EpubBook {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Error)]
pub enum ParseBookError {
    #[error("The book is missing a META-INF/container.xml file")]
    MissingContainer,

    #[error("The book is missing a package: {0}")]
    MissingPackage(String),

    #[error("Failed to parse URL")]
    UrlParseError(#[from] url::ParseError),

    #[error("Failed to parse container.xml file")]
    ParseContainerError(#[from] ContainerError),

    #[error("Failed to parse package")]
    ParsePackageError(#[from] PackageError),

    #[error("Failed to parse UTF-8")]
    Utf8Error(#[from] std::str::Utf8Error),
}

/// Parse an EPUB book.
pub async fn parse_book<F: Files>(files: &mut F) -> Result<EpubBook, ParseBookError> {
    let container = {
        let root_url = files.root_url().clone();
        let url = root_url
            .join("META-INF/container.xml")
            .map_err(ParseBookError::UrlParseError)?;
        let data = files
            .get(&url)
            .await
            .ok_or(ParseBookError::MissingContainer)?;

        let str = std::str::from_utf8(data).map_err(ParseBookError::Utf8Error)?;

        parse_container(str, &root_url).map_err(ParseBookError::ParseContainerError)?
    };

    let package_parse_options = PackageParseOptions {
        base_url: container.rootfiles[0].full_path.clone(),
        reserved_prefixes: Prefixes::reserved(),
    };

    let mut package_parser = PackageParser::new(package_parse_options);

    let mut packages = Vec::new();
    for rootfile in &container.rootfiles {
        let data = files
            .get(&rootfile.full_path)
            .await
            .ok_or_else(|| ParseBookError::MissingPackage(rootfile.full_path.to_string()))?;

        let str = std::str::from_utf8(data).map_err(ParseBookError::Utf8Error)?;

        let package = package_parser
            .parse(str)
            .map_err(ParseBookError::ParsePackageError)?;

        packages.push(package);
    }
    Ok(EpubBook(packages))
}
