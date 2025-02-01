// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::operation::scan::ScanOutput,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ScanOutput,
> {
    ::dafny_runtime::Rc::new(crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ScanOutput::ScanOutput {
        Items: ::dafny_runtime::Rc::new(match &value.items {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(&e.clone(),
    |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
    |v| crate::deps::com_amazonaws_dynamodb::conversions::attribute_value::to_dafny(v)
,
)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 Count: crate::standard_library_conversions::oint_to_dafny(Some(value.count)),
 ScannedCount: crate::standard_library_conversions::oint_to_dafny(Some(value.scanned_count)),
 LastEvaluatedKey:
::dafny_runtime::Rc::new(match &value.last_evaluated_key {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
            |v| crate::deps::com_amazonaws_dynamodb::conversions::attribute_value::to_dafny(v)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 ConsumedCapacity: ::dafny_runtime::Rc::new(match &value.consumed_capacity {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::consumed_capacity::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    })
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ScanOutput,
    >,
) -> aws_sdk_dynamodb::operation::scan::ScanOutput {
    aws_sdk_dynamodb::operation::scan::ScanOutput::builder()
          .set_items(match (*dafny_value.Items()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::dafny_runtime_conversions::DafnyMap<::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>, ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>>| ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(&e,
    |k: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
    |v: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>| crate::deps::com_amazonaws_dynamodb::conversions::attribute_value::from_dafny(v.clone())
,
)
,
            )
        ),
    _ => None
}
)
 .set_count(crate::standard_library_conversions::oint_from_dafny(dafny_value.Count().clone()))
 .set_scanned_count(crate::standard_library_conversions::oint_from_dafny(dafny_value.ScannedCount().clone()))
 .set_last_evaluated_key(match (*dafny_value.LastEvaluatedKey()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(value,
                |k: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
                |v: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>| crate::deps::com_amazonaws_dynamodb::conversions::attribute_value::from_dafny(v.clone())
,
            )
        ),
    _ => None
}
)
 .set_consumed_capacity(match (*dafny_value.ConsumedCapacity()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::consumed_capacity::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
}
