// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
package software.amazon.cryptography.encryptionsdk.wrapped;

import static software.amazon.cryptography.encryptionsdk.wrapped.KeyringToMasterKeyProvider.createMasterKeyProvider;

import Wrappers_Compile.Result;
import com.amazonaws.encryptionsdk.AwsCrypto;
import com.amazonaws.encryptionsdk.CryptoAlgorithm;
import com.amazonaws.encryptionsdk.CryptoResult;
import com.amazonaws.encryptionsdk.MasterKeyProvider;
import dafny.DafnyMap;
import dafny.DafnySequence;
import java.util.Objects;
import software.amazon.cryptography.encryptionsdk.ToDafny;
import software.amazon.cryptography.encryptionsdk.ToNative;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.DecryptInput;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.DecryptOutput;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.EncryptInput;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.EncryptOutput;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.Error;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.IAwsEncryptionSdkClient;
import software.amazon.cryptography.materialproviders.internaldafny.types.ESDKAlgorithmSuiteId;
import software.amazon.smithy.dafny.conversion.ToDafny.Simple;

@SuppressWarnings("ALL")
public class TestESDK implements IAwsEncryptionSdkClient {

  private final AwsCrypto _impl;
  private final boolean _prefer_mkp_over_keyring;

  protected TestESDK(BuilderImpl builder) {
    this._impl = builder.impl();
    this._prefer_mkp_over_keyring = builder.isPrefer_mkp_over_keyring();
  }

  public static Builder builder() {
    return new BuilderImpl();
  }

  public Result<DecryptOutput, Error> Decrypt(DecryptInput dafnyInput) {
    try {
      software.amazon.cryptography.encryptionsdk.model.DecryptInput nativeInput =
        ToNative.DecryptInput(dafnyInput);
      MasterKeyProvider<?> provider = null;

      // Convert will handle supported keyrings directly
      // Returns null for unsupported MKP to allow encryption/decryption with keyrings instead
      if (_prefer_mkp_over_keyring) {
        if (dafnyInput.dtor_keyring().is_Some()) {
          provider =
            createMasterKeyProvider(dafnyInput.dtor_keyring().dtor_value());
        } else if (dafnyInput.dtor_materialsManager().is_Some()) {
          provider =
            createMasterKeyProvider(
              dafnyInput.dtor_materialsManager().dtor_value()
            );
        }
      }

      final CryptoResult<byte[], ?> decryptResult;

      if (_prefer_mkp_over_keyring && provider != null) {
        decryptResult =
          this._impl.decryptData(provider, nativeInput.ciphertext().array());
        if (!Objects.isNull(nativeInput.encryptionContext())) {
          // For ESDK Java V2, We do not support to verify encryption context during decrypt call.
          // We have to explicitly verify for EC outside of decrypt. For V3, MKPs were deprecated.
          // TODO: Error message SHOULD include expected key-value and actual value
          // TODO: If key is missing, error message should detail which key is missing.
          if (
            !nativeInput
              .encryptionContext()
              .entrySet()
              .stream()
              .allMatch(e ->
                e
                  .getValue()
                  .equals(decryptResult.getEncryptionContext().get(e.getKey()))
              )
          ) {
            throw new IllegalStateException(
              String.format(
                "Encryption Context mismatch - Expected: %s, Actual: %s",
                nativeInput.encryptionContext(),
                decryptResult.getEncryptionContext()
              )
            );
          }
        }
      } else {
        if (Objects.isNull(nativeInput.materialsManager())) {
          // Call decrypt with keyring
          if (Objects.isNull(nativeInput.encryptionContext())) {
            decryptResult =
              this._impl.decryptData(
                  nativeInput.keyring(),
                  nativeInput.ciphertext().array()
                );
          } else {
            decryptResult =
              this._impl.decryptData(
                  nativeInput.keyring(),
                  nativeInput.ciphertext().array(),
                  nativeInput.encryptionContext()
                );
          }
        } else {
          if (Objects.isNull(nativeInput.encryptionContext())) {
            decryptResult =
              this._impl.decryptData(
                  nativeInput.materialsManager(),
                  nativeInput.ciphertext().array()
                );
          } else {
            decryptResult =
              this._impl.decryptData(
                  nativeInput.materialsManager(),
                  nativeInput.ciphertext().array(),
                  nativeInput.encryptionContext()
                );
          }
        }
      }
      // Convert Legacy ESDK-Java CryptoResult to Dafny-Java-Native ESDK DecryptOutput
      DafnySequence<? extends Byte> plaintext = Simple.ByteSequence(
        decryptResult.getResult()
      );
      DafnyMap<
        ? extends DafnySequence<? extends Byte>,
        ? extends DafnySequence<? extends Byte>
      > encryptionContext =
        software.amazon.cryptography.materialproviders.ToDafny.EncryptionContext(
          decryptResult.getEncryptionContext()
        );
      ESDKAlgorithmSuiteId algorithmSuiteId =
        software.amazon.cryptography.materialproviders.ToDafny.ESDKAlgorithmSuiteId(
          decryptResult.getCryptoAlgorithm().getAlgorithmSuiteId().ESDK()
        );
      DecryptOutput dafnyOutput = new DecryptOutput(
        plaintext,
        encryptionContext,
        algorithmSuiteId
      );

      return Result.create_Success(
        DecryptOutput._typeDescriptor(),
        Error._typeDescriptor(),
        dafnyOutput
      );
    } catch (RuntimeException ex) {
      return Result.create_Failure(
        DecryptOutput._typeDescriptor(),
        Error._typeDescriptor(),
        ToDafny.Error(ex)
      );
    }
  }

