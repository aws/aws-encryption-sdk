// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]

pub fn to_dafny(
    value: &crate::deps::aws_cryptography_keyStore::client::Client,
) -> ::dafny_runtime::Object<
    dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::IKeyStoreClient,
> {
    value.dafny_client.clone()
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Object<
      dyn crate::r#software::amazon::cryptography::keystore::internaldafny::types::IKeyStoreClient
    >,
) -> crate::deps::aws_cryptography_keyStore::client::Client {
    crate::deps::aws_cryptography_keyStore::client::Client {
        dafny_client: dafny_value,
    }
}
