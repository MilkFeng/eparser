use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};

use crate::package::prefix::prefixes::*;
use once_cell::sync::Lazy;

/// A map of prefixes to namespaces.
///
/// This trait is used to get the namespace URI for a given prefix.
pub trait PrefixMap {
    /// Get the namespace URI for a given prefix.
    ///
    /// # Arguments
    ///
    /// - `prefix` - The prefix to get the namespace URI for.
    /// If the prefix is `None`, the default namespace is returned.
    ///
    /// # Returns
    ///
    /// The namespace URI for the given prefix, or `None` if the prefix is not found.
    fn get(&self, prefix: &Option<String>) -> Option<&String>;
}

/// A prefix for a namespace.
///
/// There are a number of predefined prefixes for common namespaces:
/// - `dc`[DC] for Dublin Core
/// - `dcterms`[DCTERMS] for Dublin Core Terms
/// - `a11y`[A11Y] for Accessibility Metadata
/// - `marc`[MARC] for MARC Code Lists
/// - `media`[MEDIA] for Media Overlays
/// - `onix`[ONIX] for ONIX Code Lists
/// - `rendition`[RENDITION] for Rendition Metadata
/// - `schema`[SCHEMA] for Schema.org
/// - `xsd`[XSD] for XML Schema
/// - `msv`[MSV] for Magazine Structure Vocabulary
/// - `prism`[PRISM] for PRISM Code Lists
///
/// The default prefix for the OPF namespace is `None`, representing the default namespace `opf`[OPF].
///
///
/// # Reference
///
/// [EPUB 3.3 SPEC reserved-prefixes](https://www.w3.org/TR/epub-33/#sec-reserved-prefixes)
#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Prefix {
    /// The name of the prefix.
    ///
    /// `None` represents the default namespace.
    pub name: Option<String>,

    /// The URI of the namespace.
    pub uri: String,
}

pub mod prefixes {
    use once_cell::sync::Lazy;

    use crate::package::prefix::Prefix;

    pub static DC: Lazy<Prefix> = Lazy::new(|| Prefix {
        name: Some("dc".to_string()),
        uri: "http://purl.org/dc/elements/1.1/".to_string(),
    });

    pub static DCTERMS: Lazy<Prefix> = Lazy::new(|| Prefix {
        name: Some("dcterms".to_string()),
        uri: "http://purl.org/dc/terms/".to_string(),
    });

    pub static A11Y: Lazy<Prefix> = Lazy::new(|| Prefix {
        name: Some("a11y".to_string()),
        uri: "http://www.idpf.org/epub/vocab/package/a11y/#".to_string(),
    });

    pub static MARC: Lazy<Prefix> = Lazy::new(|| Prefix {
        name: Some("marc".to_string()),
        uri: "http://id.loc.gov/vocabulary/".to_string(),
    });

    pub static MEDIA: Lazy<Prefix> = Lazy::new(|| Prefix {
        name: Some("media".to_string()),
        uri: "http://www.idpf.org/epub/vocab/overlays/#".to_string(),
    });

    pub static ONIX: Lazy<Prefix> = Lazy::new(|| Prefix {
        name: Some("onix".to_string()),
        uri: "http://www.editeur.org/ONIX/book/codelists/current.html#".to_string(),
    });

    pub static RENDITION: Lazy<Prefix> = Lazy::new(|| Prefix {
        name: Some("rendition".to_string()),
        uri: "http://www.idpf.org/vocab/rendition/#".to_string(),
    });

    pub static SCHEMA: Lazy<Prefix> = Lazy::new(|| Prefix {
        name: Some("schema".to_string()),
        uri: "http://schema.org/".to_string(),
    });

    pub static XSD: Lazy<Prefix> = Lazy::new(|| Prefix {
        name: Some("xsd".to_string()),
        uri: "http://www.w3.org/2001/XMLSchema#".to_string(),
    });

    pub static MSV: Lazy<Prefix> = Lazy::new(|| Prefix {
        name: Some("msv".to_string()),
        uri: "http://www.idpf.org/epub/vocab/structure/magazine/#".to_string(),
    });

