#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
/// Supported ECDH Key Agreement Schemes.
pub enum KeyAgreementScheme {
    StaticConfiguration(StaticConfigurations),
}
impl Default for KeyAgreementScheme {
    fn default() -> Self {
        Self::StaticConfiguration(StaticConfigurations::default())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
/// Supported configurations for the `StaticConfiguration` Key Agreement Scheme.
pub enum StaticConfigurations {
    AwsKmsEcdh(KmsEcdhStaticConfigurations),
    RawEcdh(RawEcdhStaticConfigurations),
}
impl Default for StaticConfigurations {
    fn default() -> Self {
        Self::RawEcdh(RawEcdhStaticConfigurations::default())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
/// Allowed configurations when using `KmsEcdhStaticConfigurations`.
pub enum KmsEcdhStaticConfigurations {
    KmsPublicKeyDiscovery(KmsPublicKeyDiscovery),
    KmsPrivateKeyToStaticPublicKey(KmsPrivateKeyToStaticPublicKey),
}
impl Default for KmsEcdhStaticConfigurations {
    fn default() -> Self {
        Self::KmsPublicKeyDiscovery(KmsPublicKeyDiscovery::default())
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
#[non_exhaustive]
/// List of configurations when using `RawEcdhStaticConfigurations`.
pub enum RawEcdhStaticConfigurations {
    PublicKeyDiscovery(PublicKeyDiscovery),
    RawPrivateKeyToStaticPublicKey(RawPrivateKeyToStaticPublicKey),
    EphemeralPrivateKeyToStaticPublicKey(EphemeralPrivateKeyToStaticPublicKey),
}
impl Default for RawEcdhStaticConfigurations {
    fn default() -> Self {
        Self::PublicKeyDiscovery(PublicKeyDiscovery::default())
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
/// Inputs for creating a `KmsPublicKeyDiscovery` Configuration. This is a DECRYPT ONLY configuration.
pub struct KmsPublicKeyDiscovery {
    /// AWS KMS key identifier belonging to the recipient.
    pub recipient_kms_identifier: String,
}
impl KmsPublicKeyDiscovery {
    /// Create a new `KmsPublicKeyDiscovery` Configuration.
    pub fn new(recipient_kms_identifier: impl Into<String>) -> Self {
        Self {
            recipient_kms_identifier: recipient_kms_identifier.into(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
/// Inputs for creating a `KmsPrivateKeyToStaticPublicKey` Configuration.
pub struct KmsPrivateKeyToStaticPublicKey {
    /// Recipient Public Key. This MUST be a raw public ECC key in DER format.
    pub recipient_public_key: Vec<u8>,
    /// AWS KMS Key Identifier belonging to the sender.
    pub sender_kms_identifier: String,
    /// Sender Public Key. This is the raw public ECC key in DER format that belongs to the senderKmsIdentifier.
    pub sender_public_key: Vec<u8>,
}

impl KmsPrivateKeyToStaticPublicKey {
    /// Create a new `KmsPrivateKeyToStaticPublicKey` Configuration.
    pub fn new(
        recipient_public_key: impl Into<Vec<u8>>,
        sender_public_key: impl Into<Vec<u8>>,
        sender_kms_identifier: impl Into<String>,
    ) -> Self {
        Self {
            recipient_public_key: recipient_public_key.into(),
            sender_kms_identifier: sender_kms_identifier.into(),
            sender_public_key: sender_public_key.into(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
/// Inputs for creating a `EphemeralPrivateKeyToStaticPublicKey` Configuration.
pub struct EphemeralPrivateKeyToStaticPublicKey {
    /// The recipient's public key. MUST be DER encoded.
    pub recipient_public_key: Vec<u8>,
}

impl EphemeralPrivateKeyToStaticPublicKey {
    /// Create a new `EphemeralPrivateKeyToStaticPublicKey` Configuration.
    pub fn new(recipient_public_key: impl Into<Vec<u8>>) -> Self {
        Self {
            recipient_public_key: recipient_public_key.into(),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
/// Inputs for creating a `PublicKeyDiscovery` Configuration.
pub struct PublicKeyDiscovery {
    /// The sender's private key. MUST be PEM encoded.
    pub recipient_static_private_key: Vec<u8>,
}
impl PublicKeyDiscovery {
    /// Create a new `PublicKeyDiscovery` Configuration.
    pub fn new(recipient_static_private_key: impl Into<Vec<u8>>) -> Self {
        Self {
            recipient_static_private_key: recipient_static_private_key.into(),
        }
    }
}
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
/// Inputs for creating a `RawPrivateKeyToStaticPublicKey` Configuration.
pub struct RawPrivateKeyToStaticPublicKey {
    /// The recipient's public key. MUST be DER encoded.
    pub recipient_public_key: Vec<u8>,
    /// The sender's private key. MUST be PEM encoded.
    pub sender_static_private_key: Vec<u8>,
}
impl RawPrivateKeyToStaticPublicKey {
    /// Create a new `RawPrivateKeyToStaticPublicKey` Configuration.
    pub fn new(
        recipient_public_key: impl Into<Vec<u8>>,
        sender_static_private_key: impl Into<Vec<u8>>,
    ) -> Self {
        Self {
            recipient_public_key: recipient_public_key.into(),
            sender_static_private_key: sender_static_private_key.into(),
        }
    }
}
