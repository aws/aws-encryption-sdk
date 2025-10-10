// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "../src/Index.dfy"

include "Fixtures.dfy"

module TestEncryptDecrypt {
  import Types = AwsCryptographyEncryptionSdkTypes
  import mplTypes = AwsCryptographyMaterialProvidersTypes
  import MaterialProviders
  import ESDK
  import opened Wrappers

  import Fixtures

  method {:test} TestEncryptDecrypt()
  {
    var kmsKey :=  Fixtures.keyArn;
    print "=== TestEncryptDecrypt Starting ===\n";
    print "Using KMS Key: ", kmsKey, "\n";
    
    // The string "asdf" as bytes
    var asdf := [ 97, 115, 100, 102 ];
    print "Plaintext data: ", asdf, "\n";

    print "Creating ESDK config...\n";
    var defaultConfig := ESDK.DefaultAwsEncryptionSdkConfig();
    
    print "Initializing ESDK...\n";
    var esdkResult := ESDK.ESDK(config := defaultConfig);
    if esdkResult.Failure? {
      print "ESDK initialization failed: ", esdkResult.error, "\n";
      expect false;
    }
    var esdk := esdkResult.value;
    print "ESDK initialized successfully\n";
    
    print "Initializing MaterialProviders...\n";
    var mplResult := MaterialProviders.MaterialProviders();
    if mplResult.Failure? {
      print "MaterialProviders initialization failed: ", mplResult.error, "\n";
      expect false;
    }
    var mpl := mplResult.value;
    print "MaterialProviders initialized successfully\n";
    
    print "Creating client supplier...\n";
    var clientSupplierResult := mpl.CreateDefaultClientSupplier(mplTypes.CreateDefaultClientSupplierInput);
    if clientSupplierResult.Failure? {
      print "Client supplier creation failed: ", clientSupplierResult.error, "\n";
      expect false;
    }
    var clientSupplier := clientSupplierResult.value;
    print "Client supplier created successfully\n";
    
    print "Getting KMS client for region us-west-2...\n";
    var kmsClientResult := clientSupplier.GetClient(mplTypes.GetClientInput(region := "us-west-2"));
    if kmsClientResult.Failure? {
      print "KMS client creation failed: ", kmsClientResult.error, "\n";
      expect false;
    }
    var kmsClient := kmsClientResult.value;
    print "KMS client created successfully\n";

    print "Creating KMS keyring...\n";
    var kmsKeyringResult := mpl.CreateAwsKmsKeyring(
      mplTypes.CreateAwsKmsKeyringInput(
        kmsKeyId := kmsKey,
        kmsClient := kmsClient,
        grantTokens := None
      )
    );
    
    if kmsKeyringResult.Failure? {
      print "KMS keyring creation failed: ", kmsKeyringResult.error, "\n";
      expect false;
    }
    var kmsKeyring := kmsKeyringResult.value;
    print "KMS keyring created successfully\n";

    print "Starting encryption...\n";
    var encryptOutput := esdk.Encrypt(Types.EncryptInput(
                                        plaintext := asdf,
                                        encryptionContext := None,
                                        materialsManager := None,
                                        keyring := Some(kmsKeyring),
                                        algorithmSuiteId := None,
                                        frameLength := None
                                      ));

    if encryptOutput.Failure? {
      print "Encryption failed with error: ", encryptOutput.error, "\n";
      print "Error details: ";
      match encryptOutput.error {
        case AwsCryptographyMaterialProviders(mplError) => {
          print "MaterialProviders error: ", mplError, "\n";
          match mplError {
            case AwsCryptographicMaterialProvidersException(message) => {
              print "Exception message: ", message, "\n";
            }
            case _ => print "Other MPL error type\n";
          }
        }
        case _ => print "Non-MPL error\n";
      }
      expect false;
    }
    
    print "Encryption successful\n";
    var esdkCiphertext := encryptOutput.value.ciphertext;
    print "Ciphertext length: ", |esdkCiphertext|, " bytes\n";

    print "Starting decryption...\n";
    var decryptOutput := esdk.Decrypt(Types.DecryptInput(
                                        ciphertext := esdkCiphertext,
                                        materialsManager := None,
                                        keyring := Some(kmsKeyring),
                                        encryptionContext := None
                                      ));

    if decryptOutput.Failure? {
      print "Decryption failed with error: ", decryptOutput.error, "\n";
      expect false;
    }
    
    print "Decryption successful\n";
    var cycledPlaintext := decryptOutput.value.plaintext;
    print "Decrypted plaintext: ", cycledPlaintext, "\n";

    if cycledPlaintext == asdf {
      print "Plaintext matches original - TEST PASSED\n";
    } else {
      print "Plaintext mismatch - TEST FAILED\n";
      print "Expected: ", asdf, "\n";
      print "Got: ", cycledPlaintext, "\n";
    }
    
    expect cycledPlaintext == asdf;
    print "=== TestEncryptDecrypt Completed Successfully ===\n";
  }
}