    pub static PRISM: Lazy<Prefix> = Lazy::new(|| Prefix {
        name: Some("prism".to_string()),
        uri: "http://www.prismstandard.org/specifications/3.0/PRISM_CV_Spec_3.0.htm#".to_string(),
    });

    /// The default prefix for the OPF namespace.
    pub static OPF: Lazy<Prefix> = Lazy::new(|| Prefix {
        name: None,
        uri: "http://www.idpf.org/2007/opf".to_string(),
    });
}

pub type PrefixesInner = BTreeMap<Option<String>, String>;

/// A map of prefixes to namespaces.
#[derive(Debug, PartialEq, Clone)]
pub struct Prefixes(PrefixesInner);

/// The reserved prefixes. See [Prefix]
pub static RESERVED: Lazy<PrefixesInner> = Lazy::new(|| {
    let mut prefixes = BTreeMap::new();
    prefixes.insert(DC.name.clone(), DC.uri.clone());
    prefixes.insert(DCTERMS.name.clone(), DCTERMS.uri.clone());
    prefixes.insert(A11Y.name.clone(), A11Y.uri.clone());
    prefixes.insert(MARC.name.clone(), MARC.uri.clone());
    prefixes.insert(MEDIA.name.clone(), MEDIA.uri.clone());
    prefixes.insert(ONIX.name.clone(), ONIX.uri.clone());
    prefixes.insert(RENDITION.name.clone(), RENDITION.uri.clone());
    prefixes.insert(SCHEMA.name.clone(), SCHEMA.uri.clone());
    prefixes.insert(XSD.name.clone(), XSD.uri.clone());
    prefixes.insert(MSV.name.clone(), MSV.uri.clone());
    prefixes.insert(PRISM.name.clone(), PRISM.uri.clone());
    prefixes
});

impl PrefixMap for Prefixes {
    fn get(&self, prefix: &Option<String>) -> Option<&String> {
        self.0.get(prefix)
    }
}

impl Prefixes {
    /// Create a new Prefixes form a map of prefixes to namespaces.
    pub fn new(prefixes: PrefixesInner) -> Self {
        Prefixes(prefixes)
    }

    /// All the reserved prefixes.
    ///
    /// # Reference
    ///
    /// [EPUB 3.3 SPEC reserved-prefixes](https://www.w3.org/TR/epub-33/#sec-reserved-prefixes)
    pub fn reserved() -> Self {
        RESERVED.clone().into()
    }

    /// Get the inner map of prefixes to namespaces.
    pub fn inner(&self) -> &PrefixesInner {
        &self.0
    }
}

impl Deref for Prefixes {
    type Target = PrefixesInner;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Prefixes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Prefixes> for BTreeMap<Option<String>, String> {
    fn from(value: Prefixes) -> Self {
        value.0
    }
}

impl Into<Prefixes> for BTreeMap<Option<String>, String> {
    fn into(self) -> Prefixes {
        Prefixes(self)
    }
}

/// A stack of prefixes.
///
/// It is used to record the prefixes declared in the XML document tree.
#[derive(Debug, PartialEq, Clone)]
pub struct PrefixesStack(Vec<Prefixes>);

impl Default for PrefixesStack {
    fn default() -> Self {
        PrefixesStack(vec![])
    }
}

impl Deref for PrefixesStack {
    type Target = Vec<Prefixes>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for PrefixesStack {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl PrefixesStack {
    /// Create a new PrefixesStack from a list of Prefixes.
    pub fn new(prefixes: Vec<Prefixes>) -> Self {
        PrefixesStack(prefixes)
    }
}

impl PrefixMap for PrefixesStack {
    /// Get the namespace URI for a given prefix.
    ///
    /// It will find from the top of the stack to the bottom to see if the Prefixes has been pushed before.
    fn get(&self, prefix: &Option<String>) -> Option<&String> {
        // from top to bottom
        for prefixes in self.0.iter().rev() {
            if let Some(uri) = prefixes.get(prefix) {
                return Some(uri);
            }
        }
        None
    }
}
