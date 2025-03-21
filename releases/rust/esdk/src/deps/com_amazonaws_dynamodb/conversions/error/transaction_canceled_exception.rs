// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
#[allow(dead_code)]
pub fn to_dafny(
    value: aws_sdk_dynamodb::types::error::TransactionCanceledException,
) -> ::dafny_runtime::Rc<
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error,
> {
    ::dafny_runtime::Rc::new(
    crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error::TransactionCanceledException {
      Message: crate::standard_library_conversions::ostring_to_dafny(&value.message),
 CancellationReasons: ::dafny_runtime::Rc::new(match &value.cancellation_reasons {
    Some(x) => crate::r#_Wrappers_Compile::Option::Some { value :
        ::dafny_runtime::dafny_runtime_conversions::vec_to_dafny_sequence(x,
            |e| crate::deps::com_amazonaws_dynamodb::conversions::cancellation_reason::to_dafny(e)
,
        )
    },
    None => crate::r#_Wrappers_Compile::Option::None {}
})
,
    }
  )
}
