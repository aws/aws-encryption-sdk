// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Track memory allocations. Use `track` feature to turn on the global allocator tracking.

use crate::format::format_power2;
use std::sync::atomic::AtomicU64;
use std::sync::atomic::Ordering;

/// Tracks resource usage of various kinds of allocations
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct ResourceTracker {
    /// Total number of allocations
    pub count: u64,
    /// Total number of bytes allocated
    pub total: u64,
}

/// Results of tracking allocations
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[non_exhaustive]
pub struct ResourceResults {
    /// Total number of allocations
    pub count: u64,
    /// Total number of bytes allocated
    pub total: u64,
}

impl ResourceTracker {
    /// Create a new resource tracker
    #[must_use]
    pub fn new() -> Self {
        Self {
            count: get_counter(),
            total: get_total(),
        }
    }

    /// Reset the resource tracker to current values
    pub fn reset(&mut self) {
        self.count = get_counter();
        self.total = get_total();
    }

    /// Report outstanding allocations
    pub fn report_leak() {
        println!(
            "{} outstanding allocations totalling {} bytes.",
            get_net_counter(),
            get_net_total()
        );
    }

    #[must_use]
    /// Get the results of tracking so far
    pub fn get_results(&self) -> ResourceResults {
        ResourceResults {
            count: get_counter() - self.count,
            total: get_total() - self.total,
        }
    }

    /// Print a one line report about allocations
    pub fn report(&self, tag: &str) {
        println!(
            "{tag} : {} allocations totalling {} bytes.",
            format_power2(get_counter() - self.count),
            format_power2(get_total() - self.total)
        );
    }

    /// `report` and then `reset`. Useful for tracking allocations in multiple places.
    pub fn report_reset(&mut self, tag: &str) {
        self.report(tag);
        self.reset();
    }
}

static COUNTER: AtomicU64 = AtomicU64::new(0);
static TOTAL: AtomicU64 = AtomicU64::new(0);

static NET_COUNTER: AtomicU64 = AtomicU64::new(0);
static NET_TOTAL: AtomicU64 = AtomicU64::new(0);

#[allow(dead_code, reason = "only used with track feature")]
pub(crate) fn add_to_counter(inc: u64) {
    COUNTER.fetch_add(1, Ordering::SeqCst);
    TOTAL.fetch_add(inc, Ordering::SeqCst);
    NET_COUNTER.fetch_add(1, Ordering::SeqCst);
    NET_TOTAL.fetch_add(inc, Ordering::SeqCst);
}

#[allow(dead_code, reason = "only used with track feature")]
pub(crate) fn subtract_from_counter(inc: u64) {
    NET_COUNTER.fetch_sub(1, Ordering::SeqCst);
    NET_TOTAL.fetch_sub(inc, Ordering::SeqCst);
}

fn get_counter() -> u64 {
    COUNTER.load(Ordering::SeqCst)
}

fn get_total() -> u64 {
    TOTAL.load(Ordering::SeqCst)
}

fn get_net_counter() -> u64 {
    NET_COUNTER.load(Ordering::SeqCst)
}

fn get_net_total() -> u64 {
    NET_TOTAL.load(Ordering::SeqCst)
}
