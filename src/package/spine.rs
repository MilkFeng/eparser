use std::ops::{Deref, DerefMut};

/// The itemref element associates an item with a spine.
#[derive(Debug, PartialEq, Clone)]
pub struct SpineReference {
    /// Reference to the resource in the manifest by its ID.
    pub id: String,

    /// The linear property of the spine reference.
    pub linear: Option<bool>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Spine {
    /// The unique identifier of the spine element.
    pub id: Option<String>,

    /// The direction of the primary text progression in the spine.
    pub dir: Option<String>,

    /// The list of spine references.
    items: Vec<SpineReference>,
}


impl Deref for Spine {
    type Target = Vec<SpineReference>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}


impl DerefMut for Spine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}