use crate::keyring::KeyringReference;
use crate::suites::AlgorithmSuiteId;
use crate::error::*;
use async_trait::async_trait;
use crate::types::*;
use crate::commitment::CommitmentPolicy;

//= aws-encryption-sdk-specification/framework/cmm-interface.md#supported-cmms
//= type=implication
//# Note: A user MAY create their own custom CMM.

//= aws-encryption-sdk-specification/framework/cmm-interface.md#overview
//= type=implication
//# The CMM interface describes the interface that all CMMs MUST implement.

#[async_trait]
pub trait CryptographicMaterialsManager: Send + Sync + std::fmt::Debug {
    async fn get_encryption_materials(
        &self,
        input: &GetEncryptionMaterialsInput,
    ) -> Result<EncryptionMaterials, Error>;
    async fn decrypt_materials(
        &self,
        input: &DecryptMaterialsInput,
    ) -> Result<DecryptionMaterials, Error>;
}

pub type CryptographicMaterialsManagerReference = std::sync::Arc<dyn CryptographicMaterialsManager>;

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct GetEncryptionMaterialsInput {
    //= aws-encryption-sdk-specification/framework/cmm-interface.md#encryption-materials-request
    //= type=implication
    //# The encryption materials request MUST include the following:
    //#
    //# - [Encryption Context](structures.md#encryption-context)
    //#   - The encryption context provided MAY be empty.
    //# - [Commitment Policy](./commitment-policy.md#supported-commitment-policy-enum)
    pub encryption_context: EncryptionContext,

    pub commitment_policy: CommitmentPolicy,

    //= aws-encryption-sdk-specification/framework/cmm-interface.md#encryption-materials-request
    //= type=implication
    //# The encryption request MAY include the following:
    //#
    //# - [Algorithm Suite Id](algorithm-suites.md#algorithm-suite-id)
    //# - Required Encryption Context Keys - a set of strings.
    //# - Max Plaintext Length
    //#   - This value represents the maximum length of the plaintext to be encrypted
    //#     using the returned materials.
    //#     The length of the plaintext to be encrypted MUST not be larger than this value.
    pub algorithm_suite_id: AlgorithmSuiteId,

    pub max_plaintext_length: u64,

    pub required_encryption_context_keys: Vec<EncryptionContextKey>,
}

#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct DecryptMaterialsInput {
    //= aws-encryption-sdk-specification/framework/cmm-interface.md#decrypt-materials-request
    //= type=implication
    //# The decrypt materials request MUST include the following:
    //#
    //# - [Algorithm Suite Id](algorithm-suites.md#algorithm-suite-id)
    //# - [Commitment Policy](./commitment-policy.md#supported-commitment-policy-enum)
    //# - [Encrypted Data Keys](pub structs.md#encrypted-data-keys)
    //# - [Encryption Context](pub structs.md#encryption-context)
    //#   - The encryption context provided MAY be empty.
    pub algorithm_suite_id: AlgorithmSuiteId,

    pub commitment_policy: CommitmentPolicy,

    pub encrypted_data_keys: Vec<EncryptedDataKey>,

    pub encryption_context: EncryptionContext,

    //= aws-encryption-sdk-specification/framework/cmm-interface.md#decrypt-materials-request
    //= type=implication
    //# The decrypt materials request MAY include the following:
    //#
    //# - [Reproduced Encryption Context](pub structs.md#encryption-context)
    pub reproduced_encryption_context: EncryptionContext,
}

// CMM Constructors

///Creates a Default Cryptographic Materials Manager.")
pub fn create_default_cryptographic_materials_manager(
    _input: CreateDefaultCryptographicMaterialsManagerInput,
) -> Result<CryptographicMaterialsManagerReference, Error> {
    todo!()
}

///Inputs for creating a Default Cryptographic Materials Manager.")
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CreateDefaultCryptographicMaterialsManagerInput {
    ///The Keyring that the created Default Cryptographic Materials Manager will use to wrap data keys.")
    pub keyring: Option<KeyringReference>,
}

///Creates an Required Encryption Context Cryptographic Materials Manager.")
pub fn create_required_encryption_context_cmm(
    _input: CreateRequiredEncryptionContextCMMInput,
) -> Result<CryptographicMaterialsManagerReference, Error> {
    todo!()
}

///Inputs for creating an Required Encryption Context Cryptographic Materials Manager.")
#[derive(Debug, Clone, Default)]
#[non_exhaustive]
pub struct CreateRequiredEncryptionContextCMMInput {
    ///The Cryptographic Materials Manager that the created Required Encryption Context Cryptographic Materials Manager will delegate to. Either a Keyring or underlying Cryptographic Materials Manager must be specified.")
    pub underlying_cmm: Option<CryptographicMaterialsManagerReference>,
    ///The Keyring that the created Cryptographic Materials Manager will use to wrap data keys. The created Required Encryption Context CMM will delegate to a Default Cryptographic Materials Manager created with this Keyring. Either a Keyring or an underlying Cryptographic Materials Manager must be specified as input.")
    pub keyring: Option<KeyringReference>,

    ///A list of Encryption Context keys which are required to be supplied during encryption and decryption, and correspond to Encryption Context key-value pairs which are not stored on the resulting message.")
    pub required_encryption_context_keys: Vec<EncryptionContextKey>,
}
