// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
/// A cache that is safe for use in a multi threaded environment,
/// and tries to prevent redundant or overly parallel backend calls.
pub struct StormTrackingCache {
    /// Maximum number of entries cached.
    pub entry_capacity: ::std::option::Option<::std::primitive::i32>,
    /// Number of entries to prune at a time.
    pub entry_pruning_tail_size: ::std::option::Option<::std::primitive::i32>,
    /// How many simultaneous attempts to refresh the materials.
    pub fan_out: ::std::option::Option<::std::primitive::i32>,
    /// How much time between attempts to refresh the materials.
    pub grace_interval: ::std::option::Option<::std::primitive::i32>,
    /// How much time before expiration should an attempt be made to refresh the materials.
    ///   If zero, use a simple cache with no storm tracking.
    pub grace_period: ::std::option::Option<::std::primitive::i32>,
    /// How much time until an attempt to refresh the materials should be forgotten.
    pub in_flight_ttl: ::std::option::Option<::std::primitive::i32>,
    /// How many milliseconds should a thread sleep if fanOut is exceeded.
    pub sleep_milli: ::std::option::Option<::std::primitive::i32>,
    /// The time unit for gracePeriod, graceInterval, and inFlightTTL.
    ///   The default is seconds.
    ///   If this is set to milliseconds, then these values will be treated as milliseconds.
    pub time_units:
        ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::TimeUnits>,
}
impl StormTrackingCache {
    /// Maximum number of entries cached.
    pub fn entry_capacity(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.entry_capacity
    }
    /// Number of entries to prune at a time.
    pub fn entry_pruning_tail_size(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.entry_pruning_tail_size
    }
    /// How many simultaneous attempts to refresh the materials.
    pub fn fan_out(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.fan_out
    }
    /// How much time between attempts to refresh the materials.
    pub fn grace_interval(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.grace_interval
    }
    /// How much time before expiration should an attempt be made to refresh the materials.
    ///   If zero, use a simple cache with no storm tracking.
    pub fn grace_period(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.grace_period
    }
    /// How much time until an attempt to refresh the materials should be forgotten.
    pub fn in_flight_ttl(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.in_flight_ttl
    }
    /// How many milliseconds should a thread sleep if fanOut is exceeded.
    pub fn sleep_milli(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.sleep_milli
    }
    /// The time unit for gracePeriod, graceInterval, and inFlightTTL.
    ///   The default is seconds.
    ///   If this is set to milliseconds, then these values will be treated as milliseconds.
    pub fn time_units(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::TimeUnits>
    {
        &self.time_units
    }
}
impl StormTrackingCache {
    /// Creates a new builder-style object to manufacture [`StormTrackingCache`](crate::deps::aws_cryptography_materialProviders::types::StormTrackingCache).
    pub fn builder(
    ) -> crate::deps::aws_cryptography_materialProviders::types::builders::StormTrackingCacheBuilder
    {
        crate::deps::aws_cryptography_materialProviders::types::builders::StormTrackingCacheBuilder::default()
    }
}

/// A builder for [`StormTrackingCache`](crate::deps::aws_cryptography_materialProviders::types::StormTrackingCache).
#[non_exhaustive]
#[derive(
    ::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug,
)]
pub struct StormTrackingCacheBuilder {
    pub(crate) entry_capacity: ::std::option::Option<::std::primitive::i32>,
    pub(crate) entry_pruning_tail_size: ::std::option::Option<::std::primitive::i32>,
    pub(crate) fan_out: ::std::option::Option<::std::primitive::i32>,
    pub(crate) grace_interval: ::std::option::Option<::std::primitive::i32>,
    pub(crate) grace_period: ::std::option::Option<::std::primitive::i32>,
    pub(crate) in_flight_ttl: ::std::option::Option<::std::primitive::i32>,
    pub(crate) sleep_milli: ::std::option::Option<::std::primitive::i32>,
    pub(crate) time_units:
        ::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::TimeUnits>,
}
impl StormTrackingCacheBuilder {
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
    /// How many simultaneous attempts to refresh the materials.
    pub fn fan_out(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
        self.fan_out = ::std::option::Option::Some(input.into());
        self
    }
    /// How many simultaneous attempts to refresh the materials.
    pub fn set_fan_out(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
        self.fan_out = input;
        self
    }
    /// How many simultaneous attempts to refresh the materials.
    pub fn get_fan_out(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.fan_out
    }
    /// How much time between attempts to refresh the materials.
    pub fn grace_interval(
        mut self,
        input: impl ::std::convert::Into<::std::primitive::i32>,
    ) -> Self {
        self.grace_interval = ::std::option::Option::Some(input.into());
        self
    }
    /// How much time between attempts to refresh the materials.
    pub fn set_grace_interval(
        mut self,
        input: ::std::option::Option<::std::primitive::i32>,
    ) -> Self {
        self.grace_interval = input;
        self
    }
    /// How much time between attempts to refresh the materials.
    pub fn get_grace_interval(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.grace_interval
    }
    /// How much time before expiration should an attempt be made to refresh the materials.
    ///   If zero, use a simple cache with no storm tracking.
    pub fn grace_period(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
        self.grace_period = ::std::option::Option::Some(input.into());
        self
    }
    /// How much time before expiration should an attempt be made to refresh the materials.
    ///   If zero, use a simple cache with no storm tracking.
    pub fn set_grace_period(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
        self.grace_period = input;
        self
    }
    /// How much time before expiration should an attempt be made to refresh the materials.
    ///   If zero, use a simple cache with no storm tracking.
    pub fn get_grace_period(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.grace_period
    }
    /// How much time until an attempt to refresh the materials should be forgotten.
    pub fn in_flight_ttl(
        mut self,
        input: impl ::std::convert::Into<::std::primitive::i32>,
    ) -> Self {
        self.in_flight_ttl = ::std::option::Option::Some(input.into());
        self
    }
    /// How much time until an attempt to refresh the materials should be forgotten.
    pub fn set_in_flight_ttl(
        mut self,
        input: ::std::option::Option<::std::primitive::i32>,
    ) -> Self {
        self.in_flight_ttl = input;
        self
    }
    /// How much time until an attempt to refresh the materials should be forgotten.
    pub fn get_in_flight_ttl(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.in_flight_ttl
    }
    /// How many milliseconds should a thread sleep if fanOut is exceeded.
    pub fn sleep_milli(mut self, input: impl ::std::convert::Into<::std::primitive::i32>) -> Self {
        self.sleep_milli = ::std::option::Option::Some(input.into());
        self
    }
    /// How many milliseconds should a thread sleep if fanOut is exceeded.
    pub fn set_sleep_milli(mut self, input: ::std::option::Option<::std::primitive::i32>) -> Self {
        self.sleep_milli = input;
        self
    }
    /// How many milliseconds should a thread sleep if fanOut is exceeded.
    pub fn get_sleep_milli(&self) -> &::std::option::Option<::std::primitive::i32> {
        &self.sleep_milli
    }
    /// The time unit for gracePeriod, graceInterval, and inFlightTTL.
    ///   The default is seconds.
    ///   If this is set to milliseconds, then these values will be treated as milliseconds.
    pub fn time_units(
        mut self,
        input: impl ::std::convert::Into<
            crate::deps::aws_cryptography_materialProviders::types::TimeUnits,
        >,
    ) -> Self {
        self.time_units = ::std::option::Option::Some(input.into());
        self
    }
    /// The time unit for gracePeriod, graceInterval, and inFlightTTL.
    ///   The default is seconds.
    ///   If this is set to milliseconds, then these values will be treated as milliseconds.
    pub fn set_time_units(
        mut self,
        input: ::std::option::Option<
            crate::deps::aws_cryptography_materialProviders::types::TimeUnits,
        >,
    ) -> Self {
        self.time_units = input;
        self
    }
    /// The time unit for gracePeriod, graceInterval, and inFlightTTL.
    ///   The default is seconds.
    ///   If this is set to milliseconds, then these values will be treated as milliseconds.
    pub fn get_time_units(
        &self,
    ) -> &::std::option::Option<crate::deps::aws_cryptography_materialProviders::types::TimeUnits>
    {
        &self.time_units
    }
    /// Consumes the builder and constructs a [`StormTrackingCache`](crate::deps::aws_cryptography_materialProviders::types::StormTrackingCache).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::deps::aws_cryptography_materialProviders::types::StormTrackingCache,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(
            crate::deps::aws_cryptography_materialProviders::types::StormTrackingCache {
                entry_capacity: self.entry_capacity,
                entry_pruning_tail_size: self.entry_pruning_tail_size,
                fan_out: self.fan_out,
                grace_interval: self.grace_interval,
                grace_period: self.grace_period,
                in_flight_ttl: self.in_flight_ttl,
                sleep_milli: self.sleep_milli,
                time_units: self.time_units,
            },
        )
    }
}
