/// 32-bit width and height.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Size {
    pub width: u32,
    pub height: u32,
}

impl Size {
    pub fn len_image(&self) -> u32 {
        self.width * self.height
    }

    pub fn len_mip1(&self) -> u32 {
        self.width / 2 * self.height / 2
    }

    pub fn len_mip2(&self) -> u32 {
        self.width / 4 * self.height / 4
    }

    pub fn len_mip3(&self) -> u32 {
        self.width / 8 * self.height / 8
    }
}
