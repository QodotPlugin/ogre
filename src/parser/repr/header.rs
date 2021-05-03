/// The set of supported WAD formats.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum WadMagic {
    Wad2,
    Wad3
}

/// Header describing the format, entry count and dictionary location inside a WAD file.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Header {
    pub magic: WadMagic,
    pub num_entries: u32,
    pub dir_offset: u32,
}

