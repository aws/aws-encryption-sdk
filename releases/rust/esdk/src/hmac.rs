// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]

use crate::software::amazon::cryptography::primitives::internaldafny::types::DigestAlgorithm;
use crate::*;
use aws_lc_rs::hmac;

fn convert_algorithm(input: &DigestAlgorithm) -> hmac::Algorithm {
    match input {
        DigestAlgorithm::SHA_512 {} => hmac::HMAC_SHA512,
        DigestAlgorithm::SHA_384 {} => hmac::HMAC_SHA384,
        DigestAlgorithm::SHA_256 {} => hmac::HMAC_SHA256,
    }
}

// Let's implement HMAC::_default::Digest
impl crate::HMAC::_default {
    #[allow(non_snake_case)]
    pub fn Digest(
        input: &::std::rc::Rc<
            crate::software::amazon::cryptography::primitives::internaldafny::types::HMacInput,
        >,
    ) -> ::std::rc::Rc<
        _Wrappers_Compile::Result<
            ::dafny_runtime::Sequence<u8>,
            ::std::rc::Rc<software::amazon::cryptography::primitives::internaldafny::types::Error>,
        >,
    > {
        let key_vec: Vec<u8> = input.key().iter().collect();
        let the_key = hmac::Key::new(convert_algorithm(input.digestAlgorithm()), &key_vec);
        let message_vec: Vec<u8> = input.message().iter().collect();
        let result = hmac::sign(&the_key, &message_vec);
        ::std::rc::Rc::new(_Wrappers_Compile::Result::Success {
            value: result.as_ref().iter().cloned().collect(),
        })
    }
}

#[allow(non_snake_case)]
pub mod HMAC {
    use crate::*;
    use aws_lc_rs::hmac;
    use std::cell::RefCell;
    #[allow(non_camel_case_types)]
    pub struct _default {}

    #[derive(Debug)]
    pub struct HMacInner {
        context: Option<hmac::Context>,
        key: Option<hmac::Key>,
    }
    pub struct HMac {
        algorithm: hmac::Algorithm,
        inner: RefCell<HMacInner>,
    }

    impl dafny_runtime::UpcastObject<dyn std::any::Any> for HMac {
        dafny_runtime::UpcastObjectFn!(dyn std::any::Any);
    }

    impl HMac {
        pub fn Init(&self, salt: &::dafny_runtime::Sequence<u8>) {
            let salt: Vec<u8> = salt.iter().collect();
            self.inner.borrow_mut().key = Some(hmac::Key::new(self.algorithm, &salt));
            let context = Some(hmac::Context::with_key(
                self.inner.borrow().key.as_ref().unwrap(),
            ));
            self.inner.borrow_mut().context = context;
        }
        pub fn re_init(&self) {
            let context = Some(hmac::Context::with_key(
                self.inner.borrow().key.as_ref().unwrap(),
            ));
            self.inner.borrow_mut().context = context;
        }
        pub fn Build(
            input: &::std::rc::Rc<
                software::amazon::cryptography::primitives::internaldafny::types::DigestAlgorithm,
            >,
        ) -> ::std::rc::Rc<
            _Wrappers_Compile::Result<
                ::dafny_runtime::Object<Self>,
                ::std::rc::Rc<
                    software::amazon::cryptography::primitives::internaldafny::types::Error,
                >,
            >,
        > {
            let inner = dafny_runtime::Object::new(Self {
                algorithm: super::convert_algorithm(input),
                inner: RefCell::new(HMacInner {
                    context: None,
                    key: None,
                }),
            });

            ::std::rc::Rc::new(_Wrappers_Compile::Result::Success { value: inner })
        }
        pub fn BlockUpdate(&self, block: &::dafny_runtime::Sequence<u8>) {
            let part: Vec<u8> = block.iter().collect();
            self.inner
                .borrow_mut()
                .context
                .as_mut()
                .unwrap()
                .update(&part);
        }
        pub fn GetResult(&self) -> ::dafny_runtime::Sequence<u8> {
            let is_empty = self.inner.borrow().context.is_none();
            if is_empty {
                return [].iter().cloned().collect();
            }
            let tag = self.inner.borrow_mut().context.take().unwrap().sign();
            // other languages allow you to call BlockUpdate after GetResult
            // so we re-initialize to mimic that behavior
            self.re_init();
            tag.as_ref().iter().cloned().collect()
        }
    }
}
