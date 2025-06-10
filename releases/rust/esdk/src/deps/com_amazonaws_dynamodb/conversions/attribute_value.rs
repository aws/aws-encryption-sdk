// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::AttributeValue,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue,
>{
    ::dafny_runtime::Rc::new(match value {
        aws_sdk_dynamodb::types::AttributeValue::S(x) =>
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::S {
        S: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&x),
    },
aws_sdk_dynamodb::types::AttributeValue::N(x) =>
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::N {
        N: dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&x),
    },
aws_sdk_dynamodb::types::AttributeValue::B(x) =>
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::B {
        B: crate::standard_library_conversions::blob_to_dafny(&x),
    },
aws_sdk_dynamodb::types::AttributeValue::Ss(x) =>
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::SS {
        SS: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&x,
    |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&e),
)
,
    },
aws_sdk_dynamodb::types::AttributeValue::Ns(x) =>
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::NS {
        NS: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&x,
    |e| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&e),
)
,
    },
aws_sdk_dynamodb::types::AttributeValue::Bs(x) =>
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::BS {
        BS: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&x,
    |e| crate::standard_library_conversions::blob_to_dafny(&e),
)
,
    },
aws_sdk_dynamodb::types::AttributeValue::M(x) =>
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::M {
        M: ::dafny_runtime::dafny_runtime_conversions::hashmap_to_dafny_map(&x.clone(),
    |k| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(&k),
    |v| crate::deps::com_amazonaws_dynamodb::conversions::attribute_value::to_dafny(v)
,
)
,
    },
aws_sdk_dynamodb::types::AttributeValue::L(x) =>
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::L {
        L: ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(&x,
    |e| crate::deps::com_amazonaws_dynamodb::conversions::attribute_value::to_dafny(e)
,
)
,
    },
aws_sdk_dynamodb::types::AttributeValue::Null(x) =>
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::NULL {
        NULL: x.clone(),
    },
aws_sdk_dynamodb::types::AttributeValue::Bool(x) =>
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::BOOL {
        BOOL: x.clone(),
    },
        _ => panic!("Unknown union variant: {:?}", value),
    })
}

#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue,
    >,
) -> aws_sdk_dynamodb::types::AttributeValue {
    match &::dafny_runtime::Rc::unwrap_or_clone(dafny_value) {
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::S {
    S: x @ _,
} => aws_sdk_dynamodb::types::AttributeValue::S(dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(x)),
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::N {
    N: x @ _,
} => aws_sdk_dynamodb::types::AttributeValue::N(dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(x)),
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::B {
    B: x @ _,
} => aws_sdk_dynamodb::types::AttributeValue::B(crate::standard_library_conversions::blob_from_dafny(x.clone())),
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::SS {
    SS: x @ _,
} => aws_sdk_dynamodb::types::AttributeValue::Ss(::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(x,
    |e: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(e),
)
),
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::NS {
    NS: x @ _,
} => aws_sdk_dynamodb::types::AttributeValue::Ns(::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(x,
    |e: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(e),
)
),
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::BS {
    BS: x @ _,
} => aws_sdk_dynamodb::types::AttributeValue::Bs(::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(x,
    |e: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<u8>| crate::standard_library_conversions::blob_from_dafny(e.clone()),
)
),
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::M {
    M: x @ _,
} => aws_sdk_dynamodb::types::AttributeValue::M(::dafny_runtime::dafny_runtime_conversions::dafny_map_to_hashmap(&x,
    |k: &::dafny_runtime::dafny_runtime_conversions::DafnySequence<::dafny_runtime::dafny_runtime_conversions::DafnyCharUTF16>| dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(k),
    |v: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>| crate::deps::com_amazonaws_dynamodb::conversions::attribute_value::from_dafny(v.clone())
,
)
),
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::L {
    L: x @ _,
} => aws_sdk_dynamodb::types::AttributeValue::L(::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(x,
    |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue>| crate::deps::com_amazonaws_dynamodb::conversions::attribute_value::from_dafny(e.clone())
,
)
),
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::NULL {
    NULL: x @ _,
} => aws_sdk_dynamodb::types::AttributeValue::Null(x .clone()),
crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::AttributeValue::BOOL {
    BOOL: x @ _,
} => aws_sdk_dynamodb::types::AttributeValue::Bool(x .clone()),
    }
}
