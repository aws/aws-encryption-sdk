// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![deny(warnings, unconditional_panic)]
#![deny(nonstandard_style)]
#![deny(clippy::all)]

use aws_config::{AppName, Region, SdkConfig};
use std::sync::LazyLock;
static DAFNY_TOKIO_RUNTIME: LazyLock<tokio::runtime::Runtime> = LazyLock::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
});

impl crate::r#software::amazon::cryptography::services::kms::internaldafny::_default {
    #[allow(non_snake_case)]
    fn CreateSdkConfig() -> SdkConfig {
        let shared_config = match tokio::runtime::Handle::try_current() {
            Ok(curr) => tokio::task::block_in_place(|| {
                curr.block_on(async {
                    aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await
                })
            }),
            Err(_) => DAFNY_TOKIO_RUNTIME.block_on(aws_config::load_defaults(
                aws_config::BehaviorVersion::latest(),
            )),
        };

        Self::AddUserAgentStringToConfig(&shared_config)
    }

    #[allow(non_snake_case)]
    pub fn KMSClientForRegion(region: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>) -> ::dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::kms::internaldafny::types::IKMSClient>, ::dafny_runtime::Rc<crate::software::amazon::cryptography::services::kms::internaldafny::types::Error>>>{
        let region =
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                region,
            );

        let shared_config = &Self::CreateSdkConfig();
        let shared_config = shared_config
            .to_builder()
            .region(Region::new(region))
            .build();
        let inner = aws_sdk_kms::Client::new(&shared_config);
        let client = crate::KmsClient { inner };
        let dafny_client = ::dafny_runtime::upcast_object()(::dafny_runtime::object::new(client));
        dafny_runtime::Rc::new(crate::r#_Wrappers_Compile::Result::Success {
            value: dafny_client,
        })
    }

    #[allow(non_snake_case)]
    pub fn KMSClient() -> ::dafny_runtime::Rc<crate::r#_Wrappers_Compile::Result<::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::kms::internaldafny::types::IKMSClient>, ::dafny_runtime::Rc<crate::software::amazon::cryptography::services::kms::internaldafny::types::Error>>>{
        let shared_config = &Self::CreateSdkConfig();
        let inner = aws_sdk_kms::Client::new(shared_config);
        let client = crate::KmsClient { inner };
        let dafny_client = ::dafny_runtime::upcast_object()(::dafny_runtime::object::new(client));
        dafny_runtime::Rc::new(crate::r#_Wrappers_Compile::Result::Success {
            value: dafny_client,
        })
    }

    #[allow(non_snake_case)]
    fn AddUserAgentStringToConfig(sdkConfig: &SdkConfig) -> SdkConfig {
        let runtime = "Rust".to_string();
        let runtime_msg =
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::string_to_dafny_string(
                &runtime,
            );
        // sadly rust doesn't allow for '/' in the app name which the dafny function adds
        // so we will replace '/' with '-' which is allowed
        let user_agent_string = dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
            &crate::software::amazon::cryptography::services::kms::internaldafny::_default::DafnyUserAgentSuffix(&runtime_msg)
        );
        let replaced_user_agent_string = user_agent_string.replace("/", "-");
        // To update the user agent string we take the application name and update it.
        let current_app_name = sdkConfig
            .app_name()
            .map(|app_name| app_name.to_string())
            .unwrap_or_default();
        let new_app_name = if current_app_name.is_empty() {
            replaced_user_agent_string.to_string()
        } else {
            format!("{} {} ", current_app_name, replaced_user_agent_string)
        };
        let app_name = AppName::new(new_app_name).expect("Valid app name");
        sdkConfig.to_builder().app_name(app_name).build()
    }
    #[allow(non_snake_case)]
    pub fn RegionMatch(
        kmsClient: &::dafny_runtime::Object<dyn crate::software::amazon::cryptography::services::kms::internaldafny::types::IKMSClient>,
        region: &::dafny_runtime::Sequence<::dafny_runtime::DafnyCharUTF16>,
    ) -> ::dafny_runtime::Rc<crate::r#_Wrappers_Compile::Option<bool>> {
        let region =
            dafny_runtime::dafny_runtime_conversions::unicode_chars_false::dafny_string_to_string(
                region,
            );
        let any = dafny_runtime::cast_any_object!(kmsClient);
        let client = dafny_runtime::cast_object!(any, crate::KmsClient);
        let flag = match client.as_ref().inner.config().region() {
            Some(r) => r.as_ref() == region,
            None => false,
        };
        ::dafny_runtime::Rc::new(crate::r#_Wrappers_Compile::Option::Some { value: flag })
    }
}
