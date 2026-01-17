// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use aws_mpl_rs::keystore;
use aws_mpl_rs::types::Secret;
use std::collections::HashMap;

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub(crate) struct StaticKeyStoreInformation {
    // pub(crate) key_identifier: String,
    pub(crate) branch_key_version: String,
    pub(crate) branch_key: Secret,
    pub(crate) beacon_key: Secret,
}

// fn mpl_err(message: &str) -> aws_mpl_rs::error::Error {
//     aws_mpl_rs::error::err(aws_mpl_rs::error::ErrorKind::Consumer(message.to_string()))
// }

#[async_trait]
impl keystore::KeyStore for StaticKeyStoreInformation {
    async fn get_key_store_info(
        &self,
    ) -> Result<keystore::GetKeyStoreInfoOutput, aws_mpl_rs::error::Error> {
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
    ) -> Result<keystore::GetActiveBranchKeyOutput, aws_mpl_rs::error::Error> {
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
    ) -> Result<keystore::GetBranchKeyVersionOutput, aws_mpl_rs::error::Error> {
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
    ) -> Result<keystore::GetBeaconKeyOutput, aws_mpl_rs::error::Error> {
        let materials = keystore::BeaconKeyMaterials::new(
            input.branch_key_identifier.clone(),
            self.beacon_key.clone(),
            HashMap::default(),
            HashMap::default(),
        );
        Ok(keystore::GetBeaconKeyOutput::new(materials))
    }
}
/*
    method GetKeyStoreInfo()
      returns (output: Result<GetKeyStoreInfoOutput, Error>)
      requires
        && ValidState()
      modifies Modifies - {History} ,
               History`GetKeyStoreInfo
      // Dafny will skip type parameters when generating a default decreases clause.
      decreases Modifies - {History}
      ensures
        && ValidState()
      ensures GetKeyStoreInfoEnsuresPublicly(output)
      ensures History.GetKeyStoreInfo == old(History.GetKeyStoreInfo) + [DafnyCallEvent((), output)]
    {
      output : Success(
        AwsCryptographyKeyStoreTypes.GetKeyStoreInfoOutput(
          keyStoreId : "key-store-id",
          keyStoreName : "key-store-name",
          logicalKeyStoreName : "logical-key-store-name",
          grantTokens : [],
          kmsConfiguration : KMSConfiguration.kmsKeyArn("arn:aws:kms:us-east-2:111122223333:key/1234abcd-12ab-34cd-56ef-1234567890ab")
        )
      );
      History.GetKeyStoreInfo : History.GetKeyStoreInfo + [DafnyCallEvent((), output)];
    }

    // All methods except GetKeyStoreInfo are not supported operations in a static context

    ghost predicate CreateKeyStoreEnsuresPublicly(input: CreateKeyStoreInput , output: Result<CreateKeyStoreOutput, Error>)
    {true}
    // The public method to be called by library consumers
    method CreateKeyStore ( input: CreateKeyStoreInput )
      returns (output: Result<CreateKeyStoreOutput, Error>)
      requires
        && ValidState()
      modifies Modifies - {History} ,
               History`CreateKeyStore
      // Dafny will skip type parameters when generating a default decreases clause.
      decreases Modifies - {History}
      ensures
        && ValidState()
      ensures CreateKeyStoreEnsuresPublicly(input, output)
      ensures History.CreateKeyStore == old(History.CreateKeyStore) + [DafnyCallEvent(input, output)]
    {
      output : Failure(KeyStoreException( message : "Not Supported"));
      History.CreateKeyStore : History.CreateKeyStore + [DafnyCallEvent(input, output)];
    }

    ghost predicate CreateKeyEnsuresPublicly(input: CreateKeyInput, output: Result<CreateKeyOutput, Error>)
    {true}
    // The public method to be called by library consumers
    method CreateKey ( input: CreateKeyInput )
      returns (output: Result<CreateKeyOutput, Error>)
      requires
        && ValidState()
      modifies Modifies - {History} ,
               History`CreateKey
      // Dafny will skip type parameters when generating a default decreases clause.
      decreases Modifies - {History}
      ensures
        && ValidState()
      ensures CreateKeyEnsuresPublicly(input, output)
      ensures History.CreateKey == old(History.CreateKey) + [DafnyCallEvent(input, output)]
    {
      output : Failure(KeyStoreException( message : "Not Supported"));
      History.CreateKey : History.CreateKey + [DafnyCallEvent(input, output)];
    }

    ghost predicate VersionKeyEnsuresPublicly(input: VersionKeyInput , output: Result<VersionKeyOutput, Error>)
    {true}
    // The public method to be called by library consumers
    method VersionKey ( input: VersionKeyInput )
      returns (output: Result<VersionKeyOutput, Error>)
      requires
        && ValidState()
      modifies Modifies - {History} ,
               History`VersionKey
      // Dafny will skip type parameters when generating a default decreases clause.
      decreases Modifies - {History}
      ensures
        && ValidState()
      ensures VersionKeyEnsuresPublicly(input, output)
      ensures History.VersionKey == old(History.VersionKey) + [DafnyCallEvent(input, output)]
    {
      output : Failure(KeyStoreException( message : "Not Supported"));
      History.VersionKey : History.VersionKey + [DafnyCallEvent(input, output)];
    }

  }
}
*/
