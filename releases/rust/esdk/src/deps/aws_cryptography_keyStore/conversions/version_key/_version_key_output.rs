// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_keyStore::operation::version_key::VersionKeyOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::VersionKeyOutput,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::keystore::internaldafny::types::VersionKeyOutput::VersionKeyOutput {

    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::VersionKeyOutput,
    >,
) -> crate::deps::aws_cryptography_keyStore::operation::version_key::VersionKeyOutput {
    crate::deps::aws_cryptography_keyStore::operation::version_key::VersionKeyOutput::builder()
        .build()
        .unwrap()
}
