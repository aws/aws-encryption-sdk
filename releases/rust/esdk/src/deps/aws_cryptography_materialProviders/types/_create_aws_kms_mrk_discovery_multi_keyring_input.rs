// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// Inputs for for creating a AWS KMS MRK Discovery Multi-Keyring.
pub struct CreateAwsKmsMrkDiscoveryMultiKeyringInput {
    /// The Client Supplier which will be used to get KMS Clients for use with this Keyring. If not specified on input, a Default Client Supplier is created which creates a KMS Client for each region in the 'regions' input.
    pub client_supplier: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
    >,
    /// A filter which restricts which KMS Keys this Keyring may attempt to decrypt with by AWS partition and account.
    pub discovery_filter: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
    >,
    /// A list of grant tokens to be used when calling KMS.
    pub grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// The list of regions this Keyring will creates KMS clients for.
    pub regions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl CreateAwsKmsMrkDiscoveryMultiKeyringInput {
    /// The Client Supplier which will be used to get KMS Clients for use with this Keyring. If not specified on input, a Default Client Supplier is created which creates a KMS Client for each region in the 'regions' input.
    pub fn client_supplier(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
    > {
        &self.client_supplier
    }
    /// A filter which restricts which KMS Keys this Keyring may attempt to decrypt with by AWS partition and account.
    pub fn discovery_filter(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
    > {
        &self.discovery_filter
    }
    /// A list of grant tokens to be used when calling KMS.
    pub fn grant_tokens(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.grant_tokens
    }
    /// The list of regions this Keyring will creates KMS clients for.
    pub fn regions(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.regions
    }
}
impl CreateAwsKmsMrkDiscoveryMultiKeyringInput {
    /// Creates a new builder-style object to manufacture [`CreateAwsKmsMrkDiscoveryMultiKeyringInput`](crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsMrkDiscoveryMultiKeyringInput).
    pub fn builder() -> crate::deps::aws_cryptography_materialProviders::types::builders::CreateAwsKmsMrkDiscoveryMultiKeyringInputBuilder{
        crate::deps::aws_cryptography_materialProviders::types::builders::CreateAwsKmsMrkDiscoveryMultiKeyringInputBuilder::default()
    }
}

/// A builder for [`CreateAwsKmsMrkDiscoveryMultiKeyringInput`](crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsMrkDiscoveryMultiKeyringInput).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct CreateAwsKmsMrkDiscoveryMultiKeyringInputBuilder {
    pub(crate) client_supplier: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
    >,
    pub(crate) discovery_filter: ::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
    >,
    pub(crate) grant_tokens: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) regions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
impl CreateAwsKmsMrkDiscoveryMultiKeyringInputBuilder {
    /// The Client Supplier which will be used to get KMS Clients for use with this Keyring. If not specified on input, a Default Client Supplier is created which creates a KMS Client for each region in the 'regions' input.
    pub fn client_supplier(
        mut self,
        input: impl ::std::convert::Into<crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef>,
    ) -> Self {
        self.client_supplier = ::std::option::Option::Some(input.into());
        self
    }
    /// The Client Supplier which will be used to get KMS Clients for use with this Keyring. If not specified on input, a Default Client Supplier is created which creates a KMS Client for each region in the 'regions' input.
    pub fn set_client_supplier(
        mut self,
        input: ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef>,
    ) -> Self {
        self.client_supplier = input;
        self
    }
    /// The Client Supplier which will be used to get KMS Clients for use with this Keyring. If not specified on input, a Default Client Supplier is created which creates a KMS Client for each region in the 'regions' input.
    pub fn get_client_supplier(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::client_supplier::ClientSupplierRef,
    > {
        &self.client_supplier
    }
    /// A filter which restricts which KMS Keys this Keyring may attempt to decrypt with by AWS partition and account.
    pub fn discovery_filter(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
        >,
    ) -> Self {
        self.discovery_filter = ::std::option::Option::Some(input.into());
        self
    }
    /// A filter which restricts which KMS Keys this Keyring may attempt to decrypt with by AWS partition and account.
    pub fn set_discovery_filter(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
        >,
    ) -> Self {
        self.discovery_filter = input;
        self
    }
    /// A filter which restricts which KMS Keys this Keyring may attempt to decrypt with by AWS partition and account.
    pub fn get_discovery_filter(
        &self,
    ) -> &::std::option::Option<
        crate::deps::aws_cryptography_materialProviders::types::DiscoveryFilter,
    > {
        &self.discovery_filter
    }
    /// A list of grant tokens to be used when calling KMS.
    pub fn grant_tokens(
        mut self,
        input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.grant_tokens = ::std::option::Option::Some(input.into());
        self
    }
    /// A list of grant tokens to be used when calling KMS.
    pub fn set_grant_tokens(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.grant_tokens = input;
        self
    }
    /// A list of grant tokens to be used when calling KMS.
    pub fn get_grant_tokens(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.grant_tokens
    }
    /// The list of regions this Keyring will creates KMS clients for.
    pub fn regions(
        mut self,
        input: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.regions = ::std::option::Option::Some(input.into());
        self
    }
    /// The list of regions this Keyring will creates KMS clients for.
    pub fn set_regions(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    ) -> Self {
        self.regions = input;
        self
    }
    /// The list of regions this Keyring will creates KMS clients for.
    pub fn get_regions(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.regions
    }
    /// Consumes the builder and constructs a [`CreateAwsKmsMrkDiscoveryMultiKeyringInput`](crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsMrkDiscoveryMultiKeyringInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsMrkDiscoveryMultiKeyringInput,
        ::aws_smithy_types::error::operation::BuildError,
    >{
        ::std::result::Result::Ok(crate::deps::aws_cryptography_materialProviders::types::CreateAwsKmsMrkDiscoveryMultiKeyringInput {
            client_supplier: self.client_supplier,
discovery_filter: self.discovery_filter,
grant_tokens: self.grant_tokens,
regions: self.regions,
        })
    }
}
