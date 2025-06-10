// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// A cache that is safe for use in a multi threaded environment, but no extra functionality.
pub struct MultiThreadedCache {
    /// Maximum number of entries cached.
    pub entry_capacity: ::std::option::Option<::std::primitive::i32>,
    /// Number of entries to prune at a time.
    pub entry_pruning_tail_size: ::std::option::Option<::std::primitive::i32>,
}
impl MultiThreadedCache {
    /// Maximum number of entries cached.
    pub fn entry_capacity(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.entry_capacity
    }
    /// Number of entries to prune at a time.
    pub fn entry_pruning_tail_size(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.entry_pruning_tail_size
    }
}
impl MultiThreadedCache {
    /// Creates a new builder-style object to manufacture [`MultiThreadedCache`](crate::deps::aws_cryptography_materialProviders::types::MultiThreadedCache).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_materialProviders::types::builders::MultiThreadedCacheBuilder
    {
        crate::deps::aws_cryptography_materialProviders::types::builders::MultiThreadedCacheBuilder::default()
    }
}

/// A builder for [`MultiThreadedCache`](crate::deps::aws_cryptography_materialProviders::types::MultiThreadedCache).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct MultiThreadedCacheBuilder {
    pub(crate) entry_capacity: ::std::option::Option<::std::primitive::i32>,
    pub(crate) entry_pruning_tail_size: ::std::option::Option<::std::primitive::i32>,
}
impl MultiThreadedCacheBuilder {
    /// Maximum number of entries cached.
    pub fn entry_capacity(
        mut self,
        input: impl ::std::convert::Into<::std::primitive::i32>,
    ) -> Self {
        self.entry_capacity = ::std::option::Option::Some(input.into());
        self
    }
    /// Maximum number of entries cached.
    pub fn set_entry_capacity(
        mut self,
        input: ::std::option::Option<::std::primitive::i32>,
    ) -> Self {
        self.entry_capacity = input;
        self
    }
    /// Maximum number of entries cached.
    pub fn get_entry_capacity(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.entry_capacity
    }
    /// Number of entries to prune at a time.
    pub fn entry_pruning_tail_size(
        mut self,
        input: impl ::std::convert::Into<::std::primitive::i32>,
    ) -> Self {
        self.entry_pruning_tail_size = ::std::option::Option::Some(input.into());
        self
    }
    /// Number of entries to prune at a time.
    pub fn set_entry_pruning_tail_size(
        mut self,
        input: ::std::option::Option<::std::primitive::i32>,
    ) -> Self {
        self.entry_pruning_tail_size = input;
        self
    }
    /// Number of entries to prune at a time.
    pub fn get_entry_pruning_tail_size(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.entry_pruning_tail_size
    }
    /// Consumes the builder and constructs a [`MultiThreadedCache`](crate::deps::aws_cryptography_materialProviders::types::MultiThreadedCache).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::MultiThreadedCache,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::MultiThreadedCache {
                entry_capacity: self.entry_capacity,
                entry_pruning_tail_size: self.entry_pruning_tail_size,
            },
        )
    }
}
