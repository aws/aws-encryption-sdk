// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::_get_branch_key_id_output::GetBranchKeyIdOutputBuilder;

pub use crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::_get_branch_key_id_input::GetBranchKeyIdInputBuilder;

impl GetBranchKeyIdInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        branch_key_id_supplier: &crate::deps::aws_cryptography_materialProviders::types::branch_key_id_supplier::BranchKeyIdSupplierRef,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::GetBranchKeyIdOutput,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    >{
        let mut fluent_builder = branch_key_id_supplier.get_branch_key_id();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetBranchKeyId`.
///
/// Given the Encryption Context associated with this encryption or decryption, returns the branch key that should be responsible for unwrapping or wrapping the data key.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetBranchKeyIdFluentBuilder {
    branch_key_id_supplier: crate::deps::aws_cryptography_materialProviders::types::branch_key_id_supplier::BranchKeyIdSupplierRef,
    pub(crate) inner: crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::builders::GetBranchKeyIdInputBuilder,
}
impl GetBranchKeyIdFluentBuilder {
    /// Creates a new `GetBranchKeyId`.
    pub(crate) fn new(
        branch_key_id_supplier: crate::deps::aws_cryptography_materialProviders::types::branch_key_id_supplier::BranchKeyIdSupplierRef,
    ) -> Self {
        Self {
            branch_key_id_supplier,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetBranchKeyId as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::builders::GetBranchKeyIdInputBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::GetBranchKeyIdOutput,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    >{
        let input = self
            .inner
            .build()
            // Using Opaque since we don't have a validation-specific error yet.
            // Operations' models don't declare their own validation error,
            // and smithy-rs seems to not generate a ValidationError case unless there is.
            // Vanilla smithy-rs uses SdkError::construction_failure, but we aren't using SdkError.
            .map_err(|mut e| {
	     let msg = format!("{:?}", e);
             crate::deps::aws_cryptography_materialProviders::types::error::Error::OpaqueWithText {
                obj: ::dafny_runtime::Object::from_ref(&mut e as &mut ::dafny_runtime::DynAny),
		objMessage: msg
             }})?;
        crate::deps::aws_cryptography_materialProviders::operation::get_branch_key_id::GetBranchKeyId::send(&self.branch_key_id_supplier, input).await
    }

    /// The Encryption Context used with this encryption or decryption.
    pub fn encryption_context(
        mut self,
        input: impl ::std::convert::Into<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.encryption_context(input.into());
        self
    }
    /// The Encryption Context used with this encryption or decryption.
    pub fn set_encryption_context(
        mut self,
        input: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    ) -> Self {
        self.inner = self.inner.set_encryption_context(input);
        self
    }
    /// The Encryption Context used with this encryption or decryption.
    pub fn get_encryption_context(
        &self,
    ) -> &::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    > {
        self.inner.get_encryption_context()
    }
}
