mod entry_type;
pub use entry_type::*;

use nom::{
    number::complete::{le_u32, le_u8},
    IResult,
};

use crate::{
    impl_try_from,
    parser::{repr::Entry, parse_byte_string},
};

/// The size of an [`Entry`] in bytes.
pub const ENTRY_SIZE: usize = 32;

/// Parse an `Entry` from a byte slice.
pub fn parse_entry(i: &[u8]) -> IResult<&[u8], Entry> {
    let (i, o) = nom::sequence::tuple((le_u32, le_u32, le_u32, parse_entry_type, le_u8))(i)?;

    let (offset, size_directory, size_memory, entry_type, compression) = o;

    let (i, (name,)) = nom::sequence::tuple((parse_byte_string(16),))(&i[2..])?;

    Ok((
        i,
        Entry {
            offset,
            size_directory,
            size_memory,
            entry_type,
            compression,
            name,
        },
    ))
}

impl_try_from!(Entry<'a>, parse_entry);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::{test_entry_in, test_entry_out};

    #[test]
    fn test_entry() {
        let i = test_entry_in();
        let (_, o) = parse_entry(&i).unwrap();
        assert_eq!(o, test_entry_out());
    }
}
