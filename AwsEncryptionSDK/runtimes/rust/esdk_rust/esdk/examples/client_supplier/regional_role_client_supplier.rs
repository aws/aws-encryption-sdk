// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::regional_role_client_supplier_config;
use aws_config::Region;
use aws_esdk::material_providers::operation::get_client::GetClientInput;
use aws_esdk::material_providers::types::ClientSupplier;
use aws_esdk::material_providers::types::error::Error;
use aws_mpl_rs::com_amazonaws_kms::client::Client as kms_client;

/*
 Example class demonstrating an implementation of a custom client supplier.
 This particular implementation will create KMS clients with different IAM roles,
 depending on the region passed.
*/

pub struct RegionalRoleClientSupplier {}

impl ClientSupplier for RegionalRoleClientSupplier {
    fn get_client(&self, input: GetClientInput) -> Result<kms_client, Error> {
        let region = input.region.unwrap();

        let region_iam_role_map = regional_role_client_supplier_config::region_iam_role_map();

        if !region_iam_role_map.contains_key(&region) {
            return Err(Error::AwsCryptographicMaterialProvidersException {
                message: format!("Region {region} is not supported by this client supplier")
                    .to_string(),
            });
        }

        let arn = region_iam_role_map[&region].clone();

        use aws_config::sts::AssumeRoleProvider;

        let provider = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                AssumeRoleProvider::builder(arn)
                    .region(Region::new(region.clone()))
                    .session_name("Rust-ESDK-Client-Supplier-Example-Session")
                    .build()
                    .await
            })
        });

        let sdk_config = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await
            })
        });
        let kms_config = aws_sdk_kms::config::Builder::from(&sdk_config)
            .credentials_provider(provider)
            .region(Region::new(region))
            .build();

        let inner_client = aws_sdk_kms::Client::from_conf(kms_config);
        Ok(kms_client {
            inner: inner_client,
        })
    }
}
