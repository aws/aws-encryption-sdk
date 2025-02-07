// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for creating a Hierarchical Keyring.
pub struct CreateAwsKmsHierarchicalKeyringInput {
    /// The identifier for the single Branch Key responsible for wrapping and unwrapping the data key. Either a Branch Key ID or Branch Key Supplier must be specified.
pub branch_key_id: ::std::option::Option<::std::string::String>,
/// A Branch Key Supplier which determines what Branch Key to use to wrap and unwrap the data key. Either a Branch Key ID or Branch Key Supplier must be specified.
pub branch_key_id_supplier: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::branch_key_id_supplier::BranchKeyIdSupplierRef>,
/// Sets the type of cache for this Hierarchical Keyring. By providing an already initialized 'Shared' cache, users can determine the scope of the cache. That is, if the cache is shared across other Cryptographic Material Providers, for instance other Hierarchical Keyrings or Caching Cryptographic Materials Managers (Caching CMMs). If any other type of cache in the CacheType union is provided, the Hierarchical Keyring will initialize a cache of that type, to be used with only this Hierarchical Keyring. If not set, a DefaultCache is initialized to be used with only this Hierarchical Keyring with entryCapacity = 1000.
pub cache: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::CacheType>,
/// The Key Store which contains the Branch Key(s) responsible for wrapping and unwrapping data keys.
pub key_store: ::std::option::Option<crate::deps::aws_cryptography_keyStore::client::Client>,
/// Partition ID to distinguish Cryptographic Material Providers (i.e: Keyrings) writing to a cache. If the Partition ID is the same for two Hierarchical Keyrings (or another Material Provider), they can share the same cache entries in the cache.
pub partition_id: ::std::option::Option<::std::string::String>,
/// How many seconds the Branch Key material is allowed to be reused within the local cache before it is re-retrieved from Amazon DynamoDB and re-authenticated with AWS KMS.
pub ttl_seconds: ::std::option::Option<::std::primitive::i64>,
}
impl CreateAwsKmsHierarchicalKeyringInput {
    /// The identifier for the single Branch Key responsible for wrapping and unwrapping the data key. Either a Branch Key ID or Branch Key Supplier must be specified.
    pub fn branch_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_id
    }
    /// A Branch Key Supplier which determines what Branch Key to use to wrap and unwrap the data key. Either a Branch Key ID or Branch Key Supplier must be specified.
    pub fn branch_key_id_supplier(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::branch_key_id_supplier::BranchKeyIdSupplierRef>{
        &self.branch_key_id_supplier
    }
    /// Sets the type of cache for this Hierarchical Keyring. By providing an already initialized 'Shared' cache, users can determine the scope of the cache. That is, if the cache is shared across other Cryptographic Material Providers, for instance other Hierarchical Keyrings or Caching Cryptographic Materials Managers (Caching CMMs). If any other type of cache in the CacheType union is provided, the Hierarchical Keyring will initialize a cache of that type, to be used with only this Hierarchical Keyring. If not set, a DefaultCache is initialized to be used with only this Hierarchical Keyring with entryCapacity = 1000.
    pub fn cache(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::CacheType>
    {
        &self.cache
    }
    /// The Key Store which contains the Branch Key(s) responsible for wrapping and unwrapping data keys.
    pub fn key_store(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_keyStore::client::Client> {
        &self.key_store
    }
    /// Partition ID to distinguish Cryptographic Material Providers (i.e: Keyrings) writing to a cache. If the Partition ID is the same for two Hierarchical Keyrings (or another Material Provider), they can share the same cache entries in the cache.
    pub fn partition_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.partition_id
    }
    /// How many seconds the Branch Key material is allowed to be reused within the local cache before it is re-retrieved from Amazon DynamoDB and re-authenticated with AWS KMS.
    pub fn ttl_seconds(&self) -> &::std::option::Option<::std::primitive::i64> {
        &self.ttl_seconds
    }
}
impl CreateAwsKmsHierarchicalKeyringInput {
    /// Creates a new builder-style object to manufacture [`CreateAwsKmsHierarchicalKeyringInput`](crate::operation::create_aws_kms_hierarchical_keyring::builders::CreateAwsKmsHierarchicalKeyringInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_hierarchical_keyring::builders::CreateAwsKmsHierarchicalKeyringInputBuilder{
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_hierarchical_keyring::builders::CreateAwsKmsHierarchicalKeyringInputBuilder::default()
    }
}

