// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../LibraryIndex.dfy"

module {:options "/functionSyntax:4" } AllEsdkV4NoReqEc {
  import Types = AwsCryptographyEncryptionSdkTypes
  import mplTypes = AwsCryptographyMaterialProvidersTypes
  import keyVectorKeyTypes = AwsCryptographyMaterialProvidersTestVectorKeysTypes
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

  import AllHierarchy
  import AllKms
  import AllKmsMrkAware
  import AllKmsMrkAwareDiscovery
  import AllKmsRsa
  import AllKmsEcdh
  import AllRawAES
  import AllRawRSA
  import AllRawECDH
  import AllDefaultCmm
  import AllRequiredEncryptionContextCmm
  import AllMulti

  import UUID
  import UTF8
  import JSON.API
  import SortedSets
  import FileIO

  // This is a HACK!
  // This is *ONLY* because this is wrapping the MPL
  import AlgorithmSuites

  const frameSize: int64 := 512

  const AllKeyringTestsNoReqCmm
  := {}
  + AllHierarchy.Tests
  + AllKms.Tests
  + AllKmsMrkAware.Tests
  + AllKmsMrkAwareDiscovery.Tests
  + AllRawAES.Tests
  + AllRawRSA.Tests
  + AllMulti.Tests
  + AllRawECDH.Tests
  + AllKmsEcdh.Tests
  + AllKmsRsa.Tests

  // All these tests will use a defualt CMM
  const AllPostiveKeyringTestsNoDBESuiteNoReqEC :=
    set
      keyringConfig <- AllKeyringTestsNoReqCmm,
      algorithmSuite <-
        AllAlgorithmSuites.ESDKAlgorithmSuites | keyringConfig in AllKmsRsa.Tests ==> !algorithmSuite.signature.ECDSA?
      ::
        EsdkTestVectors.PositiveEncryptTestVector(
          name := keyringConfig.name,
          version := 4,
          manifestPath := "",
          decryptManifestPath := "",
          plaintextPath := "",
          encryptDescriptions := keyringConfig.encryptDescription,
          decryptDescriptions := keyringConfig.decryptDescription,
          frameLength := Some(frameSize),
          algorithmSuiteId := Some(algorithmSuite),
          description := keyringConfig.name
        )

  const Tests :=
    AllPostiveKeyringTestsNoDBESuiteNoReqEC
}