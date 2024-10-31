// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
include "../../../../mpl/StandardLibrary/src/Index.dfy"
  // BEGIN MANUAL EDIT
include "../../../../AwsEncryptionSDK/dafny/AwsEncryptionSdk/src/Index.dfy"
include "../../../../mpl/TestVectorsAwsCryptographicMaterialProviders/dafny/KeyVectors/src/Index.dfy"
  // END MANUAL EDIT
include "../src/Index.dfy"
abstract module WrappedAbstractAwsCryptographyEncryptionSdkService {
  import opened Wrappers
  import opened StandardLibrary.UInt
  import opened UTF8
  import opened Types = AwsCryptographyEncryptionSdkTypes
  import WrappedService : AbstractAwsCryptographyEncryptionSdkService
  function method WrappedDefaultAwsEncryptionSdkConfig(): AwsEncryptionSdkConfig
  method {:extern} WrappedESDK(config: AwsEncryptionSdkConfig := WrappedDefaultAwsEncryptionSdkConfig())
    returns (res: Result<IAwsEncryptionSdkClient, Error>)
    ensures res.Success? ==>
              && fresh(res.value)
              && fresh(res.value.Modifies)
              && fresh(res.value.History)
              && res.value.ValidState()
}
