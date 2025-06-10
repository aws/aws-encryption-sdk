// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct GetClientOutput {
    #[allow(missing_docs)]
    pub client: ::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
}
impl GetClientOutput {
    #[allow(missing_docs)]
    pub fn client(&self) -> &::std::option::Option<crate::deps::com_amazonaws_kms::client::Client> {
        &self.client
    }
}
impl GetClientOutput {
    /// Creates a new builder-style object to manufacture [`GetClientOutput`](crate::operation::get_client::builders::GetClientOutput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::get_client::builders::GetClientOutputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::get_client::builders::GetClientOutputBuilder::default()
    }
}

/// A builder for [`GetClientOutput`](crate::operation::operation::GetClientOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetClientOutputBuilder {
    pub(crate) client: ::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
}
impl GetClientOutputBuilder {
    #[allow(missing_docs)]
    pub fn client(
        mut self,
        input: impl ::std::convert::Into<crate::deps::com_amazonaws_kms::client::Client>,
    ) -> Self {
        self.client = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_client(
        mut self,
        input: ::std::option::Option<crate::deps::com_amazonaws_kms::client::Client>,
    ) -> Self {
        self.client = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_client(
        &self,
    ) -> &::std::option::Option<crate::deps::com_amazonaws_kms::client::Client> {
        &self.client
    }
    /// Consumes the builder and constructs a [`GetClientOutput`](crate::operation::operation::GetClientOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::get_client::GetClientOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::get_client::GetClientOutput {
            client: self.client,
        })
    }
}
