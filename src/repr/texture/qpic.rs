use crate::repr::{texture::Size, Image};

/// An [`Image`] paired with a [`Size`]. Analogous to `qpic_t` in the Quake codebase.
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct QPic {
    pub size: Size,
    pub image: Image,
}
