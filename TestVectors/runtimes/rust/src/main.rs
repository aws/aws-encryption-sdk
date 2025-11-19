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
pub mod validation;

#[cfg(feature = "fips")]
use aws_lc_fips_sys as aws_lc_sys_impl;

#[cfg(not(feature = "fips"))]
use aws_lc_sys as aws_lc_sys_impl;

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
pub(crate) use crate::implementation_from_dafny::_Wrappers_Compile;
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
pub(crate) use crate::implementation_from_dafny::_TestWrappedESDKMain_Compile;
pub(crate) use crate::deps::com_amazonaws_kms::client::Client as KmsClient;

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
pub(crate) mod test_vec_dir;

fn main2() {
  let args: Vec<String> = std::env::args().collect();
  let dafny_strings = args.iter().map(|x| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&x)).collect::<Vec<_>>();
  let dafny_args = dafny_runtime::Sequence::from_array_owned(dafny_strings);
  crate::implementation_from_dafny::_WrappedESDKMain_Compile::_default::Main2(&dafny_args);
}

// RUST_MIN_STACK does not work for `main`, so we need a new thread
fn main() {
  std::thread::Builder::new().stack_size(64 * 1024 * 1024).spawn(main2).unwrap().join().unwrap();
}