/// A builder for [`CreateAwsKmsHierarchicalKeyringInput`](crate::operation::operation::CreateAwsKmsHierarchicalKeyringInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateAwsKmsHierarchicalKeyringInputBuilder {
    pub(crate) branch_key_id: ::std::option::Option<::std::string::String>,
pub(crate) branch_key_id_supplier: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::branch_key_id_supplier::BranchKeyIdSupplierRef>,
pub(crate) cache: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::CacheType>,
pub(crate) key_store: ::std::option::Option<crate::deps::aws_cryptography_keyStore::client::Client>,
pub(crate) partition_id: ::std::option::Option<::std::string::String>,
pub(crate) ttl_seconds: ::std::option::Option<::std::primitive::i64>,
}
impl CreateAwsKmsHierarchicalKeyringInputBuilder {
    /// The identifier for the single Branch Key responsible for wrapping and unwrapping the data key. Either a Branch Key ID or Branch Key Supplier must be specified.
    pub fn branch_key_id(
        mut self,
        input: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.branch_key_id = ::std::option::Option::Some(input.into());
        self
    }
    /// The identifier for the single Branch Key responsible for wrapping and unwrapping the data key. Either a Branch Key ID or Branch Key Supplier must be specified.
    pub fn set_branch_key_id(
        mut self,
        input: ::std::option::Option<::std::string::String>,
    ) -> Self {
        self.branch_key_id = input;
        self
    }
    /// The identifier for the single Branch Key responsible for wrapping and unwrapping the data key. Either a Branch Key ID or Branch Key Supplier must be specified.
    pub fn get_branch_key_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.branch_key_id
    }
    /// A Branch Key Supplier which determines what Branch Key to use to wrap and unwrap the data key. Either a Branch Key ID or Branch Key Supplier must be specified.
    pub fn branch_key_id_supplier(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::branch_key_id_supplier::BranchKeyIdSupplierRef>,
    ) -> Self {
        self.branch_key_id_supplier = ::std::option::Option::Some(input.into());
        self
    }
    /// A Branch Key Supplier which determines what Branch Key to use to wrap and unwrap the data key. Either a Branch Key ID or Branch Key Supplier must be specified.
    pub fn set_branch_key_id_supplier(
        mut self,
        input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::branch_key_id_supplier::BranchKeyIdSupplierRef>,
    ) -> Self {
        self.branch_key_id_supplier = input;
        self
    }
    /// A Branch Key Supplier which determines what Branch Key to use to wrap and unwrap the data key. Either a Branch Key ID or Branch Key Supplier must be specified.
    pub fn get_branch_key_id_supplier(&self) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::branch_key_id_supplier::BranchKeyIdSupplierRef>{
        &self.branch_key_id_supplier
    }
    /// Sets the type of cache for this Hierarchical Keyring. By providing an already initialized 'Shared' cache, users can determine the scope of the cache. That is, if the cache is shared across other Cryptographic Material Providers, for instance other Hierarchical Keyrings or Caching Cryptographic Materials Managers (Caching CMMs). If any other type of cache in the CacheType union is provided, the Hierarchical Keyring will initialize a cache of that type, to be used with only this Hierarchical Keyring. If not set, a DefaultCache is initialized to be used with only this Hierarchical Keyring with entryCapacity = 1000.
    pub fn cache(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::CacheType,
        >,
    ) -> Self {
        self.cache = ::std::option::Option::Some(input.into());
        self
    }
    /// Sets the type of cache for this Hierarchical Keyring. By providing an already initialized 'Shared' cache, users can determine the scope of the cache. That is, if the cache is shared across other Cryptographic Material Providers, for instance other Hierarchical Keyrings or Caching Cryptographic Materials Managers (Caching CMMs). If any other type of cache in the CacheType union is provided, the Hierarchical Keyring will initialize a cache of that type, to be used with only this Hierarchical Keyring. If not set, a DefaultCache is initialized to be used with only this Hierarchical Keyring with entryCapacity = 1000.
    pub fn set_cache(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::CacheType,
        >,
    ) -> Self {
        self.cache = input;
        self
    }
    /// Sets the type of cache for this Hierarchical Keyring. By providing an already initialized 'Shared' cache, users can determine the scope of the cache. That is, if the cache is shared across other Cryptographic Material Providers, for instance other Hierarchical Keyrings or Caching Cryptographic Materials Managers (Caching CMMs). If any other type of cache in the CacheType union is provided, the Hierarchical Keyring will initialize a cache of that type, to be used with only this Hierarchical Keyring. If not set, a DefaultCache is initialized to be used with only this Hierarchical Keyring with entryCapacity = 1000.
    pub fn get_cache(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::CacheType>
    {
        &self.cache
    }
    /// The Key Store which contains the Branch Key(s) responsible for wrapping and unwrapping data keys.
    pub fn key_store(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_keyStore::client::Client>,
    ) -> Self {
        self.key_store = ::std::option::Option::Some(input.into());
        self
    }
    /// The Key Store which contains the Branch Key(s) responsible for wrapping and unwrapping data keys.
    pub fn set_key_store(
        mut self,
        input: ::std::option::Option<crate::deps::aws_cryptography_keyStore::client::Client>,
    ) -> Self {
        self.key_store = input;
        self
    }
    /// The Key Store which contains the Branch Key(s) responsible for wrapping and unwrapping data keys.
    pub fn get_key_store(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_keyStore::client::Client> {
        &self.key_store
    }
    /// Partition ID to distinguish Cryptographic Material Providers (i.e: Keyrings) writing to a cache. If the Partition ID is the same for two Hierarchical Keyrings (or another Material Provider), they can share the same cache entries in the cache.
    pub fn partition_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.partition_id = ::std::option::Option::Some(input.into());
        self
    }
    /// Partition ID to distinguish Cryptographic Material Providers (i.e: Keyrings) writing to a cache. If the Partition ID is the same for two Hierarchical Keyrings (or another Material Provider), they can share the same cache entries in the cache.
    pub fn set_partition_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.partition_id = input;
        self
    }
    /// Partition ID to distinguish Cryptographic Material Providers (i.e: Keyrings) writing to a cache. If the Partition ID is the same for two Hierarchical Keyrings (or another Material Provider), they can share the same cache entries in the cache.
    pub fn get_partition_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.partition_id
    }
    /// How many seconds the Branch Key material is allowed to be reused within the local cache before it is re-retrieved from Amazon DynamoDB and re-authenticated with AWS KMS.
    pub fn ttl_seconds(mut self, input: impl ::std::convert::Into<::std::primitive::i64>) -> Self {
        self.ttl_seconds = ::std::option::Option::Some(input.into());
        self
    }
    /// How many seconds the Branch Key material is allowed to be reused within the local cache before it is re-retrieved from Amazon DynamoDB and re-authenticated with AWS KMS.
    pub fn set_ttl_seconds(mut self, input: ::std::option::Option<::std::primitive::i64>) -> Self {
        self.ttl_seconds = input;
        self
    }
    /// How many seconds the Branch Key material is allowed to be reused within the local cache before it is re-retrieved from Amazon DynamoDB and re-authenticated with AWS KMS.
    pub fn get_ttl_seconds(&self) -> &::std::option::Option<::std::primitive::i64> {
        &self.ttl_seconds
    }
    /// Consumes the builder and constructs a [`CreateAwsKmsHierarchicalKeyringInput`](crate::operation::operation::CreateAwsKmsHierarchicalKeyringInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_hierarchical_keyring::CreateAwsKmsHierarchicalKeyringInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::operation::create_aws_kms_hierarchical_keyring::CreateAwsKmsHierarchicalKeyringInput {
            branch_key_id: self.branch_key_id,
branch_key_id_supplier: self.branch_key_id_supplier,
cache: self.cache,
key_store: self.key_store,
partition_id: self.partition_id,
ttl_seconds: self.ttl_seconds,
        })
    }
}
