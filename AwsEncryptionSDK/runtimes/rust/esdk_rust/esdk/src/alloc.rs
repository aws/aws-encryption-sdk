// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#![allow(dead_code)]
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

pub(crate) struct ResourceTracker {
    pub count: usize,
    pub total: usize,
}

#[derive(Debug, Default, Clone)]
pub(crate) struct ResourceResults {
    pub count_k: usize,
    pub total_m: usize,
}

impl ResourceTracker {
    pub(crate) fn new() -> Self {
        Self {
            count: get_counter(),
            total: get_total(),
        }
    }
    pub(crate) fn reset(&mut self) {
        self.count = get_counter();
        self.total = get_total();
    }
    pub(crate) fn report_leak() {
        println!(
            "{} outstanding allocations totalling {} bytes.",
            get_net_counter(),
            get_net_total()
        );
    }
    pub(crate) fn get_results(&self) -> ResourceResults {
        ResourceResults {
            count_k: (get_counter() - self.count) / 1000,
            total_m: (get_total() - self.total) / 1_000_000,
        }
    }
    pub(crate) fn report(&self, tag: &str) {
        println!(
            "{tag} : {} allocations totalling {} bytes.",
            get_counter() - self.count,
            get_total() - self.total
        );
    }
    pub(crate) fn report_reset(&mut self, tag: &str) {
        self.report(tag);
        self.reset();
    }
}

static COUNTER: AtomicUsize = AtomicUsize::new(0);
static TOTAL: AtomicUsize = AtomicUsize::new(0);

static NET_COUNTER: AtomicUsize = AtomicUsize::new(0);
static NET_TOTAL: AtomicUsize = AtomicUsize::new(0);

fn add_to_counter(inc: usize) {
    COUNTER.fetch_add(1, Ordering::SeqCst);
    TOTAL.fetch_add(inc, Ordering::SeqCst);
    NET_COUNTER.fetch_add(1, Ordering::SeqCst);
    NET_TOTAL.fetch_add(inc, Ordering::SeqCst);
}

fn subtract_from_counter(inc: usize) {
    NET_COUNTER.fetch_sub(1, Ordering::SeqCst);
    NET_TOTAL.fetch_sub(inc, Ordering::SeqCst);
}

fn get_counter() -> usize {
    COUNTER.load(Ordering::SeqCst)
}

fn get_total() -> usize {
    TOTAL.load(Ordering::SeqCst)
}

fn get_net_counter() -> usize {
    NET_COUNTER.load(Ordering::SeqCst)
}

fn get_net_total() -> usize {
    NET_TOTAL.load(Ordering::SeqCst)
}

use std::alloc::{GlobalAlloc, Layout, System};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        add_to_counter(layout.size());
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        subtract_from_counter(layout.size());
        unsafe { System.dealloc(ptr, layout) }
    }
}

// #[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;
