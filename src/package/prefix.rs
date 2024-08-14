use std::collections::BTreeMap;
use std::ops::{Deref, DerefMut};
use once_cell::sync::Lazy;


pub trait PrefixMap {
    fn get(&self, prefix: &Option<String>) -> Option<&String>;
}


#[derive(Debug, PartialEq, Clone, Hash, Eq)]
pub struct Prefix {
    pub name: Option<String>,
    pub uri: String,
}

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

/// The default prefix for the OPF namespace.
pub static OPF: Lazy<Prefix> = Lazy::new(|| Prefix {
    name: None,
    uri: "http://www.idpf.org/2007/opf".to_string(),
});

#[derive(Debug, PartialEq, Clone)]
pub struct Prefixes(pub BTreeMap<Option<String>, String>);

impl PrefixMap for Prefixes {
    fn get(&self, prefix: &Option<String>) -> Option<&String> {
        self.0.get(prefix)
    }
}

impl Prefixes {
    pub fn new(prefixes: BTreeMap<Option<String>, String>) -> Self {
        Prefixes(prefixes)
    }

    pub fn reserved() -> Self {
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
        Prefixes(prefixes)
    }
}

impl Deref for Prefixes {
    type Target = BTreeMap<Option<String>, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Prefixes {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

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

    pub fn new(prefixes: Vec<Prefixes>) -> Self {
        PrefixesStack(prefixes)
    }

    pub fn get(&self, prefix: &Option<String>) -> Option<&String> {
        for prefixes in self.0.iter().rev() {
            if let Some(uri) = prefixes.get(prefix) {
                return Some(uri);
            }
        }
        None
    }
}

impl PrefixMap for PrefixesStack {
    fn get(&self, prefix: &Option<String>) -> Option<&String> {
        self.get(prefix)
    }
}