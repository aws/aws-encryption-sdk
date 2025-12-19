use crate::error::*;
use async_trait::async_trait;
pub use crate::types::{EncryptionMaterials, DecryptionMaterials, EncryptedDataKey};

//= aws-encryption-sdk-specification/framework/cryptographic-materials-cache.md#overview
//= type=implication
//# Cryptographic materials cache (CMC) is used by the [caching cryptographic materials manager (CMM)](caching-cmm.md)
//# to store cryptographic materials for reuse.
//# This document describes the interface that all CMCs MUST implement.
#[async_trait]
pub trait CryptographicMaterialsCache: Send + Sync + std::fmt::Debug {
    //= aws-encryption-sdk-specification/framework/cryptographic-materials-cache.md#put-cache-entry
    //= type=implication
    //# This operation MUST NOT return the inserted cache entry.
    async fn put_cache_entry(&self, input: &PutCacheEntryInput) -> Result<(), Error>;
    async fn get_cache_entry(&self, input: &GetCacheEntryInput)
    -> Result<GetCacheEntryOutput, Error>;
    async fn update_usage_metadata(&self, input: &UpdateUsageMetadataInput) -> Result<(), Error>;
    async fn delete_cache_entry(&self, input: &DeleteCacheEntryInput) -> Result<(), Error>;
}

