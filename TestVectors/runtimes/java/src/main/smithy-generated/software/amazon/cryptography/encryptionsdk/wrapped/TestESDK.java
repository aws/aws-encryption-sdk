// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
package software.amazon.cryptography.encryptionsdk.wrapped;

import Wrappers_Compile.Result;
import com.amazonaws.encryptionsdk.CryptoResult;
import dafny.DafnyMap;
import dafny.DafnySequence;
import java.lang.IllegalArgumentException;
import java.lang.RuntimeException;
import java.nio.ByteBuffer;
import java.util.Map;
import java.util.Objects;
import com.amazonaws.encryptionsdk.AwsCrypto;
import software.amazon.cryptography.encryptionsdk.ESDK;
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

public class TestESDK implements IAwsEncryptionSdkClient {

  private final AwsCrypto _impl;

  protected TestESDK(BuilderImpl builder) {
    this._impl = builder.impl();
  }

  public static Builder builder() {
    return new BuilderImpl();
  }

  public Result<DecryptOutput, Error> Decrypt(DecryptInput dafnyInput) {
    try {
      software.amazon.cryptography.encryptionsdk.model.DecryptInput nativeInput =
        ToNative.DecryptInput(dafnyInput);
      final CryptoResult<byte[], ?> decryptResult;

      if (Objects.isNull(nativeInput.materialsManager())) {
        // Call decrypt with keyring
        if (Objects.isNull(nativeInput.encryptionContext())) {
          decryptResult = this._impl.decryptData(nativeInput.materialsManager(), nativeInput.ciphertext().array());
        } else {
          decryptResult = this._impl.decryptData(nativeInput.materialsManager(), nativeInput.ciphertext().array(), nativeInput.encryptionContext());
        }
      } else {
        if (Objects.isNull(nativeInput.encryptionContext())) {
          decryptResult = this._impl.decryptData(nativeInput.keyring(), nativeInput.ciphertext().array());
        } else {
          decryptResult = this._impl.decryptData(nativeInput.keyring(), nativeInput.ciphertext().array(), nativeInput.encryptionContext());
        }
      }
      DafnySequence<? extends Byte> plaintext = Simple.ByteSequence(decryptResult.getResult());
      DafnyMap<? extends DafnySequence<? extends Byte>, ? extends DafnySequence<? extends Byte>> encryptionContext =
        software.amazon.cryptography.materialproviders.ToDafny.EncryptionContext(decryptResult.getEncryptionContext());
      ESDKAlgorithmSuiteId algorithmSuiteId = software.amazon.cryptography.materialproviders.ToDafny.ESDKAlgorithmSuiteId(
        decryptResult.getCryptoAlgorithm().getAlgorithmSuiteId().ESDK()
      );
      DecryptOutput dafnyOutput = new DecryptOutput(plaintext, encryptionContext, algorithmSuiteId);

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

      if (Objects.isNull(nativeInput.materialsManager())) {
        // Call decrypt with keyring
        if (Objects.isNull(nativeInput.encryptionContext())) {
          encryptResult = this._impl.encryptData(nativeInput.keyring(), nativeInput.plaintext().array());
        } else {
          encryptResult = this._impl.encryptData(nativeInput.keyring(), nativeInput.plaintext().array(), nativeInput.encryptionContext());
        }
      } else {
        if (Objects.isNull(nativeInput.encryptionContext())) {
          encryptResult = this._impl.encryptData(nativeInput.materialsManager(), nativeInput.plaintext().array());
        } else {
          encryptResult = this._impl.encryptData(nativeInput.materialsManager(), nativeInput.plaintext().array(), nativeInput.encryptionContext());
        }
      }
      dafny.DafnySequence<? extends Byte> ciphertext = Simple.ByteSequence(encryptResult.getResult());
      DafnyMap<? extends DafnySequence<? extends Byte>, ? extends DafnySequence<? extends Byte>> encryptionContext =
        software.amazon.cryptography.materialproviders.ToDafny.EncryptionContext(encryptResult.getEncryptionContext());
      ESDKAlgorithmSuiteId algorithmSuiteId = software.amazon.cryptography.materialproviders.ToDafny.ESDKAlgorithmSuiteId(
        encryptResult.getCryptoAlgorithm().getAlgorithmSuiteId().ESDK()
      );

      EncryptOutput dafnyOutput = new EncryptOutput(ciphertext, encryptionContext, algorithmSuiteId);
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

  public interface Builder {
    Builder impl(AwsCrypto impl);

    AwsCrypto impl();

    TestESDK build();
  }

  static class BuilderImpl implements Builder {

    protected AwsCrypto impl;

    protected BuilderImpl() {}

    public Builder impl(AwsCrypto impl) {
      this.impl = impl;
      return this;
    }

    public AwsCrypto impl() {
      return this.impl;
    }

    public TestESDK build() {
      if (Objects.isNull(this.impl())) {
        throw new IllegalArgumentException(
          "Missing value for required field `impl`"
        );
      }
      return new TestESDK(this);
    }
  }
}
