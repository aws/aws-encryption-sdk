// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_keyStore::operation::get_beacon_key::GetBeaconKeyOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBeaconKeyOutput,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBeaconKeyOutput::GetBeaconKeyOutput {
        beaconKeyMaterials: crate::deps::aws_cryptography_keyStore::conversions::beacon_key_materials::to_dafny(&value.beacon_key_materials.clone().unwrap())
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::keystore::internaldafny::types::GetBeaconKeyOutput,
    >,
) -> crate::deps::aws_cryptography_keyStore::operation::get_beacon_key::GetBeaconKeyOutput {
    crate::deps::aws_cryptography_keyStore::operation::get_beacon_key::GetBeaconKeyOutput::builder()
        .set_beacon_key_materials(Some(
            crate::deps::aws_cryptography_keyStore::conversions::beacon_key_materials::from_dafny(
                dafny_value.beaconKeyMaterials().clone(),
            ),
        ))
        .build()
        .unwrap()
}
