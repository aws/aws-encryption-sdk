// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

using Amazon.DynamoDBv2;
using Amazon.KeyManagementService;
using AWS.Cryptography.KeyStore;

public class CreateBranchKey
{
    public static string createBranchKey()
    {
        // Create an AWS KMS Configuration to use with your KeyStore.
        // The KMS Configuration MUST have the right access to the resources in the KeyStore.
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
        // Create a branch key identifier with the AWS KMS Key configured in the KeyStore Configuration.
        var branchKeyId = keystore.CreateKey(new CreateKeyInput());
        
        return branchKeyId.BranchKeyIdentifier;
    }
}
