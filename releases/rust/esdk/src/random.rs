// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]

use crate::*;
use aws_lc_rs::rand;

impl crate::ExternRandom::_default {
    #[allow(non_snake_case)]
    pub fn GenerateBytes(
        num_bytes: i32,
    ) -> ::dafny_runtime::Rc<
        _Wrappers_Compile::Result<
            ::dafny_runtime::Sequence<u8>,
            ::dafny_runtime::Rc<
                software::amazon::cryptography::primitives::internaldafny::types::Error,
            >,
        >,
    > {
        let mut rand_bytes: Vec<u8> = vec![0; num_bytes as usize];
        match rand::fill(&mut rand_bytes) {
        Ok(_) => {
          ::dafny_runtime::Rc::new(
            _Wrappers_Compile::Result::Success{value :
              dafny_runtime::Sequence::from_array_owned(rand_bytes)
            }
          )
        }
        Err(_) => {
          dafny_runtime::Rc::new(_Wrappers_Compile::Result::Failure{ error : dafny_runtime::Rc::new(
            software::amazon::cryptography::primitives::internaldafny::types::Error::AwsCryptographicPrimitivesError{
              message : dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string("Error generating random bytes.")
          })})
        }
      }
    }
}
