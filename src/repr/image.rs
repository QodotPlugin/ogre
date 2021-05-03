use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use crate::repr::{ColorIndexed, ColorRgb8, Palette};

/// A list of [`ColorIndexed`] texels.
#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Image(pub Vec<ColorIndexed>);

impl Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image")
            .field("length", &self.len())
            .finish()
    }
}

impl Image {
    pub fn new(colors: Vec<ColorIndexed>) -> Image {
        Image(colors)
    }

    pub fn texels_indexed(&self) -> impl Iterator<Item = &ColorIndexed> {
        self.iter()
    }

    pub fn texels_rgb<'a>(&'a self, palette: &'a Palette) -> impl Iterator<Item = &'a ColorRgb8> {
        self.texels_indexed()
            .map(move |color| &palette.0[color.0 as usize])
    }
}

impl Deref for Image {
    type Target = Vec<ColorIndexed>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Image {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
