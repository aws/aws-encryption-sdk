// use crate::message::header::*;
use crate::message::header_types::*;
use crate::message::serializable_types::*;
use crate::*;
use aws_mpl_legacy::DecryptionMaterials;
use aws_mpl_legacy::EncryptionMaterials;
use aws_mpl_legacy::Secret;
use aws_mpl_legacy::cmm::DecryptMaterialsInput;
use aws_mpl_legacy::cmm::GetEncryptionMaterialsInput;
use aws_mpl_legacy::suites::AlgorithmSuite;

fn from_dafny_esdk_suite_id(
    legacy: aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId,
) -> aws_mpl_legacy::suites::EsdkAlgorithmSuiteId {
    use aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId as Old;
    use aws_mpl_legacy::suites::EsdkAlgorithmSuiteId as New;
    match legacy {
        Old::AlgAes128GcmIv12Tag16NoKdf => New::AlgAes128GcmIv12Tag16NoKdf,
        Old::AlgAes192GcmIv12Tag16NoKdf => New::AlgAes192GcmIv12Tag16NoKdf,
        Old::AlgAes256GcmIv12Tag16NoKdf => New::AlgAes256GcmIv12Tag16NoKdf,
        Old::AlgAes128GcmIv12Tag16HkdfSha256 => New::AlgAes128GcmIv12Tag16HkdfSha256,
        Old::AlgAes192GcmIv12Tag16HkdfSha256 => New::AlgAes192GcmIv12Tag16HkdfSha256,
        Old::AlgAes256GcmIv12Tag16HkdfSha256 => New::AlgAes256GcmIv12Tag16HkdfSha256,
        Old::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 => New::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256,
        Old::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 => New::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384,
        Old::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 => New::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384,
        Old::AlgAes256GcmHkdfSha512CommitKey => New::AlgAes256GcmHkdfSha512CommitKey,
        Old::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 => New::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384,
        _ => panic!("Unknown ESDK algorithm suite"),
    }
}

fn from_dafny_dbe_suite_id(
    legacy: aws_mpl_legacy::dafny::types::DbeAlgorithmSuiteId,
) -> aws_mpl_legacy::suites::DbeAlgorithmSuiteId {
    use aws_mpl_legacy::dafny::types::DbeAlgorithmSuiteId as Old;
    use aws_mpl_legacy::suites::DbeAlgorithmSuiteId as New;
    match legacy {
        Old::AlgAes256GcmHkdfSha512CommitKeySymsigHmacSha384 => New::AlgAes256GcmHkdfSha512CommitKeySymsigHmacSha384,
        Old::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384SymsigHmacSha384 => New::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384SymsigHmacSha384,
        _ => panic!("Unknown DBE algorithm suite"),
    }
}

fn from_dafny_suite_id(
    legacy: &aws_mpl_legacy::dafny::types::AlgorithmSuiteId,
) -> Result<aws_mpl_legacy::suites::AlgorithmSuiteId, Error> {
    use aws_mpl_legacy::dafny::types::AlgorithmSuiteId as Old;
    use aws_mpl_legacy::suites::AlgorithmSuiteId as New;
    match legacy {
        Old::Esdk(x) => Ok(New::Esdk(from_dafny_esdk_suite_id(*x))),
        Old::Dbe(x) => Ok(New::Dbe(from_dafny_dbe_suite_id(*x))),
        _ => Err(val_err("Unrecognized legacy AlgorithmSuiteId")),
    }
}

pub(crate) enum Cmm {
    Legacy(
        aws_mpl_legacy::dafny::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
    ),
    Modern(aws_mpl_legacy::cmm::CryptographicMaterialsManagerRef),
}

