// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Translate the modeled CMM/Keyring configuration into Material Providers
//! Library keyrings and cryptographic materials managers via the generated
//! builder APIs.
//!
//! Enforces the "exactly one variant member set" invariant that the Smithy
//! tagged-union-via-optional-members shapes cannot express in the type system.
//! All failures here happen during `CreateClient` and map to
//! `GenericServerError`.

use crate::error::{describe_key_store_error, describe_mpl_error, ServerError};
use crate::model;
use aws_config::{BehaviorVersion, Region};
use aws_esdk::key_store::client::Client as KeyStoreClient;
use aws_esdk::key_store::types::key_store_config::KeyStoreConfig;
use aws_esdk::key_store::types::KmsConfiguration;
use aws_esdk::material_providers::client::Client as MplClient;
use aws_esdk::material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef;
use aws_esdk::material_providers::types::keyring::KeyringRef;
use aws_esdk::material_providers::types::material_providers_config::MaterialProvidersConfig;
use aws_esdk::material_providers::types::{
    AesWrappingAlg, CacheType, DefaultCache, DiscoveryFilter, PaddingScheme,
};
use aws_smithy_types::error::display::DisplayErrorContext;
use std::future::Future;
use std::pin::Pin;

/// Build the materials manager the registered client will encrypt/decrypt with.
pub async fn build_cmm(
    cmm: &model::CryptographicMaterialsManager,
) -> Result<CryptographicMaterialsManagerRef, ServerError> {
    let config = MaterialProvidersConfig::builder()
        .build()
        .map_err(|e| construction_failure("the MPL client", &e.to_string()))?;
    let mpl = MplClient::from_conf(config)
        .map_err(|e| construction_failure("the MPL client", &describe_mpl_error(&e)))?;
    cmm_for_config(&mpl, cmm).await
}

fn construction_failure(what: &str, detail: &str) -> ServerError {
    ServerError::generic(format!("failed to construct {what}: {detail}"))
}

fn cmm_for_config<'a>(
    mpl: &'a MplClient,
    cmm: &'a model::CryptographicMaterialsManager,
) -> Pin<Box<dyn Future<Output = Result<CryptographicMaterialsManagerRef, ServerError>> + Send + 'a>>
{
    Box::pin(async move {
        let set = [
            cmm.default_cmm.is_some(),
            cmm.required_encryption_context.is_some(),
            cmm.caching.is_some(),
        ]
        .iter()
        .filter(|present| **present)
        .count();
        if set != 1 {
            return Err(ServerError::generic(format!(
                "CryptographicMaterialsManager must set exactly one variant, found {set}"
            )));
        }

        if let Some(default_cmm) = &cmm.default_cmm {
            let keyring = keyring_for_config(mpl, &default_cmm.keyring).await?;
            return mpl
                .create_default_cryptographic_materials_manager()
                .keyring(keyring)
                .send()
                .await
                .map_err(|e| construction_failure("the Default CMM", &describe_mpl_error(&e)));
        }
        if let Some(required) = &cmm.required_encryption_context {
            let underlying = cmm_for_config(mpl, &required.underlying_cmm).await?;
            return mpl
                .create_required_encryption_context_cmm()
                .underlying_cmm(underlying)
                .required_encryption_context_keys(required.required_encryption_context_keys.clone())
                .send()
                .await
                .map_err(|e| {
                    construction_failure(
                        "the RequiredEncryptionContext CMM",
                        &describe_mpl_error(&e),
                    )
                });
        }
        Err(ServerError::generic(
            "the Caching CMM is not supported: the MPL provides no caching CMM",
        ))
    })
}

