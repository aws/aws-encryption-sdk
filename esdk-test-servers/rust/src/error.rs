// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! The two modeled TestServer errors, their rpcv2Cbor wire form, and message
//! extraction from the generated `aws-esdk` error enums.

use aws_esdk::aws_cryptography_primitives::types::error::Error as PrimitivesError;
use aws_esdk::key_store::types::error::Error as KeyStoreError;
use aws_esdk::material_providers::types::error::Error as MplError;
use aws_esdk::types::error::Error as EsdkError;

/// A modeled TestServer error.
#[derive(Debug, Clone)]
pub enum ServerError {
    /// Framework-side failure (bad/unknown clientId, unset variant, streaming on
    /// a non-streaming server, construction failure, any uncaught error).
    Generic(String),
    /// A failure forwarded from the underlying ESDK; the message is the ESDK
    /// error's message.
    Esdk(String),
}

const NS: &str = "aws.cryptography.esdk.testserver";

impl ServerError {
    pub fn generic(message: impl Into<String>) -> Self {
        Self::Generic(message.into())
    }

    pub fn esdk(message: impl Into<String>) -> Self {
        Self::Esdk(message.into())
    }

    /// The `__type` shape id placed on the wire.
    pub fn type_id(&self) -> String {
        match self {
            Self::Generic(_) => format!("{NS}#GenericServerError"),
            Self::Esdk(_) => format!("{NS}#ESDKClientError"),
        }
    }

    pub fn message(&self) -> &str {
        match self {
            Self::Generic(m) | Self::Esdk(m) => m,
        }
    }

    /// Serialize to the rpcv2Cbor error body: a CBOR map `{__type, message}`.
    pub fn to_cbor(&self) -> Vec<u8> {
        let map = ciborium::value::Value::Map(vec![
            (
                ciborium::value::Value::Text("__type".to_owned()),
                ciborium::value::Value::Text(self.type_id()),
            ),
            (
                ciborium::value::Value::Text("message".to_owned()),
                ciborium::value::Value::Text(self.message().to_owned()),
            ),
        ]);
        let mut buf = Vec::new();
        // Serializing a hand-built Value to a Vec never fails.
        ciborium::into_writer(&map, &mut buf).expect("CBOR error encoding");
        buf
    }
}

fn join_causes(message: &str, causes: Vec<String>) -> String {
    if causes.is_empty() {
        message.to_owned()
    } else {
        format!("{message} [encountered: {}]", causes.join("; "))
    }
}

/// Flatten a generated ESDK error to a message, appending the messages of any
/// collected nested errors so the underlying causes are visible.
pub fn describe_esdk_error(error: &EsdkError) -> String {
    match error {
        EsdkError::AwsEncryptionSdkException { message } => message.clone(),
        EsdkError::AwsCryptographicPrimitivesError { error } => describe_primitives_error(error),
        EsdkError::AwsCryptographicMaterialProvidersError { error } => describe_mpl_error(error),
        EsdkError::CollectionOfErrors { list, message } => {
            join_causes(message, list.iter().map(describe_esdk_error).collect())
        }
        EsdkError::ValidationError(e) => e.to_string(),
        EsdkError::OpaqueWithText { objMessage, .. } => objMessage.clone(),
        EsdkError::Opaque { .. } => format!("{error:?}"),
    }
}

pub fn describe_mpl_error(error: &MplError) -> String {
    match error {
        MplError::AwsCryptographicMaterialProvidersException { message }
        | MplError::EntryAlreadyExists { message }
        | MplError::EntryDoesNotExist { message }
        | MplError::InFlightTtlExceeded { message }
        | MplError::InvalidAlgorithmSuiteInfo { message }
        | MplError::InvalidAlgorithmSuiteInfoOnDecrypt { message }
        | MplError::InvalidAlgorithmSuiteInfoOnEncrypt { message }
        | MplError::InvalidDecryptionMaterials { message }
        | MplError::InvalidDecryptionMaterialsTransition { message }
        | MplError::InvalidEncryptionMaterials { message }
        | MplError::InvalidEncryptionMaterialsTransition { message } => message.clone(),
        MplError::AwsCryptographicPrimitivesError { error } => describe_primitives_error(error),
        MplError::KeyStoreError { error } => describe_key_store_error(error),
        MplError::CollectionOfErrors { list, message } => {
            join_causes(message, list.iter().map(describe_mpl_error).collect())
        }
        MplError::ValidationError(e) => e.to_string(),
        MplError::OpaqueWithText { objMessage, .. } => objMessage.clone(),
        // KMS/DynamoDB service errors and opaque Dafny objects: the derived
        // Debug form carries the underlying SDK error detail.
        other => format!("{other:?}"),
    }
}

pub fn describe_key_store_error(error: &KeyStoreError) -> String {
    match error {
        KeyStoreError::KeyStoreException { message } => message.clone(),
        KeyStoreError::CollectionOfErrors { list, message } => {
            join_causes(message, list.iter().map(describe_key_store_error).collect())
        }
        KeyStoreError::ValidationError(e) => e.to_string(),
        KeyStoreError::OpaqueWithText { objMessage, .. } => objMessage.clone(),
        other => format!("{other:?}"),
    }
}

fn describe_primitives_error(error: &PrimitivesError) -> String {
    match error {
        PrimitivesError::AwsCryptographicPrimitivesError { message } => message.clone(),
        PrimitivesError::CollectionOfErrors { list, message } => join_causes(
            message,
            list.iter().map(describe_primitives_error).collect(),
        ),
        PrimitivesError::ValidationError(e) => e.to_string(),
        PrimitivesError::OpaqueWithText { objMessage, .. } => objMessage.clone(),
        PrimitivesError::Opaque { .. } => format!("{error:?}"),
    }
}
