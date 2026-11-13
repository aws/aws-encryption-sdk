// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Operation handlers and the `clientId` registry.
//!
//! `CreateClient` eagerly builds the materials manager from the modeled config
//! (delegating to the MPL) and the real ESDK client, registers both under a
//! fresh UUID, and returns the id. `Encrypt` and `Decrypt` resolve the entry by
//! `clientId` and drive the ESDK client through its materials manager.

use crate::error::{describe_esdk_error, ServerError};
use crate::keyring::build_cmm;
use crate::model::{
    CreateClientRequest, CreateClientResponse, DecryptRequest, DecryptResponse, EncryptRequest,
    EncryptResponse, EsdkAlgorithmSuiteId, EsdkCommitmentPolicy,
};
use aws_esdk::client::Client as EsdkClient;
use aws_esdk::material_providers::types::cryptographic_materials_manager::CryptographicMaterialsManagerRef;
use aws_esdk::material_providers::types::EsdkAlgorithmSuiteId as MplAlgorithmSuiteId;
use aws_esdk::material_providers::types::EsdkCommitmentPolicy as MplCommitmentPolicy;
use aws_esdk::AwsEncryptionSdkConfig;
use aws_smithy_types::Blob;
use std::collections::{BTreeMap, HashMap};
use std::sync::Mutex;

/// A registered client: the configured ESDK client and its materials manager.
#[derive(Clone)]
struct ClientEntry {
    esdk: EsdkClient,
    cmm: CryptographicMaterialsManagerRef,
}

/// Shared server state: the thread-safe `clientId -> ClientEntry` registry.
#[derive(Default)]
pub struct AppState {
    clients: Mutex<HashMap<String, ClientEntry>>,
}

fn map_commitment(policy: EsdkCommitmentPolicy) -> MplCommitmentPolicy {
    match policy {
        EsdkCommitmentPolicy::ForbidEncryptAllowDecrypt => {
            MplCommitmentPolicy::ForbidEncryptAllowDecrypt
        }
        EsdkCommitmentPolicy::RequireEncryptAllowDecrypt => {
            MplCommitmentPolicy::RequireEncryptAllowDecrypt
        }
        EsdkCommitmentPolicy::RequireEncryptRequireDecrypt => {
            MplCommitmentPolicy::RequireEncryptRequireDecrypt
        }
    }
}

fn to_mpl_suite(id: EsdkAlgorithmSuiteId) -> MplAlgorithmSuiteId {
    match id {
        EsdkAlgorithmSuiteId::Alg128GcmNoKdf => MplAlgorithmSuiteId::AlgAes128GcmIv12Tag16NoKdf,
        EsdkAlgorithmSuiteId::Alg192GcmNoKdf => MplAlgorithmSuiteId::AlgAes192GcmIv12Tag16NoKdf,
        EsdkAlgorithmSuiteId::Alg256GcmNoKdf => MplAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf,
        EsdkAlgorithmSuiteId::Alg128GcmHkdf => MplAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256,
        EsdkAlgorithmSuiteId::Alg192GcmHkdf => MplAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha256,
        EsdkAlgorithmSuiteId::Alg256GcmHkdf => MplAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256,
        EsdkAlgorithmSuiteId::Alg128GcmHkdfEcdsa => {
            MplAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256
        }
        EsdkAlgorithmSuiteId::Alg192GcmHkdfEcdsa => {
            MplAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384
        }
        EsdkAlgorithmSuiteId::Alg256GcmHkdfEcdsa => {
            MplAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384
        }
        EsdkAlgorithmSuiteId::Alg256GcmCommit => {
            MplAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey
        }
        EsdkAlgorithmSuiteId::Alg256GcmCommitEcdsa => {
            MplAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384
        }
    }
}

