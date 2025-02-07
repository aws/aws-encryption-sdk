// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::ConsumedCapacity,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity::ConsumedCapacity {
        TableName: crate::standard_library_conversions::ostring_to_dafny(&value.table_name),
 CapacityUnits: crate::standard_library_conversions::odouble_to_dafny(&value.capacity_units),
 ReadCapacityUnits: crate::standard_library_conversions::odouble_to_dafny(&value.read_capacity_units),
 WriteCapacityUnits: crate::standard_library_conversions::odouble_to_dafny(&value.write_capacity_units),
 Table: ::dafny_runtime::Rc::new(match &value.table {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::capacity::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 LocalSecondaryIndexes:
::dafny_runtime::Rc::new(match &value.local_secondary_indexes {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
            |v| crate::deps::com_amazonaws_dynamodb::conversions::capacity::to_dafny(v)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 GlobalSecondaryIndexes:
::dafny_runtime::Rc::new(match &value.global_secondary_indexes {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(x,
            |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
            |v| crate::deps::com_amazonaws_dynamodb::conversions::capacity::to_dafny(v)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::ConsumedCapacity,
    >,
) -> aws_sdk_dynamodb::types::ConsumedCapacity {
    aws_sdk_dynamodb::types::ConsumedCapacity::builder()
          .set_table_name(crate::standard_library_conversions::ostring_from_dafny(dafny_value.TableName().clone()))
 .set_capacity_units(crate::standard_library_conversions::odouble_from_dafny(dafny_value.CapacityUnits().clone()))
 .set_read_capacity_units(crate::standard_library_conversions::odouble_from_dafny(dafny_value.ReadCapacityUnits().clone()))
 .set_write_capacity_units(crate::standard_library_conversions::odouble_from_dafny(dafny_value.WriteCapacityUnits().clone()))
 .set_table(match (*dafny_value.Table()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::capacity::from_dafny(value.clone())),
    _ => None,
}
)
 .set_local_secondary_indexes(match (*dafny_value.LocalSecondaryIndexes()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(value,
                |k: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
                |v: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Capacity>| crate::deps::com_amazonaws_dynamodb::conversions::capacity::from_dafny(v.clone())
,
            )
        ),
    _ => None
}
)
 .set_global_secondary_indexes(match (*dafny_value.GlobalSecondaryIndexes()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(value,
                |k: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
                |v: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Capacity>| crate::deps::com_amazonaws_dynamodb::conversions::capacity::from_dafny(v.clone())
,
            )
        ),
    _ => None
}
)
          .build()
}
