use crate::package::media_type::MediaType;
use crate::package::property::{Properties, Property};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::string::ToString;
use once_cell::sync::Lazy;
use thiserror::Error;
use url::Url;
use crate::package::prefix::OPF;

/// A Publication Resource.
///
/// identifies a publication resource by the URL in its [href] attribute.
#[derive(Debug, PartialEq, Clone)]
pub struct Resource {
    /// Unique identifier for the resource.
    pub id: String,

    /// URL to the resource.
    ///
    /// On Epub 3.3, The value MUST be an [absolute-](https://url.spec.whatwg.org/#absolute-url-string)
    /// or [path-relative-scheme-less-URL](https://url.spec.whatwg.org/#path-relative-scheme-less-url-string) string.
    /// EPUB creators MUST ensure each URL is unique within the manifest scope after parsing.
    ///
    /// If original URL is a [path-relative-scheme-less-URL](https://url.spec.whatwg.org/#path-relative-scheme-less-url-string),
    /// it will be resolved against of the EPUB Publication with `epub` scheme.
    pub href: Url,

    /// The media type of the resource.
    pub media_type: MediaType,

    /// The fallback attribute specifies the fallback for the referenced publication resource
    /// It's value MUST resolve to another item in the manifest.
    ///
    /// For foreign resources, the fallback attribute is REQUIRED.
    pub fallback: Option<String>,

    /// TODO: realize the media-overlay attribute
    pub media_overlay: Option<String>,

    /// The properties attribute is a space-separated list of property values.
    pub properties: Option<Properties>,
}

#[derive(Debug, Error)]
pub enum ManifestCheckError {
    #[error("The id of the resource must be unique, but {0} is duplicated")]
    DeduplicatedId(String),

    #[error("The href of the resource must be unique, but {0} is duplicated")]
    DeduplicatedHref(Url),

    #[error("The nav resource not found")]
    NavResourceNotFound,

    #[error("The id {0} not found in the manifest")]
    IdNotFound(String),
}

/// Manifest provides an exhaustive list of publication resources used in the rendering of the content.
///
/// Do not modify it after it has been created.
#[derive(Debug, Clone)]
pub struct Manifest {
    /// The unique identifier of the manifest element.
    pub id: Option<String>,

    /// The list of resources in the manifest.
    resources: Vec<Resource>,

    /// id to resource index map
    id_to_resource: HashMap<String, usize>,

    /// href to resource index map
    href_to_resource: HashMap<Url, usize>,

    /// The nav resource
    nav_resource: usize,
}

static NAV: Lazy<Property> = Lazy::new(|| {
    Property::from_prefix(&OPF, "nav".to_string())
});

impl Manifest {
    /// Create a new Manifest
    pub fn new(id: Option<&str>, resources: Vec<Resource>) -> Result<Self, ManifestCheckError> {

        let mut id_to_resource = HashMap::new();
        let mut href_to_resource = HashMap::new();

        for (index, resource) in resources.iter().enumerate() {
            let res = id_to_resource.insert(resource.id.clone(), index);
            if res.is_some() {
                return Err(ManifestCheckError::DeduplicatedId(resource.id.clone()));
            }

            let res = href_to_resource.insert(resource.href.clone(), index);
            if res.is_some() {
                return Err(ManifestCheckError::DeduplicatedHref(resource.href.clone()));
            }
        }

        // check fallback
        for resource in resources.iter() {
            if let Some(fallback) = &resource.fallback {
                id_to_resource.get(fallback)
                    .ok_or_else(|| ManifestCheckError::IdNotFound(fallback.clone()))?;
            }
        }

        // check nav
        let nav_resource = resources.iter().position(|resource| {
            resource.properties.as_ref()
                .map(|properties| properties.contains(&NAV))
                .unwrap_or(false)
        }).ok_or(ManifestCheckError::NavResourceNotFound)?;

        Ok(Manifest {
            id: id.map(|id| id.to_string()),
            resources,
            id_to_resource,
            href_to_resource,
            nav_resource,
        })
    }

    /// Get a resource by id
    pub fn get_resource_by_id(&self, id: &str) -> Option<&Resource> {
        self.id_to_resource.get(id)
            .map(|index| &self.resources[*index])
    }

    /// Get a resource by href
    pub fn get_resource_by_href(&self, href: &Url) -> Option<&Resource> {
        self.href_to_resource.get(href)
            .map(|index| &self.resources[*index])
    }

    /// Get the nav resource
    pub fn nav_resource(&self) -> Option<&Resource> {
        self.resources.get(self.nav_resource)
    }
}

impl Deref for Manifest {
    type Target = Vec<Resource>;
    fn deref(&self) -> &Vec<Resource> {
        &self.resources
    }
}

impl DerefMut for Manifest {
    fn deref_mut(&mut self) -> &mut Vec<Resource> {
        &mut self.resources
    }
}

