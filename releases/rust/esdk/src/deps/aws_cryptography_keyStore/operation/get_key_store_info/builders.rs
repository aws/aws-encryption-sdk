// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::_get_key_store_info_output::GetKeyStoreInfoOutputBuilder;

pub use crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::_unit::UnitBuilder;

impl UnitBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_keyStore::client::Client,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::GetKeyStoreInfoOutput,
        crate::deps::aws_cryptography_keyStore::types::error::Error,
    >{
        let mut fluent_builder = client.get_key_store_info();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetKeyStoreInfo`.
///
/// Returns the configuration information for a Key Store.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetKeyStoreInfoFluentBuilder {
    client: crate::deps::aws_cryptography_keyStore::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::builders::UnitBuilder,
}
impl GetKeyStoreInfoFluentBuilder {
    /// Creates a new `GetKeyStoreInfo`.
    pub(crate) fn new(client: crate::deps::aws_cryptography_keyStore::client::Client) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetKeyStoreInfo as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::builders::UnitBuilder
    {
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::GetKeyStoreInfoOutput,
        crate::deps::aws_cryptography_keyStore::types::error::Error,
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
                crate::deps::aws_cryptography_keyStore::types::error::Error::OpaqueWithText {
                    obj: ::dafny_runtime::Object::from_ref(&mut e as &mut ::dafny_runtime::DynAny),
                    objMessage: msg,
                }
            })?;
        crate::deps::aws_cryptography_keyStore::operation::get_key_store_info::GetKeyStoreInfo::send(&self.client, input).await
    }
}
