use std::{convert::TryFrom, fmt::Debug};

/// The set of texture variants that can appear in a WAD file.
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum EntryType {
    Unknown = 0x40,
    StatusBar = 0x42,
    MipTextureRgb = 0x43,
    MipTextureIndexed = 0x44,
    ConsolePicture = 0x45,
    Font = 0x46,
}

impl TryFrom<u8> for EntryType {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x40 => Ok(EntryType::Unknown),
            0x42 => Ok(EntryType::StatusBar),
            0x43 => Ok(EntryType::MipTextureRgb),
            0x44 => Ok(EntryType::MipTextureIndexed),
            0x45 => Ok(EntryType::ConsolePicture),
            0x46 => Ok(EntryType::Font),
            i => Err(format!("Invalid entry type {}", i)),
        }
    }
}
