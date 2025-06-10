// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
#[allow(missing_docs)]
pub struct CreateKeyStoreInput {}
impl CreateKeyStoreInput {}
impl CreateKeyStoreInput {
    /// Creates a new builder-style object to manufacture [`CreateKeyStoreInput`](crate::operation::create_key_store::builders::CreateKeyStoreInput).
    pub fn builder() -> crate::deps::aws_cryptography_keyStore::operation::create_key_store::builders::CreateKeyStoreInputBuilder{
        crate::deps::aws_cryptography_keyStore::operation::create_key_store::builders::CreateKeyStoreInputBuilder::default()
    }
}

/// A builder for [`CreateKeyStoreInput`](crate::operation::operation::CreateKeyStoreInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateKeyStoreInputBuilder {}
impl CreateKeyStoreInputBuilder {
    /// Consumes the builder and constructs a [`CreateKeyStoreInput`](crate::operation::operation::CreateKeyStoreInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::operation::create_key_store::CreateKeyStoreInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::deps::aws_cryptography_keyStore::operation::create_key_store::CreateKeyStoreInput {

        })
    }
}
