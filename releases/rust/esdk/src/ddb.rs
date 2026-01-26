// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]

use crate::escape::escape_to_async;
use aws_config::Region;

#[allow(non_snake_case)]
impl crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::_default {
    pub fn DDBClientForRegion(region: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>) -> ::dafny_runtime::Rc<
    crate::r#_Wrappers_Compile::Result<
      ::dafny_runtime::Object<dyn crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient>,
      ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
      >
    >{
        let region =
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                region,
            );
        let shared_config = escape_to_async(aws_config::load_defaults(
            aws_config::BehaviorVersion::latest(),
        ));
        let shared_config = shared_config
            .to_builder()
            .region(Region::new(region))
            .build();
        let inner = aws_sdk_dynamodb::Client::new(&shared_config);
        let client = crate::DdbClient { inner };
        let dafny_client = ::dafny_runtime::upcast_object()(::dafny_runtime::object::new(client));
        dafny_runtime::Rc::new(crate::r#_Wrappers_Compile::Result::Success {
            value: dafny_client,
        })
    }

  pub fn DynamoDBClient() -> ::dafny_runtime::Rc<
  crate::r#_Wrappers_Compile::Result<
    ::dafny_runtime::Object<dyn crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::IDynamoDBClient>,
    ::dafny_runtime::Rc<crate::r#software::amazon::cryptography::services::dynamodb::internaldafny::types::Error>
    >
    >{
        let shared_config = escape_to_async(aws_config::load_defaults(
            aws_config::BehaviorVersion::latest(),
        ));
        let inner = aws_sdk_dynamodb::Client::new(&shared_config);
        let client = crate::DdbClient { inner };
        let dafny_client = ::dafny_runtime::upcast_object()(::dafny_runtime::object::new(client));
        dafny_runtime::Rc::new(crate::r#_Wrappers_Compile::Result::Success {
            value: dafny_client,
        })
    }
}
