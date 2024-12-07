// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]

use crate::*;
use aws_lc_rs::digest;
use software::amazon::cryptography::primitives::internaldafny::types::DigestAlgorithm;

impl crate::ExternDigest::_default {
    #[allow(non_snake_case)]
    pub fn Digest(
        digest_algorithm: &::std::rc::Rc<DigestAlgorithm>,
        message: &::dafny_runtime::Sequence<u8>,
    ) -> ::std::rc::Rc<
        _Wrappers_Compile::Result<
            ::dafny_runtime::Sequence<u8>,
            ::std::rc::Rc<software::amazon::cryptography::primitives::internaldafny::types::Error>,
        >,
    > {
        let algorithm = match **digest_algorithm {
            DigestAlgorithm::SHA_512 {} => &digest::SHA512,
            DigestAlgorithm::SHA_384 {} => &digest::SHA384,
            DigestAlgorithm::SHA_256 {} => &digest::SHA256,
        };
        let message_vec: Vec<u8> = message.iter().collect();
        let result = digest::digest(algorithm, &message_vec);
        ::std::rc::Rc::new(_Wrappers_Compile::Result::Success {
            value: result.as_ref().iter().cloned().collect(),
        })
    }
}
