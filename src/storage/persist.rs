extern crate failure;
extern crate random_access_storage as ras;

use self::ras::SyncMethods;
use super::Storage;
use failure::Error;

/// Persist data to a `Storage` instance.
pub trait Persist<T>
where
  T: SyncMethods,
{
  /// Create an instance from a byte vector.
  fn from_bytes(index: usize, buf: &[u8]) -> Self;

  /// Create a vector.
  fn to_vec(&self) -> Result<Vec<u8>, Error>;

  /// Persist into a storage backend.
  fn store(&self, index: usize, store: Storage<T>) -> Result<(), Error>;
}