#[allow(clippy::unused_async)]
pub(crate) async fn create_cmm_from_input(
    input_source: Option<MaterialSource>,
) -> Result<Cmm, Error> {
    match input_source.unwrap() {
        MaterialSource::LegacyCmm(cmm) => Ok(Cmm::Legacy(cmm)),
        MaterialSource::LegacyKeyring(keyring) => {
            let mpl = mpl();
            let cmm = mpl
                .create_default_cryptographic_materials_manager()
                .keyring(keyring)
                .send()
                .await?;
            Ok(Cmm::Legacy(cmm))
        }
        MaterialSource::Cmm(cmm) => Ok(Cmm::Modern(cmm)),
        MaterialSource::Keyring(keyring) => {
            //= specification/client-apis/encrypt.md#get-the-encryption-materials
            //# If instead the caller supplied a [keyring](../framework/keyring-interface.md),
            //# this behavior MUST use a [default CMM](../framework/default-cmm.md)
            //# constructed using the caller-supplied keyring as input.
            //= aws-encryption-sdk-specification/client-apis/decrypt.md#keyring
            //# If the Keyring is provided as the input, the client MUST construct a [default CMM](../framework/default-cmm.md) that uses this keyring,
            //# to obtain the [decryption materials](../framework/structures.md#decryption-materials) that is required for decryption.
            //= aws-encryption-sdk-specification/client-apis/decrypt.md#keyring
            //= type=implication
            //= reason=The default CMM constructed above will obtain decryption materials when decrypt_materials is called on it
            //# This default CMM MUST obtain the decryption materials required for decryption.
            let cmm = aws_mpl_legacy::cmm::create_default_cryptographic_materials_manager(keyring)?;
            Ok(Cmm::Modern(cmm))
        }
    }
}

pub(crate) async fn get_decryption_materials(
    cmm: Cmm,
    algorithm_suite_id: aws_mpl_legacy::suites::AlgorithmSuiteId,
    header_body: &HeaderBody,
    reproduced_encryption_context: &EncryptionContext,
    commitment_policy: aws_mpl_legacy::commitment::EsdkCommitmentPolicy,
) -> Result<DecryptionMaterials, Error> {
    let materials = match cmm {
        Cmm::Legacy(cmm) => {
            get_legacy_decryption_materials(
                cmm,
                algorithm_suite_id,
                header_body,
                reproduced_encryption_context,
                commitment_policy,
            )
            .await?
        }
        Cmm::Modern(cmm) => {
            get_modern_decryption_materials(
                cmm,
                algorithm_suite_id,
                header_body,
                reproduced_encryption_context,
                commitment_policy,
            )
            .await?
        }
    };
    if !is_esdk_encryption_context(&materials.encryption_context) {
        return Err("CMM failed to return serializable encryption materials.".into());
    }
    Ok(materials)
}

pub(crate) async fn get_encryption_materials(
    cmm: Cmm,
    algorithm_suite_id: Option<aws_mpl_legacy::suites::AlgorithmSuiteId>,
    encryption_context: EncryptionContext,
    max_plaintext_length: Option<usize>,
    commitment_policy: aws_mpl_legacy::commitment::EsdkCommitmentPolicy,
) -> Result<EncryptionMaterials, Error> {
    let materials = match cmm {
        Cmm::Legacy(cmm) => {
            get_legacy_encryption_materials(
                cmm,
                algorithm_suite_id,
                encryption_context,
                max_plaintext_length,
                commitment_policy,
            )
            .await?
        }
        Cmm::Modern(cmm) => {
            get_modern_encryption_materials(
                cmm,
                algorithm_suite_id,
                encryption_context,
                max_plaintext_length,
                commitment_policy,
            )
            .await?
        }
    };
    if !is_esdk_encryption_context(&materials.encryption_context) {
        return Err("CMM failed to return serializable encryption materials.".into());
    }
    if !is_esdk_encrypted_data_keys(&materials.encrypted_data_keys) {
        return Err("CMM failed to return serializable encrypted data keys.".into());
    }
    Ok(materials)
}

