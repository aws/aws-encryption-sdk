// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_keyStore::operation::version_key::_version_key_output::VersionKeyOutputBuilder;

pub use crate::deps::aws_cryptography_keyStore::operation::version_key::_version_key_input::VersionKeyInputBuilder;

impl VersionKeyInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_keyStore::client::Client,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::operation::version_key::VersionKeyOutput,
        crate::deps::aws_cryptography_keyStore::types::error::Error,
    > {
        let mut fluent_builder = client.version_key();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `VersionKey`.
///
/// Create a new ACTIVE version of an existing Branch Key in the Key Store, and set the previously ACTIVE version to DECRYPT_ONLY.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct VersionKeyFluentBuilder {
    client: crate::deps::aws_cryptography_keyStore::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_keyStore::operation::version_key::builders::VersionKeyInputBuilder,
}
impl VersionKeyFluentBuilder {
    /// Creates a new `VersionKey`.
    pub(crate) fn new(client: crate::deps::aws_cryptography_keyStore::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the VersionKey as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_keyStore::operation::version_key::builders::VersionKeyInputBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::operation::version_key::VersionKeyOutput,
        crate::deps::aws_cryptography_keyStore::types::error::Error,
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
                crate::deps::aws_cryptography_keyStore::types::error::Error::OpaqueWithText {
                    obj: ::dafny_runtime::Object::from_ref(&mut e as &mut ::dafny_runtime::DynAny),
                    objMessage: msg,
                }
            })?;
        crate::deps::aws_cryptography_keyStore::operation::version_key::VersionKey::send(
            &self.client,
            input,
        )
        .await
    }

    /// The identifier for the Branch Key to be versioned.
    pub fn branch_key_identifier(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.branch_key_identifier(input.into());
        self
    }
    /// The identifier for the Branch Key to be versioned.
    pub fn set_branch_key_identifier(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_branch_key_identifier(input);
        self
    }
    /// The identifier for the Branch Key to be versioned.
    pub fn get_branch_key_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_branch_key_identifier()
    }
}
