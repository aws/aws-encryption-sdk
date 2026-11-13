// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Credential-free tests driving the real rpcv2Cbor wire path in-process.
//! Multi-thread runtime: the Dafny runtime blocks in place under its async
//! facade.

use std::collections::BTreeMap;
use std::sync::Arc;

use aws_esdk_dafny_test_server::handlers::AppState;
use aws_esdk_dafny_test_server::model::{
    AesWrappingAlg, CreateClientRequest, CreateClientResponse, CryptographicMaterialsManager,
    DecryptRequest, DecryptResponse, DefaultCmmConfig, EncryptRequest, EncryptResponse,
    EsdkAlgorithmSuiteId, EsdkClientConfig, EsdkCommitmentPolicy, Keyring, RawAesKeyringConfig,
};
use aws_esdk_dafny_test_server::wire::app;
use axum::body::Body;
use axum::http::{Request, Response};
use axum::Router;
use ciborium::value::Value;
use serde::de::DeserializeOwned;
use serde::Serialize;
use tower::ServiceExt;

const GENERIC_SERVER_ERROR: &str = "aws.cryptography.esdk.testserver#GenericServerError";

fn router() -> Router {
    app(Arc::new(AppState::new()))
}

fn cbor<T: Serialize>(value: &T) -> Vec<u8> {
    let mut buf = Vec::new();
    ciborium::into_writer(value, &mut buf).expect("encode request");
    buf
}

fn raw_aes_create_client(max_encrypted_data_keys: Option<i64>) -> CreateClientRequest {
    CreateClientRequest {
        config: EsdkClientConfig {
            commitment_policy: EsdkCommitmentPolicy::RequireEncryptRequireDecrypt,
            max_encrypted_data_keys,
            cmm: CryptographicMaterialsManager {
                default_cmm: Some(DefaultCmmConfig {
                    keyring: Keyring {
                        raw_aes: Some(RawAesKeyringConfig {
                            key_namespace: "esdk-test-server".to_owned(),
                            key_name: "raw-aes-round-trip-key".to_owned(),
                            wrapping_key: (0u8..32).collect(),
                            wrapping_alg: AesWrappingAlg::Alg256,
                        }),
                        ..Default::default()
                    },
                }),
                ..Default::default()
            },
        },
    }
}

/// POST an operation with the correct rpcv2Cbor headers.
async fn call(router: &Router, operation: &str, body: Vec<u8>) -> Response<Body> {
    let request = Request::builder()
        .method("POST")
        .uri(format!("/service/ESDKTestServer/operation/{operation}"))
        .header("smithy-protocol", "rpc-v2-cbor")
        .header("content-type", "application/cbor")
        .body(Body::from(body))
        .expect("build request");
    dispatch(router, request).await
}

/// Dispatch any request and assert the response carries the contract headers.
async fn dispatch(router: &Router, request: Request<Body>) -> Response<Body> {
    let response = router.clone().oneshot(request).await.expect("dispatch");
    assert_eq!(
        response
            .headers()
            .get("smithy-protocol")
            .and_then(|v| v.to_str().ok()),
        Some("rpc-v2-cbor"),
        "every response must carry the smithy-protocol header",
    );
    assert_eq!(
        response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok()),
        Some("application/cbor"),
        "every response must carry the CBOR content-type",
    );
    response
}

async fn body_bytes(response: Response<Body>) -> Vec<u8> {
    axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .expect("read body")
        .to_vec()
}

async fn decode_ok<T: DeserializeOwned>(response: Response<Body>) -> T {
    assert_eq!(response.status(), 200, "success status expected");
    let bytes = body_bytes(response).await;
    ciborium::from_reader(&bytes[..]).expect("decode response")
}

/// Assert a 400 modeled-error response and return its (__type, message).
async fn decode_error(response: Response<Body>) -> (String, String) {
    assert_eq!(response.status(), 400, "client error status expected");
    let bytes = body_bytes(response).await;
    let value: Value = ciborium::from_reader(&bytes[..]).expect("decode error body");
    let Value::Map(map) = value else {
        panic!("error body was not a CBOR map");
    };
    let text = |key: &str| {
        map.iter()
            .find_map(|(k, v)| match (k, v) {
                (Value::Text(k), Value::Text(v)) if k == key => Some(v.clone()),
                _ => None,
            })
            .unwrap_or_else(|| panic!("error body missing text member {key}"))
    };
    (text("__type"), text("message"))
}

async fn create_client(router: &Router) -> String {
    let response = call(router, "CreateClient", cbor(&raw_aes_create_client(None))).await;
    let created: CreateClientResponse = decode_ok(response).await;
    assert!(!created.client_id.is_empty(), "clientId must be non-empty");
    created.client_id
}

