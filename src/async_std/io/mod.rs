use std::{
	io::SeekFrom,
	marker::PhantomData,
	ops::{Deref, DerefMut}
};

use crate::{
	coroutines::{
		async_fn, async_trait_fn,
		env::AsyncContext,
		runtime::{check_interrupt, get_context, is_interrupted}
	},
	error::Result,
	xx_core
};

pub mod buf;

pub use buf::*;

use super::{AsyncIterator, Iterator};

#[async_trait_fn]
pub trait Read<Context: AsyncContext> {
	/// Read into `buf`, returning the amount of bytes read
	///
	/// Returning zero strictly means EOF
	async fn read(&mut self, buf: &mut [u8]) -> Result<usize>;
}

#[async_trait_fn]
pub trait Write<Context: AsyncContext> {
	/// Write from `buf`, returning the amount of bytes read
	///
	/// Returning zero strictly means EOF
	async fn write(&mut self, buf: &[u8]) -> Result<usize>;

	/// Flush buffered data
	async fn flush(&mut self) -> Result<()>;
}

#[async_trait_fn]
pub trait Seek<Context: AsyncContext> {
	async fn seek(&mut self, seek: SeekFrom) -> Result<u64>;

	/// Whether or not stream length can be calculated without an
	/// expensive I/O operation
	fn stream_len_fast(&self) -> bool {
		false
	}

	/// Get the length of the stream in bytes
	async fn stream_len(&mut self) -> Result<u64> {
		let old_pos = self.stream_position(get_context().await)?;
		let len = self.seek(SeekFrom::End(0), get_context().await)?;

		if old_pos != len {
			self.seek(SeekFrom::Start(old_pos), get_context().await)?;
		}

		Ok(len)
	}

	/// Whether or not stream length can be calculated without an
	/// expensive I/O operation
	fn stream_position_fast(&self) -> bool {
		false
	}

	/// Get the position in the stream in bytes
	async fn stream_position(&mut self) -> Result<u64> {
		self.seek(SeekFrom::Current(0), get_context().await)
	}
}

#[async_trait_fn]
pub trait Close<Context: AsyncContext> {
	async fn close(self) -> Result<()>;
}

pub struct Stream<Context: AsyncContext, Inner> {
	inner: Inner,
	phantom: PhantomData<Context>
}

impl<Context: AsyncContext, Inner> Stream<Context, Inner> {
	pub fn new(inner: Inner) -> Self {
		Self { inner, phantom: PhantomData }
	}

	pub fn into_inner(self) -> Inner {
		self.inner
	}
}

#[async_fn]
impl<Context: AsyncContext, Inner: Read<Context>> Stream<Context, Inner> {
	pub async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		self.inner.read(buf, get_context().await)
	}

	/// Read until the buffer is filled, an I/O error, an interrupt, or an EOF
	///
	/// On interrupted, returns the number of bytes read if it is not zero
	pub async fn read_exact(&mut self, buf: &mut [u8]) -> Result<usize> {
		let mut read = 0;

		while read < buf.len() && !is_interrupted().await {
			match self.read(&mut buf[read..]).await {
				Ok(0) => break,
				Ok(n) => read += n,
				Err(err) => {
					if err.is_interrupted() {
						break;
					}

					return Err(err);
				}
			}
		}

		if read == 0 {
			check_interrupt().await?;
		}

		Ok(read)
	}

	/// Reads until an EOF, I/O error, or interrupt
	///
	/// On interrupted, returns the number of bytes read if it is not zero
	pub async fn read_to_end(&mut self, buf: &mut Vec<u8>) -> Result<usize> {
		let start_len = buf.len();

		while !is_interrupted().await {
			let mut capacity = buf.capacity();
			let len = buf.len();

			if len == capacity {
				buf.reserve(32);
			}

			unsafe {
				capacity = buf.capacity();

				match self.read(buf.get_unchecked_mut(len..capacity)).await {
					Ok(0) => break,
					Ok(read) => buf.set_len(len + read),
					Err(err) => {
						if err.is_interrupted() {
							break;
						}

						return Err(err);
					}
				}
			}

			if buf.len() == capacity {
				let mut probe = [0u8; 32];

				match self.read(&mut probe).await {
					Ok(0) => break,
					Ok(read) => {
						buf.extend_from_slice(&probe[0..read]);
					}

					Err(err) => {
						if err.is_interrupted() {
							break;
						}

						return Err(err);
					}
				}
			}
		}

		let total = buf.len() - start_len;

		if total == 0 {
			check_interrupt().await?;
		}

		Ok(total)
	}
}

