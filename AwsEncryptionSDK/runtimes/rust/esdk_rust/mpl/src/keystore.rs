// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use crate::error::*;
use crate::kms_keyring::GrantTokenList;
use crate::types::*;
use async_trait::async_trait;

type TableName = String;

pub fn make_key_store(_config: &KeyStoreConfig) -> Result<KeyStoreRef, Error> {
    not_implemented("make_key_store")
}

pub fn make_key_store_admin(_config: &KeyStoreConfig) -> Result<KeyStoreAdminRef, Error> {
    not_implemented("make_key_store_admin")
}

//= aws-encryption-sdk-specification/framework/branch-key-store.md#pub fns
//= type=implication
//# The Keystore MUST support the following pub fns:
//#
//#- [GetKeyStoreInfo](#getkeystoreinfo)
//#- [CreateKeyStore](#createkeystore)
//#- [CreateKey](#createkey)
//#- [VersionKey](#versionkey)
//#- [GetActiveBranchKey](#getactivebranchkey)
//#- [GetBranchKeyVersion](#getbranchkeyversion)
//#- [GetBeaconKey](#getbeaconkey)

#[async_trait]
pub trait KeyStore: Send + Sync + std::fmt::Debug {
    async fn get_key_store_info(&self) -> Result<GetKeyStoreInfoOutput, Error>;
    ///Get the ACTIVE version for a particular Branch Key from the Key Store.
    async fn get_active_branch_key(
        &self,
        input: &GetActiveBranchKeyInput,
    ) -> Result<GetActiveBranchKeyOutput, Error>;
    ///Get a particular version of a Branch Key from the Key Store.
    async fn get_branch_key_version(
        &self,
        input: &GetBranchKeyVersionInput,
    ) -> Result<GetBranchKeyVersionOutput, Error>;
    ///Get a Beacon Key from the Key Store.
    async fn get_beacon_key(&self, input: &GetBeaconKeyInput) -> Result<GetBeaconKeyOutput, Error>;
}
#[async_trait]
#[allow(private_bounds)]
pub trait KeyStoreAdmin: Send + Sync + std::fmt::Debug + crate::MplPrivate {
    // CreateKey will create two keys to add to the key store
    // One is the branch key, which is used in the hierarchical keyring
    // The second is a beacon key that is used as a root key to
    // derive different beacon keys per beacon.
    ///Create a new Branch Key in the Key Store. Additionally create a Beacon Key that is tied to this Branch Key.
    async fn create_key(&self, input: &CreateKeyInput) -> Result<CreateKeyOutput, Error>;
    // `VersionKey` will create a new branch key under the
    // provided branchKeyIdentifier and rotate the "older" material
    // on the key store under the branchKeyIdentifier. This pub fn MUST NOT
    // rotate the beacon key under the branchKeyIdentifier.
    ///Create a new ACTIVE version of an existing Branch Key in the Key Store, and set the previously ACTIVE version to `DECRYPT_ONLY`.
    async fn version_key(&self, input: &VersionKeyInput) -> Result<VersionKeyOutput, Error>;
}

pub type KeyStoreRef = std::sync::Arc<dyn KeyStore>;
pub type KeyStoreAdminRef = std::sync::Arc<dyn KeyStoreAdmin>;

