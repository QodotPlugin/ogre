use crate::repr::{Image, Palette, texture::QPic};

/// A set of progressively-smaller mipmap [`Image`]s.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Mips {
    pub mip1: Image,
    pub mip2: Image,
    pub mip3: Image,
}

/// A [`QPic`] combined with a set of [`Mips`]. Standard WAD2 texture format.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextureMipIndexed {
    pub qpic: QPic,
    pub mips: Mips,
}

/// A [`TextureMipIndexed`] combined with a [`Palette`]. Standard WAD3 texture format.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextureMipRgb {
    pub texture: TextureMipIndexed,
    pub palette: Palette,
}
