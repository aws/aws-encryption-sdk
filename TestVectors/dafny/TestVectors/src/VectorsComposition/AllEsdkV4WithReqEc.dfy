// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../LibraryIndex.dfy"

module {:options "/functionSyntax:4" } AllEsdkV4WithReqEc {
  import Types = AwsCryptographyEncryptionSdkTypes
  import mplTypes = AwsCryptographyMaterialProvidersTypes
  import EncryptionSdk
  import MaterialProviders
  import opened CompleteVectors
  import opened KeyDescription
  import opened Wrappers
  import opened StandardLibrary.UInt
  import HexStrings
  import opened JSON.Values
  import JSONHelpers
  import EsdkManifestOptions
  import EsdkTestVectors
  import AllEsdkV4NoReqEc

  import UUID
  import UTF8
  import JSON.API
  import SortedSets
  import FileIO

  // This is a HACK!
  // This is *ONLY* because this is wrapping the MPL
  import AlgorithmSuites

  const frameSize: int64 := 512

  const AllPositiveReqEcTests := AllRequiredEncryptionContextCmm.SuccessTestingRequiredEncryptionContextKeysReproducedEncryptionContext

  // These are only required encryption context vectors with static aes keyrings
  const AllPositiveReqEcEsdkTests :=
    set
      config <- AllPositiveReqEcTests,
      algorithmSuite <-
        AllAlgorithmSuites.ESDKAlgorithmSuites
      ::
        EsdkTestVectors.PositiveEncryptTestVector(
          version := 4,
          manifestPath := "",
          decryptManifestPath := "",
          plaintextPath := "",
          encryptDescriptions := config.encryptDescription,
          decryptDescriptions := config.decryptDescription,
          encryptionContext := Some(config.encryptionContext),
          reproducedEncryptionContext := config.reproducedEncryptionContext,
          frameLength := Some(frameSize),
          algorithmSuiteId := Some(algorithmSuite),
          description := config.name
        )

  const Tests :=
    AllPositiveReqEcEsdkTests
}