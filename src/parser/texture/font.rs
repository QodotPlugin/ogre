use nom::IResult;

use crate::{
    repr::texture::{RowData, Texture, TextureFont},
    parser::parse_image,
};

use nom::{
    multi::fill,
    number::complete::{le_u16, le_u32},
    sequence::tuple,
};

use crate::repr::texture::CharData;

use super::parse_texture_size;

/// Parse a [`RowData`] from a byte slice.
pub fn parse_row_data(i: &[u8]) -> IResult<&[u8], RowData> {
    let (i, (count, height)) = tuple((le_u32, le_u32))(i)?;
    Ok((i, RowData { count, height }))
}

/// Parse a [`CharData`] from a byte slice.
pub fn parse_char_data_single(i: &[u8]) -> IResult<&[u8], CharData> {
    let (i, (offset, width)) = tuple((le_u16, le_u16))(i)?;
    Ok((i, CharData { offset, width }))
}

/// Parse an array of 256 [`CharData`] structs from a byte slice.
pub fn parse_char_data_array(i: &[u8]) -> IResult<&[u8], [CharData; 256]> {
    let mut char_data: [CharData; 256] = [Default::default(); 256];
    let (i, _) = fill(parse_char_data_single, &mut char_data)(i)?;
    Ok((i, char_data))
}

/// Parse a [`Texture::Font`] from a byte slice.
pub fn parse_texture_font(i: &[u8]) -> IResult<&[u8], Texture> {
    let (i, size) = parse_texture_size(i)?;
    let (i, (row_data, char_data)) = tuple((parse_row_data, parse_char_data_array))(i)?;
    let (i, image) = parse_image(size.len_image() as usize)(i)?;
    Ok((
        i,
        Texture::Font(TextureFont {
            size,
            row_data,
            char_data,
            image,
        }),
    ))
}
