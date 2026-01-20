use std::time::Duration;

use crate::error::*;
use crate::{DecryptionMaterials, EncryptionMaterials};
use async_trait::async_trait;

//= aws-encryption-sdk-specification/framework/cryptographic-materials-cache.md#overview
//= type=implication
//# Cryptographic materials cache (CMC) is used by the [caching cryptographic materials manager (CMM)](caching-cmm.md)
//# to store cryptographic materials for reuse.
//# This document describes the interface that all CMCs MUST implement.
#[async_trait]
#[allow(private_bounds)]
pub trait CryptographicMaterialsCache: Send + Sync + std::fmt::Debug + crate::MplPrivate {
    //= aws-encryption-sdk-specification/framework/cryptographic-materials-cache.md#put-cache-entry
    //= type=implication
    //# This operation MUST NOT return the inserted cache entry.
    async fn put_cache_entry(&self, input: &PutCacheEntryInput) -> Result<(), Error>;
    async fn get_cache_entry(
        &self,
        input: &GetCacheEntryInput,
    ) -> Result<GetCacheEntryOutput, Error>;
    async fn update_usage_metadata(&self, input: &UpdateUsageMetadataInput) -> Result<(), Error>;
    async fn delete_cache_entry(&self, input: &DeleteCacheEntryInput) -> Result<(), Error>;
}

pub type CryptographicMaterialsCacheRef = std::sync::Arc<dyn CryptographicMaterialsCache>;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct GetCacheEntryInput {
    pub identifier: Vec<u8>,
    pub bytes_used: u64,
}

// Should inner SystemTime be private?
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Default, Eq, PartialEq)]
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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
    RequestFailed(Error),
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Materials {
    Missing(Missing),
    Encryption(EncryptionMaterials),
    Decryption(DecryptionMaterials),
    #[cfg(feature = "ddb")]
    BranchKey(crate::keystore::BranchKeyMaterials),
    #[cfg(feature = "ddb")]
    BeaconKey(crate::keystore::BeaconKeyMaterials),
}
impl Default for Materials {
    fn default() -> Self {
        Self::Missing(Missing::default())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct DeleteCacheEntryInput {
    pub identifier: Vec<u8>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct UpdateUsageMetadataInput {
    pub identifier: Vec<u8>,
    pub bytes_used: u64,
}

// error EntryAlreadyExists

pub fn create_cryptographic_materials_cache(
    _input: CreateCryptographicMaterialsCacheInput,
) -> Result<CryptographicMaterialsCacheRef, Error> {
    not_implemented("create_cryptographic_materials_cache")
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum CacheType {
    Default(DefaultCache),
    #[default]
    No,
    MultiThreaded(MultiThreadedCache),
    StormTracking(StormTrackingCache),
    // /// Shared cache across multiple Hierarchical Keyrings. For this cache type, the user should provide an already constructed CryptographicMaterialsCache to the Hierarchical Keyring at initialization.
    // Shared(crate::types::cryptographic_materials_cache::CryptographicMaterialsCacheRef),
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct CreateCryptographicMaterialsCacheInput {
    /// Which type of local cache to use.
    pub cache: CacheType,
}

/// The best choice for most situations. Probably a `StormTrackingCache`.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct DefaultCache {
    /// Maximum number of entries cached.
    pub entry_capacity: u32,
}
impl DefaultCache {
    #[must_use]
    pub const fn new(entry_capacity: u32) -> Self {
        Self { entry_capacity }
    }
}

/// A cache that is safe for use in a multi threaded environment, but no extra functionality.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct MultiThreadedCache {
    /// Maximum number of entries cached.
    pub entry_capacity: u32,

    /// Number of entries to prune at a time.
    pub entry_pruning_tail_size: u32,
}

impl Default for MultiThreadedCache {
    fn default() -> Self {
        Self {
            entry_capacity: 1000,
            entry_pruning_tail_size: 1,
        }
    }
}

impl MultiThreadedCache {
    #[must_use]
    pub fn new(entry_capacity: u32) -> Self {
        Self {
            entry_capacity,
            ..Default::default()
        }
    }
}

/// A cache that is safe for use in a multi threaded environment,
/// and tries to prevent redundant or overly parallel backend calls.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct StormTrackingCache {
    /// Maximum number of entries cached.
    pub entry_capacity: u32,

    /// Number of entries to prune at a time.
    pub entry_pruning_tail_size: u32,

    /// How much time before expiration should an attempt be made to refresh the materials.
    ///   If zero, use a simple cache with no storm tracking.
    pub grace_period: Duration,

    /// How much time between attempts to refresh the materials.
    pub grace_interval: Duration,

    /// How many simultaneous attempts to refresh the materials.
    pub fan_out: u32,

    /// How much time until an attempt to refresh the materials should be forgotten.
    pub in_flight_ttl: Duration,
}

impl Default for StormTrackingCache {
    fn default() -> Self {
        Self {
            entry_capacity: 1000,
            entry_pruning_tail_size: 1,
            grace_period: Duration::from_secs(10),
            grace_interval: Duration::from_secs(1),
            fan_out: 20,
            in_flight_ttl: Duration::from_secs(10),
        }
    }
}
impl StormTrackingCache {
    #[must_use]
    pub fn new(entry_capacity: u32) -> Self {
        Self {
            entry_capacity,
            ..Default::default()
        }
    }
}
