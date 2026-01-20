use crate::commitment::CommitmentPolicy;
use crate::error::*;
use crate::suites::AlgorithmSuiteId;
use crate::*;
use async_trait::async_trait;

//= aws-encryption-sdk-specification/framework/cmm-interface.md#supported-cmms
//= type=implication
//# Note: A user MAY create their own custom CMM.

//= aws-encryption-sdk-specification/framework/cmm-interface.md#overview
//= type=implication
//# The CMM interface describes the interface that all CMMs MUST implement.

#[async_trait]
#[allow(private_bounds)]
pub trait CryptographicMaterialsManager: Send + Sync + Debug + MplPrivate {
    async fn get_encryption_materials(
        &self,
        input: &GetEncryptionMaterialsInput,
    ) -> Result<EncryptionMaterials, Error>;
    async fn decrypt_materials(
        &self,
        input: &DecryptMaterialsInput,
    ) -> Result<DecryptionMaterials, Error>;
}

pub type CryptographicMaterialsManagerRef = std::sync::Arc<dyn CryptographicMaterialsManager>;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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
    pub algorithm_suite_id: Option<AlgorithmSuiteId>,

    pub max_plaintext_length: Option<usize>,

    pub required_encryption_context_keys: Vec<String>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
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

///Creates a Default Cryptographic Materials Manager.
pub fn create_default_cryptographic_materials_manager(
    _keyring: KeyringRef,
) -> Result<CryptographicMaterialsManagerRef, Error> {
    not_implemented("create_default_cryptographic_materials_manager")
}

///Creates an Required Encryption Context Cryptographic Materials Manager.
pub fn create_required_encryption_context_cmm(
    input: &CreateRequiredEncryptionContextCMMInput,
) -> Result<CryptographicMaterialsManagerRef, Error> {
    input.validate()?;
    not_implemented("create_required_encryption_context_cmm")
}

///Inputs for creating an Required Encryption Context Cryptographic Materials Manager.
#[derive(Clone, Debug, Default)]
#[non_exhaustive]
pub struct CreateRequiredEncryptionContextCMMInput {
    ///The Cryptographic Materials Manager that the created Required Encryption Context Cryptographic Materials Manager will delegate to. Either a Keyring or underlying Cryptographic Materials Manager must be specified.
    pub underlying_cmm: Option<CryptographicMaterialsManagerRef>,
    ///The Keyring that the created Cryptographic Materials Manager will use to wrap data keys. The created Required Encryption Context CMM will delegate to a Default Cryptographic Materials Manager created with this Keyring. Either a Keyring or an underlying Cryptographic Materials Manager must be specified as input.
    pub keyring: Option<KeyringRef>,

    ///A list of Encryption Context keys which are required to be supplied during encryption and decryption, and correspond to Encryption Context key-value pairs which are not stored on the resulting message.
    pub required_encryption_context_keys: Vec<String>,
}

impl CreateRequiredEncryptionContextCMMInput {
    pub(crate) fn validate(&self) -> Result<(), Error> {
        if self.underlying_cmm.is_none() && self.keyring.is_none() {
            return Err(mpl_err(
                "Either a Keyring or underlying Cryptographic Materials Manager must be specified."
                    .to_string(),
            ));
        }
        if self.underlying_cmm.is_some() && self.keyring.is_some() {
            return Err(mpl_err(
                "Only one of Keyring or underlying Cryptographic Materials Manager must be specified."
                    .to_string(),
            ));
        }
        Ok(())
    }

    pub fn with_keyring<T: Into<String> + Clone>(keyring: KeyringRef, keys: &[T]) -> Self {
        Self {
            underlying_cmm: None,
            keyring: Some(keyring),
            required_encryption_context_keys: keys.iter().map(|k| k.clone().into()).collect(),
        }
    }
    pub fn with_cmm<T: Into<String> + Clone>(
        cmm: CryptographicMaterialsManagerRef,
        keys: &[T],
    ) -> Self {
        Self {
            underlying_cmm: Some(cmm),
            keyring: None,
            required_encryption_context_keys: keys.iter().map(|k| k.clone().into()).collect(),
        }
    }
    pub fn go(&self) -> Result<CryptographicMaterialsManagerRef, Error> {
        create_required_encryption_context_cmm(self)
    }
}