fn keyring_for_config<'a>(
    mpl: &'a MplClient,
    keyring: &'a model::Keyring,
) -> Pin<Box<dyn Future<Output = Result<KeyringRef, ServerError>> + Send + 'a>> {
    Box::pin(async move {
        let set = [
            keyring.aws_kms.is_some(),
            keyring.aws_kms_mrk.is_some(),
            keyring.aws_kms_multi.is_some(),
            keyring.aws_kms_mrk_multi.is_some(),
            keyring.aws_kms_discovery.is_some(),
            keyring.aws_kms_mrk_discovery.is_some(),
            keyring.aws_kms_rsa.is_some(),
            keyring.raw_aes.is_some(),
            keyring.raw_rsa.is_some(),
            keyring.aws_kms_hierarchical.is_some(),
            keyring.multi.is_some(),
        ]
        .iter()
        .filter(|present| **present)
        .count();
        if set != 1 {
            return Err(ServerError::generic(format!(
                "Keyring must set exactly one variant, found {set}"
            )));
        }

        if let Some(config) = &keyring.raw_aes {
            return build_raw_aes(mpl, config).await;
        }
        if let Some(config) = &keyring.raw_rsa {
            return build_raw_rsa(mpl, config).await;
        }
        if let Some(config) = &keyring.aws_kms {
            return build_aws_kms(mpl, config).await;
        }
        if let Some(config) = &keyring.aws_kms_mrk {
            return build_aws_kms_mrk(mpl, config).await;
        }
        if let Some(config) = &keyring.aws_kms_multi {
            return build_aws_kms_multi(mpl, config).await;
        }
        if let Some(config) = &keyring.aws_kms_mrk_multi {
            return build_aws_kms_mrk_multi(mpl, config).await;
        }
        if let Some(config) = &keyring.aws_kms_discovery {
            return build_aws_kms_discovery(mpl, config).await;
        }
        if let Some(config) = &keyring.aws_kms_mrk_discovery {
            return build_aws_kms_mrk_discovery(mpl, config).await;
        }
        if let Some(config) = &keyring.aws_kms_rsa {
            return build_aws_kms_rsa(mpl, config).await;
        }
        if let Some(config) = &keyring.aws_kms_hierarchical {
            return build_hierarchical(mpl, config).await;
        }
        let config = keyring.multi.as_ref().expect("one variant is set");
        build_multi(mpl, config).await
    })
}

/// The region KMS clients default to when the key carries none.
fn default_region() -> String {
    [
        "AWS_REGION",
        "AWS_DEFAULT_REGION",
        "ESDK_TESTSERVER_KMS_REGION",
    ]
    .iter()
    .find_map(|name| std::env::var(name).ok().filter(|v| !v.is_empty()))
    .unwrap_or_else(|| "us-west-2".to_owned())
}

/// The region embedded in a key ARN, falling back to the default region for a
/// bare key id or alias. KMS rejects an ARN whose region differs from the
/// client's region, so keyed keyrings need a client in the key's own region.
fn region_for_key(kms_key_id: &str) -> String {
    if let Some(rest) = kms_key_id.strip_prefix("arn:") {
        // arn:<partition>:<service>:<region>:...
        if let Some(region) = rest.split(':').nth(2).filter(|r| !r.is_empty()) {
            return region.to_owned();
        }
    }
    default_region()
}

/// No credentials are required here: nothing calls AWS until a keyring is used.
async fn sdk_config(region: String) -> aws_config::SdkConfig {
    aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(region))
        .load()
        .await
}

async fn kms_client(region: String) -> aws_sdk_kms::Client {
    aws_sdk_kms::Client::new(&sdk_config(region).await)
}

async fn build_raw_aes(
    mpl: &MplClient,
    config: &model::RawAesKeyringConfig,
) -> Result<KeyringRef, ServerError> {
    let wrapping_alg = match config.wrapping_alg {
        model::AesWrappingAlg::Alg128 => AesWrappingAlg::AlgAes128GcmIv12Tag16,
        model::AesWrappingAlg::Alg192 => AesWrappingAlg::AlgAes192GcmIv12Tag16,
        model::AesWrappingAlg::Alg256 => AesWrappingAlg::AlgAes256GcmIv12Tag16,
    };
    mpl.create_raw_aes_keyring()
        .key_namespace(config.key_namespace.clone())
        .key_name(config.key_name.clone())
        .wrapping_key(config.wrapping_key.clone())
        .wrapping_alg(wrapping_alg)
        .send()
        .await
        .map_err(|e| construction_failure("the RawAes keyring", &describe_mpl_error(&e)))
}

async fn build_raw_rsa(
    mpl: &MplClient,
    config: &model::RawRsaKeyringConfig,
) -> Result<KeyringRef, ServerError> {
    let padding_scheme = match config.padding_scheme {
        model::PaddingScheme::Pkcs1 => PaddingScheme::Pkcs1,
        model::PaddingScheme::OaepSha1Mgf1 => PaddingScheme::OaepSha1Mgf1,
        model::PaddingScheme::OaepSha256Mgf1 => PaddingScheme::OaepSha256Mgf1,
        model::PaddingScheme::OaepSha384Mgf1 => PaddingScheme::OaepSha384Mgf1,
        model::PaddingScheme::OaepSha512Mgf1 => PaddingScheme::OaepSha512Mgf1,
    };
    let mut builder = mpl
        .create_raw_rsa_keyring()
        .key_namespace(config.key_namespace.clone())
        .key_name(config.key_name.clone())
        .padding_scheme(padding_scheme);
    if let Some(public_key) = &config.public_key {
        builder = builder.public_key(public_key.clone());
    }
    if let Some(private_key) = &config.private_key {
        builder = builder.private_key(private_key.clone());
    }
    builder
        .send()
        .await
        .map_err(|e| construction_failure("the RawRsa keyring", &describe_mpl_error(&e)))
}

