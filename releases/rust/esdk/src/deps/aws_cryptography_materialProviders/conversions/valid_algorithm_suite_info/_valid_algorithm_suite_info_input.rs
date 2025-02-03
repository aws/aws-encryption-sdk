// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: crate::deps::aws_cryptography_materialProviders::operation::valid_algorithm_suite_info::AlgorithmSuiteInfo,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::AlgorithmSuiteInfo,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::AlgorithmSuiteInfo::AlgorithmSuiteInfo {
        id: crate::deps::aws_cryptography_materialProviders::conversions::algorithm_suite_id::to_dafny(&value.id.clone().unwrap())
,
 binaryId: crate::standard_library_conversions::blob_to_dafny(&value.binary_id.unwrap()),
 messageVersion: value.message_version.clone().unwrap(),
 encrypt: crate::deps::aws_cryptography_materialProviders::conversions::encrypt::to_dafny(&value.encrypt.clone().unwrap())
,
 kdf: crate::deps::aws_cryptography_materialProviders::conversions::derivation_algorithm::to_dafny(&value.kdf.clone().unwrap())
,
 commitment: crate::deps::aws_cryptography_materialProviders::conversions::derivation_algorithm::to_dafny(&value.commitment.clone().unwrap())
,
 signature: crate::deps::aws_cryptography_materialProviders::conversions::signature_algorithm::to_dafny(&value.signature.clone().unwrap())
,
 symmetricSignature: crate::deps::aws_cryptography_materialProviders::conversions::symmetric_signature_algorithm::to_dafny(&value.symmetric_signature.clone().unwrap())
,
 edkWrapping: crate::deps::aws_cryptography_materialProviders::conversions::edk_wrapping_algorithm::to_dafny(&value.edk_wrapping.clone().unwrap())
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::materialproviders::internaldafny::types::AlgorithmSuiteInfo,
    >,
) -> crate::deps::aws_cryptography_materialProviders::operation::valid_algorithm_suite_info::AlgorithmSuiteInfo{
    crate::deps::aws_cryptography_materialProviders::operation::valid_algorithm_suite_info::AlgorithmSuiteInfo::builder()
        .set_id(Some( crate::deps::aws_cryptography_materialProviders::conversions::algorithm_suite_id::from_dafny(dafny_value.id().clone())
 ))
 .set_binary_id(Some(crate::standard_library_conversions::blob_from_dafny(dafny_value.binaryId().clone())))
 .set_message_version(Some( dafny_value.messageVersion() .clone() ))
 .set_encrypt(Some( crate::deps::aws_cryptography_materialProviders::conversions::encrypt::from_dafny(dafny_value.encrypt().clone())
 ))
 .set_kdf(Some( crate::deps::aws_cryptography_materialProviders::conversions::derivation_algorithm::from_dafny(dafny_value.kdf().clone())
 ))
 .set_commitment(Some( crate::deps::aws_cryptography_materialProviders::conversions::derivation_algorithm::from_dafny(dafny_value.commitment().clone())
 ))
 .set_signature(Some( crate::deps::aws_cryptography_materialProviders::conversions::signature_algorithm::from_dafny(dafny_value.signature().clone())
 ))
 .set_symmetric_signature(Some( crate::deps::aws_cryptography_materialProviders::conversions::symmetric_signature_algorithm::from_dafny(dafny_value.symmetricSignature().clone())
 ))
 .set_edk_wrapping(Some( crate::deps::aws_cryptography_materialProviders::conversions::edk_wrapping_algorithm::from_dafny(dafny_value.edkWrapping().clone())
 ))
        .build()
        .unwrap()
}
