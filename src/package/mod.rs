use std::marker::PhantomData;
use manifest::Manifest;
use metadata::Metadata;
use spine::Spine;
use crate::package::manifest::Resource;
use crate::package::spine::SpineReference;

pub mod manifest;
pub mod property;
pub mod spine;
pub mod media_type;
pub mod metadata;
pub mod nav;
pub mod parser;
pub mod prefix;

#[derive(Debug)]
pub struct Package {
    /// The unique identifier of the package element.
    pub id: Option<String>,

    /// The unique identifier reference of the package.
    unique_identifier_ref: String,

    /// The version of the EPUB specification to which the publication conforms.
    pub version: String,

    /// [Metadata] provides a standard way to include publication metadata.
    pub metadata: Metadata,

    /// [Manifest] provides an exhaustive list of publication resources used in the rendering of the content.
    pub manifest: Manifest,

    /// [Spine] provides the linear reading order of the [Resource]s in the [Manifest].
    pub spine: Spine,

    pub prefix: Option<String>,
    pub dir: Option<String>,
    pub lang: Option<String>,
}

impl Package {
    pub fn get_res_by_id(&self, id: &str) -> Option<&Resource> {
        self.manifest.get_resource_by_id(id)
    }

    pub fn get_res_by_ref(&self, ref_: &SpineReference) -> Option<&Resource> {
        self.manifest.get_resource_by_id(&ref_.id)
    }
}