pub type CryptographicMaterialsCacheRef = std::sync::Arc<dyn CryptographicMaterialsCache>;

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct PutCacheEntryInput {
    pub identifier: Vec<u8>,
    pub materials: Materials,
    pub creation_time: Time,
    //= aws-encryption-sdk-specification/framework/cryptographic-materials-cache.md#put-cache-entry
    //= type=implication
    //# The cache entry MUST include all [usage metadata](#usage-metadata)
    //# since this information can not be updated after the put operation.
    pub expiry_time: Time,
    pub messages_used: u64,
    pub bytes_used: u64,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct GetCacheEntryInput {
    pub identifier: Vec<u8>,
    pub bytes_used: u64,
}

// Should inner SystemTime be private?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub struct Time(pub std::time::SystemTime);
impl Default for Time {
    fn default() -> Self {
        Self(std::time::UNIX_EPOCH)
    }
}
//= aws-encryption-sdk-specification/framework/cryptographic-materials-cache.md#cache-entry
//= type=implication
//# A cache entry represents an entry in the cryptographic materials cache
//# and MUST have the following information.
//#
//# - [Materials](#materials)
//# - [Creation Time](#creation-time)
//# - [Expiry Time](#expiry-time)
//# - [Usage Metadata](#usage-metadata)
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct GetCacheEntryOutput {
    pub materials: Materials,
    pub creation_time: Time,
    //= aws-encryption-sdk-specification/framework/cryptographic-materials-cache.md#time-to-live-ttl
    //= type=implication
    //# Each cache entry has a time-to-live (TTL)
    //# that represents a point in time at which the cache entry
    //# MUST be considered invalid.
    pub expiry_time: Time,
    pub messages_used: u64,
    pub bytes_used: u64,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub enum Missing {
    /// Thrown if a request is waiting for longer than `inflightTTL`.
    /// The Storm Tracking Cache protects against unbounded parallelism.
    /// The Storm Tracking Cache will only work `fanOut` number of concurrent requests.
    /// As requests are completed,
    /// queued requests are worked.
    /// If a request is not worked in less than `inflightTTL`,
    /// this exception is thrown.

    /// Note that this exception does NOT imply that the material requested
    /// is invalid or unreachable;
    /// it only implies that the cache had more requests to handle than it could
    /// with the given `fanOut` and `inflightTTL` constraints.
    InFlightTTLExceeded,
    #[default]
    EntryDoesNotExist,
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Materials {
    Missing(Missing),
    Encryption(EncryptionMaterials),
    Decryption(DecryptionMaterials),
    // BranchKey(crate::deps::aws_cryptography_keyStore::types::BranchKeyMaterials),
    // BeaconKey(crate::deps::aws_cryptography_keyStore::types::BeaconKeyMaterials),
}
impl Default for Materials {
    fn default() -> Self {
        Self::Missing(Missing::default())
    }
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct DeleteCacheEntryInput {
    pub identifier: Vec<u8>,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct UpdateUsageMetadataInput {
    pub identifier: Vec<u8>,
    pub bytes_used: u64,
}

// error EntryAlreadyExists

#[derive(Debug, Clone, Default)]
struct DefaultCryptographicMaterialsCache {}
#[async_trait]
impl CryptographicMaterialsCache for DefaultCryptographicMaterialsCache {
    async fn put_cache_entry(&self, _input: &PutCacheEntryInput) -> Result<(), Error> {
        Ok(())
    }

    async fn get_cache_entry(
        &self,
        _input: &GetCacheEntryInput,
    ) -> Result<GetCacheEntryOutput, Error> {
        Ok(GetCacheEntryOutput::default())
    }

    async fn update_usage_metadata(&self, _input: &UpdateUsageMetadataInput) -> Result<(), Error> {
        Ok(())
    }

    async fn delete_cache_entry(&self, _input: &DeleteCacheEntryInput) -> Result<(), Error> {
        Ok(())
    }
}

pub fn create_cryptographic_materials_cache(
    _input: CreateCryptographicMaterialsCacheInput,
) -> Result<CryptographicMaterialsCacheRef, Error> {
    Ok(std::sync::Arc::new(DefaultCryptographicMaterialsCache {}))
}

#[derive(Debug, Clone, Default, Copy)]
#[non_exhaustive]
pub enum CacheType {
// Default(crate::types::DefaultCache),
#[default]
No,
// SingleThreaded(crate::types::SingleThreadedCache),
// MultiThreaded(crate::types::MultiThreadedCache),
// StormTracking(crate::types::StormTrackingCache),
// /// Shared cache across multiple Hierarchical Keyrings. For this cache type, the user should provide an already constructed CryptographicMaterialsCache to the Hierarchical Keyring at initialization.
// Shared(crate::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef),
}

#[derive(Debug, Clone, Default, Copy)]
#[non_exhaustive]
pub struct CreateCryptographicMaterialsCacheInput {
  /// Which type of local cache to use.
  pub cache: CacheType
}

/* 
@javadoc("The best choice for most situations. Probably a StormTrackingCache.")
structure DefaultCache {
  @required
  @javadoc("Maximum number of entries cached.")
  entryCapacity: CountingNumber
}

@javadoc("A cache that is NOT safe for use in a multi threaded environment.")
structure SingleThreadedCache {
  //= aws-encryption-sdk-specification/framework/local-cryptographic-materials-cache.md#initialization
  //= type=implication
  //# On initialization of the local CMC,
  //# the caller MUST provide the following:
  //#
  //# - [Entry Capacity](#entry-capacity)
  //#
  //# The local CMC MUST also define the following:
  //#
  //# - [Entry Pruning Tail Size](#entry-pruning-tail-size)
  @required
  @javadoc("Maximum number of entries cached.")
  entryCapacity: CountingNumber,

  @javadoc("Number of entries to prune at a time.")
  entryPruningTailSize: CountingNumber,
}

@javadoc("A cache that is safe for use in a multi threaded environment, but no extra functionality.")
structure MultiThreadedCache {
  @required
  @javadoc("Maximum number of entries cached.")
  entryCapacity: CountingNumber,

  @javadoc("Number of entries to prune at a time.")
  entryPruningTailSize: CountingNumber,
}

@javadoc("A cache that is safe for use in a multi threaded environment,
and tries to prevent redundant or overly parallel backend calls.")
structure StormTrackingCache {
  @required
  @javadoc("Maximum number of entries cached.")
  entryCapacity: CountingNumber,

  @javadoc("Number of entries to prune at a time.")
  entryPruningTailSize: CountingNumber,

  @required
  @javadoc("How much time before expiration should an attempt be made to refresh the materials.
  If zero, use a simple cache with no storm tracking.")
  gracePeriod: CountingNumber,

  @required
  @javadoc("How much time between attempts to refresh the materials.")
  graceInterval: CountingNumber,

  @required
  @javadoc("How many simultaneous attempts to refresh the materials.")
  fanOut: CountingNumber,

  @required
  @javadoc("How much time until an attempt to refresh the materials should be forgotten.")
  inFlightTTL: CountingNumber,

  @required
  @javadoc("How many milliseconds should a thread sleep if fanOut is exceeded.")
  sleepMilli: CountingNumber,

  @javadoc("The time unit for gracePeriod, graceInterval, and inFlightTTL.
  The default is seconds.
  If this is set to milliseconds, then these values will be treated as milliseconds.")
  timeUnits: TimeUnits
}

*/
