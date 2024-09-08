use minidom::Element;
use thiserror::Error;
use url::Url;

/// The type of the nav.
pub enum NavType {
    TOC,
    Landmarks,
    PageList,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct Nav {
    /// The `epub:type` attribute of the nav.
    pub ty: String,

    /// The title of the nav.
    pub title: Option<NavTitle>,

    /// All the children nav points.
    pub children: Vec<NavPoint>,
}

#[derive(Debug, Clone)]
pub struct NavTitle {
    /// The text content of the nav title.
    pub text: String,

    /// The level of the title. The value is between 1 and 6 referring to h1 to h6.
    pub level: usize,
}

#[derive(Debug, Clone)]
pub struct NavLabel {
    /// The text content of the nav label.
    pub text: String,

    /// The href attribute of the nav label.
    pub href: Option<Url>,
}

#[derive(Debug, Clone)]
pub struct NavPoint {
    /// The title content of the nav point.
    pub label: NavLabel,

    /// The order of the nav point in the nav.
    pub order: usize,

    /// All the children nav points.
    pub children: Vec<NavPoint>,
}

const XHTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

#[derive(Debug, Error)]
pub enum NavParseError {
    #[error("Invalid XML, {0}")]
    ParseError(#[from] minidom::Error),

    #[error("Invalid root element, expected nav but found {0}")]
    InvalidRoot(String),
}

/// Parse the nav document.
///
/// The structure of the nav document is as follows:
///
/// ```xml
/// <nav>
///     <h1>Table of Contents</h1>
///     <ol>
///         <li><a href="cover.xhtml">Cover</a></li>
///         <li><a href="chapter1.xhtml">Chapter 1</a></li>
///         <li><a href="chapter2.xhtml">Chapter 2</a></li>
///         <li><a href="chapter3.xhtml">Chapter 3</a></li>
///         <li><a href="chapter4.xhtml">Chapter 4</a></li>
///         <li><a href="chapter5.xhtml">Chapter 5</a></li>
///         <li><a href="chapter6.xhtml">Chapter 6</a></li>
///         <li><a href="chapter7.xhtml">Chapter 7</a></li>
///     </ol>
/// </nav>
/// ```
pub fn parse_nav(str: &str) -> Result<Nav, NavParseError> {
    let root_elem = str.parse::<Element>()?;

    if root_elem.name() != "nav" {
        return Err(NavParseError::InvalidRoot(root_elem.name().to_string()));
    }

    let ty = root_elem.attr("epub:type").map(|s| s.to_string());

    unimplemented!()
}
