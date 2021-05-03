use nom::{multi::count, IResult};

use crate::parser::repr::Directory;
use crate::parser::parse_entry;

/// Given an entry count, returns a function that parses a [`Directory`] from a byte slice.
pub fn parse_directory(entry_count: usize) -> impl Fn(&[u8]) -> IResult<&[u8], Directory> {
    move |i: &[u8]| {
        let (i, entries) = count(parse_entry, entry_count)(i)?;
        Ok((i, Directory(entries)))
    }
}
