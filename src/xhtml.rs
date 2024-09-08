use minidom::Element;
use thiserror::Error;

#[derive(Debug)]
pub struct XHTML {
    element: Element,
}

static XHTML_NAMESPACE: &str = "http://www.w3.org/1999/xhtml";

impl XHTML {
    pub fn new(element: Element) -> Result<Self, XHTMLParseError> {
        // check that the root element is an XHTML element
        if element.name() != "html" {
            return Err(XHTMLParseError::InvalidRootElement);
        }

        // check header and body elements
        if element.get_child("head", XHTML_NAMESPACE).is_none() {
            return Err(XHTMLParseError::MissingHead);
        }

        if element.get_child("body", XHTML_NAMESPACE).is_none() {
            return Err(XHTMLParseError::MissingBody);
        }

        Ok(Self { element })
    }

    fn elem_to_str(elem: &Element) -> String {
        let mut bytes = vec![];
        elem.write_to(&mut bytes).unwrap();
        String::from_utf8(bytes).unwrap()
    }

    pub fn head(&self) -> &Element {
        self.element.get_child("head", XHTML_NAMESPACE).unwrap()
    }

    pub fn head_str(&self) -> String {
        Self::elem_to_str(self.head())
    }

    pub fn body(&self) -> &Element {
        self.element.get_child("body", XHTML_NAMESPACE).unwrap()
    }

    pub fn body_str(&self) -> String {
        Self::elem_to_str(self.body())
    }

    pub fn root(&self) -> &Element {
        &self.element
    }

    pub fn root_str(&self) -> String {
        Self::elem_to_str(&self.element)
    }
}

#[derive(Debug, Error)]
pub enum XHTMLParseError {
    #[error("Failed to parse XHTML")]
    ParseError(#[from] minidom::Error),

    #[error("Missing head element")]
    MissingHead,

    #[error("Missing body element")]
    MissingBody,

    #[error("Invalid root element")]
    InvalidRootElement,
}


pub fn parse_xhtml(s: &str) -> Result<XHTML, XHTMLParseError> {
    let xhtml = s.parse::<Element>()?;
    XHTML::new(xhtml)
}