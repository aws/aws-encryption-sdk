// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_materialProviders::operation::get_client::_get_client_output::GetClientOutputBuilder;

pub use crate::deps::aws_cryptography_materialProviders::operation::get_client::_get_client_input::GetClientInputBuilder;

impl GetClientInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client_supplier: &crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
    ) -> ::std::result::Result<
        crate::deps::com_amazonaws_kms::client::Client,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        let mut fluent_builder = client_supplier.get_client();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetClient`.
///
/// Returns an AWS KMS Client.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetClientFluentBuilder {
    client_supplier: crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
    pub(crate) inner: crate::deps::aws_cryptography_materialProviders::operation::get_client::builders::GetClientInputBuilder,
}
impl GetClientFluentBuilder {
    /// Creates a new `GetClient`.
    pub(crate) fn new(
        client_supplier: crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
    ) -> Self {
        Self {
            client_supplier,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetClient as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_materialProviders::operation::get_client::builders::GetClientInputBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::deps::com_amazonaws_kms::client::Client,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
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
        crate::deps::aws_cryptography_materialProviders::operation::get_client::GetClient::send(
            &self.client_supplier,
            input,
        )
        .await
    }

    /// The region the client should be created in.
    pub fn region(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.region(input.into());
        self
    }
    /// The region the client should be created in.
    pub fn set_region(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_region(input);
        self
    }
    /// The region the client should be created in.
    pub fn get_region(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_region()
    }
}
