// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_discovery_multi_keyring::_create_keyring_output::CreateKeyringOutputBuilder;

pub use crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_discovery_multi_keyring::_create_aws_kms_mrk_discovery_multi_keyring_input::CreateAwsKmsMrkDiscoveryMultiKeyringInputBuilder;

impl CreateAwsKmsMrkDiscoveryMultiKeyringInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        let mut fluent_builder = client.create_aws_kms_mrk_discovery_multi_keyring();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateAwsKmsMrkDiscoveryMultiKeyring`.
///
/// Creates an AWS KMS MRK Discovery Multi-Keyring that supports unwrapping data keys wrapped by a symmetric AWS KMS Key or AWS KMS Multi-Region Key, for a single region.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAwsKmsMrkDiscoveryMultiKeyringFluentBuilder {
    client: crate::deps::aws_cryptography_materialProviders::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_discovery_multi_keyring::builders::CreateAwsKmsMrkDiscoveryMultiKeyringInputBuilder,
}
impl CreateAwsKmsMrkDiscoveryMultiKeyringFluentBuilder {
    /// Creates a new `CreateAwsKmsMrkDiscoveryMultiKeyring`.
    pub(crate) fn new(
        client: crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateAwsKmsMrkDiscoveryMultiKeyring as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_discovery_multi_keyring::builders::CreateAwsKmsMrkDiscoveryMultiKeyringInputBuilder{
        &self.inner
    }
    /// Sends the request and returns the response.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
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
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_mrk_discovery_multi_keyring::CreateAwsKmsMrkDiscoveryMultiKeyring::send(&self.client, input).await
    }

    /// The Client Supplier which will be used to get KMS Clients for use with this Keyring. If not specified on input, a Default Client Supplier is created which creates a KMS Client for each region in the 'regions' input.
    pub fn client_supplier(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef>,
    ) -> Self {
        self.inner = self.inner.client_supplier(input.into());
        self
    }
    /// The Client Supplier which will be used to get KMS Clients for use with this Keyring. If not specified on input, a Default Client Supplier is created which creates a KMS Client for each region in the 'regions' input.
    pub fn set_client_supplier(
        mut self,
        input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef>,
    ) -> Self {
        self.inner = self.inner.set_client_supplier(input);
        self
    }
    /// The Client Supplier which will be used to get KMS Clients for use with this Keyring. If not specified on input, a Default Client Supplier is created which creates a KMS Client for each region in the 'regions' input.
    pub fn get_client_supplier(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
    > {
        self.inner.get_client_supplier()
    }
    /// A filter which restricts which KMS Keys this Keyring may attempt to decrypt with by AWS partition and account.
    pub fn discovery_filter(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
        >,
    ) -> Self {
        self.inner = self.inner.discovery_filter(input.into());
        self
    }
    /// A filter which restricts which KMS Keys this Keyring may attempt to decrypt with by AWS partition and account.
    pub fn set_discovery_filter(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
        >,
    ) -> Self {
        self.inner = self.inner.set_discovery_filter(input);
        self
    }
    /// A filter which restricts which KMS Keys this Keyring may attempt to decrypt with by AWS partition and account.
    pub fn get_discovery_filter(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
    > {
        self.inner.get_discovery_filter()
    }
    /// A list of grant tokens to be used when calling KMS.
    pub fn grant_tokens(
        mut self,
        input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.grant_tokens(input.into());
        self
    }
    /// A list of grant tokens to be used when calling KMS.
    pub fn set_grant_tokens(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_grant_tokens(input);
        self
    }
    /// A list of grant tokens to be used when calling KMS.
    pub fn get_grant_tokens(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_grant_tokens()
    }
    /// The list of regions this Keyring will creates KMS clients for.
    pub fn regions(
        mut self,
        input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.regions(input.into());
        self
    }
    /// The list of regions this Keyring will creates KMS clients for.
    pub fn set_regions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_regions(input);
        self
    }
    /// The list of regions this Keyring will creates KMS clients for.
    pub fn get_regions(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_regions()
    }
}
