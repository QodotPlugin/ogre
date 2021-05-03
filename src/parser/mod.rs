//! `nom` functions for parsing the WAD format.

pub mod repr;

mod color;
mod directory;
mod entry;
mod header;
mod image;
mod palette;
mod texture;
mod wad;

pub use color::*;
pub use directory::*;
pub use entry::*;
pub use header::*;
pub use image::*;
pub use palette::*;
pub use texture::*;
pub use wad::*;

use nom::IResult;
pub use palette::*;
pub use wad::*;

/// Given a length, returns a function that parses a null-terminated string slice from a byte slice.
pub fn parse_byte_string(length: usize) -> impl Fn(&[u8]) -> IResult<&[u8], &str> {
    move |i: &[u8]| {
        let string_len = i
            .iter()
            .take(length)
            .position(|i| *i == b'\0')
            .unwrap_or(length);

        let byte_string = std::str::from_utf8(&i[..string_len])
            .expect("Failed to convert name into a string slice");

        Ok((&i[length..], byte_string))
    }
}

/// Implements [`std::convert::TryFrom`] for type `$ty` using parser function `$parser`.
#[macro_export]
macro_rules! impl_try_from {
    ($ty:ty, $parser:tt) => {
        impl<'a> std::convert::TryFrom<&'a [u8]> for $ty {
            type Error = String;

            fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
                match $parser(value) {
                    Ok((_, o)) => Ok(o),
                    Err(e) => Err(format!("{}", e)),
                }
            }
        }
    };
}
