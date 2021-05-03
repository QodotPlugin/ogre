use nom::{number::complete::le_u8, IResult};

use crate::repr::ColorIndexed;
use crate::impl_try_from;

/// Parse a [`ColorIndexed`] from a byte slice.
pub fn parse_color_indexed(i: &[u8]) -> IResult<&[u8], ColorIndexed> {
    let (i, o) = le_u8(i)?;
    Ok((i, ColorIndexed(o)))
}

impl_try_from!(ColorIndexed, parse_color_indexed);

#[cfg(test)]
mod tests {
    use crate::test_data::{test_color_indexed_in, test_color_indexed_out};

    use super::*;
    use std::{convert::TryInto, error::Error};

    #[test]
    fn test_color_indexed() -> Result<(), Box<dyn Error>> {
        let color_indexed: ColorIndexed = test_color_indexed_in().try_into()?;
        assert_eq!(color_indexed, test_color_indexed_out());
        Ok(())
    }
}
