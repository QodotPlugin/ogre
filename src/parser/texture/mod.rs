mod font;
mod mip;

pub use font::*;
pub use mip::*;

use nom::{number::complete::le_u32, sequence::tuple, IResult};

use crate::{
    repr::texture::{QPic, Size, Texture},
    parser::repr::EntryType,
};

use super::{parse_byte_string, parse_image, parse_palette};

/// Parse a string slice of up to 16 null-terminated characters from a byte slice.
pub fn parse_texture_name(i: &[u8]) -> IResult<&[u8], &str> {
    parse_byte_string(16)(i)
}

/// Parse a [`Size`] from a byte slice.
pub fn parse_texture_size(i: &[u8]) -> IResult<&[u8], Size> {
    let (i, (width, height)) = tuple((le_u32, le_u32))(i)?;
    Ok((i, Size { width, height }))
}

/// Given an [`EntryType`], returns a function that parses a [`Texture`] from a byte slice.
pub fn parse_texture(entry_type: EntryType) -> impl Fn(&[u8]) -> IResult<&[u8], Texture> {
    move |i: &[u8]| match entry_type {
        EntryType::Unknown => parse_texture_unknown(i),
        EntryType::StatusBar => parse_texture_status_bar(i),
        EntryType::MipTextureRgb => parse_texture_mip_rgb(i),
        EntryType::MipTextureIndexed => parse_texture_mip_indexed(i),
        EntryType::ConsolePicture => parse_texture_console_picture(i),
        EntryType::Font => parse_texture_font(i),
    }
}

/// Parse a [`Texture::StatusBar`] from a byte slice.
pub fn parse_texture_status_bar(i: &[u8]) -> IResult<&[u8], Texture> {
    let (i, size) = parse_texture_size(i)?;
    let (i, image) = parse_image(size.len_image() as usize)(i)?;
    Ok((i, Texture::StatusBar(QPic { size, image })))
}

/// Parse a [`Texture::ConsolePicture`] from a byte slice.
pub fn parse_texture_console_picture(i: &[u8]) -> IResult<&[u8], Texture> {
    let (i, size) = parse_texture_size(&i[16..])?;
    let (i, image) = parse_image(size.len_image() as usize)(i)?;
    Ok((i, Texture::ConsolePicture(QPic { size, image })))
}
