//! Integration tests for the test-vectors harness. Runs the harness against
//! committed test-vector manifests. Feature-gated to match `aws_esdk::test_vectors`.

#![cfg(feature = "test_vectors")]

use aws_esdk::test_vectors::{decrypt_test_vectors, encrypt_test_vectors};

// Decrypts the Java implementation's test-vector manifest, proving cross-implementation
// decrypt interop against ciphertexts this crate did not produce.
#[tokio::test(flavor = "multi_thread")]
async fn test_java_decrypt() {
    decrypt_test_vectors("test_vectors_java", "decrypt-manifest.json", "")
        .await
        .expect("Java decrypt test vectors must pass");
}

// Decrypts the Python implementation's test-vector manifest (independent producer).
#[tokio::test(flavor = "multi_thread")]
async fn test_python_decrypt() {
    decrypt_test_vectors("test_vectors_python", "decrypt_message.json", "")
        .await
        .expect("Python decrypt test vectors must pass");
}

// Round-trips this crate's own vectors through the harness: encrypt the Rust manifest,
// then decrypt the generated manifest.
#[tokio::test(flavor = "multi_thread")]
async fn test_rust_encrypt_decrypt() {
    let manifest_path = "test_vectors_rust";
    encrypt_test_vectors(manifest_path, manifest_path, "")
        .await
        .expect("Rust encrypt test vectors must pass");
    decrypt_test_vectors(manifest_path, "decrypt-manifest.json", "")
        .await
        .expect("Rust decrypt test vectors must pass");
}
