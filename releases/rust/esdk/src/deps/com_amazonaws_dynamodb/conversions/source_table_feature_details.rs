// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: &aws_sdk_dynamodb::types::SourceTableFeatureDetails,
) -> ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SourceTableFeatureDetails>{
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SourceTableFeatureDetails::SourceTableFeatureDetails {
        LocalSecondaryIndexes: ::dafny_runtime::Rc::new(match &value.local_secondary_indexes {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::local_secondary_index_info::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 GlobalSecondaryIndexes: ::dafny_runtime::Rc::new(match &value.global_secondary_indexes {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::global_secondary_index_info::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
 StreamDescription: ::dafny_runtime::Rc::new(match &value.stream_description {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::stream_specification::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 TimeToLiveDescription: ::dafny_runtime::Rc::new(match &value.time_to_live_description {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::time_to_live_description::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
 SSEDescription: ::dafny_runtime::Rc::new(match &value.sse_description {
    Some(x) => crate::_Wrappers_Compile::Option::Some { value: crate::deps::com_amazonaws_dynamodb::conversions::sse_description::to_dafny(x) },
    None => crate::_Wrappers_Compile::Option::None { }
})
,
    }
  )
}
#[allow(dead_code)]
pub fn from_dafny(
    dafny_value: ::dafny_runtime::Rc<
        crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::SourceTableFeatureDetails,
    >,
) -> aws_sdk_dynamodb::types::SourceTableFeatureDetails {
    aws_sdk_dynamodb::types::SourceTableFeatureDetails::builder()
          .set_local_secondary_indexes(match (*dafny_value.LocalSecondaryIndexes()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::LocalSecondaryIndexInfo>| crate::deps::com_amazonaws_dynamodb::conversions::local_secondary_index_info::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_global_secondary_indexes(match (*dafny_value.GlobalSecondaryIndexes()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(
            ::dafny_runtime::dafny_runtime_conversions::dafny_sequence_to_vec(value,
                |e: &::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::GlobalSecondaryIndexInfo>| crate::deps::com_amazonaws_dynamodb::conversions::global_secondary_index_info::from_dafny(e.clone())
,
            )
        ),
    _ => None
}
)
 .set_stream_description(match (*dafny_value.StreamDescription()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::stream_specification::from_dafny(value.clone())),
    _ => None,
}
)
 .set_time_to_live_description(match (*dafny_value.TimeToLiveDescription()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::time_to_live_description::from_dafny(value.clone())),
    _ => None,
}
)
 .set_sse_description(match (*dafny_value.SSEDescription()).as_ref() {
    crate::r#_Wrappers_Compile::Option::Some { value } =>
        Some(crate::deps::com_amazonaws_dynamodb::conversions::sse_description::from_dafny(value.clone())),
    _ => None,
}
)
          .build()
}
