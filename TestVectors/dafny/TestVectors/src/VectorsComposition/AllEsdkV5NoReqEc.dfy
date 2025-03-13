// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../LibraryIndex.dfy"

module {:options "/functionSyntax:4"} AllEsdkV5NoReqEc {
  import Types = AwsCryptographyEncryptionSdkTypes
  import mplTypes = AwsCryptographyMaterialProvidersTypes
  import keyVectorKeyTypes = AwsCryptographyMaterialProvidersTestVectorKeysTypes
  import ESDK
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
  import EncryptionContextUtils

  import UUID
  import UTF8
  import JSON.API
  import SortedSets
  import FileIO

  // This is a HACK!
  // This is *ONLY* because this is wrapping the MPL
  import AlgorithmSuites

  const frameSize: int64 := 512

  const AllPositiveKeyringTestsNoReqCmmNoKmsRsa
  := {}
  + AllDefaultCmm.SuccessTestingRequiredEncryptionContextKeysReproducedEncryptionContext
  + AllHierarchy.Tests
  + AllKms.Tests
  + AllKmsMrkAware.Tests
  + AllKmsMrkAwareDiscovery.Tests
  + AllRawAES.Tests
  + AllRawRSA.Tests
  + AllMulti.Tests
  + AllRawECDH.Tests
  + AllKmsEcdh.Tests

  const AwsKmsRsaTests := AllKmsRsa.Tests

  const esdkAlgorithmSuitesKmsRsa := set suite <- AllAlgorithmSuites.AllAlgorithmSuites
                                         | !suite.signature.ECDSA? && suite.id.ESDK?:: suite

  // All these tests will use a defualt CMM
  const AllPostiveKeyringTestsNoDBESuiteNoReqEC :=
    set
      keyringConfig <- AllPositiveKeyringTestsNoReqCmmNoKmsRsa | !keyringConfig.NegativeEncryptKeyringVector?,
      algorithmSuite <- AllAlgorithmSuites.ESDKAlgorithmSuites
      ::
        EsdkTestVectors.PositiveEncryptTestVector(
          version := 4,
          manifestPath := "",
          decryptManifestPath := "",
          plaintextPath := "",
          encryptDescriptions := keyringConfig.encryptDescription,
          decryptDescriptions := keyringConfig.decryptDescription,
          reproducedEncryptionContext := Some(keyringConfig.encryptionContext),
          encryptionContext := Some(keyringConfig.encryptionContext),
          frameLength := Some(frameSize),
          algorithmSuiteId := Some(algorithmSuite),
          description := keyringConfig.name
        )

  const AllPositiveKeyringTestsNoDBEKmsRsa :=
    set
      keyringConfig <- AwsKmsRsaTests | !keyringConfig.NegativeEncryptKeyringVector?,
      algorithmSuite <- esdkAlgorithmSuitesKmsRsa
      ::
        EsdkTestVectors.PositiveEncryptTestVector(
          version := 4,
          manifestPath := "",
          decryptManifestPath := "",
          plaintextPath := "",
          encryptDescriptions := keyringConfig.encryptDescription,
          decryptDescriptions := keyringConfig.decryptDescription,
          reproducedEncryptionContext := Some(keyringConfig.encryptionContext),
          encryptionContext := Some(keyringConfig.encryptionContext),
          frameLength := Some(frameSize),
          algorithmSuiteId := Some(algorithmSuite),
          description := keyringConfig.name
        )

  const Tests :=
    AllPostiveKeyringTestsNoDBESuiteNoReqEC
    + AllPositiveKeyringTestsNoDBEKmsRsa
}