#[tokio::test(flavor = "multi_thread")]
async fn raw_aes_round_trip() {
    let router = router();
    let client_id = create_client(&router).await;

    let plaintext = b"rust-dafny language server round trip".to_vec();
    let encryption_context = BTreeMap::from([("tenant".to_owned(), "round-trip".to_owned())]);
    let encrypted: EncryptResponse = decode_ok(
        call(
            &router,
            "Encrypt",
            cbor(&EncryptRequest {
                client_id: client_id.clone(),
                plaintext: plaintext.clone(),
                encryption_context: Some(encryption_context),
                algorithm_suite_id: None,
                frame_length: None,
            }),
        )
        .await,
    )
    .await;
    assert_ne!(encrypted.ciphertext, plaintext, "ciphertext must differ");

    let decrypted: DecryptResponse = decode_ok(
        call(
            &router,
            "Decrypt",
            cbor(&DecryptRequest {
                client_id,
                ciphertext: encrypted.ciphertext.clone(),
                encryption_context: None,
            }),
        )
        .await,
    )
    .await;
    assert_eq!(decrypted.plaintext, plaintext, "plaintext must round-trip");
    // Signed suites add the reserved aws-crypto-public-key pair to the
    // header-carried context; the caller's pair must be reported back.
    let reported = decrypted
        .encryption_context
        .expect("decrypt must report the header-carried encryption context");
    assert_eq!(
        reported.get("tenant").map(String::as_str),
        Some("round-trip"),
        "decrypt must report the caller's encryption context pair",
    );
    assert_eq!(
        decrypted.algorithm_suite_id,
        Some(EsdkAlgorithmSuiteId::Alg256GcmCommitEcdsa),
        "the default committing+signing suite must map back to its wire name",
    );

    // A second client configured with the same wrapping key decrypts the first
    // client's message; the encryption context rides in the message header.
    let second_client_id = create_client(&router).await;
    let second: DecryptResponse = decode_ok(
        call(
            &router,
            "Decrypt",
            cbor(&DecryptRequest {
                client_id: second_client_id,
                ciphertext: encrypted.ciphertext,
                encryption_context: None,
            }),
        )
        .await,
    )
    .await;
    assert_eq!(second.plaintext, plaintext, "second client must decrypt");
}

#[tokio::test(flavor = "multi_thread")]
async fn missing_or_wrong_smithy_protocol_header_is_generic_error() {
    let router = router();
    let uri = "/service/ESDKTestServer/operation/CreateClient";

    let missing = Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/cbor")
        .body(Body::from(cbor(&raw_aes_create_client(None))))
        .expect("build request");
    let (error_type, _) = decode_error(dispatch(&router, missing).await).await;
    assert_eq!(error_type, GENERIC_SERVER_ERROR);

    let wrong = Request::builder()
        .method("POST")
        .uri(uri)
        .header("smithy-protocol", "rpc-v2-json")
        .header("content-type", "application/cbor")
        .body(Body::from(cbor(&raw_aes_create_client(None))))
        .expect("build request");
    let (error_type, _) = decode_error(dispatch(&router, wrong).await).await;
    assert_eq!(error_type, GENERIC_SERVER_ERROR);
}

#[tokio::test(flavor = "multi_thread")]
async fn unknown_operation_is_generic_error() {
    let router = router();
    let (error_type, message) =
        decode_error(call(&router, "Frobnicate", cbor(&raw_aes_create_client(None))).await).await;
    assert_eq!(error_type, GENERIC_SERVER_ERROR);
    assert!(
        message.contains("unknown operation"),
        "unexpected message: {message}",
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn encrypt_stream_is_generic_error() {
    let router = router();
    let client_id = create_client(&router).await;
    let (error_type, message) = decode_error(
        call(
            &router,
            "EncryptStream",
            cbor(&EncryptRequest {
                client_id,
                plaintext: b"stream me".to_vec(),
                encryption_context: None,
                algorithm_suite_id: None,
                frame_length: None,
            }),
        )
        .await,
    )
    .await;
    assert_eq!(error_type, GENERIC_SERVER_ERROR);
    assert_eq!(
        message,
        "streaming operations are not supported by the rust-dafny language server",
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn unknown_client_id_on_encrypt_is_generic_error() {
    let router = router();
    let (error_type, message) = decode_error(
        call(
            &router,
            "Encrypt",
            cbor(&EncryptRequest {
                client_id: "no-such-client".to_owned(),
                plaintext: b"plaintext".to_vec(),
                encryption_context: None,
                algorithm_suite_id: None,
                frame_length: None,
            }),
        )
        .await,
    )
    .await;
    assert_eq!(error_type, GENERIC_SERVER_ERROR);
    assert!(
        message.contains("no-such-client"),
        "unexpected message: {message}",
    );
}

#[tokio::test(flavor = "multi_thread")]
async fn create_client_with_zero_max_edks_fails_eagerly() {
    let router = router();
    let (error_type, message) = decode_error(
        call(
            &router,
            "CreateClient",
            cbor(&raw_aes_create_client(Some(0))),
        )
        .await,
    )
    .await;
    assert_eq!(error_type, GENERIC_SERVER_ERROR);
    assert!(
        message.contains("max_encrypted_data_keys"),
        "the failure must come from the maxEncryptedDataKeys constraint: {message}",
    );
}
