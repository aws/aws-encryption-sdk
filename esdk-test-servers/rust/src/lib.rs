// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Hand-rolled ESDK TestServer Language_Server backed by the Dafny-generated
//! AWS Encryption SDK for Rust.
//!
//! Speaks the rpcv2Cbor wire contract of the single source-of-truth Smithy model
//! owned by aws-crypto-tools-commons (`esdk/test-server/model`) and delegates
//! each operation to the committed `aws-esdk` crate at `releases/rust/esdk`.

pub mod error;
pub mod handlers;
pub mod keyring;
pub mod model;
pub mod wire;
