use std::path::Path;

struct RelativeUrl {
    path: Path,
}

enum Url {
    Absolute(url::Url),
    Relative(Box<RelativeUrl>),
}

