// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use aws_mpl_legacy::Secret;
use aws_mpl_legacy::keystore;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) struct StaticKeyStoreInformation {
    pub(crate) branch_key_version: String,
    pub(crate) branch_key: Secret,
    pub(crate) beacon_key: Secret,
}

#[async_trait]
impl keystore::KeyStore for StaticKeyStoreInformation {
    async fn get_key_store_info(
        &self,
    ) -> Result<keystore::GetKeyStoreInfoOutput, aws_mpl_legacy::error::Error> {
        Ok(keystore::GetKeyStoreInfoOutput::new(
            "key-store-id".to_string(),
            "key-store-name".to_string(),
            "logical-key-store-name".to_string(),
            keystore::KmsConfiguration::KmsKeyArn(
                "arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab"
                    .to_string(),
            ),
            Vec::default(),
        ))
    }
    async fn get_active_branch_key(
        &self,
        input: &keystore::GetActiveBranchKeyInput,
    ) -> Result<keystore::GetActiveBranchKeyOutput, aws_mpl_legacy::error::Error> {
        let materials = keystore::BranchKeyMaterials::new(
            input.branch_key_identifier.clone(),
            self.branch_key_version.clone(),
            self.branch_key.clone(),
            HashMap::default(),
        );
        Ok(keystore::GetActiveBranchKeyOutput::new(materials))
    }

    async fn get_branch_key_version(
        &self,
        input: &keystore::GetBranchKeyVersionInput,
    ) -> Result<keystore::GetBranchKeyVersionOutput, aws_mpl_legacy::error::Error> {
        let materials = keystore::BranchKeyMaterials::new(
            input.branch_key_identifier.clone(),
            self.branch_key_version.clone(),
            self.branch_key.clone(),
            HashMap::default(),
        );
        Ok(keystore::GetBranchKeyVersionOutput::new(materials))
    }

    async fn get_beacon_key(
        &self,
        input: &keystore::GetBeaconKeyInput,
    ) -> Result<keystore::GetBeaconKeyOutput, aws_mpl_legacy::error::Error> {
        let materials = keystore::BeaconKeyMaterials::new(
            input.branch_key_identifier.clone(),
            self.beacon_key.clone(),
            HashMap::default(),
            HashMap::default(),
        );
        Ok(keystore::GetBeaconKeyOutput::new(materials))
    }
}
