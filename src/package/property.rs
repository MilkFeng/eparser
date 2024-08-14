use crate::package::prefix::{Prefix, PrefixMap};
use std::cmp::PartialEq;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Invalid namespace: {0:?}")]
pub struct NamespaceError(Option<String>);

#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct WithNamespace {
    pub ns: String,
    pub reference: String,
}

impl WithNamespace {
    pub fn new(ns: String, reference: String) -> Self {
        WithNamespace { ns, reference }
    }

    pub fn from_prefix(prefix: &Prefix, reference: String) -> Self {
        WithNamespace::new(prefix.uri.clone(), reference)
    }

    pub fn from_str(s: &str, prefixes: &impl PrefixMap) -> Result<Self, NamespaceError> {
        let split: Vec<&str> = s.split(':').collect();
        let prefix = if split.len() == 2 { Some(split[0].to_string()) } else { None };
        let namespace = prefixes.get(&prefix)
            .ok_or(NamespaceError(prefix.clone()))?
            .clone();
        let reference = split.last().unwrap().to_string();
        Ok(WithNamespace {
            ns: namespace,
            reference,
        })
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
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Property(WithNamespace);

impl Deref for Property {
    type Target = WithNamespace;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Property {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


impl Property {
    /// Create a new Property
    pub fn new(ns: String, reference: String) -> Self {
        Property(WithNamespace::new(ns, reference))
    }

    pub fn from_prefix(prefix: &Prefix, reference: String) -> Self {
        Property(WithNamespace::from_prefix(prefix, reference))
    }

    pub fn from_str(s: &str, prefixes: &impl PrefixMap) -> Result<Self, NamespaceError> {
        Ok(Property(WithNamespace::from_str(s, prefixes)?))
    }
}


/// A white space-separated list of property values.
#[derive(Debug, PartialEq, Clone)]
pub struct Properties(Vec<Property>);

impl Properties {
    /// Create a new Properties
    pub fn new(properties: Vec<Property>) -> Self {
        Properties(properties)
    }

    pub fn from_str(s: &str, prefixes: &impl PrefixMap) -> Result<Self, NamespaceError> {
        let properties = s.split_whitespace()
            .map(|property| Property::from_str(property, prefixes))
            .collect::<Result<Vec<Property>, NamespaceError>>()?;
        Ok(Properties(properties))
    }

    pub fn contains(&self, property: &Property) -> bool {
        self.iter().any(|p| p == property)
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