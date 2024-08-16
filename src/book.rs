use std::fmt::{Debug, Display};

use thiserror::Error;
use url::Url;

use crate::file::Files;
use crate::oebps::{Container, ContainerError};
use crate::package::manifest::Resource;
use crate::package::Package;
use crate::package::parser::{PackageError, PackageParseOptions, PackageParser};
use crate::package::prefix::Prefixes;

#[derive(Debug)]
pub struct EpubBook<F: Files> {
    /// All the packages in the book.
    packages: Vec<Package>,

    /// All the files in the book.
    files: F,
}

impl<F: Files> EpubBook<F> {
    /// Get all the packages in the book.
    pub fn packages(&self) -> &Vec<Package> {
        &self.packages
    }

    /// Get all the files in the book.
    pub fn files(&self) -> &F {
        &self.files
    }

    /// Get a package by its index.
    pub fn get_file_by_resource(&mut self, resource: &Resource) -> Option<&Vec<u8>> {
        self.get_file_by_url(&resource.href)
    }

    /// Get a file by its URL.
    pub fn get_file_by_url(&mut self, url: &Url) -> Option<&Vec<u8>> {
        self.files.get(url)
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

    #[error("Failed to parse UTF-8")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("Failed to parse package")]
    ParsePackageError(#[from] PackageError),
}

/// Parse an EPUB book.
pub fn parse_book<F: Files>(mut files: F) -> Result<EpubBook<F>, ParseBookError> {
    let container = {
        let url = files.root_url()
            .join("META-INF/container.xml")
            .map_err(ParseBookError::UrlParseError)?;

        let data = files.get(&url)
            .ok_or(ParseBookError::MissingContainer)?;

        let str = std::str::from_utf8(data)
            .map_err(ParseBookError::Utf8Error)?;

        str.parse::<Container>()
            .map_err(ParseBookError::ParseContainerError)?
    };

    let package_parse_options = PackageParseOptions {
        base_url: container.rootfiles[0].full_path.clone(),
        reserved_prefixes: Prefixes::reserved(),
    };

    let mut package_parser = PackageParser::new(package_parse_options);

    let packages = container.rootfiles.iter()
        .map(|rootfile| {
            let data = files.get(&rootfile.full_path)
                .ok_or_else(|| ParseBookError::MissingPackage(rootfile.full_path.to_string()))?;

            let str = std::str::from_utf8(data)
                .map_err(ParseBookError::Utf8Error)?;

            package_parser.parse(str)
                .map_err(ParseBookError::ParsePackageError)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(EpubBook { packages, files })
}