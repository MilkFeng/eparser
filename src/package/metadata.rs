use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use chrono::{Date, DateTime, Utc};
use url::Url;
use crate::property::{Properties, Property};

/// The basic metadata element of an EPUB.
///
/// # References
///
/// [EPUB 3.3 SPEC metadata-elem](https://www.w3.org/TR/epub-33/#sec-metadata-elem)
#[derive(Debug, PartialEq, Clone)]
pub struct MetadataElement {
    /// The ID of the meta element.
    pub id: Option<String>,

    /// The `xml:lang` attribute of the meta element.
    pub lang: Option<String>,

    /// The `dir` attribute of the meta element.
    pub dir: Option<String>,

    /// The property of the meta element.
    ///
    /// # Examples
    ///
    /// `dc:title`, `dc:creator`, `dc:language`
    pub property: Property,
}


/// Establishes an association between the current expression and
/// the element or resource identified by its value.
/// EPUB creators MUST use as the value a path-relative-scheme-less-URL string,
/// optionally followed by U+0023 (#) and a URL-fragment string that references
/// the resource or element they are describing.
///
/// The path-relative-scheme-less-URL string will be parsed as a URL with the
/// "refines" scheme, and the URL-fragment string will be parsed as a URL fragment.
#[derive(Debug, PartialEq, Clone)]
pub struct Refines(Url);

impl Deref for Refines {
    type Target = Url;

    fn deref(&self) -> &Url {
        &self.0
    }
}

impl DerefMut for Refines {
    fn deref_mut(&mut self) -> &mut Url {
        &mut self.0
    }
}

impl From<Url> for Refines {
    fn from(url: Url) -> Self {
        Refines(url)
    }
}

impl From<&str> for Refines {
    fn from(url: &str) -> Self {
        Refines(Url::parse(url).unwrap())
    }
}

/// Meta element
#[derive(Debug, PartialEq, Clone)]
pub struct Meta {
    /// The unique identifier of the \<meta\> element.
    pub id: Option<String>,

    /// The `xml:lang` attribute of the \<meta\> element.
    pub lang: Option<String>,

    /// The `dir` attribute of the \<meta\> element.
    pub dir: Option<String>,

    /// The property attribute of the meta element.
    pub property: Property,

    /// The refines attribute of the meta element.
    pub refines: Option<Refines>,

    /// The scheme attribute identifies the system or scheme the EPUB creator obtained the element's value from.
    /// The value of the attribute MUST be a property data type value that resolves to the resource that defines the scheme.
    /// The scheme attribute does not have a default vocabulary (i.e., all values require a prefix).
    pub scheme: Option<Property>,

    /// The value of the meta element.
    pub value: String,
}

/// The link element associates resources with an EPUB publication, such as metadata records.
#[derive(Debug, PartialEq, Clone)]
pub struct Link {
    /// The unique identifier of the link element.
    pub id: Option<String>,

    /// A valid URL string that references a resource.
    pub href: Url,

    /// The REQUIRED rel attribute takes a space-separated list of property values that
    /// establish the relationship the linked resource has with the EPUB publication.
    pub rel: Properties,

    /// The OPTIONAL [hreflang] attribute identifies the language of the linked resource.
    pub hreflang: Option<String>,

    /// The media-type attribute is OPTIONAL when a linked resource is located outside the EPUB container,
    ///
    /// as more than one media type could be served from the same URL.
    /// EPUB creators MUST specify the attribute for all linked resources within the EPUB container.
    pub media_type: Option<String>,

    /// The properties attribute is a space-separated list of property values.
    pub property: Option<Property>,

    /// The refines attribute associates the link element with the element or resource it is refining.
    pub refines: Option<Refines>,

    /// The value of the link element.
    pub value: String
}


/// The metadata section of an EPUB Publication.
#[derive(Debug, Clone)]
pub struct Metadata {

    /// All metadata elements
    ///
    /// The metadata elements are used to provide information about the publication.
    ///
    /// It MUST contain Dublin Core Metadata Element Set
    pub elems: HashMap<Property, Vec<MetadataElement>>,

    /// All meta elements
    pub metas: Vec<Meta>,

    /// All link elements
    pub links: Vec<Link>,

    /// The date and time the metadata was last modified.
    ///
    /// The metadata section MUST contain exactly one dcterms:modified property containing the last modification date.
    /// The value of this property MUST be an xmlschema-2 dateTime conformant date of the form: CCYY-MM-DDThh:mm:ssZ
    pub last_modified: DateTime<Utc>,
}

impl Metadata {
    /// Create a new Metadata
    pub fn new(
        elems: Vec<MetadataElement>,
        metas: Vec<Meta>,
        links: Vec<Link>,
    ) -> Self {
        let elems = {
            let mut elems_map: HashMap<Property, Vec<MetadataElement>> = HashMap::new();

            // group metadata elements by property
            for elem in elems {
                let property = elem.property.clone();
                elems_map.entry(property)
                    .or_insert_with(Vec::new)
                    .push(elem);
            }
            elems_map
        };

        // check dublin core metadata element set
        {
            fn error_msg(property: &str) -> String {
                format!("The metadata section MUST contain exactly at least one {} element.", property)
            }

            fn check_property(elems_map: &HashMap<Property, Vec<MetadataElement>>, property: &str) {
                let elems = elems_map.get(property.to());
                assert!(elems.is_some() && elems.unwrap().len() >= 1, "{}", error_msg(property));
            }

            check_property(&elems, "dc:title");
            check_property(&elems, "dc:language");
            check_property(&elems, "dc:identifier");
        }

        // check lastModified
        let last_modified = {
            let last_modified = metas.iter()
                .find(|meta| meta.property.eq("dcterms:modified"))
                .expect("The metadata section MUST contain exactly one dcterms:modified property containing the last modification date.");

            DateTime::parse_from_rfc3339(&last_modified.value)
                .expect("The value of this property MUST be an xmlschema-2 dateTime conformant date of the form: CCYY-MM-DDThh:mm:ssZ")
                .to_utc()
        };

        Metadata {
            elems,
            metas,
            links,
            last_modified,
        }
    }

    /// All dc:title elements
    fn titles(&self) -> Vec<&MetadataElement> {
        self.elems.get("dc:title".to()).unwrap()
    }

    /// All dc:language elements
    fn languages(&self) -> Vec<&MetadataElement> {
        self.elems.get("dc:language".to()).unwrap()
    }

    /// All dc:identifier elements
    fn identifiers(&self) -> Vec<&MetadataElement> {
        self.elems.get("dc:identifier".to()).unwrap()
    }
}