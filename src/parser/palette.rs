use nom::IResult;

use crate::{repr::{ColorRgb8, Palette}, impl_try_from};
use crate::parser::parse_color_rgb8;

/// Parse a [`Palette`] from a byte slice.
pub fn parse_palette(i: &[u8]) -> IResult<&[u8], Palette> {
    let (i, o) = nom::multi::count(parse_color_rgb8, 256)(i)?;
    let mut color_arr = [ColorRgb8(0, 0, 0); 256];
    color_arr.copy_from_slice(&o);
    Ok((i, Palette(color_arr)))
}

impl_try_from!(Palette, parse_palette);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::*;

    #[test]
    fn test_palette() {
        let (_, palette) = parse_palette(test_palette_in()).unwrap();
        assert_eq!(palette, test_palette_out());
    }
}
