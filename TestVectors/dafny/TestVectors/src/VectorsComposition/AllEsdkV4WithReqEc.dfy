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

  datatype PositiveESDKDescriptionJSON = PositiveESDKDescriptionJSON(
    description: string,
    inputEncryptionContext: string,
    requiredEncryptionContextKeys: string,
    reproducedEncryptionContext: string,
    encrypt: JSON,
    decrypt: JSON
  )

  datatype SmallEncryptionContextVariation = Empty | A | AB | BA

  const AllSmallEncryptionContextVariants := ["A", "AB", "BA"]
  const RequiredEncryptionContextKeys := ["A", "B"]

  const AllReqECCmmInfo :=
    set
      ec <- AllSmallEncryptionContextVariants,
      requiredKeys <- RequiredEncryptionContextKeys
      ::
        var cmmOnEncryptDescription := Object([
                                                ("type", String("Required Encryption Context CMM")),
                                                ("Input Encryption Context", String(ec)),
                                                ("Required Encryption Context Keys", String(requiredKeys))
                                              ]);
        var cmmOnDecryptDescription := Object([
                                                ("type", String("Required Encryption Context CMM")),
                                                ("Reproduced Encryption Context", String(ec)),
                                                ("Required Encryption Context Keys", String(requiredKeys))
                                              ]);
        PositiveESDKDescriptionJSON(
          description := "Generated with Required Encryption Context Keys " + requiredKeys,
          inputEncryptionContext := ec,
          requiredEncryptionContextKeys := requiredKeys,
          reproducedEncryptionContext := ec,
          encrypt := cmmOnEncryptDescription,
          decrypt := cmmOnDecryptDescription
        )
  
  const frameSize: int64 := 512

  const AllPositiveReqEcTests := AllRequiredEncryptionContextCmm.SuccessTestingRequiredEncryptionContextKeysReproducedEncryptionContext

  // These are only required encryption context vectors with static aes keyrings
  const AllPositveReqEcEsdkTests :=
    set 
     config <- AllPositiveReqEcTests,
     algorithmSuite <-
      AllAlgorithmSuites.ESDKAlgorithmSuites
     ::
        EsdkTestVectors.PositiveEncryptTestVector(
          name := config.name,
          version := 4,
          manifestPath := "",
          decryptManifestPath := "",
          plaintextPath := "",
          encryptDescriptions := config.encryptDescription,
          decryptDescriptions := config.decryptDescription,
          encryptionContext := Some(config.encryptionContext),
          requiredEncryptionContextKeys := config.requiredEncryptionContextKeys,
          requiredECDescription := Some(config.name),
          frameLength := Some(frameSize),
          algorithmSuiteId := Some(algorithmSuite),
          description := config.name
        )
  
  const Tests := 
    AllPositveReqEcEsdkTests 
    
}