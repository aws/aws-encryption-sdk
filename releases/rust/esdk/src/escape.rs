// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::future::Future;
use tokio::runtime::Builder;
use tokio::runtime::Handle;

pub(crate) fn escape_to_async<F, O>(fut: F) -> O
where
    F: Future<Output = O> + Send,
    O: Send,
{
    match Handle::try_current() {
        // Any ambient runtime: run the future on a scoped thread with its own
        // runtime. Blocking the calling thread is what a sync bridge does, but
        // the future must never depend on the caller's runtime for progress:
        // `block_in_place` + `Handle::block_on` parks a worker on a future
        // that same runtime has to drive, and when that worker is the last
        // one awake (the shared IO/timer driver unowned, all other workers
        // parked) the future can never complete and the whole runtime
        // deadlocks permanently. A fresh current-thread runtime drives the
        // future independently on every flavor.
        Ok(_) => std::thread::scope(move |t| {
            t.spawn(move || {
                Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .unwrap()
                    .block_on(fut)
            })
            .join()
            .unwrap()
        }),
        Err(_) => Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(fut),
    }
}
