use nom::{number::complete::le_u32, IResult};

use crate::impl_try_from;
use crate::parser::{
    repr::{Header, WadMagic},
    parse_byte_string,
};

/// The size of a [`Header`] in bytes.
pub const HEADER_SIZE: usize = 12;

/// Parse a [`WadMagic`] from a byte slice.
pub fn parse_wad_magic(i: &[u8]) -> IResult<&[u8], WadMagic> {
    let (i, magic) = parse_byte_string(4)(i)?;
    let magic = match magic {
        "WAD2" => WadMagic::Wad2,
        "WAD3" => WadMagic::Wad3,
        _ => panic!("Invalid WAD magic"),
    };
    Ok((i, magic))
}

/// Parse a [`Header`] from a byte slice.
pub fn parse_header(i: &[u8]) -> IResult<&[u8], Header> {
    let (i, o) = nom::sequence::tuple((parse_wad_magic, le_u32, le_u32))(i)?;

    let (magic, num_entries, dir_offset) = o;

    Ok((
        i,
        Header {
            magic,
            num_entries,
            dir_offset,
        },
    ))
}

impl_try_from!(Header, parse_header);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::{test_header_in, test_header_out};

    #[test]
    fn test_header() {
        let i = test_header_in();
        let (_, o) = parse_header(&i).unwrap();
        assert_eq!(o, test_header_out());
    }
}
