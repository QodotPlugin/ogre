//! Parallelized WAD parsing implementation using [`async_std`].
//!
//! [`parse_wad`] is the primary user-facing entrypoint, and will parse a complete [`Wad`].
//!
//! For more granular async work, [`parse_textures`] returns a [`TryStream`] of `(String, Texture)` tuples that can be used to chain further computations,
//! such as dumping to a file or conversion into some other format.
//!
//! These functions operate by spawning one [`async_std::task`] per texture, and thus require a way for each task to get its own thread-safe handle to the WAD data.
//!
//! These handles are represented by the [`WadSource`] trait, which is implemented for any type that can read and seek through a source of bytes via [`ReadExt`] and [`SeekExt`].
//!
//! The [`WadSourceStream`] trait represents a stream that can generate an infinite amount of these handles given some source data,
//! concrete implementations of which are provided via [`SourceStreamSlice`], [`SourceStreamVec`], and [`SourceStreamFile`].

#[cfg(doc)]
use async_std::io::{ReadExt, prelude::SeekExt};

pub mod parse;
pub mod wad_source;

mod arc_vec;

use parse::*;
use wad_source::*;

use futures::{StreamExt, TryFutureExt, TryStream};

use crate::repr::{texture::Texture, Wad};

pub type IoError = async_std::io::Error;

/// [`TryStream`] of `(String, Texture)` pairs.
pub trait TextureStream: TryStream<Ok = (String, Texture), Error = IoError> {}
impl<T> TextureStream for T where T: TryStream<Ok = (String, Texture), Error = IoError> {}

/// Async parse a [`Wad`] from a [`WadSourceStream`].
pub async fn parse_wad<S, W>(handle_stream: S) -> Result<Wad, IoError>
where
    S: 'static + WadSourceStream<W>,
    W: 'static + WadSource + Send + Unpin,
{
    parse_textures::<S, W>(handle_stream)
        .and_then(Wad::from_stream)
        .await
}

/// Async parse a [`TextureStream`] from a [`WadSourceStream`].
pub async fn parse_textures<S, W>(mut s: S) -> Result<TexStream, IoError>
where
    S: 'static + WadSourceStream<W>,
    W: 'static + WadSource + Send + Unpin,
{
    let handle = s.next().await.unwrap()?;
    let header = parse_header(handle).await?;
    let stream = parse_header_textures(s, header).await?;
    Ok(stream)
}
