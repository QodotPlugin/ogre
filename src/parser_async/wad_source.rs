//! Machinery for handling WAD data sources.

use std::{
    pin::Pin,
    task::{Context, Poll},
};

use async_std::{fs::File, io::{prelude::SeekExt, Cursor, ReadExt}, path::PathBuf};
use futures::{future::BoxFuture, FutureExt, Stream, TryStreamExt};

use crate::repr::Wad;

use super::{arc_vec::ArcVec, IoError, TextureStream};

/// A source of WAD bytes that can read and seek.
pub trait WadSource: ReadExt + SeekExt {}
impl<T> WadSource for T where T: ReadExt + SeekExt {}

/// Async-specific [`Wad`] extension methods.
impl Wad {
    /// Constructs a [`Wad`] from a stream of `(String, Texture)` tuples.
    pub async fn from_stream(textures: impl TextureStream) -> Result<Wad, IoError> {
        textures
            .try_fold(Wad::default(), |mut acc, (name, texture)| async move {
                acc.insert(name, texture);
                Ok(acc)
            })
            .await
    }
}

#[cfg(doc)]
use futures::TryStream;

/// [`TryStream`] of [`WadSource`] handles
pub trait WadSourceStream<W>:
    futures::TryStream<Item = Result<W, IoError>, Ok = W, Error = IoError> + Unpin
where
    W: WadSource,
{
}

impl<W, T> WadSourceStream<W> for T
where
    T: futures::TryStream<Item = Result<W, IoError>, Ok = W, Error = IoError> + Unpin,
    W: WadSource,
{
}

/// [`WadSourceStream`] for generating [`WadSource`] handles from `&'a [u8]`
///
/// For cases when WAD data is available as a slice, such as via [`std::include_bytes`].
pub struct SourceStreamSlice<'a>(&'a [u8]);

impl<'a> From<&'a [u8]> for SourceStreamSlice<'a> {
    fn from(slice: &'a [u8]) -> Self {
        SourceStreamSlice(slice)
    }
}

impl<'a> Stream for SourceStreamSlice<'a> {
    type Item = Result<Cursor<&'a [u8]>, IoError>;

    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(Some(Ok(Cursor::new(self.0))))
    }
}

/// [`WadSourceStream`] for generating [`WadSource`] handles from [`Vec<u8>`]
///
/// For cases when WAD data is available as a vector, such as via [`async_std::fs::read`]
pub struct SourceStreamVec(ArcVec<u8>);

impl From<Vec<u8>> for SourceStreamVec {
    fn from(vec: Vec<u8>) -> Self {
        SourceStreamVec(ArcVec::new(vec))
    }
}

impl Stream for SourceStreamVec {
    type Item = Result<Cursor<ArcVec<u8>>, IoError>;

    fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Poll::Ready(Some(Ok(Cursor::new(self.0.clone()))))
    }
}

/// [`WadSourceStream`] for generating [`WadSource`] handles from a file path.
///
/// This gives each texture parsing task its own [`File`] handle under the hood,
/// which is slower than parsing a complete slice or vector of WAD data, but lighter on memory.
pub struct SourceStreamFile {
    path: PathBuf,
    file_fut: BoxFuture<'static, Result<File, IoError>>,
}

impl From<PathBuf> for SourceStreamFile {
    fn from(path: PathBuf) -> Self {
        let file_fut = File::open(path.clone()).boxed();
        SourceStreamFile { path, file_fut }
    }
}

impl Stream for SourceStreamFile {
    type Item = Result<File, IoError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let self_mut = self.get_mut();
        match self_mut.file_fut.poll_unpin(cx) {
            Poll::Ready(result) => {
                self_mut.file_fut = File::open(self_mut.path.clone()).boxed();
                Poll::Ready(Some(result))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
