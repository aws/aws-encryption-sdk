use crate::error::*;
use crate::key_agreement::{RawEcdhStaticConfigurations};
use crate::types::*;
use async_trait::async_trait;
use aws_mpl_primitives::EcdhCurveSpec;

#[async_trait]
#[allow(private_bounds)]
pub trait Keyring: Send + Sync + std::fmt::Debug + crate::MplPrivate {
    async fn on_encrypt(&self, input: &OnEncryptInput) -> Result<OnEncryptOutput, Error>;
    async fn on_decrypt(&self, input: &OnDecryptInput) -> Result<OnDecryptOutput, Error>;
}

pub type KeyringRef = std::sync::Arc<dyn Keyring>;
pub type KeyringList = Vec<KeyringRef>;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct OnEncryptInput {
    pub materials: EncryptionMaterials,
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct OnEncryptOutput {
    pub materials: EncryptionMaterials,
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
//= aws-encryption-sdk-specification/framework/keyring-interface.md#ondecrypt
//= type=implication
//# This interface MUST take [decryption materials](structs.md#decryption-materials) and
//# a list of [encrypted data keys](structs.md#encrypted-data-key) as input.
pub struct OnDecryptInput {
    pub materials: DecryptionMaterials,
    pub encrypted_data_keys: Vec<EncryptedDataKey>,
}
#[derive(Clone, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub struct OnDecryptOutput {
    pub materials: DecryptionMaterials,
}

///Creates a Multi-Keyring comprised of one or more other Keyrings.
pub fn create_multi_keyring(_input: &CreateMultiKeyringInput) -> Result<KeyringRef, Error> {
    not_implemented("create_multi_keyring")
}

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
///Inputs for creating a Multi-Keyring.
pub struct CreateMultiKeyringInput {
    ///A keyring responsible for wrapping and unwrapping the data key. This is the first keyring that will be used to wrap the data key, and may be responsible for additionally generating the data key.
    pub generator: Option<KeyringRef>,

    // We'll represent "no children" as an empty list
    ///A list of keyrings (other than the generator) responsible for wrapping and unwrapping the data key.
    pub child_keyrings: KeyringList,
}

impl CreateMultiKeyringInput {
    pub fn new(generator: KeyringRef, child_keyrings: impl Into<KeyringList>) -> Self {
        Self {
            generator: Some(generator),
            child_keyrings: child_keyrings.into(),
        }
    }

    pub fn go(&self) -> Result<KeyringRef, Error> {
        create_multi_keyring(self)
    }
}

// Raw

///Creates a Raw AES Keyring, which wraps and unwraps data keys locally using `AES_GCM`.
pub fn create_raw_aes_keyring(_input: &CreateRawAesKeyringInput) -> Result<KeyringRef, Error> {
    not_implemented("create_raw_aes_keyring")
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
///Inputs for creating a Raw AES Keyring.
pub struct CreateRawAesKeyringInput {
    ///A namespace associated with this wrapping key.
    pub key_namespace: String,

    ///A name associated with this wrapping key.
    pub key_name: String,

    ///The AES key used with `AES_GCM` encryption and decryption.
    pub wrapping_key: Vec<u8>,

    ///The `AES_GCM` algorithm this Keyring uses to wrap and unwrap data keys.
    pub wrapping_alg: AesWrappingAlg,
}

impl CreateRawAesKeyringInput {
    pub fn new(
        key_namespace: impl Into<String>,
        key_name: impl Into<String>,
        wrapping_key: impl Into<Vec<u8>>,
        wrapping_alg: AesWrappingAlg,
    ) -> Self {
        Self {
            key_namespace: key_namespace.into(),
            key_name: key_name.into(),
            wrapping_key: wrapping_key.into(),
            wrapping_alg,
        }
    }

    pub fn go(&self) -> Result<KeyringRef, Error> {
        create_raw_aes_keyring(self)
    }
}

///Creates a Raw RSA Keyring, which wraps and unwraps data keys locally using RSA.
pub fn create_raw_rsa_keyring(_input: &CreateRawRsaKeyringInput) -> Result<KeyringRef, Error> {
    not_implemented("create_raw_rsa_keyring")
}

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
///Inputs for creating a Raw RAW Keyring.
pub struct CreateRawRsaKeyringInput {
    ///A namespace associated with this wrapping key.
    pub key_namespace: String,

    ///A name associated with this wrapping key.
    pub key_name: String,

    ///The RSA padding scheme to use with this keyring.
    pub padding_scheme: PaddingScheme,

    // One or both is required
    ///The public RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded X.509 `SubjectPublicKeyInfo` struct. If not specified, this Keyring cannot be used on encrypt. A public key and/or a private key must be specified.
    pub public_key: Vec<u8>,
    ///The private RSA Key responsible for wrapping data keys, as a UTF8 encoded, PEM encoded PKCS #8 `PrivateKeyInfo` struct. If not specified, this Keyring cannot be used on decrypt. A public key and/or a private key must be specified.
    pub private_key: Vec<u8>,
}

impl CreateRawRsaKeyringInput {
    pub fn new(
        key_namespace: impl Into<String>,
        key_name: impl Into<String>,
        padding_scheme: PaddingScheme,
    ) -> Self {
        Self {
            key_namespace: key_namespace.into(),
            key_name: key_name.into(),
            padding_scheme,
            ..Default::default()
        }
    }

    pub fn go(&self) -> Result<KeyringRef, Error> {
        create_raw_rsa_keyring(self)
    }
}

///Creates a Raw ECDH Keyring, which wraps and unwraps data keys by deriving a shared data key from the established shared secret between parties through the ECDH protocol.
pub fn create_raw_ecdh_keyring(_input: &CreateRawEcdhKeyringInput) -> Result<KeyringRef, Error> {
    not_implemented("create_raw_ecdh_keyring")
}

#[derive(Clone, Debug, Default)]
#[non_exhaustive]
///Inputs for creating a raw ECDH Keyring.
pub struct CreateRawEcdhKeyringInput {
    ///The Key Agreement Scheme configuration that is responsible for how the shared secret is calculated.
    pub key_agreement_scheme: RawEcdhStaticConfigurations,

    ///The the curve on which the points for the sender's private and recipient's public key lie.
    pub curve_spec: EcdhCurveSpec,
}

impl CreateRawEcdhKeyringInput {
    #[must_use]
    pub const fn new(
        key_agreement_scheme: RawEcdhStaticConfigurations,
        curve_spec: EcdhCurveSpec,
    ) -> Self {
        Self {
            key_agreement_scheme,
            curve_spec,
        }
    }

    pub fn go(&self) -> Result<KeyringRef, Error> {
        create_raw_ecdh_keyring(self)
    }
}
