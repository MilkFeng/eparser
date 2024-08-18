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

/// A Package is made up of:
/// - A [Metadata]: provides a standard way to include publication metadata.
/// contains titles, authors, identifiers, languages, and other metadata.
/// - A [Manifest]: provides an exhaustive list of publication resources used in the rendering of the content.
/// Like xhtml files, images, stylesheets, fonts, and other media.
/// - A [Spine]: provides the linear reading order of the [Resource]s in the [Manifest].
///
/// It is important to point out that [Manifest] contains exactly one [Nav] [Resource] which is a special resource
/// that provides the table of contents of the publication.
///
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
    /// A sugar method to get the [Resource] by id in the [Manifest].
    pub fn get_res_by_id(&self, id: &str) -> Option<&Resource> {
        self.manifest.get_resource_by_id(id)
    }

    /// A sugar method to get the [Resource] by [SpineReference] in the [Manifest].
    pub fn get_res_by_ref(&self, ref_: &SpineReference) -> Option<&Resource> {
        self.manifest.get_resource_by_id(&ref_.id)
    }

    /// A sugar method to get the nav resource in the manifest.
    pub fn nav_resource(&self) -> Option<&Resource> {
        self.manifest.nav_resource()
    }
}