mod font;
mod mip;
mod qpic;
mod size;

pub use font::*;
pub use mip::*;
pub use qpic::*;
pub use size::*;

use crate::repr::{Image, Palette};

/// The set of texture types that can appear in a WAD file.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Texture {
    Unknown(TextureMipIndexed), // Can allegedly turn up in tempdecal.wad
    StatusBar(QPic),
    MipRgb(TextureMipRgb),
    MipIndexed(TextureMipIndexed),
    ConsolePicture(QPic),
    Font(TextureFont),
}

impl Texture {
    pub fn size(&self) -> Size {
        match self {
            Texture::Unknown(texture) => texture.qpic.size,
            Texture::StatusBar(texture) => texture.size,
            Texture::MipRgb(texture) => texture.texture.qpic.size,
            Texture::MipIndexed(texture) => texture.qpic.size,
            Texture::ConsolePicture(texture) => texture.size,
            Texture::Font(texture) => texture.size,
        }
    }

    pub fn image(&self) -> &Image {
        match self {
            Texture::Unknown(texture) => &texture.qpic.image,
            Texture::StatusBar(texture) => &texture.image,
            Texture::MipRgb(texture) => &texture.texture.qpic.image,
            Texture::MipIndexed(texture) => &texture.qpic.image,
            Texture::ConsolePicture(texture) => &texture.image,
            Texture::Font(texture) => &texture.image,
        }
    }

    pub fn palette(&self) -> Option<&Palette> {
        match self {
            Texture::Unknown(_) => None,
            Texture::StatusBar(_) => None,
            Texture::MipRgb(texture) => Some(&texture.palette),
            Texture::MipIndexed(_) => None,
            Texture::ConsolePicture(_) => None,
            Texture::Font(_) => None,
        }
    }

    pub fn mips(&self) -> Option<&Mips> {
        match self {
            Texture::Unknown(texture) => Some(&texture.mips),
            Texture::StatusBar(_) => None,
            Texture::MipRgb(texture) => Some(&texture.texture.mips),
            Texture::MipIndexed(texture) => Some(&texture.mips),
            Texture::ConsolePicture(_) => None,
            Texture::Font(_) => None,
        }
    }

    pub fn font_row_count(&self) -> Option<u32> {
        if let Texture::Font(texture) = self {
            Some(texture.row_data.count)
        } else {
            None
        }
    }

    pub fn font_row_height(&self) -> Option<u32> {
        if let Texture::Font(texture) = self {
            Some(texture.row_data.height)
        } else {
            None
        }
    }

    pub fn font_char_data(&self) -> Option<&[CharData; 256]> {
        if let Texture::Font(texture) = self {
            Some(&texture.char_data)
        } else {
            None
        }
    }
}
