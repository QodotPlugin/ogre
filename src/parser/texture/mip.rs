use nom::{IResult, sequence::tuple};

use crate::repr::texture::{Mips, QPic, Size, Texture, TextureMipIndexed, TextureMipRgb};

use super::{parse_image, parse_palette, parse_texture_size};

/// Parse a [`Mips`] from a byte slice.
pub fn parse_mips(size: Size) -> impl Fn(&[u8]) -> IResult<&[u8], Mips> {
    move |i: &[u8]| {
        let (i, (mip1, mip2, mip3)) = tuple((
            parse_image(size.len_mip1() as usize),
            parse_image(size.len_mip2() as usize),
            parse_image(size.len_mip3() as usize),
        ))(i)?;

        Ok((i, Mips { mip1, mip2, mip3 }))
    }
}

/// Parse a [`Texture::Unknown`] from a byte slice.
pub fn parse_texture_unknown(i: &[u8]) -> IResult<&[u8], Texture> {
    let (i, size) = parse_texture_size(&i[16..])?;
    let (i, image) = parse_image(size.len_image() as usize)(&i[16..])?;
    let (i, mips) = parse_mips(size)(i)?;
    Ok((
        i,
        Texture::Unknown(TextureMipIndexed {
            qpic: QPic { size, image },
            mips,
        }),
    ))
}

/// Parse a [`Texture::MipRgb`] from a byte slice.
pub fn parse_texture_mip_rgb(i: &[u8]) -> IResult<&[u8], Texture> {
    let (i, size) = parse_texture_size(&i[16..])?;
    let (i, image) = parse_image(size.len_image() as usize)(&i[16..])?;
    let (i, mips) = parse_mips(size)(i)?;
    let (i, palette) = parse_palette(&i[2..])?;
    Ok((
        i,
        Texture::MipRgb(TextureMipRgb {
            texture: TextureMipIndexed {
                qpic: QPic { size, image },
                mips,
            },
            palette,
        }),
    ))
}

/// Parse a [`Texture::MipIndexed`] from a byte slice.
pub fn parse_texture_mip_indexed(i: &[u8]) -> IResult<&[u8], Texture> {
    let (i, size) = parse_texture_size(&i[16..])?;
    let (i, image) = parse_image(size.len_image() as usize)(&i[16..])?;
    let (i, mips) = parse_mips(size)(i)?;
    Ok((
        i,
        Texture::MipIndexed(TextureMipIndexed {
            qpic: QPic { size, image },
            mips,
        }),
    ))
}
