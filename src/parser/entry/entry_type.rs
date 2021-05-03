use std::convert::TryInto;

use nom::{number::complete::le_u8, IResult};

use crate::{impl_try_from, parser::repr::EntryType};

/// Parse an [`EntryType`] from a byte slice.
pub fn parse_entry_type(i: &[u8]) -> IResult<&[u8], EntryType> {
    let (i, o) = le_u8(i)?;
    let entry_type: EntryType = o.try_into().expect("Invalid entry type");
    Ok((i, entry_type))
}

impl_try_from!(EntryType, parse_entry_type);
