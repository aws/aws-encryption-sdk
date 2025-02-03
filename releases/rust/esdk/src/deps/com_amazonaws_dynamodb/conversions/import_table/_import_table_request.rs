// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::import_table::ImportTableInput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportTableInput,
>{
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportTableInput::ImportTableInput {
        ClientToken: crate::standard_library_conversions::ostring_to_dafny(&value.client_token),
 S3BucketSource: crate::deps::com_amazonaws_dynamodb::conversions::s3_bucket_source::to_dafny(&value.s3_bucket_source.clone().unwrap())
,
 InputFormat: crate::deps::com_amazonaws_dynamodb::conversions::input_format::to_dafny(value.input_format.clone().unwrap()),
 InputFormatOptions: ::dafny_runtime::Rc::new(match &value.input_format_options {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::input_format_options::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 InputCompressionType: ::dafny_runtime::Rc::new(match &value.input_compression_type {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::input_compression_type::to_dafny(x.clone()) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 TableCreationParameters: crate::deps::com_amazonaws_dynamodb::conversions::table_creation_parameters::to_dafny(&value.table_creation_parameters.clone().unwrap())
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ImportTableInput,
    >,
) -> aws_sdk_dynamodb::operation::import_table::ImportTableInput {
    aws_sdk_dynamodb::operation::import_table::ImportTableInput::builder()
          .set_client_token(crate::standard_library_conversions::ostring_from_dafny(dafny_value.ClientToken().clone()))
 .set_s3_bucket_source(Some( crate::deps::com_amazonaws_dynamodb::conversions::s3_bucket_source::from_dafny(dafny_value.S3BucketSource().clone())
 ))
 .set_input_format(Some( crate::deps::com_amazonaws_dynamodb::conversions::input_format::from_dafny(dafny_value.InputFormat()) ))
 .set_input_format_options(match (*dafny_value.InputFormatOptions()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::input_format_options::from_dafny(value.clone())),
    _ => None,
}
)
 .set_input_compression_type(match &**dafny_value.InputCompressionType() {
    crate::r#_Wrappers_Compile::Option::Some { value } => Some(
        crate::deps::com_amazonaws_dynamodb::conversions::input_compression_type::from_dafny(value)
    ),
    _ => None,
}
)
 .set_table_creation_parameters(Some( crate::deps::com_amazonaws_dynamodb::conversions::table_creation_parameters::from_dafny(dafny_value.TableCreationParameters().clone())
 ))
          .build()
          .unwrap()
}
