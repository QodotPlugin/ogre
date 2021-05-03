use nom::{number::complete::le_u8, IResult};

use crate::{repr::ColorRgb8, impl_try_from};

/// Parse a [`ColorRgb8`] from a byte slice.
pub fn parse_color_rgb8(i: &[u8]) -> IResult<&[u8], ColorRgb8> {
    let (i, (r, g, b)) = nom::sequence::tuple((le_u8, le_u8, le_u8))(i)?;
    Ok((i, ColorRgb8(r, g, b)))
}

impl_try_from!(ColorRgb8, parse_color_rgb8);

#[cfg(test)]
mod tests {
    use crate::test_data::{test_color_rgb8_in, test_color_rgb8_out};

    use super::*;

    #[test]
    fn test_color_rgb8() {
        let (_, o) = parse_color_rgb8(test_color_rgb8_in()).unwrap();
        assert_eq!(o, test_color_rgb8_out());
    }
}
