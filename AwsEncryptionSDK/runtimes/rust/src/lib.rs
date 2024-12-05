// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(warnings, unconditional_panic)]
#![allow(nonstandard_style)]

pub mod client;
pub mod conversions;
pub mod deps;
/// Common errors and error handling utilities.
pub mod error;
/// All operations that this crate can perform.
pub mod operation;
pub mod types;

#[cfg(feature = "wrapped-client")]
pub mod wrapped;

mod standard_library_conversions;
mod standard_library_externs;

pub use client::Client;
pub use types::aws_encryption_sdk_config::AwsEncryptionSdkConfig;

pub use crate::deps::aws_cryptography_keyStore;
pub use crate::deps::aws_cryptography_materialProviders;
pub use crate::deps::aws_cryptography_primitives;
pub use crate::deps::com_amazonaws_kms;

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
pub(crate) mod random;
pub(crate) mod rsa;
pub(crate) mod sets;
pub(crate) mod software_externs;
pub(crate) mod storm_tracker;
pub(crate) mod time;
pub(crate) mod uuid;
