// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../LibraryIndex.dfy"

module {:options "/functionSyntax:4"} AllEsdkV4NoReqEc {
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

  // A V4 manifest is can be parsed by JS, C, Older versions of Java and Python
  // To make it easier on use we will only include keyrings that have
  // an equivalent in these ESDKs.
  const AllPositiveKeyringTestsNoReqCmmNoKmsRsa
  := {}
  + AllDefaultCmm.SuccessTestingRequiredEncryptionContextKeysReproducedEncryptionContext
  + AllKms.Tests
  + AllKmsMrkAware.Tests
  + AllKmsMrkAwareDiscovery.Tests
  + AllRawAES.TestsNoEc
  + AllRawAES.TestsBasicEc
  + AllRawAES.TestControlEc
  + AllRawAES.TestsWithOnePairOfHighCodePointUtf8ValuesInEc
  + AllRawRSA.Tests
  + AllMulti.Tests

  // All these tests will use a default CMM
  const AllPostiveKeyringTestsNoDBESuiteNoReqEC :=
    set
      keyringConfig <- AllPositiveKeyringTestsNoReqCmmNoKmsRsa | !keyringConfig.NegativeEncryptKeyringVector?,
      algorithmSuite <- AllAlgorithmSuites.ESDKAlgorithmSuites | algorithmSuite.commitment.None?
      ::
        EsdkTestVectors.PositiveEncryptTestVector(
          version := 4,
          manifestPath := "",
          decryptManifestPath := "",
          plaintextPath := "",
          encryptDescriptions := keyringConfig.encryptDescription,
          decryptDescriptions := keyringConfig.decryptDescription,
          // For now, interop with the ESDK-JS is tricky because of it's sorting of the aad.
          // To deal with this detail we will only include encryption context in the test vector if the algorithm
          // suite does not have an asymmetric signature.
          reproducedEncryptionContext := if algorithmSuite.signature.ECDSA? then Some(map[]) else Some(keyringConfig.encryptionContext),
          encryptionContext := if algorithmSuite.signature.ECDSA? then Some(map[]) else Some(keyringConfig.encryptionContext),
          frameLength := Some(frameSize),
          algorithmSuiteId := Some(algorithmSuite),
          description := keyringConfig.name
        )

  const Tests :=
    AllPostiveKeyringTestsNoDBESuiteNoReqEC
}