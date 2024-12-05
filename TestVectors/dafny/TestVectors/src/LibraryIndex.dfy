// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../Model/AwsCryptographyEncryptionSdkTypesWrapped.dfy"

module
  {:extern "software.amazon.cryptography.encryptionsdk.internaldafny.wrapped" }
  WrappedESDK refines WrappedAbstractAwsCryptographyEncryptionSdkService
{
  import WrappedService = ESDK

  function method WrappedDefaultAwsEncryptionSdkConfig(): AwsEncryptionSdkConfig
  {
    AwsEncryptionSdkConfig(
      commitmentPolicy := Some(AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy.REQUIRE_ENCRYPT_REQUIRE_DECRYPT),
      maxEncryptedDataKeys := None,
      netV4_0_0_RetryPolicy := None
    )
  }

  function method WrappedAwsEncryptionSdkConfigWithSuppliedCommitment(
    commitmentPolicy: AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy
  ): AwsEncryptionSdkConfig
  {
    AwsEncryptionSdkConfig(
      commitmentPolicy := Some(commitmentPolicy),
      maxEncryptedDataKeys := None,
      netV4_0_0_RetryPolicy := None
    )
  }
  function method WrappedAwsEncryptionSdkConfig(
    commitmentPolicy: AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy,
    maxEncryptedDataKeys: CountingNumbers,
    netV4_0_0_RetryPolicy: NetV4_0_0_RetryPolicy
  ) : AwsEncryptionSdkConfig
  {
    AwsEncryptionSdkConfig(
      commitmentPolicy := Some(commitmentPolicy),
      maxEncryptedDataKeys := Some(maxEncryptedDataKeys),
      netV4_0_0_RetryPolicy := Some(netV4_0_0_RetryPolicy)
    )
  }
}