  public Result<EncryptOutput, Error> Encrypt(EncryptInput dafnyInput) {
    try {
      software.amazon.cryptography.encryptionsdk.model.EncryptInput nativeInput =
        ToNative.EncryptInput(dafnyInput);

      final CryptoResult<byte[], ?> encryptResult;
      MasterKeyProvider<?> provider = null;

      // Convert will handle supported keyrings directly
      // Returns null for unsupported MKP to allow encryption/decryption with keyrings instead
      if (_prefer_mkp_over_keyring) {
        if (dafnyInput.dtor_keyring().is_Some()) {
          provider =
            createMasterKeyProvider(dafnyInput.dtor_keyring().dtor_value());
        } else if (dafnyInput.dtor_materialsManager().is_Some()) {
          provider =
            createMasterKeyProvider(
              dafnyInput.dtor_materialsManager().dtor_value()
            );
        }
      }

      // Java ESDK is special and you have to set the algorithm suite both in the keyring which the
      // test vectors do, but also in the client itself.
      CryptoAlgorithm cryptoAlgorithm = _getAlgorithmSuite(
        nativeInput.algorithmSuiteId()
      );
      this._impl.setEncryptionAlgorithm(cryptoAlgorithm);

      if (_prefer_mkp_over_keyring && provider != null) {
        // Call decrypt with MKP
        if (Objects.isNull(nativeInput.encryptionContext())) {
          encryptResult =
            this._impl.encryptData(provider, nativeInput.plaintext().array());
        } else {
          encryptResult =
            this._impl.encryptData(
                provider,
                nativeInput.plaintext().array(),
                nativeInput.encryptionContext()
              );
        }
      } else {
        // If the CMM is null, it MUST be a Keyring
        if (Objects.isNull(nativeInput.materialsManager())) {
          // Call decrypt with keyring
          if (Objects.isNull(nativeInput.encryptionContext())) {
            encryptResult =
              this._impl.encryptData(
                  nativeInput.keyring(),
                  nativeInput.plaintext().array()
                );
          } else {
            encryptResult =
              this._impl.encryptData(
                  nativeInput.keyring(),
                  nativeInput.plaintext().array(),
                  nativeInput.encryptionContext()
                );
          }
        } else { // We are in the CMM case
          if (Objects.isNull(nativeInput.encryptionContext())) {
            encryptResult =
              this._impl.encryptData(
                  nativeInput.materialsManager(),
                  nativeInput.plaintext().array()
                );
          } else {
            encryptResult =
              this._impl.encryptData(
                  nativeInput.materialsManager(),
                  nativeInput.plaintext().array(),
                  nativeInput.encryptionContext()
                );
          }
        }
      }
      dafny.DafnySequence<? extends Byte> ciphertext = Simple.ByteSequence(
        encryptResult.getResult()
      );
      DafnyMap<
        ? extends DafnySequence<? extends Byte>,
        ? extends DafnySequence<? extends Byte>
      > encryptionContext =
        software.amazon.cryptography.materialproviders.ToDafny.EncryptionContext(
          encryptResult.getEncryptionContext()
        );
      ESDKAlgorithmSuiteId algorithmSuiteId =
        software.amazon.cryptography.materialproviders.ToDafny.ESDKAlgorithmSuiteId(
          encryptResult.getCryptoAlgorithm().getAlgorithmSuiteId().ESDK()
        );

      EncryptOutput dafnyOutput = new EncryptOutput(
        ciphertext,
        encryptionContext,
        algorithmSuiteId
      );
      return Result.create_Success(
        EncryptOutput._typeDescriptor(),
        Error._typeDescriptor(),
        dafnyOutput
      );
    } catch (RuntimeException ex) {
      return Result.create_Failure(
        EncryptOutput._typeDescriptor(),
        Error._typeDescriptor(),
        ToDafny.Error(ex)
      );
    }
  }

