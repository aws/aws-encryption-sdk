// use crate::serialize::header::*;
use crate::serialize::header_types::*;
use crate::serialize::serializable_types::*;
use crate::*;
use aws_mpl_rs::DecryptionMaterials;
use aws_mpl_rs::EncryptionMaterials;
#[cfg(feature = "legacy")]
use aws_mpl_rs::Secret;
use aws_mpl_rs::cmm::DecryptMaterialsInput;
use aws_mpl_rs::cmm::GetEncryptionMaterialsInput;
#[cfg(feature = "legacy")]
use aws_mpl_rs::suites::AlgorithmSuite;

pub(crate) enum Cmm {
    #[cfg(feature = "legacy")]
    Legacy(
        aws_mpl_legacy::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
    ),
    Modern(aws_mpl_rs::cmm::CryptographicMaterialsManagerRef),
}

#[allow(clippy::unused_async, reason = "without legacy, it's not async")]
pub(crate) async fn create_cmm_from_input(
    input_source: Option<MaterialSource>,
) -> Result<Cmm, Error> {
    match input_source.unwrap() {
        #[cfg(feature = "legacy")]
        MaterialSource::LegacyCmm(cmm) => Ok(Cmm::Legacy(cmm)),
        #[cfg(feature = "legacy")]
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
            let cmm = aws_mpl_rs::cmm::create_default_cryptographic_materials_manager(keyring)?;
            Ok(Cmm::Modern(cmm))
        }
    }
}

