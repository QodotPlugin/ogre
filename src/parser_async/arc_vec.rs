use std::{convert::TryInto, ops::Deref};

use async_std::sync::Arc;

/// [`Arc<Vec<T>>`] wrapper.
///
/// Primarily intended to bridge [`Arc<Vec<u8>>`] with the `ReadExt` and `SeekExt` traits by way of an `AsRef<[T]>` implementation
#[derive(Debug, Default, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArcVec<T>(Arc<Vec<T>>);
impl<T> ArcVec<T> {
    pub fn new(v: Vec<T>) -> Self {
        ArcVec(Arc::new(v))
    }
}

impl<T> From<Vec<T>> for ArcVec<T> {
    fn from(v: Vec<T>) -> Self {
        ArcVec::new(v)
    }
}

impl<T> TryInto<Vec<T>> for ArcVec<T> {
    type Error = ArcVec<T>;

    fn try_into(self) -> Result<Vec<T>, Self::Error> {
        match Arc::try_unwrap(self.0) {
            Ok(v) => Ok(v),
            Err(e) => Err(ArcVec(e)),
        }
    }
}

impl<T> Deref for ArcVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<T> AsRef<[T]> for ArcVec<T> {
    fn as_ref(&self) -> &[T] {
        self.0.as_ref().as_slice()
    }
}
