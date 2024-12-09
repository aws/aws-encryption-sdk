// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: crate::deps::aws_cryptography_primitives::types::crypto_config::CryptoConfig,
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::CryptoConfig,
> {
    ::std::rc::Rc::new(to_dafny_plain(value))
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::CryptoConfig,
    >,
) -> crate::deps::aws_cryptography_primitives::types::crypto_config::CryptoConfig {
    plain_from_dafny(&*dafny_value)
}


#[allow(dead_code)]
pub fn to_dafny_plain(
    value: crate::deps::aws_cryptography_primitives::types::crypto_config::CryptoConfig,
) -> crate::r#software::amazon::cryptography::primitives::internaldafny::types::CryptoConfig {
    crate::r#software::amazon::cryptography::primitives::internaldafny::types::CryptoConfig::CryptoConfig {

    }
}

#[allow(dead_code)]
pub fn plain_from_dafny(
    dafny_value: &crate::r#software::amazon::cryptography::primitives::internaldafny::types::CryptoConfig,
) -> crate::deps::aws_cryptography_primitives::types::crypto_config::CryptoConfig {
    match dafny_value {
        crate::r#software::amazon::cryptography::primitives::internaldafny::types::CryptoConfig::CryptoConfig {..} =>
            crate::deps::aws_cryptography_primitives::types::crypto_config::CryptoConfig::builder()

                .build()
                .unwrap()
    }
}
