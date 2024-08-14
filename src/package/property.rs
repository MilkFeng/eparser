use std::cmp::PartialEq;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::ops::{Deref, DerefMut};
use std::str::FromStr;
use url::Url;

/// The prefix attribute defines prefix mappings for use in property staticues.
///
/// # References
///
/// [EPUB 3.3 SPEC prefix-attr](https://www.w3.org/TR/epub-33/#sec-prefix-attr)
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Prefix {
    /// A string that holds the prefix
    pub prefix: String,

    /// The URL
    pub url: Url,
}

impl Prefix {
    /// Create a new Prefix
    pub fn new(prefix: &str, url: &str) -> Self {
        Prefix {
            prefix: prefix.to_string(),
            url: Url::parse(url).unwrap(),
        }
    }

    /// Convert a string to a [Prefix].
    ///
    /// If the string does not contains ":", find the prefix in reserved prefixes.
    /// If the string contains ":", split the string by ":" and return a new prefix.
    pub fn from_string(string: &str) -> Result<Self, ()> {
        let parts: Vec<&str> = string.splitn(2, ':').collect();
        if parts.len() == 1 {
            reserved_prefixes::ALL_RESERVED_PREFIXES.iter()
                .find(|&&prefix| prefix.prefix == string)
                .map(|&prefix| prefix.deref().clone())
                .ok_or(())
        } else {
            Ok(Prefix::new(parts[0], parts[1]))
        }
    }

    /// Check if the prefix is reserved
    fn is_reserved(&self) -> bool {
        reserved_prefixes::ALL_RESERVED_PREFIXES.iter()
            .any(|&prefix| prefix.deref().prefix == self.prefix)
    }
}


impl PartialEq<str> for Prefix {
    fn eq(&self, other: &str) -> bool {
        Prefix::from_string(other)
            .map(|other| *self == other)
            .unwrap_or(false)
    }
}

impl From<&str> for Prefix {
    fn from(string: &str) -> Self {
        Prefix::from_string(string).unwrap()
    }
}

impl FromStr for Prefix {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Prefix::from_string(s)
    }
}

/// Contains All reserved prefixes for EPUB 3.3
///
/// Reserved prefixes do not need to be declared in the prefix attribute.
pub(crate) mod reserved_prefixes {
    use crate::property::Prefix;
    use once_cell::sync::Lazy;

    /// Contains following "Conformance Properties":
    /// - certifiedBy
    /// - certifierCredential
    /// - certifierReport
    pub static A11Y: Lazy<Prefix> = Lazy::new(|| Prefix::new("a11y", "http://www.idpf.org/epub/vocab/package/a11y/#"));

    /// Contains properties from [dcterms](http://purl.org/dc/terms/)
    pub static DCTERMS: Lazy<Prefix> = Lazy::new(|| Prefix::new("dcterms", "http://purl.org/dc/terms/"));

    /// Contains properties from [marc](http://id.loc.gov/vocabulary/)
    pub static MARC: Lazy<Prefix> = Lazy::new(|| Prefix::new("marc", "http://id.loc.gov/vocabulary/"));

    /// Contains following "Media Overlays Properties":
    /// - active-class
    /// - duration
    /// - narration
    /// - playback-active-class
    pub static MEDIA: Lazy<Prefix> = Lazy::new(|| Prefix::new("media", "http://www.idpf.org/epub/vocab/overlays/#"));

    /// Contains properties from [onix](http://www.editeur.org/ONIX/book/codelists/current.html#)
    pub static ONIX: Lazy<Prefix> = Lazy::new(|| Prefix::new("onix", "http://www.editeur.org/ONIX/book/codelists/current.html#"));

    /// Contains following "Metadata meta Properties":
    /// - flow
    /// - layout
    /// - orientation
    /// - spread
    /// - viewport
    ///
    /// Contains following "Spine `itemref` Properties":
    /// - align-x-center
    /// - flow-auto
    /// - flow-paginated
    /// - flow-scrolled-continuous
    /// - flow-scrolled-doc
    /// - layout-pre-paginated
    /// - layout-reflowable
    /// - orientation-auto
    /// - orientation-landscape
    /// - orientation-portrait
    /// - page-spread-center
    /// - page-spread-left
    /// - page-spread-right
    /// - spread-auto
    /// - spread-both
    /// - spread-landscape
    /// - spread-none
    /// - spread-portrait
    pub static RENDITION: Lazy<Prefix> = Lazy::new(|| Prefix::new("rendition", "http://www.idpf.org/vocab/rendition/#"));

    /// Contains properties from [schema](http://schema.org/)
    pub static SCHEMA: Lazy<Prefix> = Lazy::new(|| Prefix::new("schema", "http://schema.org/"));

