use std::io::Read;
use crate::internal::buffer::Buffer;

/// A buffered reader for reading database index data.
///
/// This object acts as a wrapper around another object which implements `Read`. This object will
/// perform buffered read operations on `inner`.
pub struct IndexReader<R: Read> {
    /// The inner `Read` object.
    inner: R,

    /// The buffer used for performing buffered reads to `inner`.
    buffer: Buffer,
}

impl<R: Read> IndexReader<R> {
    /// Creates a new instance of `IndexReader`, using a set capacity for the internal buffer.
    ///
    /// `capacity` sets the number of bytes that can be read into the buffer at a time.
    pub fn with_capacity(inner: R, capacity: usize) -> Self {
        Self {
            inner,
            buffer: Buffer::new(capacity)
        }
    }
}

impl<R: Read + Default> Default for IndexReader<R> {
    fn default() -> Self {
        Self {
            inner: R::default(),
            buffer: Buffer::new(8192)
        }
    }
}