fn from_mpl_suite(id: MplAlgorithmSuiteId) -> EsdkAlgorithmSuiteId {
    match id {
        MplAlgorithmSuiteId::AlgAes128GcmIv12Tag16NoKdf => EsdkAlgorithmSuiteId::Alg128GcmNoKdf,
        MplAlgorithmSuiteId::AlgAes192GcmIv12Tag16NoKdf => EsdkAlgorithmSuiteId::Alg192GcmNoKdf,
        MplAlgorithmSuiteId::AlgAes256GcmIv12Tag16NoKdf => EsdkAlgorithmSuiteId::Alg256GcmNoKdf,
        MplAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256 => EsdkAlgorithmSuiteId::Alg128GcmHkdf,
        MplAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha256 => EsdkAlgorithmSuiteId::Alg192GcmHkdf,
        MplAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha256 => EsdkAlgorithmSuiteId::Alg256GcmHkdf,
        MplAlgorithmSuiteId::AlgAes128GcmIv12Tag16HkdfSha256EcdsaP256 => {
            EsdkAlgorithmSuiteId::Alg128GcmHkdfEcdsa
        }
        MplAlgorithmSuiteId::AlgAes192GcmIv12Tag16HkdfSha384EcdsaP384 => {
            EsdkAlgorithmSuiteId::Alg192GcmHkdfEcdsa
        }
        MplAlgorithmSuiteId::AlgAes256GcmIv12Tag16HkdfSha384EcdsaP384 => {
            EsdkAlgorithmSuiteId::Alg256GcmHkdfEcdsa
        }
        MplAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKey => {
            EsdkAlgorithmSuiteId::Alg256GcmCommit
        }
        MplAlgorithmSuiteId::AlgAes256GcmHkdfSha512CommitKeyEcdsaP384 => {
            EsdkAlgorithmSuiteId::Alg256GcmCommitEcdsa
        }
    }
}

fn to_esdk_context(map: BTreeMap<String, String>) -> HashMap<String, String> {
    map.into_iter().collect()
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Eagerly build and register a client; returns its new id.
    pub async fn create_client(
        &self,
        req: CreateClientRequest,
    ) -> Result<CreateClientResponse, ServerError> {
        let cmm = build_cmm(&req.config.cmm).await?;
        let mut builder = AwsEncryptionSdkConfig::builder()
            .commitment_policy(map_commitment(req.config.commitment_policy));
        if let Some(max) = req.config.max_encrypted_data_keys {
            builder = builder.max_encrypted_data_keys(max);
        }
        let config = builder.build().map_err(|e| {
            ServerError::generic(format!("failed to construct the ESDK client: {e}"))
        })?;
        let esdk = EsdkClient::from_conf(config).map_err(|e| {
            ServerError::generic(format!(
                "failed to construct the ESDK client: {}",
                describe_esdk_error(&e)
            ))
        })?;
        let client_id = uuid::Uuid::new_v4().to_string();
        self.clients
            .lock()
            .expect("registry mutex")
            .insert(client_id.clone(), ClientEntry { esdk, cmm });
        Ok(CreateClientResponse { client_id })
    }

    pub async fn encrypt(&self, req: EncryptRequest) -> Result<EncryptResponse, ServerError> {
        let entry = self.entry(&req.client_id)?;
        let mut builder = entry
            .esdk
            .encrypt()
            .plaintext(Blob::new(req.plaintext))
            .materials_manager(entry.cmm);
        if let Some(context) = req.encryption_context {
            builder = builder.encryption_context(to_esdk_context(context));
        }
        if let Some(suite) = req.algorithm_suite_id {
            builder = builder.algorithm_suite_id(to_mpl_suite(suite));
        }
        if let Some(frame_length) = req.frame_length {
            builder = builder.frame_length(frame_length);
        }
        let output = builder
            .send()
            .await
            .map_err(|e| ServerError::esdk(describe_esdk_error(&e)))?;
        let ciphertext = output
            .ciphertext
            .map(Blob::into_inner)
            .ok_or_else(|| ServerError::generic("encrypt returned no ciphertext"))?;
        Ok(EncryptResponse { ciphertext })
    }

    pub async fn decrypt(&self, req: DecryptRequest) -> Result<DecryptResponse, ServerError> {
        let entry = self.entry(&req.client_id)?;
        let mut builder = entry
            .esdk
            .decrypt()
            .ciphertext(Blob::new(req.ciphertext))
            .materials_manager(entry.cmm);
        if let Some(context) = req.encryption_context {
            builder = builder.encryption_context(to_esdk_context(context));
        }
        let output = builder
            .send()
            .await
            .map_err(|e| ServerError::esdk(describe_esdk_error(&e)))?;
        let plaintext = output
            .plaintext
            .map(Blob::into_inner)
            .ok_or_else(|| ServerError::generic("decrypt returned no plaintext"))?;
        Ok(DecryptResponse {
            plaintext,
            encryption_context: output
                .encryption_context
                .filter(|map| !map.is_empty())
                .map(|map| map.into_iter().collect()),
            algorithm_suite_id: output.algorithm_suite_id.map(from_mpl_suite),
        })
    }

    /// Look up a registered client by id, cloning the shared handles.
    fn entry(&self, client_id: &str) -> Result<ClientEntry, ServerError> {
        if client_id.is_empty() {
            return Err(ServerError::generic("clientId must be non-empty"));
        }
        let clients = self.clients.lock().expect("registry mutex");
        clients
            .get(client_id)
            .cloned()
            .ok_or_else(|| ServerError::generic(format!("unknown clientId: {client_id}")))
    }
}