async fn build_aws_kms(
    mpl: &MplClient,
    config: &model::AwsKmsKeyringConfig,
) -> Result<KeyringRef, ServerError> {
    let client = kms_client(region_for_key(&config.kms_key_id)).await;
    let mut builder = mpl
        .create_aws_kms_keyring()
        .kms_key_id(config.kms_key_id.clone())
        .kms_client(client);
    if let Some(grant_tokens) = &config.grant_tokens {
        builder = builder.grant_tokens(grant_tokens.clone());
    }
    builder
        .send()
        .await
        .map_err(|e| construction_failure("the AwsKms keyring", &describe_mpl_error(&e)))
}

async fn build_aws_kms_mrk(
    mpl: &MplClient,
    config: &model::AwsKmsMrkKeyringConfig,
) -> Result<KeyringRef, ServerError> {
    let client = kms_client(region_for_key(&config.kms_key_id)).await;
    let mut builder = mpl
        .create_aws_kms_mrk_keyring()
        .kms_key_id(config.kms_key_id.clone())
        .kms_client(client);
    if let Some(grant_tokens) = &config.grant_tokens {
        builder = builder.grant_tokens(grant_tokens.clone());
    }
    builder
        .send()
        .await
        .map_err(|e| construction_failure("the AwsKmsMrk keyring", &describe_mpl_error(&e)))
}

// The KMS multi-keyrings take no client: the MPL builds one per key region.

async fn build_aws_kms_multi(
    mpl: &MplClient,
    config: &model::AwsKmsMultiKeyringConfig,
) -> Result<KeyringRef, ServerError> {
    let mut builder = mpl.create_aws_kms_multi_keyring();
    if let Some(generator) = &config.generator {
        builder = builder.generator(generator.clone());
    }
    if let Some(kms_key_ids) = &config.kms_key_ids {
        builder = builder.kms_key_ids(kms_key_ids.clone());
    }
    builder
        .send()
        .await
        .map_err(|e| construction_failure("the AwsKmsMultiKeyring", &describe_mpl_error(&e)))
}

async fn build_aws_kms_mrk_multi(
    mpl: &MplClient,
    config: &model::AwsKmsMrkMultiKeyringConfig,
) -> Result<KeyringRef, ServerError> {
    let mut builder = mpl.create_aws_kms_mrk_multi_keyring();
    if let Some(generator) = &config.generator {
        builder = builder.generator(generator.clone());
    }
    if let Some(kms_key_ids) = &config.kms_key_ids {
        builder = builder.kms_key_ids(kms_key_ids.clone());
    }
    builder
        .send()
        .await
        .map_err(|e| construction_failure("the AwsKmsMrkMultiKeyring", &describe_mpl_error(&e)))
}

fn discovery_filter(filter: &model::DiscoveryFilter) -> Result<DiscoveryFilter, ServerError> {
    DiscoveryFilter::builder()
        .partition(filter.partition.clone())
        .account_ids(filter.account_ids.clone())
        .build()
        .map_err(|e| construction_failure("the discovery filter", &e.to_string()))
}

async fn build_aws_kms_discovery(
    mpl: &MplClient,
    config: &model::AwsKmsDiscoveryKeyringConfig,
) -> Result<KeyringRef, ServerError> {
    let client = kms_client(default_region()).await;
    let mut builder = mpl.create_aws_kms_discovery_keyring().kms_client(client);
    if let Some(filter) = &config.discovery_filter {
        builder = builder.discovery_filter(discovery_filter(filter)?);
    }
    if let Some(grant_tokens) = &config.grant_tokens {
        builder = builder.grant_tokens(grant_tokens.clone());
    }
    builder
        .send()
        .await
        .map_err(|e| construction_failure("the AwsKmsDiscovery keyring", &describe_mpl_error(&e)))
}

async fn build_aws_kms_mrk_discovery(
    mpl: &MplClient,
    config: &model::AwsKmsMrkDiscoveryKeyringConfig,
) -> Result<KeyringRef, ServerError> {
    let client = kms_client(config.region.clone()).await;
    let mut builder = mpl
        .create_aws_kms_mrk_discovery_keyring()
        .kms_client(client)
        .region(config.region.clone());
    if let Some(filter) = &config.discovery_filter {
        builder = builder.discovery_filter(discovery_filter(filter)?);
    }
    if let Some(grant_tokens) = &config.grant_tokens {
        builder = builder.grant_tokens(grant_tokens.clone());
    }
    builder.send().await.map_err(|e| {
        construction_failure("the AwsKmsMrkDiscovery keyring", &describe_mpl_error(&e))
    })
}

