mod entry_type;
pub use entry_type::*;

#[cfg(doc)]
use crate::parser::repr::Directory;

#[cfg(doc)]
use crate::repr::texture::Texture;

/// An entry in a WAD [`Directory`] containing metadata for a single [`Texture`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry<'a> {
    pub offset: u32,
    pub size_directory: u32,
    pub size_memory: u32,
    pub entry_type: EntryType,
    pub compression: u8,
    pub name: &'a str,
}
