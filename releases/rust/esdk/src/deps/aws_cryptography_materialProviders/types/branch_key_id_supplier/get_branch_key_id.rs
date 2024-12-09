// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
impl crate::deps::aws_cryptography_materialProviders::types::branch_key_id_supplier::BranchKeyIdSupplierRef {
    /// Constructs a fluent builder for the [`GetBranchKeyId`](crate::operation::get_branch_key_id::builders::GetBranchKeyIdFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`encryption_context(impl Into<Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>>)`](crate::operation::get_branch_key_id::builders::GetBranchKeyIdFluentBuilder::encryption_context) / [`set_encryption_context(Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>)`](crate::operation::get_branch_key_id::builders::GetBranchKeyIdFluentBuilder::set_encryption_context): (undocumented)<br>
    /// - On success, responds with [`GetBranchKeyIdOutput`](crate::operation::get_branch_key_id::GetBranchKeyIdOutput) with field(s):
    ///   - [`branch_key_id(Option<::std::string::String>)`](crate::operation::get_branch_key_id::GetBranchKeyIdOutput::branch_key_id): (undocumented)
    /// - On failure, responds with [`SdkError<GetBranchKeyIdError>`](crate::operation::get_branch_key_id::GetBranchKeyIdError)
    pub fn get_branch_key_id(&self) -> crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::builders::GetBranchKeyIdFluentBuilder {
        crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::builders::GetBranchKeyIdFluentBuilder::new(self.clone())
    }
}