async fn build_aws_kms_rsa(
    mpl: &MplClient,
    config: &model::AwsKmsRsaKeyringConfig,
) -> Result<KeyringRef, ServerError> {
    let client = kms_client(region_for_key(&config.kms_key_id)).await;
    let public_key = match &config.public_key {
        Some(bytes) => bytes.clone(),
        // The keyring needs the public key to OnEncrypt; fetch it from KMS.
        None => {
            let output = client
                .get_public_key()
                .key_id(config.kms_key_id.clone())
                .send()
                .await
                .map_err(|e| {
                    construction_failure(
                        "the AwsKmsRsa keyring",
                        &format!("KMS GetPublicKey failed: {}", DisplayErrorContext(&e)),
                    )
                })?;
            let der = output.public_key.ok_or_else(|| {
                construction_failure(
                    "the AwsKmsRsa keyring",
                    "KMS GetPublicKey returned no public key",
                )
            })?;
            der_to_public_key_pem(&der.into_inner())
        }
    };
    let mut builder = mpl
        .create_aws_kms_rsa_keyring()
        .kms_key_id(config.kms_key_id.clone())
        .public_key(aws_smithy_types::Blob::new(public_key))
        .kms_client(client);
    if let Some(algorithm) = config.encryption_algorithm {
        builder = builder.encryption_algorithm(match algorithm {
            model::KmsRsaEncryptionAlgorithm::OaepSha1 => {
                aws_sdk_kms::types::EncryptionAlgorithmSpec::RsaesOaepSha1
            }
            model::KmsRsaEncryptionAlgorithm::OaepSha256 => {
                aws_sdk_kms::types::EncryptionAlgorithmSpec::RsaesOaepSha256
            }
        });
    }
    if let Some(grant_tokens) = &config.grant_tokens {
        builder = builder.grant_tokens(grant_tokens.clone());
    }
    builder
        .send()
        .await
        .map_err(|e| construction_failure("the AwsKmsRsa keyring", &describe_mpl_error(&e)))
}

/// Wrap DER (X.509 SubjectPublicKeyInfo) bytes as a PEM `PUBLIC KEY` block, as
/// the keyring input expects. Pass through unchanged if the bytes already look
/// like PEM.
fn der_to_public_key_pem(bytes: &[u8]) -> Vec<u8> {
    if bytes.starts_with(b"-----BEGIN ") {
        return bytes.to_vec();
    }
    pem::encode(&pem::Pem::new("PUBLIC KEY", bytes)).into_bytes()
}

async fn build_hierarchical(
    mpl: &MplClient,
    config: &model::AwsKmsHierarchicalKeyringConfig,
) -> Result<KeyringRef, ServerError> {
    let sdk_config = sdk_config(default_region()).await;
    let key_store_config = KeyStoreConfig::builder()
        .ddb_client(aws_sdk_dynamodb::Client::new(&sdk_config))
        .kms_client(aws_sdk_kms::Client::new(&sdk_config))
        .ddb_table_name(config.key_store_table_name.clone())
        .logical_key_store_name(config.logical_key_store_name.clone())
        .kms_configuration(KmsConfiguration::KmsKeyArn(config.kms_key_arn.clone()))
        .build()
        .map_err(|e| construction_failure("the key store", &e.to_string()))?;
    let key_store = KeyStoreClient::from_conf(key_store_config)
        .map_err(|e| construction_failure("the key store", &describe_key_store_error(&e)))?;
    let cache = DefaultCache::builder()
        .entry_capacity(100)
        .build()
        .map_err(|e| construction_failure("the branch key cache", &e.to_string()))?;
    mpl.create_aws_kms_hierarchical_keyring()
        .key_store(key_store)
        .branch_key_id(config.branch_key_id.clone())
        .ttl_seconds(i64::from(config.ttl_seconds))
        .cache(CacheType::Default(cache))
        .send()
        .await
        .map_err(|e| {
            construction_failure("the AwsKmsHierarchical keyring", &describe_mpl_error(&e))
        })
}

async fn build_multi(
    mpl: &MplClient,
    config: &model::MultiKeyringConfig,
) -> Result<KeyringRef, ServerError> {
    let mut children = Vec::with_capacity(config.child_keyrings.len());
    for child in &config.child_keyrings {
        children.push(keyring_for_config(mpl, child).await?);
    }
    let mut builder = mpl.create_multi_keyring().child_keyrings(children);
    if let Some(generator) = &config.generator {
        builder = builder.generator(keyring_for_config(mpl, generator).await?);
    }
    builder
        .send()
        .await
        .map_err(|e| construction_failure("the Multi keyring", &describe_mpl_error(&e)))
}
