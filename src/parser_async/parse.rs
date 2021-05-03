//! Internal parsing functions.

use async_std::{io::SeekFrom, task::JoinHandle};
use futures::{stream::FuturesUnordered, StreamExt};

use crate::repr::texture::Texture;
use crate::parser::{repr::Header, ENTRY_SIZE, HEADER_SIZE};

use super::{IoError, WadSource, WadSourceStream};

pub type TexStream = FuturesUnordered<JoinHandle<Result<(String, Texture), std::io::Error>>>;

#[cfg(doc)]
use super::TextureStream;

/// Given a [`WadSourceStream`] and a [`Header`], spawn one [`async_std::task`] per texture and return a [`TextureStream`] of the results.
pub async fn parse_header_textures<S, W>(
    mut s: S,
    header: Header,
) -> Result<FuturesUnordered<JoinHandle<Result<(String, Texture), std::io::Error>>>, std::io::Error>
where
    S: 'static + WadSourceStream<W>,
    W: 'static + WadSource + Send + Unpin,
{
    let num_entries = header.num_entries as usize;
    let dir_offset = header.dir_offset as usize;

    let futures = FuturesUnordered::default();
    for i in 0..num_entries {
        let handle = s.next().await.unwrap()?;

        let entry_offset = i * ENTRY_SIZE;
        let ofs = (dir_offset + entry_offset) as u64;

        futures.push(async_std::task::spawn(parse_texture(handle, ofs)));
    }
    Ok(futures)
}

/// Read and parse a [`Header`] from a [`WadSource`] handle.
pub async fn parse_header(mut wad_bytes: impl WadSource + Unpin) -> Result<Header, IoError> {
    let mut header_buf = [0u8; HEADER_SIZE];
    wad_bytes.read_exact(&mut header_buf).await?;

    let header = crate::parser::parse_header(&header_buf)
        .expect("Invalid WAD header")
        .1;

    Ok(header)
}

/// Read and parse a [`Texture`] from a [`WadSource`] handle.
pub async fn parse_texture<W>(
    mut wad_bytes: W,
    entry_offset: u64,
) -> Result<(String, Texture), IoError>
where
    W: WadSource + Unpin,
{
    let mut entry_buf = [0u8; ENTRY_SIZE];
    wad_bytes.seek(SeekFrom::Start(entry_offset)).await?;
    wad_bytes.read_exact(&mut entry_buf).await?;
    let entry = crate::parser::parse_entry(&entry_buf)
        .expect("Invalid entry")
        .1;

    let mut texture_buf = std::iter::repeat(0u8)
        .take(entry.size_directory as usize)
        .collect::<Vec<_>>();
    wad_bytes.seek(SeekFrom::Start(entry.offset as u64)).await?;
    wad_bytes.read_exact(&mut texture_buf).await?;

    let name = entry.name.to_string();
    let texture = crate::parser::parse_texture(entry.entry_type)(&texture_buf)
        .unwrap_or_else(|e| panic!("Failed to parse texture: {}", e))
        .1;

    Ok((name, texture))
}
