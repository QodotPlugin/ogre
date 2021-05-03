use std::fmt::Debug;

use super::ColorRgb8;

use serde_big_array::BigArray;

#[cfg(doc)]
use super::ColorIndexed;

/// An 8-bit palette of [`ColorRgb8`] that can be mapped from a [`ColorIndexed`].
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Palette(#[serde(with = "BigArray")] pub [ColorRgb8; 256]);

impl Debug for Palette {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Palette").finish()
    }
}