#[async_trait_fn]
impl<Context: AsyncContext, Inner: Read<Context>> Read<Context> for Stream<Context, Inner> {
	async fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
		self.read(buf).await
	}
}

#[async_fn]
impl<Context: AsyncContext, Inner: Write<Context>> Stream<Context, Inner> {
	pub async fn write(&mut self, buf: &[u8]) -> Result<usize> {
		self.inner.write(buf, get_context().await)
	}

	/// Try to write the entire buffer, returning on I/O error, interrupt, or
	/// EOF
	///
	/// On interrupted, returns the number of bytes read if it is not zero
	pub async fn write_exact(&mut self, buf: &[u8]) -> Result<usize> {
		let mut wrote = 0;

		while wrote < buf.len() {
			match self.write(&buf[wrote..]).await {
				Ok(0) => break,
				Ok(n) => wrote += n,
				Err(err) => {
					if err.is_interrupted() {
						break;
					}

					return Err(err);
				}
			}
		}

		if wrote == 0 {
			check_interrupt().await?;
		}

		Ok(wrote)
	}

	pub async fn flush(&mut self) -> Result<()> {
		self.inner.flush(get_context().await)
	}
}

#[async_trait_fn]
impl<Context: AsyncContext, Inner: Write<Context>> Write<Context> for Stream<Context, Inner> {
	async fn write(&mut self, buf: &[u8]) -> Result<usize> {
		self.write(buf).await
	}

	async fn flush(&mut self) -> Result<()> {
		self.flush().await
	}
}

#[async_fn]
impl<Context: AsyncContext, Inner: Seek<Context>> Stream<Context, Inner> {
	pub async fn seek(&mut self, seek: SeekFrom) -> Result<u64> {
		self.inner.seek(seek, get_context().await)
	}

	pub async fn rewind(&mut self) -> Result<()> {
		self.seek(SeekFrom::Start(0)).await?;

		Ok(())
	}

	pub async fn rewind_exact(&mut self, amount: i64) -> Result<u64> {
		self.seek(SeekFrom::Current(-amount)).await
	}

	pub async fn skip_exact(&mut self, amount: i64) -> Result<u64> {
		self.seek(SeekFrom::Current(amount)).await
	}

	pub fn stream_len_fast(&self) -> bool {
		self.inner.stream_len_fast()
	}

	pub async fn stream_len(&mut self) -> Result<u64> {
		self.inner.stream_len(get_context().await)
	}

	pub fn stream_position_fast(&self) -> bool {
		self.inner.stream_position_fast()
	}

	pub async fn stream_position(&mut self) -> Result<u64> {
		self.inner.stream_position(get_context().await)
	}
}

#[async_trait_fn]
impl<Context: AsyncContext, Inner: Seek<Context>> Seek<Context> for Stream<Context, Inner> {
	async fn seek(&mut self, seek: SeekFrom) -> Result<u64> {
		self.seek(seek).await
	}

	async fn stream_len(&mut self) -> Result<u64> {
		self.stream_len().await
	}

	async fn stream_position(&mut self) -> Result<u64> {
		self.stream_position().await
	}
}

#[async_fn]
impl<Context: AsyncContext, Inner: Close<Context>> Stream<Context, Inner> {
	pub async fn close(self) -> Result<()> {
		self.inner.close(get_context().await)
	}
}

#[async_trait_fn]
impl<Context: AsyncContext, Inner: Close<Context>> Close<Context> for Stream<Context, Inner> {
	async fn close(self) -> Result<()> {
		self.close().await
	}
}

impl<Context: AsyncContext, Inner> Deref for Stream<Context, Inner> {
	type Target = Inner;

	fn deref(&self) -> &Inner {
		&self.inner
	}
}

impl<Context: AsyncContext, Inner> DerefMut for Stream<Context, Inner> {
	fn deref_mut(&mut self) -> &mut Inner {
		&mut self.inner
	}
}

pub struct Lines<Context: AsyncContext, R: Read<Context>> {
	reader: BufReader<Context, R>
}

impl<Context: AsyncContext, R: Read<Context>> Lines<Context, R> {
	pub fn new(reader: BufReader<Context, R>) -> Iterator<Context, Self> {
		Iterator::new(Self { reader })
	}
}

#[async_trait_fn]
impl<Context: AsyncContext, R: Read<Context>> AsyncIterator<Context> for Lines<Context, R> {
	type Item = Result<String>;

	async fn next(&mut self) -> Option<Self::Item> {
		let mut line = String::new();

		match self.reader.read_line(&mut line).await {
			Err(err) => Some(Err(err)),
			Ok(Some(_)) => Some(Ok(line)),
			Ok(None) => None
		}
	}
}
