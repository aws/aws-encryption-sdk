// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]
#![allow(dead_code)]

#[allow(non_snake_case)]
pub mod ConcurrentCall {

    fn de_const(
        p: *const dafny_runtime::Object<(dyn Callee + 'static)>,
    ) -> *mut dafny_runtime::Object<(dyn Callee + 'static)> {
        p as _
    }

    pub struct FakeCallee {
        callee: *const dafny_runtime::Object<(dyn Callee + 'static)>,
    }
    impl FakeCallee {
        fn new(callee: &dafny_runtime::Object<(dyn Callee + 'static)>) -> Self {
            Self {
                callee: std::ptr::from_ref(callee),
            }
        }
        fn call(&self, x: u32, y: u32) {
            let mptr = de_const(self.callee);
            let value: &mut dafny_runtime::Object<(dyn Callee + 'static)> = unsafe { &mut *mptr };
            value.as_mut().call(x, y);
        }
    }
    unsafe impl Send for FakeCallee {}

    #[allow(nonstandard_style)]
    pub struct _default {}
    use crate::ConcurrentCall::Callee;
    impl _default {
        pub fn ConcurrentCall(
            callee: &dafny_runtime::Object<(dyn Callee + 'static)>,
            serial_iters: u32,
            concurrent_iters: u32,
        ) {
            let mut children = vec![];

            for i in 0..concurrent_iters {
                // Spin up another thread
                let fake = FakeCallee::new(callee);
                children.push(std::thread::spawn(move || {
                    for j in 0..serial_iters {
                        fake.call(j, i);
                    }
                }));
            }

            for child in children {
                let _ = child.join();
            }
        }
    }
}
