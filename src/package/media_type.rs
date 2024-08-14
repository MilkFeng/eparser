use std::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

/// MIME media type of a resource
///
/// Resources fall into three categories based on their reading system support:
/// "core media type resources", "foreign resources", and "exempt resources".
///
/// # Core Media Type Resources
/// A core media type resource is one that reading systems have to support,
/// so it can be used without restriction in EPUB or foreign content documents.
///
/// For example, `GIF` and `JPG` are core media types.
///
/// # References
/// [EPUB 3.3 SPEC](https://www.w3.org/TR/epub-33/#sec-core-media-types)
#[derive(Debug, PartialEq, Clone)]
pub struct MediaType(String);

impl Deref for MediaType {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl DerefMut for MediaType {
    fn deref_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

impl FromStr for MediaType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MediaType(s.to_string()))
    }
}

impl Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl MediaType {

    /// Create a new media type
    pub fn new(media_type: &str) -> Self {
        MediaType(media_type.to_string())
    }

    /// Check if the media type is a core media type
    pub fn is_core_media_type(&self) -> bool {
        media_types::ALL_CORE_MEDIA_TYPES.iter()
            .any(|&core_media_type| core_media_type.eq(self))
    }
}

/// Core media types
pub mod media_types {
    use crate::package::media_type::MediaType;
    use once_cell::sync::Lazy;

    // Core media types
    // images
    pub static GIF: Lazy<MediaType> = Lazy::new(|| MediaType("image/gif".to_string()));
    pub static JPG: Lazy<MediaType> = Lazy::new(|| MediaType("image/jpeg".to_string()));
    pub static PNG: Lazy<MediaType> = Lazy::new(|| MediaType("image/png".to_string()));
    pub static SVG: Lazy<MediaType> = Lazy::new(|| MediaType("image/svg+xml".to_string()));
    pub static WEBP: Lazy<MediaType> = Lazy::new(|| MediaType("image/webp".to_string()));

    // audio
    pub static MP3: Lazy<MediaType> = Lazy::new(|| MediaType("audio/mpeg".to_string()));
    pub static MP4: Lazy<MediaType> = Lazy::new(|| MediaType("video/mp4".to_string()));
    pub static OGG: Lazy<MediaType> = Lazy::new(|| MediaType("audio/ogg; codecs=opus".to_string()));

    // style
    pub static CSS: Lazy<MediaType> = Lazy::new(|| MediaType("text/css".to_string()));

    // fonts
    pub static TTF: Lazy<MediaType> = Lazy::new(|| MediaType("font/ttf".to_string()));
    pub static OTF: Lazy<MediaType> = Lazy::new(|| MediaType("font/otf".to_string()));
    pub static WOFF: Lazy<MediaType> = Lazy::new(|| MediaType("font/woff".to_string()));
    pub static WOFF2: Lazy<MediaType> = Lazy::new(|| MediaType("font/woff2".to_string()));
    pub static SFNT: Lazy<MediaType> = Lazy::new(|| MediaType("application/font-sfnt".to_string()));
    pub static VND_MS: Lazy<MediaType> = Lazy::new(|| MediaType("application/vnd.ms-opentype".to_string()));
    pub static APP_WOFF: Lazy<MediaType> = Lazy::new(|| MediaType("application/font-woff".to_string()));

    // other
    pub static XHTML: Lazy<MediaType> = Lazy::new(|| MediaType("application/xhtml+xml".to_string()));
    pub static TEXT_JAVASCRIPT: Lazy<MediaType> = Lazy::new(|| MediaType("text/javascript".to_string()));
    pub static APP_JAVASCRIPT: Lazy<MediaType> = Lazy::new(|| MediaType("application/javascript".to_string()));
    pub static ECMASCRIPT: Lazy<MediaType> = Lazy::new(|| MediaType("application/ecmascript".to_string()));
    pub static NCX: Lazy<MediaType> = Lazy::new(|| MediaType("application/x-dtbncx+xml".to_string()));
    pub static SMIL: Lazy<MediaType> = Lazy::new(|| MediaType("application/smil+xml".to_string()));

    // all media types
    pub static ALL_CORE_MEDIA_TYPES: [&Lazy<MediaType>; 22] = [
        &GIF, &JPG, &PNG, &SVG, &WEBP,
        &MP3, &MP4, &OGG,
        &CSS,
        &TTF, &OTF, &WOFF, &WOFF2, &SFNT, &VND_MS, &APP_WOFF,
        &XHTML, &TEXT_JAVASCRIPT, &APP_JAVASCRIPT, &ECMASCRIPT, &NCX, &SMIL
    ];

    // epub media type
    pub static EPUB: Lazy<MediaType> = Lazy::new(|| MediaType("application/epub+zip".to_string()));

    // oebps media type
    pub static OEBPS: Lazy<MediaType> = Lazy::new(|| MediaType("application/oebps-package+xml".to_string()));
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_core_media_types() {
        media_types::ALL_CORE_MEDIA_TYPES.iter().for_each(|&media_type| {
            assert!(media_type.is_core_media_type());
        });
    }
}