use std::cmp::PartialEq;
use std::fmt::Display;
use std::ops::{Deref, DerefMut};

use thiserror::Error;

use crate::package::prefix::{Prefix, PrefixMap};

#[derive(Debug, Error)]
#[error("Invalid namespace: {0:?}")]
pub struct NamespaceError(Option<String>);


/// A value with a namespace.
///
/// It can be used to represent a property or a tag name.
#[derive(Debug, PartialEq, Clone, Hash, Eq, Ord, PartialOrd)]
pub struct WithNamespace {
    /// The namespace of the value.
    pub ns: String,

    /// The reference.
    pub reference: String,
}

impl WithNamespace {
    /// Create a new WithNamespace from a namespace and a reference.
    pub fn new(ns: String, reference: String) -> Self {
        WithNamespace { ns, reference }
    }

    /// Create a new WithNamespace from a prefix and a reference.
    ///
    /// The prefix contains the namespace URI.
    pub fn from_prefix(prefix: &Prefix, reference: String) -> Self {
        WithNamespace::new(prefix.uri.clone(), reference)
    }

    /// Create a new WithNamespace from a string.
    ///
    /// The string must have the format `prefix:reference` or `reference`.
    /// It will use `prefix` to get the namespace URI from the `prefixes` map.
    /// If `s` does not contain a prefix, it will use the `None` to get the namespace URI.
    ///
    /// # Arguments
    ///
    /// - `s` - A string with the format `prefix:reference` or `reference`.
    /// - `prefixes` - A map of prefixes.
    ///
    /// # Errors
    ///
    /// It will return an error if the prefix is not found in the `prefixes` map.
    ///
    /// # Examples
    ///
    /// ```
    /// use eparser::package::prefix::{Prefix, PrefixMap, Prefixes, DC};
    /// use eparser::package::property::WithNamespace;
    ///
    /// fn main() {
    ///     let prefixes = Prefixes::reserved();
    ///     let with_ns = WithNamespace::from_str("dc:title", &prefixes);
    ///
    ///     assert!(with_ns.is_ok());
    ///
    ///     let with_ns = with_ns.unwrap();
    ///     assert_eq!(&with_ns.ns, &DC.uri);
    ///     assert_eq!(with_ns.reference, "title");
    /// }
    /// ```
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
/// # Reference
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

    /// Create a new Property from a prefix and a reference.
    pub fn from_prefix(prefix: &Prefix, reference: String) -> Self {
        Property(WithNamespace::from_prefix(prefix, reference))
    }

    /// Create a new Property from a string with the format `prefix:reference` or `reference`.
    ///
    /// See [WithNamespace::from_str]
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

    /// Create a new Properties from a string.
    ///
    /// The string must have the format `property1 property2 property3`.
    ///
    /// See [Property::from_str] for more information .
    pub fn from_str(s: &str, prefixes: &impl PrefixMap) -> Result<Self, NamespaceError> {
        let properties = s.split_whitespace()
            .map(|property| Property::from_str(property, prefixes))
            .collect::<Result<Vec<Property>, NamespaceError>>()?;
        Ok(Properties(properties))
    }

    /// Check if the properties contains a property.
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