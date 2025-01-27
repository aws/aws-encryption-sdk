// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

using Amazon.DynamoDBv2;
using Amazon.KeyManagementService;
using AWS.Cryptography.KeyStore;

public class VersionBranchKey
{
   public static void versionBranchKey(string branchKeyId)
   {
        // Create an AWS KMS Configuration to use with your KeyStore.
        // The KMS Configuration MUST have the right access to the resource in the KeyStore.
        var kmsConfig = new KMSConfiguration { KmsKeyArn = ExampleUtils.ExampleUtils.GetBranchKeyArn() };
        // Create a KeyStore
        var keystoreConfig = new KeyStoreConfig
        {
            KmsClient = new AmazonKeyManagementServiceClient(),
            KmsConfiguration = kmsConfig,
            DdbTableName = ExampleUtils.ExampleUtils.GetKeyStoreName(),
            DdbClient = new AmazonDynamoDBClient(),
            LogicalKeyStoreName = ExampleUtils.ExampleUtils.GetLogicalKeyStoreName() 
        };
        var keystore = new KeyStore(keystoreConfig);
        
        // To version a branch key you MUST have access to kms:ReEncrypt* and kms:GenerateDataKeyWithoutPlaintext
        keystore.VersionKey(new VersionKeyInput{BranchKeyIdentifier = branchKeyId});
   }
}