//   errors: [KeyStoreException]

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct KeyStoreConfig {
    //= aws-encryption-sdk-specification/framework/branch-key-store.md#initialization
    //= type=implication
    //# The following inputs MUST be specified to create a KeyStore:
    //#
    //# - [Table Name](#table-name)
    //# - [AWS KMS Configuration](#aws-kms-configuration)
    //# - [Logical KeyStore Name](#logical-keystore-name)
    ///The `DynamoDB` table name that backs this Key Store.
    pub ddb_table_name: TableName,

    ///Configures Key Store's KMS Key ARN restrictions.
    pub kms_configuration: KmsConfiguration,

    ///The logical name for this Key Store, which is cryptographically bound to the keys it holds. This appears in the Encryption Context of KMS requests as `tablename`.
    pub logical_key_store_name: String,

    //= aws-encryption-sdk-specification/framework/branch-key-store.md#initialization
    //= type=implication
    //# The following inputs MAY be specified to create a KeyStore:
    //#
    //# - [ID](#keystore-id)
    //# - [AWS KMS Grant Tokens](#aws-kms-grant-tokens)
    //# - [`DynamoDB` Client](#`DynamoDB`-client)
    //# - [KMS Client](#kms-client)
    ///An identifier for this Key Store.
    pub id: String,
    ///The AWS KMS grant tokens that are used when this Key Store calls to AWS KMS.
    pub grant_tokens: GrantTokenList,
    ///The `DynamoDB` client this Key Store uses to call Amazon `DynamoDB`. If None is provided and the KMS ARN is, the KMS ARN is used to determine the Region of the default client.
    pub ddb_client: Option<aws_sdk_dynamodb::Client>,
    ///The KMS client this Key Store uses to call AWS KMS.  If None is provided and the KMS ARN is, the KMS ARN is used to determine the Region of the default client.©
    pub kms_client: Option<aws_sdk_kms::Client>,
}

//= aws-encryption-sdk-specification/framework/branch-key-store.md#aws-kms-configuration
//= type=implication
//# `KMS Key ARN` and `KMS MRKey ARN` MUST take an additional argument
//# that is a KMS ARN.
///Configures Key Store's KMS Key ARN restrictions.
#[non_exhaustive]
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
/// Configures Key Store's KMS Key ARN restrictions.
pub enum KmsConfiguration {
    /// Key Store is restricted to only this KMS Key ARN. If a different KMS Key ARN is encountered when creating, versioning, or getting a Branch Key or Beacon Key, KMS is never called and an exception is thrown. While a Multi-Region Key (MKR) may be provided, the whole ARN, including the Region, is persisted in Branch Keys and MUST strictly equal this value to be considered valid.
    KmsKeyArn(String),
    /// If an MRK ARN is provided, and the Key Store table holds an MRK ARN, then those two ARNs may differ in region, although they must be otherwise equal. If either ARN is not an MRK ARN, then mrkKmsKeyArn behaves exactly as kmsKeyArn.
    KmsMrKeyArn(String),
    /// The Key Store can use the KMS Key ARNs already persisted in the Backing Table. The `VersionKey` and `CreateKey` pub fns are NOT supported and will fail with a runtime exception. There is no Multi-Region logic with this configuration; if a Multi-Region Key is encountered, and the region in the ARN is not the region of the KMS Client, requests will Fail with KMS Exceptions.
    Discovery(Discovery),
    /// The Key Store can use the KMS Key ARNs already persisted in the Backing Table. The `VersionKey` and `CreateKey` pub fns are NOT supported and will fail with a runtime exception. If a Multi-Region Key is encountered, the region in the ARN is changed to the configured region.
    MrDiscovery(MrDiscovery),
}
impl Default for KmsConfiguration {
    fn default() -> Self {
        Self::Discovery(Discovery {})
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct Discovery {}

//= aws-encryption-sdk-specification/framework/branch-key-store.md#aws-kms-configuration
//= type=implication
//# `MRDiscovery` MUST take an additional argument, which is a region.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct MrDiscovery {
    ///Any MRK ARN discovered will have its region replaced with this.
    region: String,
}
impl MrDiscovery {
    #[must_use]
    pub const fn new(region: String) -> Self {
        Self { region }
    }
}

///The configuration information for a Key Store.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct GetKeyStoreInfoOutput {
    ///An identifier for this Key Store.
    pub key_store_id: String,

    ///The `DynamoDB` table name that backs this Key Store.
    pub key_store_name: TableName,

