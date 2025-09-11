// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
use serde::Serialize;
use std::sync::atomic::AtomicIsize;
use std::sync::atomic::Ordering;

pub struct ResourceTracker {
    pub count: isize,
    pub total: isize,
    pub net_total: isize,
    pub net_count: isize,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct ResourceResults {
    pub count_k: isize,
    pub total_m: isize,
    pub max_bytes_m: isize,
    pub net_count: isize,
    pub net_total: isize,
}

impl ResourceTracker {
    pub fn new() -> Self {
        clear_max();
        Self {
            count: get_counter(),
            total: get_total(),
            net_total: get_net_total(),
            net_count: get_net_counter(),
        }
    }
    pub fn get_results(&self) -> ResourceResults {
        ResourceResults {
            count_k: (get_counter() - self.count) / 1000,
            total_m: (get_total() - self.total) / 1_000_000,
            max_bytes_m: (get_max_total() - self.net_total) / 1_000_000,
            net_count: get_net_counter() - self.net_count,
            net_total: (get_net_total() - self.net_total) / 1_000_000,
        }
    }
}

// total number of allocations made over the life of the program
static COUNTER: AtomicIsize = AtomicIsize::new(0);

// total number of bytes allocated over the life of the program
static TOTAL: AtomicIsize = AtomicIsize::new(0);

// number allocations not yet deallocated
static NET_COUNTER: AtomicIsize = AtomicIsize::new(0);

// number bytes not yet deallocated
static NET_TOTAL: AtomicIsize = AtomicIsize::new(0);

// the peak value reached by NET_TOTAL
// This is reset whenever a ResourceTracker is created
// so it gives the right answer for a single operation
// but it does not handle nested ResourceTrackers correctly.
static MAX_NET_TOTAL: AtomicIsize = AtomicIsize::new(0);

fn add_to_counter(inc: isize) {
    COUNTER.fetch_add(1, Ordering::SeqCst);
    TOTAL.fetch_add(inc, Ordering::SeqCst);
    NET_COUNTER.fetch_add(1, Ordering::SeqCst);
    NET_TOTAL.fetch_add(inc, Ordering::SeqCst);
    MAX_NET_TOTAL.fetch_max(NET_TOTAL.load(Ordering::SeqCst), Ordering::SeqCst);
}

fn subtract_from_counter(inc: isize) {
    NET_COUNTER.fetch_sub(1, Ordering::SeqCst);
    NET_TOTAL.fetch_sub(inc, Ordering::SeqCst);
}

fn get_counter() -> isize {
    COUNTER.load(Ordering::SeqCst)
}

fn get_total() -> isize {
    TOTAL.load(Ordering::SeqCst)
}

fn get_net_counter() -> isize {
    NET_COUNTER.load(Ordering::SeqCst)
}

fn get_net_total() -> isize {
    NET_TOTAL.load(Ordering::SeqCst)
}

fn clear_max() {
    MAX_NET_TOTAL.store(0, Ordering::SeqCst)
}

fn get_max_total() -> isize {
    MAX_NET_TOTAL.load(Ordering::SeqCst)
}

use std::alloc::{GlobalAlloc, Layout, System};

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        add_to_counter(layout.size() as isize);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        subtract_from_counter(layout.size() as isize);
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;
