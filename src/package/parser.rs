use crate::package::manifest::{Manifest, ManifestCheckError, Resource};
use crate::package::metadata::{Link, Meta, Metadata, MetadataCheckError, MetadataElement, Refines};
use crate::package::prefix::{Prefixes, PrefixesStack, DC};
use crate::package::property::{Properties, Property, WithNamespace};
use crate::package::spine::{Spine, SpineReference};
use crate::package::Package;
use crate::utils::invert;
use minidom::Element;
use std::marker::PhantomData;
use std::str::FromStr;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum PackageError {
    #[error("Invalid XML, {0}")]
    ParseError(#[from] minidom::Error),

    #[error("Invalid element: {0}")]
    InvalidElementError(String),

    #[error("Invalid element attribute: {0}")]
    InvalidElementAttrError(String),

    #[error("throw error when checking manifest: {0}")]
    ManifestCheckError(#[from] ManifestCheckError),

    #[error("throw error when checking metadata: {0}")]
    MetadataCheckError(#[from] MetadataCheckError),

    #[error("Unsupported version: {0}, only support 3.0")]
    UnsupportedVersion(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParseOptions {
    /// base url of the package document.
    ///
    /// every url in the package document will be resolved against this url.
    pub base_url: Url,

    pub reserved_prefixes: Prefixes,
}

#[derive(Debug)]
pub struct ParseState {
    pub prefixes_stack: PrefixesStack,
}

#[derive(Debug)]
pub struct PackageParser {
    /// parse options
    pub options: ParseOptions,
    pub parse_state: ParseState,

    _private: PhantomData<()>,
}


impl PackageParser {
    pub fn new(options: ParseOptions) -> Self {
        PackageParser {
            options,
            parse_state: ParseState { prefixes_stack: PrefixesStack::default() },
            _private: Default::default(),
        }
    }

    pub fn clear(&mut self) {
        self.parse_state.prefixes_stack.clear();
    }

    pub fn parse(&mut self, str: &str) -> Result<Package, PackageError> {
        self.clear();
        self.parse_state.prefixes_stack.push(self.options.reserved_prefixes.clone());

        let root = Element::from_reader_with_prefixes(
            str.as_bytes(),
            self.options.reserved_prefixes.inner().clone(),
        ).map_err(PackageError::ParseError)?;

        if root.name() != "package" {
            return Err(PackageError::InvalidElementError("root element is not package".to_string()));
        }

        let package_elem = root;

        let prefixes = package_elem.prefixes
            .declared_prefixes()
            .clone();
        self.parse_state.prefixes_stack.push(Prefixes::new(prefixes));

        let res = self.parse_package(&package_elem);
        self.parse_state.prefixes_stack.pop();
        res
    }

    fn parse_package(&mut self, package_elem: &Element) -> Result<Package, PackageError> {
        // get unique-identifier
        let unique_identifier_ref = parse_attr_some::<String>(&package_elem, "unique-identifier")?;

        // get version
        let version = parse_attr_some(&package_elem, "version")?;

        if version != "3.0" {
            return Err(PackageError::UnsupportedVersion(version));
        }

        // get more attributes
        let prefix = parse_attr(&package_elem, "prefix")?;
        let dir = parse_attr(&package_elem, "dir")?;
        let id = parse_attr(&package_elem, "id")?;
        let lang = parse_attr(&package_elem, "xml:lang")?;

        // get metadata
        let metadata_elem = package_elem.children()
            .find(|n| n.name() == "metadata")
            .ok_or(PackageError::InvalidElementError("metadata is missing".to_string()))?;

        let metadata = self.parse_metadata(metadata_elem)?;

        // get manifest
        let manifest_elem = package_elem.children()
            .find(|n| n.name() == "manifest")
            .ok_or(PackageError::InvalidElementError("manifest is missing".to_string()))?;

        let manifest = self.parse_manifest(manifest_elem)?;

        // get spine
        let spine_elem = package_elem.children()
            .find(|n| n.name() == "spine")
            .ok_or(PackageError::InvalidElementError("spine is missing".to_string()))?;

        let spine = self.parse_spine(spine_elem)?;

        Ok(Package { unique_identifier_ref, version, prefix, dir, id, lang, metadata, manifest, spine })
    }

    fn parse_metadata(&mut self, metadata_elem: &Element) -> Result<Metadata, PackageError> {
        let mut elems = Vec::new();
        let mut metas = Vec::new();
        let mut links = Vec::new();

        let metadata_prefixes = metadata_elem.prefixes.declared_prefixes().clone();
        self.parse_state.prefixes_stack.push(Prefixes::new(metadata_prefixes));

        for elem in metadata_elem.children() {
            let elem_prefixes = elem.prefixes.declared_prefixes().clone();
            self.parse_state.prefixes_stack.push(Prefixes::new(elem_prefixes));

            let res = self.parse_metadata_elem(elem, &mut elems, &mut metas, &mut links);
            self.parse_state.prefixes_stack.pop();
            res?
        }

        Ok(Metadata::new(elems, metas, links)?)
    }

    fn parse_metadata_elem(
        &self,
        elem: &Element,
        elems: &mut Vec<MetadataElement>,
        metas: &mut Vec<Meta>,
        links: &mut Vec<Link>,
    ) -> Result<(), PackageError> {
        match elem.name() {
            // meta element
            "meta" => {
                let id = parse_attr(elem, "id")?;
                let lang = parse_attr(elem, "xml:lang")?;
                let dir = parse_attr(elem, "dir")?;
                let property = parse_attr_some_fn(elem, "property", |s| Property::from_str(s, &self.parse_state.prefixes_stack))?;
                let refines = parse_attr_fn(
                    elem, "refines",
                    |s| Refines::from_relative_url(s, &self.options.base_url),
                )?;
                let scheme = parse_attr_fn(elem, "scheme", |s| Property::from_str(s, &self.parse_state.prefixes_stack))?;
                let value = elem.text();

                metas.push(Meta { id, lang, dir, property, refines, scheme, value });
                Ok(())
            }

            // link element
            "link" => {
                let id = parse_attr(elem, "id")?;
                let href = parse_attr_some_fn(elem, "href", |s| self.options.base_url.join(s))?;
                let hreflang = parse_attr(elem, "hreflang")?;
                let rel = parse_attr_some_fn(elem, "rel", |s| Properties::from_str(s, &self.parse_state.prefixes_stack))?;
                let media_type = parse_attr(elem, "media-type")?;
                let property = parse_attr_fn(elem, "properties", |s| Property::from_str(s, &self.parse_state.prefixes_stack))?;
                let refines = parse_attr_fn(
                    elem, "refines",
                    |s| Refines::from_relative_url(s, &self.options.base_url),
                )?;
                let value = elem.text();

                links.push(Link { id, href, rel, hreflang, media_type, property, refines, value });
                Ok(())
            }

            // other dc: elements
            _ => {
                if elem.ns() == DC.uri {
                    let id = parse_attr(elem, "id")?;
                    let lang = parse_attr(elem, "xml:lang")?;
                    let dir = parse_attr(elem, "dir")?;

                    let tag_name = WithNamespace {
                        ns: elem.ns(),
                        reference: elem.name().to_string(),
                    };

                    elems.push(MetadataElement { id, lang, dir, tag_name });
                    Ok(())
                } else {
                    Err(PackageError::InvalidElementError(format!("Invalid metadata element: {}", elem.name())))
                }
            }
        }
    }

    fn parse_manifest(&mut self, manifest_elem: &Element) -> Result<Manifest, PackageError> {
        let id = manifest_elem.attr("id");
        let resources = manifest_elem.children()
            .map(|elem| {
                let elem_prefixes = elem.prefixes.declared_prefixes().clone();
                self.parse_state.prefixes_stack.push(Prefixes::new(elem_prefixes));

                let res = self.parse_manifest_elem(elem);
                self.parse_state.prefixes_stack.pop();
                res
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Manifest::new(id, resources)?)
    }

    fn parse_manifest_elem(&self, elem: &Element) -> Result<Resource, PackageError> {
        if elem.name() != "item" {
            return Err(PackageError::InvalidElementError("Invalid manifest item".to_string()));
        }

        let id = parse_attr_some(elem, "id")?;
        let href = parse_attr_some_fn(elem, "href", |s| self.options.base_url.join(s))?;
        let media_type = parse_attr_some(elem, "media-type")?;
        let properties = parse_attr_fn(elem, "properties", |s| Properties::from_str(s, &self.parse_state.prefixes_stack))?;
        let fallback = parse_attr(elem, "fallback")?;
        let media_overlay = parse_attr(elem, "media-overlay")?;

        Ok(Resource { id, href, media_type, properties, fallback, media_overlay })
    }

    fn parse_spine(&mut self, spine_elem: &Element) -> Result<Spine, PackageError> {
        let id = parse_attr(spine_elem, "id")?;
        let dir = parse_attr(spine_elem, "page-progression-direction")?;
        let refs = spine_elem.children()
            .map(|elem| {
                let elem_prefixes = elem.prefixes.declared_prefixes().clone();
                self.parse_state.prefixes_stack.push(Prefixes::new(elem_prefixes));

                let res = self.parse_spine_elem(elem);
                self.parse_state.prefixes_stack.pop();
                res
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Spine { id, dir, refs })
    }


    fn parse_spine_elem(&self, elem: &Element) -> Result<SpineReference, PackageError> {
        if elem.name() != "itemref" {
            return Err(PackageError::InvalidElementError("Invalid spine itemref".to_string()));
        }

        let id = parse_attr_some(elem, "idref")?;
        let linear = parse_attr(elem, "linear")?;

        Ok(SpineReference { id, linear })
    }
}

fn parse_attr<T>(elem: &Element, name: &str) -> Result<Option<T>, PackageError>
where
    T: FromStr,
{
    parse_attr_fn(elem, name, |s| s.parse::<T>())
}

fn parse_attr_fn<T, F, E>(
    elem: &Element,
    name: &str,
    f: F,
) -> Result<Option<T>, PackageError>
where
    F: FnOnce(&str) -> Result<T, E>,
{
    let attr_str = elem.attr(name);
    let res = attr_str.map(f);
    invert(res)
        .map_err(|_| PackageError::InvalidElementAttrError(format!("{} is invalid: {}", name, attr_str.unwrap())))
}

fn parse_attr_primitive<'a>(elem: &'a Element, name: &str) -> Result<&'a str, PackageError>
{
    elem.attr(name)
        .ok_or(PackageError::InvalidElementAttrError(format!("{} is missing", name)))
}

fn parse_attr_some<T>(elem: &Element, name: &str) -> Result<T, PackageError>
where
    T: FromStr,
{
    parse_attr_some_fn(elem, name, |s| s.parse::<T>())
}

fn parse_attr_some_fn<T, F, E>(
    elem: &Element,
    name: &str,
    f: F,
) -> Result<T, PackageError>
where
    F: FnOnce(&str) -> Result<T, E>,
{
    let attr_str = elem.attr(name);
    let res = attr_str.ok_or(PackageError::InvalidElementAttrError(format!("{} is missing", name)))?;
    f(res).map_err(|_| PackageError::InvalidElementAttrError(format!("{} is invalid: {}", name, attr_str.unwrap())))
}