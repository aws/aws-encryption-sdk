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
        input: &::dafny_runtime::Rc<
            crate::software::amazon::cryptography::primitives::internaldafny::types::HMacInput,
        >,
    ) -> ::dafny_runtime::Rc<
        _Wrappers_Compile::Result<
            ::dafny_runtime::Sequence<u8>,
            ::dafny_runtime::Rc<
                software::amazon::cryptography::primitives::internaldafny::types::Error,
            >,
        >,
    > {
        let key_vec = &input.key().to_array();
        let the_key = hmac::Key::new(convert_algorithm(input.digestAlgorithm()), key_vec);
        let message_vec = &input.message().to_array();
        let result = hmac::sign(&the_key, message_vec);
        ::dafny_runtime::Rc::new(_Wrappers_Compile::Result::Success {
            value: dafny_runtime::Sequence::from_array_owned(result.as_ref().to_vec()),
        })
    }
}

#[allow(non_snake_case)]
pub mod HMAC {
    use crate::*;
    use ::dafny_runtime::RefCell;
    use aws_lc_rs::hmac;
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

    impl dafny_runtime::UpcastObject<dafny_runtime::DynAny> for HMac {
        dafny_runtime::UpcastObjectFn!(dafny_runtime::DynAny);
    }

    impl HMac {
        pub fn Init(&self, salt: &::dafny_runtime::Sequence<u8>) {
            let salt = &salt.to_array();
            self.inner.lock().unwrap().key = Some(hmac::Key::new(self.algorithm, salt));
            let context = Some(hmac::Context::with_key(
                self.inner.lock().unwrap().key.as_ref().unwrap(),
            ));
            self.inner.lock().unwrap().context = context;
        }
        pub fn re_init(&self) {
            let context = Some(hmac::Context::with_key(
                self.inner.lock().unwrap().key.as_ref().unwrap(),
            ));
            self.inner.lock().unwrap().context = context;
        }
        pub fn Build(
            input: &::dafny_runtime::Rc<
                software::amazon::cryptography::primitives::internaldafny::types::DigestAlgorithm,
            >,
        ) -> ::dafny_runtime::Rc<
            _Wrappers_Compile::Result<
                ::dafny_runtime::Object<Self>,
                ::dafny_runtime::Rc<
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

            ::dafny_runtime::Rc::new(_Wrappers_Compile::Result::Success { value: inner })
        }
        pub fn BlockUpdate(&self, block: &::dafny_runtime::Sequence<u8>) {
            let part = &block.to_array();
            self.inner
                .lock()
                .unwrap()
                .context
                .as_mut()
                .unwrap()
                .update(part);
        }
        pub fn GetResult(&self) -> ::dafny_runtime::Sequence<u8> {
            let is_empty = self.inner.lock().unwrap().context.is_none();
            if is_empty {
                return dafny_runtime::Sequence::from_array_owned(vec![]);
            }
            let tag = self.inner.lock().unwrap().context.take().unwrap().sign();
            // other languages allow you to call BlockUpdate after GetResult
            // so we re-initialize to mimic that behavior
            self.re_init();
            dafny_runtime::Sequence::from_array_owned(tag.as_ref().to_vec())
        }
    }
}
