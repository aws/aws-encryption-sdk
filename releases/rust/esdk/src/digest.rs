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
        digest_algorithm: &::dafny_runtime::Rc<DigestAlgorithm>,
        message: &::dafny_runtime::Sequence<u8>,
    ) -> ::dafny_runtime::Rc<
        _Wrappers_Compile::Result<
            ::dafny_runtime::Sequence<u8>,
            ::dafny_runtime::Rc<
                software::amazon::cryptography::primitives::internaldafny::types::Error,
            >,
        >,
    > {
        let algorithm = match **digest_algorithm {
            DigestAlgorithm::SHA_512 {} => &digest::SHA512,
            DigestAlgorithm::SHA_384 {} => &digest::SHA384,
            DigestAlgorithm::SHA_256 {} => &digest::SHA256,
        };
        let message_vec = &message.to_array();
        let result = digest::digest(algorithm, message_vec);
        ::dafny_runtime::Rc::new(_Wrappers_Compile::Result::Success {
            value: dafny_runtime::Sequence::from_array_owned(result.as_ref().to_vec()),
        })
    }
}