pub(crate) async fn get_modern_decryption_materials(
    cmm: aws_mpl_legacy::cmm::CryptographicMaterialsManagerRef,
    algorithm_suite_id: aws_mpl_legacy::suites::AlgorithmSuiteId,
    header_body: &HeaderBody,
    reproduced_encryption_context: &EncryptionContext,
    commitment_policy: aws_mpl_legacy::commitment::EsdkCommitmentPolicy,
) -> Result<DecryptionMaterials, Error> {
    let encryption_context = from_canonical_pairs(header_body.encryption_context().clone());
    let mut input = DecryptMaterialsInput::default();
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //# The call to the CMM's [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) operation
    //# MUST be constructed as follows:
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //# - Algorithm Suite ID: This MUST be the parsed
    //# [algorithm suite ID](../data-format/message-header.md#algorithm-suite-id)
    //# from the message header.
    input.algorithm_suite_id = algorithm_suite_id;
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //# - Commitment Policy: This MUST be the commitment policy configured on the client.
    input.commitment_policy = aws_mpl_legacy::commitment::CommitmentPolicy::Esdk(commitment_policy);
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //# - Encrypted Data Keys: This MUST be the parsed [encrypted data keys](../data-format/message-header#encrypted-data-keys)
    //# from the message header.
    input.encrypted_data_keys = header_body.encrypted_data_keys().into();
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //# - Encryption Context: This MUST be the parsed [encryption context](../data-format/message-header.md#aad)
    //# from the message header.
    input.encryption_context = encryption_context;
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //# - Reproduced Encryption Context: This MUST be the [input](#input) encryption context.
    input
        .reproduced_encryption_context
        .clone_from(reproduced_encryption_context);
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //# This operation MUST obtain this set of [decryption materials](../framework/structures.md#decryption-materials),
    //# by calling [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) on a [CMM](../framework/cmm-interface.md).
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#cryptographic-materials-manager
    //# This CMM MUST obtain the [decryption materials](../framework/structures.md#decryption-materials) required for decryption.
    let materials = cmm.decrypt_materials(&input).await?;
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //# If this algorithm suite is not [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum)
    //# encrypt MUST yield an error.
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //# If the algorithm suite is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) decrypt MUST yield an error.
    aws_mpl_legacy::commitment::validate_commitment_policy_on_decrypt(
        aws_mpl_legacy::commitment::ValidateCommitmentPolicyOnDecryptInput::new(
            materials.algorithm_suite.id,
            aws_mpl_legacy::commitment::CommitmentPolicy::Esdk(commitment_policy),
        ),
    )?;

    aws_mpl_legacy::materials::decryption_materials_with_plaintext_data_key(&materials)?;
    Ok(materials)
}

fn convert_commit(
    x: aws_mpl_legacy::commitment::EsdkCommitmentPolicy,
) -> aws_mpl_legacy::dafny::types::CommitmentPolicy {
    match x {
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt => {
            aws_mpl_legacy::dafny::types::CommitmentPolicy::Esdk(
                aws_mpl_legacy::dafny::types::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
            )
        }
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::RequireEncryptAllowDecrypt => {
            aws_mpl_legacy::dafny::types::CommitmentPolicy::Esdk(
                aws_mpl_legacy::dafny::types::EsdkCommitmentPolicy::RequireEncryptAllowDecrypt,
            )
        }
        aws_mpl_legacy::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt => {
            aws_mpl_legacy::dafny::types::CommitmentPolicy::Esdk(
                aws_mpl_legacy::dafny::types::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
            )
        }
        _ => panic!(),
    }
}

fn convert_edk(x: &aws_mpl_legacy::EncryptedDataKey) -> aws_mpl_legacy::dafny::types::EncryptedDataKey {
    aws_mpl_legacy::dafny::types::EncryptedDataKey::builder()
        .key_provider_id(x.key_provider_id.clone())
        .key_provider_info(x.key_provider_info.clone())
        .ciphertext(x.ciphertext.clone())
        .build()
        .unwrap()
}

fn convert_edks(
    x: &[aws_mpl_legacy::EncryptedDataKey],
) -> Vec<aws_mpl_legacy::dafny::types::EncryptedDataKey> {
    x.iter().map(convert_edk).collect()
}

fn convert_esdk_alg(
    x: aws_mpl_legacy::suites::EsdkAlgorithmSuiteId,
) -> aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId {
    match x {
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16NoKdf => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16NoKdf
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16NoKdf => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16NoKdf
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256 => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha256 => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha256
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256 => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey
        }
        aws_mpl_legacy::suites::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 => {
            aws_mpl_legacy::dafny::types::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384
        }
        _ => panic!(),
    }
}

fn convert_alg(x: aws_mpl_legacy::suites::AlgorithmSuiteId) -> aws_mpl_legacy::dafny::types::AlgorithmSuiteId {
    match x {
        aws_mpl_legacy::suites::AlgorithmSuiteId::Esdk(a) => {
            aws_mpl_legacy::dafny::types::AlgorithmSuiteId::Esdk(convert_esdk_alg(a))
        }
        _ => panic!(),
    }
}

