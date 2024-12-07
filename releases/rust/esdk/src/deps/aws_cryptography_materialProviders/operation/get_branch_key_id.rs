// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
/// Orchestration and serialization glue logic for `GetBranchKeyId`.
#[derive(::std::clone::Clone, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetBranchKeyId;
impl GetBranchKeyId {
    /// Creates a new `GetBranchKeyId`
    pub fn new() -> Self {
        Self
    }

    pub(crate) async fn send(
        branch_key_id_supplier: &crate::deps::aws_cryptography_materialProviders::types::branch_key_id_supplier::BranchKeyIdSupplierRef,
        input: crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::GetBranchKeyIdInput,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::GetBranchKeyIdOutput,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        if input.encryption_context.is_none() {
    return ::std::result::Result::Err(::aws_smithy_types::error::operation::BuildError::missing_field(
        "encryption_context",
        "encryption_context was not specified but it is required when building GetBranchKeyIdInput",
    )).map_err(crate::deps::aws_cryptography_materialProviders::types::error::Error::wrap_validation_err);
}
        branch_key_id_supplier.inner.borrow_mut().get_branch_key_id(input)
    }
}

pub use crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::_get_branch_key_id_output::GetBranchKeyIdOutput;

pub use crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::_get_branch_key_id_input::GetBranchKeyIdInput;

pub(crate) mod _get_branch_key_id_output;

pub(crate) mod _get_branch_key_id_input;

/// Builders
pub mod builders;
