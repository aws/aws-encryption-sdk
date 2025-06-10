// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
pub use crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_hierarchical_keyring::_create_keyring_output::CreateKeyringOutputBuilder;

pub use crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_hierarchical_keyring::_create_aws_kms_hierarchical_keyring_input::CreateAwsKmsHierarchicalKeyringInputBuilder;

impl CreateAwsKmsHierarchicalKeyringInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::keyring::KeyringRef,
        crate::deps::aws_cryptography_materialProviders::types::error::Error,
    > {
        let mut fluent_builder = client.create_aws_kms_hierarchical_keyring();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateAwsKmsHierarchicalKeyring`.
///
/// Creates a Hierarchical Keyring, which supports wrapping and unwrapping data keys using Branch Keys persisted in DynamoDB and protected by a symmetric AWS KMS Key or AWS KMS Multi-Region Key.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateAwsKmsHierarchicalKeyringFluentBuilder {
    client: crate::deps::aws_cryptography_materialProviders::client::Client,
    pub(crate) inner: crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_hierarchical_keyring::builders::CreateAwsKmsHierarchicalKeyringInputBuilder,
}
impl CreateAwsKmsHierarchicalKeyringFluentBuilder {
    /// Creates a new `CreateAwsKmsHierarchicalKeyring`.
    pub(crate) fn new(
        client: crate::deps::aws_cryptography_materialProviders::client::Client,
    ) -> Self {
        Self {
            client,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the CreateAwsKmsHierarchicalKeyring as a reference.
    pub fn as_input(&self) -> &crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_hierarchical_keyring::builders::CreateAwsKmsHierarchicalKeyringInputBuilder{
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
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_hierarchical_keyring::CreateAwsKmsHierarchicalKeyring::send(&self.client, input).await
    }

    /// The identifier for the single Branch Key responsible for wrapping and unwrapping the data key. Either a Branch Key ID or Branch Key Supplier must be specified.
    pub fn branch_key_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.branch_key_id(input.into());
        self
    }
    /// The identifier for the single Branch Key responsible for wrapping and unwrapping the data key. Either a Branch Key ID or Branch Key Supplier must be specified.
    pub fn set_branch_key_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.set_branch_key_id(input);
        self
    }
    /// The identifier for the single Branch Key responsible for wrapping and unwrapping the data key. Either a Branch Key ID or Branch Key Supplier must be specified.
    pub fn get_branch_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_branch_key_id()
    }
    /// A Branch Key Supplier which determines what Branch Key to use to wrap and unwrap the data key. Either a Branch Key ID or Branch Key Supplier must be specified.
    pub fn branch_key_id_supplier(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::branch_key_id_supplier::BranchKeyIdSupplierRef>,
    ) -> Self {
        self.inner = self.inner.branch_key_id_supplier(input.into());
        self
    }
    /// A Branch Key Supplier which determines what Branch Key to use to wrap and unwrap the data key. Either a Branch Key ID or Branch Key Supplier must be specified.
    pub fn set_branch_key_id_supplier(
        mut self,
        input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::branch_key_id_supplier::BranchKeyIdSupplierRef>,
    ) -> Self {
        self.inner = self.inner.set_branch_key_id_supplier(input);
        self
    }
    /// A Branch Key Supplier which determines what Branch Key to use to wrap and unwrap the data key. Either a Branch Key ID or Branch Key Supplier must be specified.
    pub fn get_branch_key_id_supplier(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::branch_key_id_supplier::BranchKeyIdSupplierRef>{
        self.inner.get_branch_key_id_supplier()
    }
    /// Sets the type of cache for this Hierarchical Keyring. By providing an already initialized 'Shared' cache, users can determine the scope of the cache. That is, if the cache is shared across other Cryptographic Material Providers, for instance other Hierarchical Keyrings or Caching Cryptographic Materials Managers (Caching CMMs). If any other type of cache in the CacheType union is provided, the Hierarchical Keyring will initialize a cache of that type, to be used with only this Hierarchical Keyring. If not set, a DefaultCache is initialized to be used with only this Hierarchical Keyring with entryCapacity = 1000.
    pub fn cache(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::CacheType,
        >,
    ) -> Self {
        self.inner = self.inner.cache(input.into());
        self
    }
    /// Sets the type of cache for this Hierarchical Keyring. By providing an already initialized 'Shared' cache, users can determine the scope of the cache. That is, if the cache is shared across other Cryptographic Material Providers, for instance other Hierarchical Keyrings or Caching Cryptographic Materials Managers (Caching CMMs). If any other type of cache in the CacheType union is provided, the Hierarchical Keyring will initialize a cache of that type, to be used with only this Hierarchical Keyring. If not set, a DefaultCache is initialized to be used with only this Hierarchical Keyring with entryCapacity = 1000.
    pub fn set_cache(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::CacheType,
        >,
    ) -> Self {
        self.inner = self.inner.set_cache(input);
        self
    }
    /// Sets the type of cache for this Hierarchical Keyring. By providing an already initialized 'Shared' cache, users can determine the scope of the cache. That is, if the cache is shared across other Cryptographic Material Providers, for instance other Hierarchical Keyrings or Caching Cryptographic Materials Managers (Caching CMMs). If any other type of cache in the CacheType union is provided, the Hierarchical Keyring will initialize a cache of that type, to be used with only this Hierarchical Keyring. If not set, a DefaultCache is initialized to be used with only this Hierarchical Keyring with entryCapacity = 1000.
    pub fn get_cache(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::CacheType>
    {
        self.inner.get_cache()
    }
    /// The Key Store which contains the Branch Key(s) responsible for wrapping and unwrapping data keys.
    pub fn key_store(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_keyStore::client::Client>,
    ) -> Self {
        self.inner = self.inner.key_store(input.into());
        self
    }
    /// The Key Store which contains the Branch Key(s) responsible for wrapping and unwrapping data keys.
    pub fn set_key_store(
        mut self,
        input: ::std::option::Option<crate::deps::aws_cryptography_keyStore::client::Client>,
    ) -> Self {
        self.inner = self.inner.set_key_store(input);
        self
    }
    /// The Key Store which contains the Branch Key(s) responsible for wrapping and unwrapping data keys.
    pub fn get_key_store(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_keyStore::client::Client> {
        self.inner.get_key_store()
    }
    /// Partition ID to distinguish Cryptographic Material Providers (i.e: Keyrings) writing to a cache. If the Partition ID is the same for two Hierarchical Keyrings (or another Material Provider), they can share the same cache entries in the cache.
    pub fn partition_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.partition_id(input.into());
        self
    }
    /// Partition ID to distinguish Cryptographic Material Providers (i.e: Keyrings) writing to a cache. If the Partition ID is the same for two Hierarchical Keyrings (or another Material Provider), they can share the same cache entries in the cache.
    pub fn set_partition_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_partition_id(input);
        self
    }
    /// Partition ID to distinguish Cryptographic Material Providers (i.e: Keyrings) writing to a cache. If the Partition ID is the same for two Hierarchical Keyrings (or another Material Provider), they can share the same cache entries in the cache.
    pub fn get_partition_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_partition_id()
    }
    /// How many seconds the Branch Key material is allowed to be reused within the local cache before it is re-retrieved from Amazon DynamoDB and re-authenticated with AWS KMS.
    pub fn ttl_seconds(mut self, input: impl ::std::convert::Into<::std::primitive::i64>) -> Self {
        self.inner = self.inner.ttl_seconds(input.into());
        self
    }
    /// How many seconds the Branch Key material is allowed to be reused within the local cache before it is re-retrieved from Amazon DynamoDB and re-authenticated with AWS KMS.
    pub fn set_ttl_seconds(mut self, input: ::std::option::Option<::std::primitive::i64>) -> Self {
        self.inner = self.inner.set_ttl_seconds(input);
        self
    }
    /// How many seconds the Branch Key material is allowed to be reused within the local cache before it is re-retrieved from Amazon DynamoDB and re-authenticated with AWS KMS.
    pub fn get_ttl_seconds(&self) -> &::std::option::Option<::std::primitive::i64> {
        self.inner.get_ttl_seconds()
    }
}