pub(crate) async fn get_decryption_materials(
    cmm: Cmm,
    algorithm_suite_id: aws_mpl_rs::suites::AlgorithmSuiteId,
    header_body: &HeaderBody,
    reproduced_encryption_context: &EncryptionContext,
    commitment_policy: aws_mpl_rs::commitment::EsdkCommitmentPolicy,
) -> Result<DecryptionMaterials, Error> {
    let materials = match cmm {
        #[cfg(feature = "legacy")]
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
    algorithm_suite_id: Option<aws_mpl_rs::suites::AlgorithmSuiteId>,
    encryption_context: EncryptionContext,
    max_plaintext_length: Option<usize>,
    commitment_policy: aws_mpl_rs::commitment::EsdkCommitmentPolicy,
) -> Result<EncryptionMaterials, Error> {
    let materials = match cmm {
        #[cfg(feature = "legacy")]
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
    cmm: aws_mpl_rs::cmm::CryptographicMaterialsManagerRef,
    algorithm_suite_id: aws_mpl_rs::suites::AlgorithmSuiteId,
    header_body: &HeaderBody,
    reproduced_encryption_context: &EncryptionContext,
    commitment_policy: aws_mpl_rs::commitment::EsdkCommitmentPolicy,
) -> Result<DecryptionMaterials, Error> {
    let encryption_context = from_canonical_pairs(header_body.encryption_context().clone());
    let mut input = DecryptMaterialsInput::default();
    //#*  Algorithm Suite ID: This is the parsed algorithm suite ID
    //#   (../data-format/message-header.md#algorithm-suite-id) from the
    //#   message header.
    input.algorithm_suite_id = algorithm_suite_id;
    input.commitment_policy = aws_mpl_rs::commitment::CommitmentPolicy::Esdk(commitment_policy);
    //#*  Encrypted Data Keys: This is the parsed encrypted data keys
    //#   (../data-format/message-header#encrypted-data-keys) from the
    //#   message header.
    input.encrypted_data_keys = header_body.encrypted_data_keys().into();
    //#*  Encryption Context: This is the parsed encryption context
    //#   (../data-format/message-header.md#aad) from the message header.
    input.encryption_context = encryption_context;
    //#* Reproduced Encryption Context: This is the
    //# [input](#input) encryption context.
    input
        .reproduced_encryption_context
        .clone_from(reproduced_encryption_context);
    let materials = cmm.decrypt_materials(&input).await?;
    aws_mpl_rs::commitment::validate_commitment_policy_on_decrypt(
        aws_mpl_rs::commitment::ValidateCommitmentPolicyOnDecryptInput::new(
            materials.algorithm_suite.id,
            aws_mpl_rs::commitment::CommitmentPolicy::Esdk(commitment_policy),
        ),
    )?;

    aws_mpl_rs::materials::decryption_materials_with_plaintext_data_key(&materials)?;
    Ok(materials)
}

#[cfg(feature = "legacy")]
fn convert_commit(
    x: aws_mpl_rs::commitment::EsdkCommitmentPolicy,
) -> aws_mpl_legacy::types::CommitmentPolicy {
    match x {
        aws_mpl_rs::commitment::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt => {
            aws_mpl_legacy::types::CommitmentPolicy::Esdk(
                aws_mpl_legacy::types::EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
            )
        }
        aws_mpl_rs::commitment::EsdkCommitmentPolicy::RequireEncryptAllowDecrypt => {
            aws_mpl_legacy::types::CommitmentPolicy::Esdk(
                aws_mpl_legacy::types::EsdkCommitmentPolicy::RequireEncryptAllowDecrypt,
            )
        }
        aws_mpl_rs::commitment::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt => {
            aws_mpl_legacy::types::CommitmentPolicy::Esdk(
                aws_mpl_legacy::types::EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt,
            )
        }
        _ => panic!(),
    }
}

#[cfg(feature = "legacy")]
fn convert_edk(x: &aws_mpl_rs::EncryptedDataKey) -> aws_mpl_legacy::types::EncryptedDataKey {
    aws_mpl_legacy::types::EncryptedDataKey::builder()
        .key_provider_id(x.key_provider_id.clone())
        .key_provider_info(x.key_provider_info.clone())
        .ciphertext(x.ciphertext.clone())
        .build()
        .unwrap()
}

#[cfg(feature = "legacy")]
fn convert_edks(
    x: &[aws_mpl_rs::EncryptedDataKey],
) -> Vec<aws_mpl_legacy::types::EncryptedDataKey> {
    x.iter().map(convert_edk).collect()
}

#[cfg(feature = "legacy")]
fn convert_esdk_alg(
    x: aws_mpl_rs::suites::EsdkAlgorithmSuiteId,
) -> aws_mpl_legacy::types::EsdkAlgorithmSuiteId {
    match x {
        aws_mpl_rs::suites::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16NoKdf => {
            aws_mpl_legacy::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16NoKdf
        }
        aws_mpl_rs::suites::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16NoKdf => {
            aws_mpl_legacy::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16NoKdf
        }
        aws_mpl_rs::suites::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf => {
            aws_mpl_legacy::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf
        }
        aws_mpl_rs::suites::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256 => {
            aws_mpl_legacy::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256
        }
        aws_mpl_rs::suites::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha256 => {
            aws_mpl_legacy::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha256
        }
        aws_mpl_rs::suites::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256 => {
            aws_mpl_legacy::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256
        }
        aws_mpl_rs::suites::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 => {
            aws_mpl_legacy::types::EsdkAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256
        }
        aws_mpl_rs::suites::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 => {
            aws_mpl_legacy::types::EsdkAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384
        }
        aws_mpl_rs::suites::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 => {
            aws_mpl_legacy::types::EsdkAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384
        }
        aws_mpl_rs::suites::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey => {
            aws_mpl_legacy::types::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey
        }
        aws_mpl_rs::suites::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 => {
            aws_mpl_legacy::types::EsdkAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384
        }
        _ => panic!(),
    }
}

#[cfg(feature = "legacy")]
fn convert_alg(x: aws_mpl_rs::suites::AlgorithmSuiteId) -> aws_mpl_legacy::types::AlgorithmSuiteId {
    match x {
        aws_mpl_rs::suites::AlgorithmSuiteId::Esdk(a) => {
            aws_mpl_legacy::types::AlgorithmSuiteId::Esdk(convert_esdk_alg(a))
        }
        _ => panic!(),
    }
}

#[cfg(feature = "legacy")]
fn from_legacy_aes(
    x: &aws_mpl_legacy::aws_cryptography_primitives::types::AesGcm,
) -> aws_mpl_primitives::AesGcm {
    match x.key_length().unwrap() {
        16 => aws_mpl_primitives::AesGcm::Aes128Gcm,
        24 => aws_mpl_primitives::AesGcm::Aes192Gcm,
        32 => aws_mpl_primitives::AesGcm::Aes256Gcm,
        _ => panic!(),
    }
}

#[cfg(feature = "legacy")]
fn from_legacy_encrypt(x: aws_mpl_legacy::types::Encrypt) -> aws_mpl_primitives::AesGcm {
    match x {
        aws_mpl_legacy::types::Encrypt::AesGcm(x) => from_legacy_aes(&x),
        _ => panic!(),
    }
}

#[cfg(feature = "legacy")]
const fn from_legacy_hmac(
    x: aws_mpl_legacy::aws_cryptography_primitives::types::DigestAlgorithm,
) -> aws_mpl_primitives::DigestAlg {
    use aws_mpl_legacy::aws_cryptography_primitives::types::DigestAlgorithm as Old;
    use aws_mpl_primitives::DigestAlg as New;
    match x {
        Old::Sha256 => New::Sha256,
        Old::Sha384 => New::Sha384,
        Old::Sha512 => New::Sha512,
    }
}

#[cfg(feature = "legacy")]
fn from_legacy_hkdf(x: &aws_mpl_legacy::types::Hkdf) -> aws_mpl_rs::suites::Hkdf {
    let mut n = aws_mpl_rs::suites::Hkdf::default();
    n.hmac = from_legacy_hmac(x.hmac.unwrap());
    n.salt_length = x.salt_length.unwrap() as u32;
    n.input_key_length = x.input_key_length.unwrap() as u32;
    n.output_key_length = x.output_key_length.unwrap() as u32;
    n
}

#[cfg(feature = "legacy")]
fn from_legacy_da(
    x: aws_mpl_legacy::types::DerivationAlgorithm,
) -> aws_mpl_rs::suites::DerivationAlgorithm {
    use aws_mpl_legacy::types::DerivationAlgorithm as Old;
    use aws_mpl_rs::suites::DerivationAlgorithm as New;
    match x {
        Old::Hkdf(x) => New::Hkdf(from_legacy_hkdf(&x)),
        Old::Identity(_x) => New::Identity,
        Old::None(_x) => New::None,
        _ => panic!(),
    }
}

#[cfg(feature = "legacy")]
const fn from_legacy_ecdsa(
    x: &aws_mpl_legacy::types::Ecdsa,
) -> aws_mpl_primitives::EcdsaSignatureAlgorithm {
    use aws_mpl_legacy::aws_cryptography_primitives::types::EcdsaSignatureAlgorithm as Old;
    use aws_mpl_primitives::EcdsaSignatureAlgorithm as New;
    match x.curve.unwrap() {
        Old::EcdsaP256 => New::EcdsaP256,
        Old::EcdsaP384 => New::EcdsaP384,
    }
}
#[cfg(feature = "legacy")]
fn from_legacy_sig(
    x: aws_mpl_legacy::types::SignatureAlgorithm,
) -> aws_mpl_rs::suites::SignatureAlgorithm {
    use aws_mpl_legacy::types::SignatureAlgorithm as Old;
    use aws_mpl_rs::suites::SignatureAlgorithm as New;
    match x {
        Old::Ecdsa(x) => New::Ecdsa(from_legacy_ecdsa(&x)),
        Old::None(_x) => New::None,
        _ => panic!(),
    }
}

#[cfg(feature = "legacy")]
fn from_legacy_ssig(
    x: aws_mpl_legacy::types::SymmetricSignatureAlgorithm,
) -> aws_mpl_rs::suites::SymmetricSignatureAlgorithm {
    use aws_mpl_legacy::types::SymmetricSignatureAlgorithm as Old;
    use aws_mpl_rs::suites::SymmetricSignatureAlgorithm as New;
    match x {
        Old::Hmac(x) => New::Hmac(from_legacy_hmac(x)),
        Old::None(_x) => New::None,
        _ => panic!(),
    }
}

#[cfg(feature = "legacy")]
fn from_legacy_inter_wrap(
    x: aws_mpl_legacy::types::IntermediateKeyWrapping,
) -> aws_mpl_rs::suites::IntermediateKeyWrapping {
    let mut r = aws_mpl_rs::suites::IntermediateKeyWrapping::default();
    r.key_encryption_key_kdf = from_legacy_da(x.key_encryption_key_kdf.unwrap());
    r.mac_key_kdf = from_legacy_da(x.mac_key_kdf.unwrap());
    r.pdk_encrypt_algorithm =
        aws_mpl_rs::suites::Encrypt::AesGcm(from_legacy_encrypt(x.pdk_encrypt_algorithm.unwrap()));
    r
}

#[cfg(feature = "legacy")]
fn from_legacy_wrap(
    x: aws_mpl_legacy::types::EdkWrappingAlgorithm,
) -> aws_mpl_rs::suites::EdkWrappingAlgorithm {
    use aws_mpl_legacy::types::EdkWrappingAlgorithm as Old;
    use aws_mpl_rs::suites::EdkWrappingAlgorithm as New;
    match x {
        Old::DirectKeyWrapping(_x) => New::DirectKeyWrapping,
        Old::IntermediateKeyWrapping(x) => New::IntermediateKeyWrapping(from_legacy_inter_wrap(x)),
        _ => panic!(),
    }
}
#[cfg(feature = "legacy")]
fn from_legacy_as(x: aws_mpl_legacy::types::AlgorithmSuiteInfo) -> Result<AlgorithmSuite, Error> {
    let mut s = AlgorithmSuite::default();
    s.id = legacy::from_legacy_suite_id(&x.id.unwrap())?;
    s.binary_id = x.binary_id.unwrap().as_ref()[0..2].try_into().unwrap();
    s.message_version = x.message_version.unwrap() as u32;
    s.encrypt = aws_mpl_rs::suites::Encrypt::AesGcm(from_legacy_encrypt(x.encrypt.unwrap()));
    s.kdf = from_legacy_da(x.kdf.unwrap());
    s.commitment = from_legacy_da(x.commitment.unwrap());
    s.signature = from_legacy_sig(x.signature.as_ref().unwrap().clone());
    s.symmetric_signature = from_legacy_ssig(x.symmetric_signature.unwrap());
    s.edk_wrapping = from_legacy_wrap(x.edk_wrapping.unwrap());
    Ok(s)
}
#[cfg(feature = "legacy")]
fn from_legacy_em(
    x: aws_mpl_legacy::types::EncryptionMaterials,
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
            aws_mpl_rs::EncryptedDataKey::new(
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

#[cfg(feature = "legacy")]
fn from_legacy_dm(
    x: aws_mpl_legacy::types::DecryptionMaterials,
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
#[cfg(feature = "legacy")]
pub(crate) async fn get_legacy_decryption_materials(
    cmm: aws_mpl_legacy::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
    algorithm_suite_id: aws_mpl_rs::suites::AlgorithmSuiteId,
    header_body: &HeaderBody,
    reproduced_encryption_context: &EncryptionContext,
    commitment_policy: aws_mpl_rs::commitment::EsdkCommitmentPolicy,
) -> Result<DecryptionMaterials, Error> {
    let encryption_context = from_canonical_pairs(header_body.encryption_context().clone());
    let output = cmm
        .decrypt_materials()
        //#*  Algorithm Suite ID: This is the parsed algorithm suite ID
        //#   (../data-format/message-header.md#algorithm-suite-id) from the
        //#   message header.
        .algorithm_suite_id(convert_alg(algorithm_suite_id))
        .commitment_policy(convert_commit(commitment_policy))
        //#*  Encrypted Data Keys: This is the parsed encrypted data keys
        //#   (../data-format/message-header#encrypted-data-keys) from the
        //#   message header.
        .encrypted_data_keys(convert_edks(header_body.encrypted_data_keys()))
        //#*  Encryption Context: This is the parsed encryption context
        //#   (../data-format/message-header.md#aad) from the message header.
        .encryption_context(encryption_context)
        //#* Reproduced Encryption Context: This is the
        //# [input](#input) encryption context.
        .reproduced_encryption_context(reproduced_encryption_context.clone())
        .send()
        .await?;

    let materials = output.decryption_materials.unwrap();
    let return_materials = materials.clone();
    let mpl = mpl();
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
    cmm: aws_mpl_rs::cmm::CryptographicMaterialsManagerRef,
    algorithm_suite_id: Option<aws_mpl_rs::suites::AlgorithmSuiteId>,
    encryption_context: EncryptionContext,
    max_plaintext_length: Option<usize>,
    commitment_policy: aws_mpl_rs::commitment::EsdkCommitmentPolicy,
) -> Result<EncryptionMaterials, Error> {
    let mut input = GetEncryptionMaterialsInput::default();
    input.algorithm_suite_id = algorithm_suite_id;
    input.commitment_policy = aws_mpl_rs::commitment::CommitmentPolicy::Esdk(commitment_policy);
    input.encryption_context = encryption_context;
    input.max_plaintext_length = max_plaintext_length;
    // input.required_encryption_context_keys = required_encryption_context_keys.clone();
    let materials = cmm.get_encryption_materials(&input).await?;

    aws_mpl_rs::commitment::validate_commitment_policy_on_encrypt(
        &aws_mpl_rs::commitment::ValidateCommitmentPolicyOnEncryptInput::new(
            materials.algorithm_suite.id,
            aws_mpl_rs::commitment::CommitmentPolicy::Esdk(commitment_policy),
        ),
    )?;

    aws_mpl_rs::materials::encryption_materials_has_plaintext_data_key(&materials)?;
    Ok(materials)
}

#[cfg(feature = "legacy")]
pub(crate) async fn get_legacy_encryption_materials(
    cmm: aws_mpl_legacy::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef,
    algorithm_suite_id: Option<aws_mpl_rs::suites::AlgorithmSuiteId>,
    encryption_context: EncryptionContext,
    max_plaintext_length: Option<usize>,
    commitment_policy: aws_mpl_rs::commitment::EsdkCommitmentPolicy,
) -> Result<EncryptionMaterials, Error> {
    let mpl = mpl();
    #[expect(
        clippy::cast_possible_wrap,
        reason = "max_plaintext_length is i64 in legacy mpl"
    )]
    let output = cmm
        .get_encryption_materials()
        .encryption_context(encryption_context)
        .commitment_policy(convert_commit(commitment_policy))
        .set_algorithm_suite_id(algorithm_suite_id.map(convert_alg))
        .set_max_plaintext_length(max_plaintext_length.map(|x| x as i64))
        .send()
        .await?;

    let materials = output.encryption_materials.unwrap();
    let return_materials = materials.clone();
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
