// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct CreateDefaultClientSupplierOutput {
    #[allow(missing_docs)]
    pub client: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
    >,
}
impl CreateDefaultClientSupplierOutput {
    #[allow(missing_docs)]
    pub fn client(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
    > {
        &self.client
    }
}
impl CreateDefaultClientSupplierOutput {
    /// Creates a new builder-style object to manufacture [`CreateDefaultClientSupplierOutput`](crate::operation::create_default_client_supplier::builders::CreateDefaultClientSupplierOutput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::create_default_client_supplier::builders::CreateDefaultClientSupplierOutputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_default_client_supplier::builders::CreateDefaultClientSupplierOutputBuilder::default()
    }
}

/// A builder for [`CreateDefaultClientSupplierOutput`](crate::operation::operation::CreateDefaultClientSupplierOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateDefaultClientSupplierOutputBuilder {
    pub(crate) client: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
    >,
}
impl CreateDefaultClientSupplierOutputBuilder {
    #[allow(missing_docs)]
    pub fn client(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef>,
    ) -> Self {
        self.client = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_client(
        mut self,
        input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef>,
    ) -> Self {
        self.client = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_client(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
    > {
        &self.client
    }
    /// Consumes the builder and constructs a [`CreateDefaultClientSupplierOutput`](crate::operation::operation::CreateDefaultClientSupplierOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::create_default_client_supplier::CreateDefaultClientSupplierOutput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::create_default_client_supplier::CreateDefaultClientSupplierOutput {
            client: self.client,
        })
    }
}
