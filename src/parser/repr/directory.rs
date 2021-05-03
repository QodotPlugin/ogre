use std::ops::{Deref, DerefMut};

use crate::parser::repr::Entry;

/// The set of [`Entry`]s describing the contents of a WAD file.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Directory<'a>(#[serde(borrow)] pub Vec<Entry<'a>>);

impl<'a> Deref for Directory<'a> {
    type Target = Vec<Entry<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Directory<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
