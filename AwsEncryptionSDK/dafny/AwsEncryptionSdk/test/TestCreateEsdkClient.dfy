// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../src/Index.dfy"

module TestCreateEsdkClient {
  import Types = AwsCryptographyEncryptionSdkTypes
  import mplTypes = AwsCryptographyMaterialProvidersTypes
  import ESDK
  import MaterialProviders
  import opened Wrappers
  import opened UInt = StandardLibrary.UInt


  // THIS IS AN INCORRECTLY SERIALIZED CIPHERTEXT PRODUCED BY
  // THE ESDK .NET V4.0.0
  // This message was constructucted with a zeroed 32 byte AES Key
  // using v4.0.0 of the Encryption SDK for .NET which incorrectly
  // serializes the message header making messages unreadble in
  // other implementations and making this version unable to
  // read other implementation's messages.
  const ESDK_NET_V400_MESSAGE: seq<uint8> := [
    2, 5, 120, 238, 5, 239, 107, 129, 136, 211, 103, 75, 18, 140,
    11, 74, 26, 191, 92, 27, 202, 170, 33, 28, 9, 117, 252, 29, 29,
    92, 213, 21, 231, 172, 234, 0, 95, 0, 1, 0, 21, 97, 119, 115, 45,
    99, 114, 121, 112, 116, 111, 45, 112, 117, 98, 108, 105, 99, 45, 107,
    101, 121, 0, 68, 65, 119, 102, 117, 103, 90, 99, 107, 57, 116, 100, 53,
    104, 78, 108, 49, 78, 108, 75, 111, 47, 104, 105, 114, 53, 85, 47, 48, 81,
    109, 98, 73, 111, 107, 79, 72, 81, 87, 97, 72, 83, 43, 115, 117, 119, 75,
    73, 77, 82, 76, 99, 67, 80, 49, 54, 55, 56, 43, 49, 82, 75, 49, 48, 82,
    101, 119, 61, 61, 0, 1, 0, 21, 83, 111, 109, 101, 32, 109, 97, 110, 97,
    103, 101, 100, 32, 114, 97, 119, 32, 107, 101, 121, 115, 0, 47, 77, 121,
    32, 50, 53, 54, 45, 98, 105, 116, 32, 65, 69, 83, 32, 119, 114, 97, 112,
    112, 105, 110, 103, 32, 107, 101, 121, 0, 0, 0, 128, 0, 0, 0, 12, 229, 254,
    197, 205, 110, 124, 222, 48, 217, 121, 252, 11, 0, 48, 64, 60, 232, 232, 76,
    229, 15, 118, 224, 152, 79, 93, 113, 166, 255, 172, 255, 148, 185, 150, 195, 179,
    78, 52, 186, 38, 216, 48, 118, 45, 113, 204, 71, 102, 116, 148, 199, 109, 178,
    19, 2, 203, 150, 201, 65, 32, 199, 180, 2, 0, 0, 16, 0, 67, 72, 208, 112, 230,
    137, 188, 187, 0, 28, 183, 198, 192, 45, 248, 108, 2, 129, 34, 42, 59, 155, 70,
    117, 182, 216, 239, 27, 210, 78, 62, 104, 181, 247, 141, 50, 133, 42, 72, 200,
    185, 57, 20, 49, 193, 240, 171, 140, 255, 255, 255, 255, 0, 0, 0, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 15, 67, 37, 106, 11, 15, 23, 78, 239, 208,
    185, 4, 36, 182, 9, 63, 62, 83, 97, 42, 250, 252, 185, 165, 14, 182, 231, 83,
    176, 227, 191, 92, 0, 103, 48, 101, 2, 49, 0, 193, 152, 7, 169, 197, 137, 244,
    88, 9, 1, 6, 56, 96, 13, 220, 201, 56, 16, 50, 68, 70, 36, 174, 38, 14, 241, 207,
    11, 139, 154, 166, 224, 191, 20, 12, 175, 56, 117, 183, 120, 119, 228, 173, 130,
    71, 110, 211, 189, 2, 48, 99, 98, 250, 36, 53, 182, 2, 204, 198, 55, 150, 51,
    159, 101, 231, 34, 42, 30, 57, 204, 88, 114, 138, 94, 12, 79, 52, 71, 178,
    34, 61, 246, 55, 163, 145, 95, 80, 61, 85, 143, 32, 0, 98, 20, 88, 251, 204, 5
  ]

  method {:test} TestClientCreation() {
    var defaultConfig := ESDK.DefaultAwsEncryptionSdkConfig();

    var esdk: Types.IAwsEncryptionSdkClient :- expect ESDK.ESDK(config := defaultConfig);
    expect esdk is ESDK.ESDKClient;
    var esdkClient := esdk as ESDK.ESDKClient;

    expect esdkClient.config.commitmentPolicy == defaultConfig.commitmentPolicy.value;
    expect esdkClient.config.maxEncryptedDataKeys == defaultConfig.maxEncryptedDataKeys;
    expect esdkClient.config.netV4_0_0_RetryPolicy == Types.NetV4_0_0_RetryPolicy.ALLOW_RETRY;
  }

  method {:test} TestNetRetryFlag() {
    var mpl :- expect MaterialProviders.MaterialProviders();
    var keyNamespace := "Some managed raw keys";
    var keyName := "My 256-bit AES wrapping key";
    var expectedMessage : seq<uint8> := [84,104,105,115,32,105,115,32,97,32,116,101,115,116,46];

    var rawAesKeyring :- expect mpl.CreateRawAesKeyring(mplTypes.CreateRawAesKeyringInput(
                                                          keyNamespace := keyNamespace,
                                                          keyName := keyName,
                                                          wrappingKey := seq(32, i => 0),
                                                          wrappingAlg := mplTypes.ALG_AES256_GCM_IV12_TAG16
                                                        ));

    // Attempt to decrypt the v4.0.0 message without the retry flag and expect
    // decryption to fail
    var esdkConfig := Types.AwsEncryptionSdkConfig(
      commitmentPolicy := Some(mplTypes.ESDKCommitmentPolicy.REQUIRE_ENCRYPT_REQUIRE_DECRYPT),
      maxEncryptedDataKeys := None,
      netV4_0_0_RetryPolicy := Some(Types.NetV4_0_0_RetryPolicy.FORBID_RETRY)
    );

    var noRetryEsdk :- expect ESDK.ESDK(config := esdkConfig);

    var expectFailureDecryptOutput := noRetryEsdk.Decrypt(Types.DecryptInput(
                                                            ciphertext := ESDK_NET_V400_MESSAGE,
                                                            materialsManager := None,
                                                            keyring := Some(rawAesKeyring),
                                                            encryptionContext := None
                                                          ));

    expect expectFailureDecryptOutput.Failure?;

    // Decrypt v4.0.0 message with the default configuration which is to retry
    // and expect decryption to pass
    var defaultConfig := ESDK.DefaultAwsEncryptionSdkConfig();
    var esdk :- expect ESDK.ESDK(config := defaultConfig);

    var decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                        ciphertext := ESDK_NET_V400_MESSAGE,
                                        materialsManager := None,
                                        keyring := Some(rawAesKeyring),
                                        encryptionContext := None
                                      ));

    expect decryptOutput.Success?;
    expect decryptOutput.value.plaintext == expectedMessage;
  }

}