fn from_legacy_aes(
    x: &aws_mpl_legacy::dafny::aws_cryptography_primitives::types::AesGcm,
) -> aws_mpl_legacy::primitives::AesGcm {
    match x.key_length().unwrap() {
        16 => aws_mpl_legacy::primitives::AesGcm::Aes128Gcm,
        24 => aws_mpl_legacy::primitives::AesGcm::Aes192Gcm,
        32 => aws_mpl_legacy::primitives::AesGcm::Aes256Gcm,
        _ => panic!(),
    }
}

fn from_legacy_encrypt(x: aws_mpl_legacy::dafny::types::Encrypt) -> aws_mpl_legacy::primitives::AesGcm {
    match x {
        aws_mpl_legacy::dafny::types::Encrypt::AesGcm(x) => from_legacy_aes(&x),
        _ => panic!(),
    }
}

const fn from_legacy_hmac(
    x: aws_mpl_legacy::dafny::aws_cryptography_primitives::types::DigestAlgorithm,
) -> aws_mpl_legacy::primitives::DigestAlg {
    use aws_mpl_legacy::dafny::aws_cryptography_primitives::types::DigestAlgorithm as Old;
    use aws_mpl_legacy::primitives::DigestAlg as New;
    match x {
        Old::Sha256 => New::Sha256,
        Old::Sha384 => New::Sha384,
        Old::Sha512 => New::Sha512,
    }
}

fn from_legacy_hkdf(x: &aws_mpl_legacy::dafny::types::Hkdf) -> aws_mpl_legacy::suites::Hkdf {
    let mut n = aws_mpl_legacy::suites::Hkdf::default();
    n.hmac = from_legacy_hmac(x.hmac.unwrap());
    n.salt_length = x.salt_length.unwrap() as u32;
    n.input_key_length = x.input_key_length.unwrap() as u32;
    n.output_key_length = x.output_key_length.unwrap() as u32;
    n
}

fn from_legacy_da(
    x: aws_mpl_legacy::dafny::types::DerivationAlgorithm,
) -> aws_mpl_legacy::suites::DerivationAlgorithm {
    use aws_mpl_legacy::dafny::types::DerivationAlgorithm as Old;
    use aws_mpl_legacy::suites::DerivationAlgorithm as New;
    match x {
        Old::Hkdf(x) => New::Hkdf(from_legacy_hkdf(&x)),
        Old::Identity(_x) => New::Identity,
        Old::None(_x) => New::None,
        _ => panic!(),
    }
}

const fn from_legacy_ecdsa(
    x: &aws_mpl_legacy::dafny::types::Ecdsa,
) -> aws_mpl_legacy::primitives::EcdsaSignatureAlgorithm {
    use aws_mpl_legacy::dafny::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm as Old;
    use aws_mpl_legacy::primitives::EcdsaSignatureAlgorithm as New;
    match x.curve.unwrap() {
        Old::EcdsaP256 => New::EcdsaP256,
        Old::EcdsaP384 => New::EcdsaP384,
    }
}
fn from_legacy_sig(
    x: aws_mpl_legacy::dafny::types::SignatureAlgorithm,
) -> aws_mpl_legacy::suites::SignatureAlgorithm {
    use aws_mpl_legacy::dafny::types::SignatureAlgorithm as Old;
    use aws_mpl_legacy::suites::SignatureAlgorithm as New;
    match x {
        Old::Ecdsa(x) => New::Ecdsa(from_legacy_ecdsa(&x)),
        Old::None(_x) => New::None,
        _ => panic!(),
    }
}

fn from_legacy_ssig(
    x: aws_mpl_legacy::dafny::types::SymmetricSignatureAlgorithm,
) -> aws_mpl_legacy::suites::SymmetricSignatureAlgorithm {
    use aws_mpl_legacy::dafny::types::SymmetricSignatureAlgorithm as Old;
    use aws_mpl_legacy::suites::SymmetricSignatureAlgorithm as New;
    match x {
        Old::Hmac(x) => New::Hmac(from_legacy_hmac(x)),
        Old::None(_x) => New::None,
        _ => panic!(),
    }
}

fn from_legacy_inter_wrap(
    x: aws_mpl_legacy::dafny::types::IntermediateKeyWrapping,
) -> aws_mpl_legacy::suites::IntermediateKeyWrapping {
    let mut r = aws_mpl_legacy::suites::IntermediateKeyWrapping::default();
    r.key_encryption_key_kdf = from_legacy_da(x.key_encryption_key_kdf.unwrap());
    r.mac_key_kdf = from_legacy_da(x.mac_key_kdf.unwrap());
    r.pdk_encrypt_algorithm =
        aws_mpl_legacy::suites::Encrypt::AesGcm(from_legacy_encrypt(x.pdk_encrypt_algorithm.unwrap()));
    r
}

