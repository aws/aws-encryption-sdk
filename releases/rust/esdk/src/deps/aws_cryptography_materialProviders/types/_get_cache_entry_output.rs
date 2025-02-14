// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct GetCacheEntryOutput {
    #[allow(missing_docs)]
    pub bytes_used: ::std::option::Option<::std::primitive::i32>,
    #[allow(missing_docs)]
    pub creation_time: ::std::option::Option<::std::primitive::i64>,
    #[allow(missing_docs)]
    pub expiry_time: ::std::option::Option<::std::primitive::i64>,
    #[allow(missing_docs)]
    pub materials:
        ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::Materials>,
    #[allow(missing_docs)]
    pub messages_used: ::std::option::Option<::std::primitive::i32>,
}
impl GetCacheEntryOutput {
    #[allow(missing_docs)]
    pub fn bytes_used(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.bytes_used
    }
    #[allow(missing_docs)]
    pub fn creation_time(&self) -> &::std::option::Option<::std::primitive::i64> {
        &self.creation_time
    }
    #[allow(missing_docs)]
    pub fn expiry_time(&self) -> &::std::option::Option<::std::primitive::i64> {
        &self.expiry_time
    }
    #[allow(missing_docs)]
    pub fn materials(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::Materials>
    {
        &self.materials
    }
    #[allow(missing_docs)]
    pub fn messages_used(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.messages_used
    }
}
impl GetCacheEntryOutput {
    /// Creates a new builder-style object to manufacture [`GetCacheEntryOutput`](crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryOutput).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_materialProviders::types::builders::GetCacheEntryOutputBuilder
    {
        crate::deps::aws_cryptography_materialProviders::types::builders::GetCacheEntryOutputBuilder::default()
    }
}

/// A builder for [`GetCacheEntryOutput`](crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryOutput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct GetCacheEntryOutputBuilder {
    pub(crate) bytes_used: ::std::option::Option<::std::primitive::i32>,
    pub(crate) creation_time: ::std::option::Option<::std::primitive::i64>,
    pub(crate) expiry_time: ::std::option::Option<::std::primitive::i64>,
    pub(crate) materials:
        ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::Materials>,
    pub(crate) messages_used: ::std::option::Option<::std::primitive::i32>,
}
impl GetCacheEntryOutputBuilder {
    #[allow(missing_docs)]
    pub fn bytes_used(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
        self.bytes_used = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_bytes_used(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
        self.bytes_used = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_bytes_used(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.bytes_used
    }
    #[allow(missing_docs)]
    pub fn creation_time(
        mut self,
        input: impl ::std::convert::Into<::std::primitive::i64>,
    ) -> Self {
        self.creation_time = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_creation_time(
        mut self,
        input: ::std::option::Option<::std::primitive::i64>,
    ) -> Self {
        self.creation_time = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_creation_time(&self) -> &::std::option::Option<::std::primitive::i64> {
        &self.creation_time
    }
    #[allow(missing_docs)]
    pub fn expiry_time(mut self, input: impl ::std::convert::Into<::std::primitive::i64>) -> Self {
        self.expiry_time = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_expiry_time(mut self, input: ::std::option::Option<::std::primitive::i64>) -> Self {
        self.expiry_time = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_expiry_time(&self) -> &::std::option::Option<::std::primitive::i64> {
        &self.expiry_time
    }
    #[allow(missing_docs)]
    pub fn materials(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::Materials,
        >,
    ) -> Self {
        self.materials = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_materials(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::Materials,
        >,
    ) -> Self {
        self.materials = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_materials(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::Materials>
    {
        &self.materials
    }
    #[allow(missing_docs)]
    pub fn messages_used(
        mut self,
        input: impl ::std::convert::Into<::std::primitive::i32>,
    ) -> Self {
        self.messages_used = ::std::option::Option::Some(input.into());
        self
    }
    #[allow(missing_docs)]
    pub fn set_messages_used(
        mut self,
        input: ::std::option::Option<::std::primitive::i32>,
    ) -> Self {
        self.messages_used = input;
        self
    }
    #[allow(missing_docs)]
    pub fn get_messages_used(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.messages_used
    }
    /// Consumes the builder and constructs a [`GetCacheEntryOutput`](crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryOutput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryOutput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::GetCacheEntryOutput {
                bytes_used: self.bytes_used,
                creation_time: self.creation_time,
                expiry_time: self.expiry_time,
                materials: self.materials,
                messages_used: self.messages_used,
            },
        )
    }
}