  private CryptoAlgorithm _getAlgorithmSuite(
    software.amazon.cryptography.materialproviders.model.ESDKAlgorithmSuiteId esdkAlgorithmSuiteId
  ) {
    switch (esdkAlgorithmSuiteId) {
      case ALG_AES_128_GCM_IV12_TAG16_NO_KDF:
        return CryptoAlgorithm.ALG_AES_128_GCM_IV12_TAG16_NO_KDF;
      case ALG_AES_192_GCM_IV12_TAG16_NO_KDF:
        return CryptoAlgorithm.ALG_AES_192_GCM_IV12_TAG16_NO_KDF;
      case ALG_AES_256_GCM_IV12_TAG16_NO_KDF:
        return CryptoAlgorithm.ALG_AES_256_GCM_IV12_TAG16_NO_KDF;
      case ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256:
        return CryptoAlgorithm.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256;
      case ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256:
        return CryptoAlgorithm.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256;
      case ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256:
        return CryptoAlgorithm.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256;
      case ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256:
        return CryptoAlgorithm.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256;
      case ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384:
        return CryptoAlgorithm.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384;
      case ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384:
        return CryptoAlgorithm.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384;
      case ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY:
        return CryptoAlgorithm.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY;
      case ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384:
        return CryptoAlgorithm.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384;
      default:
        throw new IllegalArgumentException(
          "Unrecognized ESDK algorithmSuiteId: " + esdkAlgorithmSuiteId
        );
    }
  }

  public interface Builder {
    Builder impl(AwsCrypto impl);

    AwsCrypto impl();

    TestESDK build();
  }

  static class BuilderImpl implements Builder {

    protected AwsCrypto impl;
    // Default to false
    protected boolean prefer_mkp_over_keyring = false;

    protected BuilderImpl() {}

    public Builder impl(AwsCrypto impl) {
      this.impl = impl;
      return this;
    }

    public AwsCrypto impl() {
      return this.impl;
    }

    public boolean isPrefer_mkp_over_keyring() {
      return this.prefer_mkp_over_keyring;
    }

    public TestESDK build() {
      if (Objects.isNull(this.impl())) {
        throw new IllegalArgumentException(
          "Missing value for required field `impl`"
        );
      }

      if (
        System.getenv("masterkey") != null &&
        System.getenv("masterkey").equals("true")
      ) {
        prefer_mkp_over_keyring = true;
      }

      return new TestESDK(this);
    }
  }
}