    ///The logical name for this Key Store, which is cryptographically bound to the keys it holds.
    pub logical_key_store_name: String,

    ///The AWS KMS grant tokens that are used when this Key Store calls to AWS KMS.
    pub grant_tokens: GrantTokenList,

    ///Configures Key Store's KMS Key ARN restrictions.
    pub kms_configuration: KmsConfiguration,
}
impl GetKeyStoreInfoOutput {
    #[must_use]
    pub const fn new(
        key_store_id: String,
        key_store_name: TableName,
        logical_key_store_name: String,
        kms_configuration: KmsConfiguration,
        grant_tokens: GrantTokenList,
    ) -> Self {
        Self {
            key_store_id,
            key_store_name,
            logical_key_store_name,
            grant_tokens,
            kms_configuration,
        }
    }
}

//= aws-encryption-sdk-specification/framework/branch-key-store.md#createkey
//= type=implication
//# The CreateKey caller MUST provide:
//# - An optional branch key id
//# - An optional encryption context
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct CreateKeyInput {
    ///The identifier for the created Branch Key.
    pub branch_key_identifier: String,

    ///Custom encryption context for the Branch Key. Required if branchKeyIdentifier is set.
    pub encryption_context: EncryptionContext,
}
impl CreateKeyInput {
    #[must_use]
    pub const fn new(branch_key_identifier: String, encryption_context: EncryptionContext) -> Self {
        Self {
            branch_key_identifier,
            encryption_context,
        }
    }
}

///Outputs for Branch Key creation.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct CreateKeyOutput {
    ///A identifier for the created Branch Key.
    pub branch_key_identifier: String,
}
impl CreateKeyOutput {
    #[must_use]
    pub const fn new(branch_key_identifier: String) -> Self {
        Self {
            branch_key_identifier,
        }
    }
}

///Inputs for versioning a Branch Key.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct VersionKeyInput {
    //= aws-encryption-sdk-specification/framework/branch-key-store.md#versionkey
    //= type=implication
    //# - MUST supply a `branch-key-id`
    ///The identifier for the Branch Key to be versioned.
    pub branch_key_identifier: String,
}
impl VersionKeyInput {
    #[must_use]
    pub const fn new(branch_key_identifier: String) -> Self {
        Self {
            branch_key_identifier,
        }
    }
}

///Outputs for versioning a Branch Key.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct VersionKeyOutput {}
impl VersionKeyOutput {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

///Inputs for getting a Branch Key's ACTIVE version.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct GetActiveBranchKeyInput {
    ///The identifier for the Branch Key to get the ACTIVE version for.
    pub branch_key_identifier: String,
}
impl GetActiveBranchKeyInput {
    #[must_use]
    pub const fn new(branch_key_identifier: String) -> Self {
        Self {
            branch_key_identifier,
        }
    }
}

///Outputs for getting a Branch Key's ACTIVE version.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct GetActiveBranchKeyOutput {
    //= aws-encryption-sdk-specification/framework/branch-key-store.md#getactivebranchkey
    //= type=implication
    //# - MUST supply a `branch-key-id`
    ///The materials for the Branch Key.
    pub branch_key_materials: BranchKeyMaterials,
}
impl GetActiveBranchKeyOutput {
    #[must_use]
    pub const fn new(branch_key_materials: BranchKeyMaterials) -> Self {
        Self {
            branch_key_materials,
        }
    }
}

///Inputs for getting a version of a Branch Key.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct GetBranchKeyVersionInput {
    //= aws-encryption-sdk-specification/framework/branch-key-store.md#getbranchkeyversion
    //= type=implication
    //# - MUST supply a `branch-key-id`
    ///The identifier for the Branch Key to get a particular version for.
    pub branch_key_identifier: String,

    //= aws-encryption-sdk-specification/framework/branch-key-store.md#getbranchkeyversion
    //= type=implication
    //# - MUST supply a `branchKeyVersion`
    ///The version to get.
    pub branch_key_version: String,
}
impl GetBranchKeyVersionInput {
    #[must_use]
    pub const fn new(branch_key_identifier: String, branch_key_version: String) -> Self {
        Self {
            branch_key_identifier,
            branch_key_version,
        }
    }
}