    /// Contains properties from [xsd](http://www.w3.org/2001/XMLSchema#)
    pub static XSD: Lazy<Prefix> = Lazy::new(|| Prefix::new("xsd", "http://www.w3.org/2001/XMLSchema#"));

    /// Contains all reserved prefixes
    pub static ALL_RESERVED_PREFIXES: [&Lazy<Prefix>; 8] = [
        &A11Y, &DCTERMS, &MARC, &MEDIA, &ONIX, &RENDITION, &SCHEMA, &XSD
    ];
}

/// A reference to a prefix
#[derive(Debug, PartialEq, Clone)]
pub(crate) enum PrefixReference {
    /// A prefix reference that holds a prefix
    Prefix(String),

    /// A prefix reference that holds the default vocabulary
    Default,
}

impl PrefixReference {
    /// Create a new PrefixReference
    pub fn new(prefix: Option<&str>) -> Self {
        let prefix = prefix
            .map(|prefix| prefix.trim().to_string());

        match prefix {
            Some(prefix) => PrefixReference::Prefix(prefix),
            None => PrefixReference::Default,
        }
    }

    /// Convert a string to a [PrefixReference].
    ///
    /// # Arguments
    ///
    /// - `string` - A string slice that holds the prefix reference
    ///
    /// # Examples
    ///
    /// - "dcterms" -> PrefixReference(Some(dcterms))
    /// - "" -> PrefixReference(None)
    pub fn from_string(string: &str) -> Self {
        let prefix = string.trim();
        if prefix.is_empty() {
            PrefixReference::Default
        } else {
            PrefixReference::Prefix(prefix.to_string())
        }
    }
}

impl PartialEq<str> for PrefixReference {
    fn eq(&self, other: &str) -> bool {
        PrefixReference::from_string(other) == *self
    }
}

impl From<&str> for PrefixReference {
    fn from(string: &str) -> Self {
        PrefixReference::from_string(string)
    }
}

impl FromStr for PrefixReference {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PrefixReference::from_string(s))
    }
}


/// The property data type is a compact means of expressing a URL and
/// consists of an OPTIONAL prefix separated from a reference by a colon.
///
/// Refer to each element's definition for the reserved vocabulary for the attribute.
/// For example: `dcterms:modified`, `schema:version`, `mathml`
///
/// # References
///
/// [EPUB 3.3 SPEC property-datatype](https://www.w3.org/TR/epub-33/#sec-property-datatype)
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Property {
    /// A prefix reference
    pub prefix: PrefixReference,

    /// A string that holds the reference
    pub reference: String,
}

impl Property {
    /// Create a new Property
    pub fn new(prefix: PrefixReference, reference: &str) -> Self {
        Property {
            prefix,
            reference: reference.trim().to_string(),
        }
    }

    /// Convert a string to a [Property].
    ///
    /// # Arguments
    ///
    /// - `string` - A string slice that holds the property
    ///
    /// # Examples
    ///
    /// - "dcterms:modified" -> Property(dcterms, modified)
    /// - "modified" -> Property(null, modified)
    pub fn from_string(string: &str) -> Result<Self, ()> {
        let parts: Vec<&str> = string.splitn(2, ':').collect();
        if parts.len() == 1 {
            Ok(Property::new(PrefixReference::Default, string.trim()))
        } else {
            let prefix = PrefixReference::from_string(parts[0]);
            Ok(Property::new(prefix, parts[1].trim()))
        }
    }
}

impl From<String> for Property {
    fn from(string: String) -> Self {
        Property::from_string(&string).unwrap()
    }
}

impl FromStr for Property {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Property::from_string(s)
    }
}

impl PartialEq<str> for Property {
    fn eq(&self, other: &str) -> bool {
        Property::from_string(other)
            .map(|other| *self == other)
            .unwrap_or(false)
    }
}


/// A white space-separated list of property values.
#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Properties(Vec<Property>);

impl Properties {
    /// Create a new Properties
    pub fn new(properties: Vec<Property>) -> Self {
        Properties(properties)
    }

    /// Convert a string to a [Properties].
    ///
    /// # Arguments
    ///
    /// - `string` - A string slice that holds the properties
    pub fn from_string(string: &str) -> Result<Self, ()> {
        let properties: Vec<Property> = string.split_whitespace()
            .map(|property| Property::from_string(property))
            .collect::<Result<Vec<Property>, ()>>()?;
        Ok(Properties::new(properties))
    }
}

impl Deref for Properties {
    type Target = Vec<Property>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Properties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<&str> for Properties {
    fn from(string: &str) -> Self {
        Properties::from_string(&string).unwrap()
    }
}

impl FromStr for Properties {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Properties::from_string(s)
    }
}


// TODO: Implement Error

#[derive(Debug, PartialEq, Clone)]
struct PropertyParseError(String);

impl Display for PropertyParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse property: {}", self.0)
    }
}

impl Error for PropertyParseError {}