fn from_legacy_wrap(
    x: aws_mpl_legacy::dafny::types::EdkWrappingAlgorithm,
) -> aws_mpl_legacy::suites::EdkWrappingAlgorithm {
    use aws_mpl_legacy::dafny::types::EdkWrappingAlgorithm as Old;
    use aws_mpl_legacy::suites::EdkWrappingAlgorithm as New;
    match x {
        Old::DirectKeyWrapping(_x) => New::DirectKeyWrapping,
        Old::IntermediateKeyWrapping(x) => New::IntermediateKeyWrapping(from_legacy_inter_wrap(x)),
        _ => panic!(),
    }
}
fn from_legacy_as(x: aws_mpl_legacy::dafny::types::AlgorithmSuiteInfo) -> Result<AlgorithmSuite, Error> {
    let mut s = AlgorithmSuite::default();
    s.id = from_dafny_suite_id(&x.id.unwrap())?;
    s.binary_id = x.binary_id.unwrap().as_ref()[0..2].try_into().unwrap();
    s.message_version = x.message_version.unwrap() as u32;
    s.encrypt = aws_mpl_legacy::suites::Encrypt::AesGcm(from_legacy_encrypt(x.encrypt.unwrap()));
    s.kdf = from_legacy_da(x.kdf.unwrap());
    s.commitment = from_legacy_da(x.commitment.unwrap());
    s.signature = from_legacy_sig(x.signature.as_ref().unwrap().clone());
    s.symmetric_signature = from_legacy_ssig(x.symmetric_signature.unwrap());
    s.edk_wrapping = from_legacy_wrap(x.edk_wrapping.unwrap());
    Ok(s)
}
fn from_legacy_em(
    x: aws_mpl_legacy::dafny::types::EncryptionMaterials,
) -> Result<EncryptionMaterials, Error> {
    let mut ret = EncryptionMaterials::default();
    ret.symmetric_signing_keys = match x.symmetric_signing_keys.as_ref() {
        None => vec![],
        Some(x) => x.iter().map(|x| Secret(x.clone().into_inner())).collect(),
    };
    ret.encrypted_data_keys = x
        .encrypted_data_keys()
        .as_ref()
        .unwrap()
        .iter()
        .map(|x| {
            aws_mpl_legacy::EncryptedDataKey::new(
                x.key_provider_id().as_ref().unwrap().clone(),
                x.key_provider_info().as_ref().unwrap().clone(),
                x.ciphertext().as_ref().unwrap().clone(),
            )
        })
        .collect();
    ret.algorithm_suite = from_legacy_as(x.algorithm_suite.unwrap())?;
    ret.encryption_context = x.encryption_context.unwrap();
    ret.required_encryption_context_keys = x.required_encryption_context_keys.unwrap();
    ret.plaintext_data_key = x.plaintext_data_key.map(|y| Secret(y.into_inner()));
    ret.signing_key = x.signing_key.map(|y| Secret(y.into_inner()));
    Ok(ret)
}