///Outputs for getting a version of a Branch Key.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct GetBranchKeyVersionOutput {
    ///The materials for the Branch Key.
    pub branch_key_materials: BranchKeyMaterials,
}
impl GetBranchKeyVersionOutput {
    #[must_use]
    pub const fn new(branch_key_materials: BranchKeyMaterials) -> Self {
        Self {
            branch_key_materials,
        }
    }
}

///Inputs for getting a Beacon Key
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct GetBeaconKeyInput {
    //= aws-encryption-sdk-specification/framework/branch-key-store.md#getbeaconkey
    //= type=implication
    //# - MUST supply a `branch-key-id`
    ///The identifier of the Branch Key the Beacon Key is associated with.
    pub branch_key_identifier: String,
}
impl GetBeaconKeyInput {
    #[must_use]
    pub const fn new(branch_key_identifier: String) -> Self {
        Self {
            branch_key_identifier,
        }
    }
}

///Outputs for getting a Beacon Key
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct GetBeaconKeyOutput {
    ///The materials for the Beacon Key.
    pub beacon_key_materials: BeaconKeyMaterials,
}
impl GetBeaconKeyOutput {
    #[must_use]
    pub const fn new(beacon_key_materials: BeaconKeyMaterials) -> Self {
        Self {
            beacon_key_materials,
        }
    }
}

//= aws-encryption-sdk-specification/framework/pub structs.md#pub struct-3
//= type=implication
//# This pub struct MUST include all of the following fields:
//#
//# - [Branch Key](#branch-key)
//# - [Branch Key Id](#branch-key-id)
//# - [Branch Key Version](#branch-key-version)
//# - [Encryption Context](#encryption-context-3)
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct BranchKeyMaterials {
    pub branch_key_identifier: String,
    pub branch_key_version: String,
    pub branch_key: Secret,
    pub encryption_context: EncryptionContext,
}
impl BranchKeyMaterials {
    #[must_use]
    pub const fn new(
        branch_key_identifier: String,
        branch_key_version: String,
        branch_key: Secret,
        encryption_context: EncryptionContext,
    ) -> Self {
        Self {
            branch_key_identifier,
            branch_key_version,
            branch_key,
            encryption_context,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct BeaconKeyMaterials {
    //= aws-encryption-sdk-specification/framework/pub structs.md#pub struct-4
    //= type=implication
    //# This pub struct MUST include the following fields:
    //# - [Beacon Key Id](#beacon-key-id)
    //# - [Encryption Context](#encryption-context-4)
    pub beacon_key_identifier: String,

    //= aws-encryption-sdk-specification/framework/pub structs.md#pub struct-4
    //= type=implication
    //# This pub struct MAY include the following fields:
    //# - [Beacon Key](#beacon-key)
    //# - [HMAC Keys](#hmac-keys)
    pub beacon_key: Secret,

    pub hmac_keys: HmacKeyMap,
    pub encryption_context: EncryptionContext,
}
impl BeaconKeyMaterials {
    #[must_use]
    pub const fn new(
        beacon_key_identifier: String,
        beacon_key: Secret,
        hmac_keys: HmacKeyMap,
        encryption_context: EncryptionContext,
    ) -> Self {
        Self {
            beacon_key_identifier,
            beacon_key,
            hmac_keys,
            encryption_context,
        }
    }
}

/// The key refers to the beacon name for which this value was derived.
/// The value is the HKDF derived from the beacon key and the UTF Encoding of the beacon name.
type HmacKeyMap = std::collections::HashMap<String, Secret>;

// Errors
// KeyStoreException
