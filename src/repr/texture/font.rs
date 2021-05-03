use crate::repr::{texture::Size, Image};

use serde_big_array::BigArray;

/// Row data for `Font`-type textures.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RowData {
    pub count: u32,
    pub height: u32,
}

/// Character data for `Font`-type textures.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CharData {
    pub offset: u16,
    pub width: u16,
}

/// A WAD font.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextureFont {
    pub size: Size,
    pub row_data: RowData,
    #[serde(with = "BigArray")]
    pub char_data: [CharData; 256],
    pub image: Image,
}
