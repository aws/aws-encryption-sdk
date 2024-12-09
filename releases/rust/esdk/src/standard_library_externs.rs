// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Annotation to ignore the case of this module
use crate::r#_Wrappers_Compile;
use crate::implementation_from_dafny::UTF8;

impl crate::implementation_from_dafny::UTF8::_default {
    pub fn Encode(
        s: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
    ) -> ::std::rc::Rc<
        r#_Wrappers_Compile::Result<
            UTF8::ValidUTF8Bytes,
            ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
        >,
    > {
        let v = s.to_array();
        let mut _accumulator: Vec<u8> = vec![];
        // Use of .encode_utf8 method.
        let mut surrogate: Option<u16> = None;
        for c in v.iter() {
            let s = if let Some(s) = surrogate {
                String::from_utf16(&[s, c.0])
            } else {
                String::from_utf16(&[c.0])
            };
            surrogate = None;
            match s {
                Ok(value) => {
                    _accumulator.extend(value.as_bytes());
                    continue;
                }
                Err(e) => {
                    if 0xD800 <= c.0 && c.0 <= 0xDFFF {
                        surrogate = Some(c.0);
                        continue;
                    }
                    return ::std::rc::Rc::new(r#_Wrappers_Compile::Result::<UTF8::ValidUTF8Bytes, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>::Failure {
            error: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(
              &e.to_string())
          });
                }
            }
        }
        if let Some(s) = surrogate {
            return ::std::rc::Rc::new(r#_Wrappers_Compile::Result::<UTF8::ValidUTF8Bytes, ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>::Failure {
        error: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(
          &format!("Surrogate pair missing: 0x{:04x}", s))
      });
        }
        ::std::rc::Rc::new(r#_Wrappers_Compile::Result::<
            UTF8::ValidUTF8Bytes,
            ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
        >::Success {
            value: ::dafny_runtime::Sequence::from_array_owned(_accumulator),
        })
    }
    pub fn Decode(
        b: &::dafny_runtime::Sequence<u8>,
    ) -> ::std::rc::Rc<
        r#_Wrappers_Compile::Result<
            ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
            ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
        >,
    > {
        let b = String::from_utf8(b.to_array().as_ref().clone());
        match b {
      Ok(s) => {
        ::std::rc::Rc::new(r#_Wrappers_Compile::Result::<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
          ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>::Success {
            value: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&s)
        })
      },
      Err(e) => {
        return ::std::rc::Rc::new(r#_Wrappers_Compile::Result::<::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
          ::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>>::Failure {
            error: ::dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(
              &e.to_string())
          })
      }
    }
    }
}
