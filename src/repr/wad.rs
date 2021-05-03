use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

use super::texture::Texture;

/// A set of `String` / `Texture` pairs representing the parsed contents of a WAD file.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Wad(BTreeMap<String, Texture>);

impl Wad {
    pub fn new(textures: BTreeMap<String, Texture>) -> Self {
        Wad(textures)
    }
}

impl Deref for Wad {
    type Target = BTreeMap<String, Texture>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Wad {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
