use url::Url;

/// The type of the nav.
enum NavType {
    TOC,
    Landmarks,
    PageList,
    Custom(String),
}

struct Nav {
    /// The `epub:type` attribute of the nav.
    pub ty: String,

    /// The title of the nav.
    pub title: Option<NavTitle>,

    /// All the children nav points.
    pub children: Vec<NavPoint>,
}

struct NavTitle {
    /// The text content of the nav title.
    pub text: String,

    /// The level of the title. The value is between 1 and 6 referring to h1 to h6.
    pub level: usize,
}


struct NavLabel {
    /// The text content of the nav label.
    pub text: String,

    /// The href attribute of the nav label.
    pub href: Option<Url>,
}


struct NavPoint {
    /// The title content of the nav point.
    pub label: NavLabel,

    /// The order of the nav point in the nav.
    pub order: usize,

    /// All the children nav points.
    pub children: Vec<NavPoint>,
}