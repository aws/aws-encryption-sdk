// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_kms::operation::generate_mac::GenerateMacOutput
) -> ::std::rc::Rc<
    crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateMacResponse,
>{
    ::std::rc::Rc::new(crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateMacResponse::GenerateMacResponse {
        Mac: crate::standard_library_conversions::oblob_to_dafny(&value.mac),
 MacAlgorithm: ::std::rc::Rc::new(match &value.mac_algorithm {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_kms::conversions::mac_algorithm_spec::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 KeyId: crate::standard_library_conversions::ostring_to_dafny(&value.key_id),
    })
}
 #[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::std::rc::Rc<
        crate::r#software::amazon::cryptography::services::kms::internaldafny::types::GenerateMacResponse,
    >
) -> aws_sdk_kms::operation::generate_mac::GenerateMacOutput {
    aws_sdk_kms::operation::generate_mac::GenerateMacOutput::builder()
          .set_mac(crate::standard_library_conversions::oblob_from_dafny(dafny_value.Mac().clone()))
 .set_mac_algorithm(match &**dafny_value.MacAlgorithm() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_kms::conversions::mac_algorithm_spec::from_dafny(value)
    ),
    _ => None,
}
)
 .set_key_id(crate::standard_library_conversions::ostring_from_dafny(dafny_value.KeyId().clone()))
          .build()


}
