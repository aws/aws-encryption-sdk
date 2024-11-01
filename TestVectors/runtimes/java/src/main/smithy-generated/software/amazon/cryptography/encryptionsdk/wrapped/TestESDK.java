// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
package software.amazon.cryptography.encryptionsdk.wrapped;

import Wrappers_Compile.Result;
import java.lang.IllegalArgumentException;
import java.lang.RuntimeException;
import java.util.Objects;
import software.amazon.cryptography.encryptionsdk.ESDK;
import software.amazon.cryptography.encryptionsdk.ToDafny;
import software.amazon.cryptography.encryptionsdk.ToNative;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.DecryptInput;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.DecryptOutput;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.EncryptInput;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.EncryptOutput;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.Error;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.IAwsEncryptionSdkClient;

public class TestESDK implements IAwsEncryptionSdkClient {

  private final ESDK _impl;

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
      software.amazon.cryptography.encryptionsdk.model.DecryptOutput nativeOutput =
        this._impl.Decrypt(nativeInput);
      DecryptOutput dafnyOutput = ToDafny.DecryptOutput(nativeOutput);
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
      software.amazon.cryptography.encryptionsdk.model.EncryptOutput nativeOutput =
        this._impl.Encrypt(nativeInput);
      EncryptOutput dafnyOutput = ToDafny.EncryptOutput(nativeOutput);
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
    Builder impl(ESDK impl);

    ESDK impl();

    TestESDK build();
  }

  static class BuilderImpl implements Builder {

    protected ESDK impl;

    protected BuilderImpl() {}

    public Builder impl(ESDK impl) {
      this.impl = impl;
      return this;
    }

    public ESDK impl() {
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
