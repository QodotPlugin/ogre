use nom::{multi::count, IResult};

use crate::{repr::Image, parser::parse_color_indexed};

/// Given a texel count, returns a function that parses an [`Image`] from a byte slice.
pub fn parse_image<'a>(texture_size: usize) -> impl Fn(&[u8]) -> IResult<&[u8], Image> + 'a {
    move |i: &[u8]| {
        let (i, o) = count(parse_color_indexed, texture_size)(i)?;
        Ok((i, Image::new(o)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data::{test_image_in, test_image_out};

    #[test]
    fn test_image() {
        let (_, o) = parse_image(4 * 4)(test_image_in()).unwrap();
        assert_eq!(o, test_image_out());
    }
}
