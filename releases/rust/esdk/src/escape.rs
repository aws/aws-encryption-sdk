// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use std::future::Future;
use tokio::runtime::Builder;
use tokio::runtime::Handle;
use tokio::runtime::RuntimeFlavor;

pub(crate) fn escape_to_async<F, O>(fut: F) -> O
where
    F: Future<Output = O> + Send,
    O: Send,
{
    match Handle::try_current() {
        Ok(handle) => match handle.runtime_flavor() {
            RuntimeFlavor::CurrentThread => std::thread::scope(move |t| {
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
            _ => tokio::task::block_in_place(move || handle.block_on(fut)),
        },
        Err(_) => Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(fut),
    }
}
