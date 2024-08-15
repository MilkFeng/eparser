use std::error::Error;
use std::fmt::Display;
use std::ops::Deref;
use std::str::FromStr;

use minidom::Element;
use thiserror::Error;
use url::Url;

use crate::package::media_type::media_types::OEBPS;
use crate::package::media_type::MediaType;

/// The rootfile element of the container.xml file.
///
/// Each one represents a root file of package document.
#[derive(Debug)]
pub struct Rootfile {
    pub full_path: Url,
    pub media_type: MediaType,
}

/// Container.xml
#[derive(Debug)]
pub struct Container {
    /// The rootfiles element of the container.xml file.
    pub rootfiles: Vec<Rootfile>,
}

/// Errors that can occur when parsing the container.xml file.
#[derive(Debug, Error)]
pub enum ContainerError {
    #[error("Missing rootfiles element")]
    MissingRootfiles,

    #[error("Root file MUST have a full-path attribute but it is missing")]
    MissingFullPath,

    #[error("Root file MUST have a media-type attribute but it is missing")]
    MissingMediaType,

    #[error("Invalid media type, expected application/oebps-package+xml but found {0}")]
    InvalidMediaType(MediaType),

    #[error("Invalid full path, {0}")]
    InvalidFullPath(#[from] url::ParseError),

    #[error("Invalid XML, {0}")]
    ParseError(#[from] minidom::Error),
}

impl FromStr for Container {
    type Err = ContainerError;

    /// Parse the container.xml file.
    ///
    /// Note that if the `full-path` attribute of the `rootfile` element starts with `OPS/`, it will be replaced with `OEBPS/`.
    ///
    /// The structure of the container.xml file is as follows:
    ///
    /// ```xml
    /// <?xml version="1.0" encoding="UTF-8"?>
    /// <container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
    ///     <rootfiles>
    ///         <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
    ///    </rootfiles>
    /// </container>
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_container(s)
    }
}


/// Parse the container.xml file.
fn parse_container(str: &str) -> Result<Container, ContainerError> {
    let rootfiles = str.parse::<Element>()
        .map_err(ContainerError::ParseError)?

        // container
        .children()
        .find(|n| n.name() == "rootfiles")
        .ok_or(ContainerError::MissingRootfiles)?

        // container -> rootfiles
        .children()
        .filter(|n| n.name() == "rootfile")

        // container -> rootfiles -> rootfile
        .map(|n| {
            let full_path_str = n.attr("full-path")
                .ok_or(ContainerError::MissingFullPath)?;
            let media_type_str = n.attr("media-type")
                .ok_or(ContainerError::MissingMediaType)?;

            let full_path_url_str = format!("epub:/{}", full_path_str);
            let full_path = Url::parse(&full_path_url_str)?;

            let media_type = MediaType::new(media_type_str);
            if &media_type != OEBPS.deref() {
                return Err(ContainerError::InvalidMediaType(media_type));
            }

            Ok::<_, ContainerError>(Rootfile {
                full_path,
                media_type,
            })
        })
        .collect::<Result<Vec<Rootfile>, ContainerError>>()?;

    Ok(Container { rootfiles })
}

#[cfg(test)]
mod tests {
    use crate::oebps::parse_container;

    #[test]
    fn test_parse_container() {
        let data = r#"<?xml version="1.0" encoding="UTF-8"?>
<container version="1.0" xmlns="urn:oasis:names:tc:opendocument:xmlns:container">
    <rootfiles>
        <rootfile full-path="OEBPS/content.opf" media-type="application/oebps-package+xml"/>
    </rootfiles>
</container>"#;

        let container = parse_container(data).unwrap();

        assert_eq!(container.rootfiles.len(), 1);
    }
}