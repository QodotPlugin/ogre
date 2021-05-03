use std::collections::BTreeMap;

use nom::IResult;

use crate::{
    repr::texture::Texture,
    parser::{parse_directory, parse_header, parse_texture},
};
use crate::{repr::Wad, impl_try_from};

/// Parse a [`Wad`] from a byte slice.
pub fn parse_wad(i: &[u8]) -> IResult<&[u8], Wad> {
    let header = parse_header(i)?.1;

    let directory =
        parse_directory(header.num_entries as usize)(&i[header.dir_offset as usize..])?.1;

    let mut textures: BTreeMap<String, Texture> = Default::default();

    for entry in directory.iter() {
        let (_, texture) = parse_texture(entry.entry_type)(&i[entry.offset as usize..])?;
        textures.insert(entry.name.to_string(), texture);
    }

    Ok((i, Wad::new(textures)))
}

impl_try_from!(Wad, parse_wad);
