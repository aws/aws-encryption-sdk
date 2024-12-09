// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &crate::deps::aws_cryptography_primitives::types::DecompressPublicKeyInput,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::DecompressPublicKeyInput,
> {
    ::std::rc::Rc::new(to_dafny_plain(value.clone()))
}

#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_primitives::types::DecompressPublicKeyInput,
) -> crate::r#software::amazon::cryptography::primitives::internaldafny::types::DecompressPublicKeyInput {
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::DecompressPublicKeyInput::DecompressPublicKeyInput {
        compressedPublicKey: crate::standard_library_conversions::blob_to_dafny(&value.compressed_public_key.unwrap()),
 eccCurve: crate::deps::aws_cryptography_primitives::conversions::ecdh_curve_spec::to_dafny(value.ecc_curve.clone().unwrap()),
    }
}

#[allow(dead_code)]
pub fn option_to_dafny(
  value: ::std::option::Option<crate::deps::aws_cryptography_primitives::types::DecompressPublicKeyInput>,
) -> ::std::rc::Rc<crate::_Wrappers_Compile::Option<::std::rc::Rc<
  crate::r#software::amazon::cryptography::primitives::internaldafny::types::DecompressPublicKeyInput,
>>>{
    ::std::rc::Rc::new(match value {
        ::std::option::Option::None => crate::_Wrappers_Compile::Option::None {},
        ::std::option::Option::Some(x) => crate::_Wrappers_Compile::Option::Some {
            value: ::std::rc::Rc::new(to_dafny_plain(x)),
        },
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::DecompressPublicKeyInput,
    >,
) -> crate::deps::aws_cryptography_primitives::types::DecompressPublicKeyInput {
    plain_from_dafny(&*dafny_value)
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::primitives::internaldafny::types::DecompressPublicKeyInput,
) -> crate::deps::aws_cryptography_primitives::types::DecompressPublicKeyInput {
    match dafny_value {
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::DecompressPublicKeyInput::DecompressPublicKeyInput {..} =>
            crate::deps::aws_cryptography_primitives::types::DecompressPublicKeyInput::builder()
                .set_compressed_public_key(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.compressedPublicKey().clone())))
 .set_ecc_curve(Some( crate::deps::aws_cryptography_primitives::conversions::ecdh_curve_spec::from_dafny(dafny_value.eccCurve()) ))
                .build()
                .unwrap()
    }
}

#[allow(dead_code)]
pub fn option_from_dafny(
    dafny_value: ::std::rc::Rc<crate::_Wrappers_Compile::Option<::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::DecompressPublicKeyInput,
    >>>,
) -> ::std::option::Option<crate::deps::aws_cryptography_primitives::types::DecompressPublicKeyInput> {
    match &*dafny_value {
        crate::_Wrappers_Compile::Option::Some { value } => {
            ::std::option::Option::Some(plain_from_dafny(value))
        }
        _ => ::std::option::Option::None,
    }
}
