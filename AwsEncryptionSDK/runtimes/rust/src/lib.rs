// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! The AWS Encryption SDK enables secure client-side encryption.
//! 
//! Running `cargo test --examples` for this library runs these example keyrings.
//! 
//! For details see the [Examples](https://github.com/aws/aws-encryption-sdk-dafny/tree/mainline/releases/rust/esdk/examples)
//! or the [Developer Guide](https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/introduction.html)
//! 
//! One of the most common keyrings that you can use is the AWS KMS Keyring.
//! The AWS KMS keyring uses symmetric encryption KMS keys to generate, encrypt and
//! decrypt data keys. You provide the KMS Key and KMS client configuration while
//! providing the keyring.
//! 
//! [See full example](https://github.com/aws/aws-encryption-sdk-dafny/blob/mainline/releases/rust/esdk/examples/keyring/aws_kms_keyring_example.rs)
//! 
//! ```text
//! // Initialize ESDK client and MPL client
//! let esdk_config = AwsEncryptionSdkConfig::builder().build()?;
//! let esdk_client = esdk_client::Client::from_conf(esdk_config)?;
//! 
//! let mpl_config = MaterialProvidersConfig::builder().build()?;
//! let mpl = mpl_client::Client::from_conf(mpl_config)?;
//! 
//! // Create KMS Keyring
//! let kms_keyring = mpl
//!     .create_aws_kms_keyring()
//!     // your configuration here
//!     .send()
//!     .await?;
//! 
//! // Encrypt
//! let encryption_response = esdk_client.encrypt()
//!     .plaintext(plaintext)
//!     .keyring(kms_keyring)
//!     .encryption_context(encryption_context)
//!     .send()
//!     .await?;
//! 
//! let ciphertext = encryption_response
//!                 .ciphertext
//!                 .expect("Unable to unwrap ciphertext from encryption response");
//! 
//! // Decrypt
//! let decryption_response = esdk_client.decrypt()
//!     .ciphertext(ciphertext)
//!     .keyring(kms_keyring)
//!     .encryption_context(encryption_context)
//!     .send()
//!     .await?;
//! 
//! let decrypted_plaintext = decryption_response
//!                         .plaintext
//!                         .expect("Unable to unwrap plaintext from decryption response");
//! 
//! // Demonstrate that the decrypted plaintext is identical to the original plaintext.
//! // (This is an example for demonstration; you do not need to do this in your own code.)
//! assert_eq!(decrypted_plaintext, aws_smithy_types::Blob::new(plaintext),
//!     "Decrypted plaintext should be identical to the original plaintext. Invalid decryption");
//!     
//! ```

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]
#![allow(clippy::never_loop)]
#![allow(clippy::absurd_extreme_comparisons)]

/// Client for using encrypt and decrypt operations
pub mod client;
/// Errors and error handling utilities.
pub mod error;
/// All operations that this crate can perform.
pub mod operation;
/// Types for the transform client.
pub mod types;

#[cfg(feature = "fips")]
use aws_lc_fips_sys as aws_lc_sys_impl;

#[cfg(not(feature = "fips"))]
use aws_lc_sys as aws_lc_sys_impl;

pub use client::Client;
pub use types::aws_encryption_sdk_config::AwsEncryptionSdkConfig;

/// Branch key support. See [Key Stores](https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/keystores.html)
pub use crate::deps::aws_cryptography_keyStore as key_store;
/// [Key Rings](https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/choose-keyring.html) and other fundamentals.
pub use crate::deps::aws_cryptography_materialProviders as material_providers;
pub use crate::deps::aws_cryptography_primitives;
/// Rarely needed internal KMS Client, needed for [ClientSupplier](https://github.com/aws/aws-encryption-sdk-dafny/blob/mainline/releases/rust/esdk/examples/client_supplier/regional_role_client_supplier.rs)
pub use crate::deps::com_amazonaws_kms;

mod standard_library_conversions;
mod standard_library_externs;

pub(crate) mod implementation_from_dafny;
pub(crate) use crate::implementation_from_dafny::r#_Wrappers_Compile;
pub(crate) use crate::implementation_from_dafny::software;
pub(crate) use crate::implementation_from_dafny::AesKdfCtr;
pub(crate) use crate::implementation_from_dafny::ConcurrentCall;
pub(crate) use crate::implementation_from_dafny::DafnyLibraries;
pub(crate) use crate::implementation_from_dafny::ExternDigest;
pub(crate) use crate::implementation_from_dafny::ExternRandom;
pub(crate) use crate::implementation_from_dafny::Signature;
pub(crate) use crate::implementation_from_dafny::Time;
pub(crate) use crate::implementation_from_dafny::_LocalCMC_Compile;
pub(crate) use crate::implementation_from_dafny::_StormTracker_Compile;
pub(crate) use crate::implementation_from_dafny::ECDH;
pub(crate) use crate::implementation_from_dafny::HMAC;
pub(crate) use crate::implementation_from_dafny::UTF8;
pub(crate) use crate::implementation_from_dafny::UUID;
pub(crate) use crate::deps::com_amazonaws_kms::client::Client as KmsClient;

// Import smithy-generated modules
pub(crate) mod conversions;
pub(crate) mod deps;
pub(crate) mod validation;

// Import externs
pub(crate) mod aes_gcm;
pub(crate) mod aes_kdf_ctr;
pub(crate) mod concurrent_call;
pub(crate) mod dafny_libraries;
pub(crate) mod ddb;
pub(crate) mod digest;
pub(crate) mod ecdh;
pub(crate) mod ecdsa;
pub(crate) mod hmac;
pub(crate) mod kms;
pub(crate) mod local_cmc;
pub(crate) mod oslang;
pub(crate) mod random;
pub(crate) mod rsa;
pub(crate) mod sets;
pub(crate) mod software_externs;
pub(crate) mod storm_tracker;
pub(crate) mod time;
pub(crate) mod uuid;
