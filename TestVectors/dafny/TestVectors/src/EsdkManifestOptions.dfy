// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "LibraryIndex.dfy"

module {:options "-functionSyntax:4"} EsdkManifestOptions {
  import opened Wrappers
  import Types = AwsCryptographyEncryptionSdkTypes

  datatype ManifestOptions =
    | Decrypt(
        nameonly manifestPath: string,
        nameonly manifestFileName: string,
        nameonly retryPolicy: Types.NetV4_0_0_RetryPolicy,
        nameonly testName: Option<string> := None
      )
    | Encrypt(
        nameonly manifestPath: string,
        nameonly manifest: string,
        nameonly decryptManifestOutput: string,
        nameonly testName: Option<string> := None,
        nameonly legacyOutput: int := 5
      )
    | DecryptSingle(
        nameonly keysPath: string,
        nameonly keyDescription: string,
        nameonly base64Bytes: string
      )
    | EncryptManifest(
        nameonly encryptManifestOutput: string,
        nameonly version: nat
      )

}