fn from_legacy_dm(
    x: aws_mpl_legacy::dafny::types::DecryptionMaterials,
) -> Result<DecryptionMaterials, Error> {
    let mut ret = DecryptionMaterials::default();
    ret.algorithm_suite = from_legacy_as(x.algorithm_suite.unwrap())?;
    ret.encryption_context = x.encryption_context.unwrap();
    ret.required_encryption_context_keys = x.required_encryption_context_keys.unwrap();

    ret.plaintext_data_key = x.plaintext_data_key.map(|y| Secret(y.into_inner()));
    ret.verification_key = x.verification_key.map(|y| Secret(y.into_inner()));
    ret.symmetric_signing_key = x.symmetric_signing_key.map(|y| Secret(y.into_inner()));
    Ok(ret)
}
pub(crate) async fn get_legacy_decryption_materials(
    cmm: aws_mpl_legacy::dafny::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
    algorithm_suite_id: aws_mpl_legacy::suites::AlgorithmSuiteId,
    header_body: &HeaderBody,
    reproduced_encryption_context: &EncryptionContext,
    commitment_policy: aws_mpl_legacy::commitment::EsdkCommitmentPolicy,
) -> Result<DecryptionMaterials, Error> {
    let encryption_context = from_canonical_pairs(header_body.encryption_context().clone());

    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //# This operation MUST obtain this set of [decryption materials](../framework/structures.md#decryption-materials),
    //# by calling [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) on a [CMM](../framework/cmm-interface.md).
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#cryptographic-materials-manager
    //# This CMM MUST obtain the [decryption materials](../framework/structures.md#decryption-materials) required for decryption.
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //# The call to the CMM's [Decrypt Materials](../framework/cmm-interface.md#decrypt-materials) operation
    //# MUST be constructed as follows:
    let output = cmm
        .decrypt_materials()
        //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
        //# - Algorithm Suite ID: This MUST be the parsed
        //# [algorithm suite ID](../data-format/message-header.md#algorithm-suite-id)
        //# from the message header.
        .algorithm_suite_id(convert_alg(algorithm_suite_id))
        //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
        //# - Commitment Policy: This MUST be the commitment policy configured on the client.
        .commitment_policy(convert_commit(commitment_policy))
        //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
        //# - Encrypted Data Keys: This MUST be the parsed [encrypted data keys](../data-format/message-header#encrypted-data-keys)
        //# from the message header.
        .encrypted_data_keys(convert_edks(header_body.encrypted_data_keys()))
        //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
        //# - Encryption Context: This MUST be the parsed [encryption context](../data-format/message-header.md#aad)
        //# from the message header.
        .encryption_context(encryption_context)
        //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
        //# - Reproduced Encryption Context: This MUST be the [input](#input) encryption context.
        .reproduced_encryption_context(reproduced_encryption_context.clone())
        .send()
        .await?;

    let materials = output.decryption_materials.unwrap();
    let return_materials = materials.clone();
    let mpl = mpl();
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //# If this algorithm suite is not [supported for the ESDK](../framework/algorithm-suites.md#supported-algorithm-suites-enum)
    //# encrypt MUST yield an error.
    //= aws-encryption-sdk-specification/client-apis/decrypt.md#get-the-decryption-materials
    //# If the algorithm suite is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) decrypt MUST yield an error.
    mpl.validate_commitment_policy_on_decrypt()
        .algorithm(
            materials
                .algorithm_suite
                .as_ref()
                .unwrap()
                .id()
                .as_ref()
                .unwrap()
                .clone(),
        )
        .commitment_policy(convert_commit(commitment_policy))
        .send()
        .await?;

    mpl.decryption_materials_with_plaintext_data_key()
        .set_algorithm_suite(materials.algorithm_suite.clone())
        .set_encryption_context(materials.encryption_context)
        .set_plaintext_data_key(materials.plaintext_data_key)
        .set_required_encryption_context_keys(materials.required_encryption_context_keys)
        .set_symmetric_signing_key(materials.symmetric_signing_key)
        .set_verification_key(materials.verification_key)
        .send()
        .await?;
    from_legacy_dm(return_materials)
}

/*
    // pub encryption_context: EncryptionContext,
    // pub commitment_policy: CommitmentPolicy,
    // pub algorithm_suite_id: Option<AlgorithmSuiteId>,
    // pub max_plaintext_length: Option<u64>,
    pub required_encryption_context_keys: Vec<EncryptionContextKey>,
*/
pub(crate) async fn get_modern_encryption_materials(
    cmm: aws_mpl_legacy::cmm::CryptographicMaterialsManagerRef,
    algorithm_suite_id: Option<aws_mpl_legacy::suites::AlgorithmSuiteId>,
    encryption_context: EncryptionContext,
    max_plaintext_length: Option<usize>,
    commitment_policy: aws_mpl_legacy::commitment::EsdkCommitmentPolicy,
) -> Result<EncryptionMaterials, Error> {
    let mut input = GetEncryptionMaterialsInput::default();
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= reason=algorithm_suite_id is Option; None means the field is not set on the input
    //# If no Algorithm Suite is provided, this field MUST NOT be included.
    input.algorithm_suite_id = algorithm_suite_id;
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# - Commitment Policy: This MUST be the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) exposing this encrypt function.
    input.commitment_policy = aws_mpl_legacy::commitment::CommitmentPolicy::Esdk(commitment_policy);
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# - Encryption Context: If provided, this MUST be the [input encryption context](#encryption-context).
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= reason=Encryption context is empty by default
    //# Otherwise, this MUST be an empty encryption context.
    input.encryption_context = encryption_context;
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# - Max Plaintext Length: If the [input plaintext](#plaintext) has known length,
    //# this length MUST be used.
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //= reason=max_plaintext_length is Option; None means the field is not set on the input
    //# If no Plaintext Length Bound is provided, this field MUST NOT be included.
    input.max_plaintext_length = max_plaintext_length;
    // input.required_encryption_context_keys = required_encryption_context_keys.clone();
    let materials = cmm.get_encryption_materials(&input).await?;

    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# If this [algorithm suite](../framework/algorithm-suites.md) is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) encrypt MUST yield an error.
    aws_mpl_legacy::commitment::validate_commitment_policy_on_encrypt(
        &aws_mpl_legacy::commitment::ValidateCommitmentPolicyOnEncryptInput::new(
            materials.algorithm_suite.id,
            aws_mpl_legacy::commitment::CommitmentPolicy::Esdk(commitment_policy),
        ),
    )?;

    aws_mpl_legacy::materials::encryption_materials_has_plaintext_data_key(&materials)?;
    Ok(materials)
}

