use crate::file::Files;
use crate::oebps::{Container, ContainerError};
use crate::package::parser::{PackageError, PackageParser, ParseOptions};
use crate::package::Package;
use std::fmt::{Debug, Display};
use std::io::BufReader;
use minidom::Element;
use thiserror::Error;
use crate::package::prefix::Prefixes;

#[derive(Debug)]
pub struct EpubBook {
    /// All the packages in the book.
    packages: Vec<Package>,

    /// All the files in the book.
    files: Files,
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
pub fn parse_book(files: Files) -> Result<EpubBook, ParseBookError> {
    let container = {
        let url = files.root_url
            .join("META-INF/container.xml")
            .map_err(ParseBookError::UrlParseError)?;

        let data = files.get(&url)
            .ok_or(ParseBookError::MissingContainer)?;

        let str = std::str::from_utf8(data)
            .map_err(ParseBookError::Utf8Error)?;

        str.parse::<Container>()
            .map_err(ParseBookError::ParseContainerError)?
    };

    let package_parse_options = ParseOptions {
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