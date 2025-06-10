// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::LocalSecondaryIndexDescription,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::LocalSecondaryIndexDescription>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::LocalSecondaryIndexDescription::LocalSecondaryIndexDescription {
        IndexName: crate::standard_library_conversions::ostring_to_dafny(&value.index_name),
 KeySchema: ::dafny_runtime::Rc::new(match &value.key_schema {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::key_schema_element::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 Projection: ::dafny_runtime::Rc::new(match &value.projection {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::projection::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 IndexSizeBytes: crate::standard_library_conversions::olong_to_dafny(&value.index_size_bytes),
 ItemCount: crate::standard_library_conversions::olong_to_dafny(&value.item_count),
 IndexArn: crate::standard_library_conversions::ostring_to_dafny(&value.index_arn),
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::LocalSecondaryIndexDescription,
    >,
) -> aws_sdk_dynamodb::types::LocalSecondaryIndexDescription {
    aws_sdk_dynamodb::types::LocalSecondaryIndexDescription::builder()
          .set_index_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.IndexName().clone()))
 .set_key_schema(match (*dafny_value.KeySchema()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::KeySchemaElement>| crate::deps::com_amazonaws_dynamodb::conversions::key_schema_element::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_projection(match (*dafny_value.Projection()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::projection::from_dafny(value.clone())),
    _ => None,
}
)
 .set_index_size_bytes(crate::standard_library_conversions::olong_from_dafny(dafny_value.IndexSizeBytes().clone()))
 .set_item_count(crate::standard_library_conversions::olong_from_dafny(dafny_value.ItemCount().clone()))
 .set_index_arn(crate::standard_library_conversions::ostring_from_dafny(dafny_value.IndexArn().clone()))
          .build()
}