pub(crate) async fn get_legacy_encryption_materials(
    cmm: aws_mpl_legacy::dafny::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
    algorithm_suite_id: Option<aws_mpl_legacy::suites::AlgorithmSuiteId>,
    encryption_context: EncryptionContext,
    max_plaintext_length: Option<usize>,
    commitment_policy: aws_mpl_legacy::commitment::EsdkCommitmentPolicy,
) -> Result<EncryptionMaterials, Error> {
    let mpl = mpl();
    #[expect(
        clippy::cast_possible_wrap,
        reason = "max_plaintext_length is i64 in legacy mpl"
    )]
    let output = cmm
        .get_encryption_materials()
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //= type=implication
        //= reason=The caller passes an empty EncryptionContext when none is provided as input
        //# Otherwise, this MUST be an empty encryption context.
        .encryption_context(encryption_context)
        .commitment_policy(convert_commit(commitment_policy))
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //= type=implication
        //= reason=algorithm_suite_id is Option; .set_ with None means the field is not set
        //# If no Algorithm Suite is provided, this field MUST NOT be included.
        .set_algorithm_suite_id(algorithm_suite_id.map(convert_alg))
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //= type=implication
        //= reason=The caller resolves known length vs Plaintext Length Bound before calling; this receives the resolved value
        //# If the input [plaintext](#plaintext) has unknown length and a [Plaintext Length Bound](#plaintext-length-bound)
        //# was provided, this MUST be the [Plaintext Length Bound](#plaintext-length-bound).
        //= specification/client-apis/encrypt.md#get-the-encryption-materials
        //= type=implication
        //= reason=max_plaintext_length is Option; .set_ with None means the field is not set
        //# If no Plaintext Length Bound is provided, this field MUST NOT be included.
        .set_max_plaintext_length(max_plaintext_length.map(|x| x as i64))
        .send()
        .await?;

    let materials = output.encryption_materials.unwrap();
    let return_materials = materials.clone();
    //= specification/client-apis/encrypt.md#get-the-encryption-materials
    //# If this [algorithm suite](../framework/algorithm-suites.md) is not supported by the [commitment policy](client.md#commitment-policy)
    //# configured in the [client](client.md) encrypt MUST yield an error.
    mpl.validate_commitment_policy_on_encrypt()
        .algorithm(
            materials
                .algorithm_suite
                .as_ref()
                .unwrap()
                .id
                .as_ref()
                .unwrap()
                .clone(),
        )
        .commitment_policy(convert_commit(commitment_policy))
        .send()
        .await?;

    mpl.encryption_materials_has_plaintext_data_key()
        .set_algorithm_suite(materials.algorithm_suite.clone())
        .set_encryption_context(materials.encryption_context)
        .set_encrypted_data_keys(materials.encrypted_data_keys)
        .set_required_encryption_context_keys(materials.required_encryption_context_keys)
        .set_plaintext_data_key(materials.plaintext_data_key)
        .set_signing_key(materials.signing_key)
        .set_symmetric_signing_keys(materials.symmetric_signing_keys)
        .send()
        .await?;

    from_legacy_em(return_materials)
}
