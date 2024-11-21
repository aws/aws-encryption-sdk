// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
package software.amazon.cryptography.encryptionsdk;

import java.lang.IllegalArgumentException;
import java.lang.RuntimeException;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.Error;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_AwsEncryptionSdkException;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_CollectionOfErrors;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_Opaque;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_OpaqueWithText;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.IAwsEncryptionSdkClient;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.NetV4__0__0__RetryPolicy;
import software.amazon.cryptography.encryptionsdk.model.AwsEncryptionSdkConfig;
import software.amazon.cryptography.encryptionsdk.model.AwsEncryptionSdkException;
import software.amazon.cryptography.encryptionsdk.model.CollectionOfErrors;
import software.amazon.cryptography.encryptionsdk.model.DecryptInput;
import software.amazon.cryptography.encryptionsdk.model.DecryptOutput;
import software.amazon.cryptography.encryptionsdk.model.EncryptInput;
import software.amazon.cryptography.encryptionsdk.model.EncryptOutput;
import software.amazon.cryptography.encryptionsdk.model.NetV4_0_0_RetryPolicy;
import software.amazon.cryptography.encryptionsdk.model.OpaqueError;
import software.amazon.cryptography.encryptionsdk.model.OpaqueWithTextError;

public class ToNative {

  public static OpaqueError Error(Error_Opaque dafnyValue) {
    OpaqueError.Builder nativeBuilder = OpaqueError.builder();
    nativeBuilder.obj(dafnyValue.dtor_obj());
    return nativeBuilder.build();
  }

  public static OpaqueWithTextError Error(Error_OpaqueWithText dafnyValue) {
    OpaqueWithTextError.Builder nativeBuilder = OpaqueWithTextError.builder();
    nativeBuilder.obj(dafnyValue.dtor_obj());
    nativeBuilder.objMessage(
      software.amazon.smithy.dafny.conversion.ToNative.Simple.String(
        dafnyValue.dtor_objMessage()
      )
    );
    return nativeBuilder.build();
  }

  public static CollectionOfErrors Error(Error_CollectionOfErrors dafnyValue) {
    CollectionOfErrors.Builder nativeBuilder = CollectionOfErrors.builder();
    nativeBuilder.list(
      software.amazon.smithy.dafny.conversion.ToNative.Aggregate.GenericToList(
        dafnyValue.dtor_list(),
        ToNative::Error
      )
    );
    nativeBuilder.message(
      software.amazon.smithy.dafny.conversion.ToNative.Simple.String(
        dafnyValue.dtor_message()
      )
    );
    return nativeBuilder.build();
  }

  public static AwsEncryptionSdkException Error(
    Error_AwsEncryptionSdkException dafnyValue
  ) {
    AwsEncryptionSdkException.Builder nativeBuilder =
      AwsEncryptionSdkException.builder();
    nativeBuilder.message(
      software.amazon.smithy.dafny.conversion.ToNative.Simple.String(
        dafnyValue.dtor_message()
      )
    );
    return nativeBuilder.build();
  }

  public static RuntimeException Error(Error dafnyValue) {
    if (dafnyValue.is_AwsEncryptionSdkException()) {
      return ToNative.Error((Error_AwsEncryptionSdkException) dafnyValue);
    }
    if (dafnyValue.is_Opaque()) {
      return ToNative.Error((Error_Opaque) dafnyValue);
    }
    if (dafnyValue.is_OpaqueWithText()) {
      return ToNative.Error((Error_OpaqueWithText) dafnyValue);
    }
    if (dafnyValue.is_CollectionOfErrors()) {
      return ToNative.Error((Error_CollectionOfErrors) dafnyValue);
    }
    if (dafnyValue.is_AwsCryptographyPrimitives()) {
      return software.amazon.cryptography.primitives.ToNative.Error(
        dafnyValue.dtor_AwsCryptographyPrimitives()
      );
    }
    if (dafnyValue.is_AwsCryptographyMaterialProviders()) {
      return software.amazon.cryptography.materialproviders.ToNative.Error(
        dafnyValue.dtor_AwsCryptographyMaterialProviders()
      );
    }
    OpaqueError.Builder nativeBuilder = OpaqueError.builder();
    nativeBuilder.obj(dafnyValue);
    return nativeBuilder.build();
  }

  public static AwsEncryptionSdkConfig AwsEncryptionSdkConfig(
    software.amazon.cryptography.encryptionsdk.internaldafny.types.AwsEncryptionSdkConfig dafnyValue
  ) {
    AwsEncryptionSdkConfig.Builder nativeBuilder =
      AwsEncryptionSdkConfig.builder();
    if (dafnyValue.dtor_commitmentPolicy().is_Some()) {
      nativeBuilder.commitmentPolicy(
        software.amazon.cryptography.materialproviders.ToNative.ESDKCommitmentPolicy(
          dafnyValue.dtor_commitmentPolicy().dtor_value()
        )
      );
    }
    if (dafnyValue.dtor_maxEncryptedDataKeys().is_Some()) {
      nativeBuilder.maxEncryptedDataKeys(
        (dafnyValue.dtor_maxEncryptedDataKeys().dtor_value())
      );
    }
    if (dafnyValue.dtor_netV4__0__0__RetryPolicy().is_Some()) {
      nativeBuilder.netV4_0_0_RetryPolicy(
        ToNative.NetV4_0_0_RetryPolicy(
          dafnyValue.dtor_netV4__0__0__RetryPolicy().dtor_value()
        )
      );
    }
    return nativeBuilder.build();
  }

  public static DecryptInput DecryptInput(
    software.amazon.cryptography.encryptionsdk.internaldafny.types.DecryptInput dafnyValue
  ) {
    DecryptInput.Builder nativeBuilder = DecryptInput.builder();
    nativeBuilder.ciphertext(
      software.amazon.smithy.dafny.conversion.ToNative.Simple.ByteBuffer(
        dafnyValue.dtor_ciphertext()
      )
    );
    if (dafnyValue.dtor_materialsManager().is_Some()) {
      nativeBuilder.materialsManager(
        software.amazon.cryptography.materialproviders.ToNative.CryptographicMaterialsManager(
          dafnyValue.dtor_materialsManager().dtor_value()
        )
      );
    }
    if (dafnyValue.dtor_keyring().is_Some()) {
      nativeBuilder.keyring(
        software.amazon.cryptography.materialproviders.ToNative.Keyring(
          dafnyValue.dtor_keyring().dtor_value()
        )
      );
    }
    if (dafnyValue.dtor_encryptionContext().is_Some()) {
      nativeBuilder.encryptionContext(
        software.amazon.cryptography.materialproviders.ToNative.EncryptionContext(
          dafnyValue.dtor_encryptionContext().dtor_value()
        )
      );
    }
    return nativeBuilder.build();
  }

  public static DecryptOutput DecryptOutput(
    software.amazon.cryptography.encryptionsdk.internaldafny.types.DecryptOutput dafnyValue
  ) {
    DecryptOutput.Builder nativeBuilder = DecryptOutput.builder();
    nativeBuilder.plaintext(
      software.amazon.smithy.dafny.conversion.ToNative.Simple.ByteBuffer(
        dafnyValue.dtor_plaintext()
      )
    );
    nativeBuilder.encryptionContext(
      software.amazon.cryptography.materialproviders.ToNative.EncryptionContext(
        dafnyValue.dtor_encryptionContext()
      )
    );
    nativeBuilder.algorithmSuiteId(
      software.amazon.cryptography.materialproviders.ToNative.ESDKAlgorithmSuiteId(
        dafnyValue.dtor_algorithmSuiteId()
      )
    );
    return nativeBuilder.build();
  }

  public static EncryptInput EncryptInput(
    software.amazon.cryptography.encryptionsdk.internaldafny.types.EncryptInput dafnyValue
  ) {
    EncryptInput.Builder nativeBuilder = EncryptInput.builder();
    nativeBuilder.plaintext(
      software.amazon.smithy.dafny.conversion.ToNative.Simple.ByteBuffer(
        dafnyValue.dtor_plaintext()
      )
    );
    if (dafnyValue.dtor_encryptionContext().is_Some()) {
      nativeBuilder.encryptionContext(
        software.amazon.cryptography.materialproviders.ToNative.EncryptionContext(
          dafnyValue.dtor_encryptionContext().dtor_value()
        )
      );
    }
    if (dafnyValue.dtor_materialsManager().is_Some()) {
      nativeBuilder.materialsManager(
        software.amazon.cryptography.materialproviders.ToNative.CryptographicMaterialsManager(
          dafnyValue.dtor_materialsManager().dtor_value()
        )
      );
    }
    if (dafnyValue.dtor_keyring().is_Some()) {
      nativeBuilder.keyring(
        software.amazon.cryptography.materialproviders.ToNative.Keyring(
          dafnyValue.dtor_keyring().dtor_value()
        )
      );
    }
    if (dafnyValue.dtor_algorithmSuiteId().is_Some()) {
      nativeBuilder.algorithmSuiteId(
        software.amazon.cryptography.materialproviders.ToNative.ESDKAlgorithmSuiteId(
          dafnyValue.dtor_algorithmSuiteId().dtor_value()
        )
      );
    }
    if (dafnyValue.dtor_frameLength().is_Some()) {
      nativeBuilder.frameLength((dafnyValue.dtor_frameLength().dtor_value()));
    }
    return nativeBuilder.build();
  }

  public static EncryptOutput EncryptOutput(
    software.amazon.cryptography.encryptionsdk.internaldafny.types.EncryptOutput dafnyValue
  ) {
    EncryptOutput.Builder nativeBuilder = EncryptOutput.builder();
    nativeBuilder.ciphertext(
      software.amazon.smithy.dafny.conversion.ToNative.Simple.ByteBuffer(
        dafnyValue.dtor_ciphertext()
      )
    );
    nativeBuilder.encryptionContext(
      software.amazon.cryptography.materialproviders.ToNative.EncryptionContext(
        dafnyValue.dtor_encryptionContext()
      )
    );
    nativeBuilder.algorithmSuiteId(
      software.amazon.cryptography.materialproviders.ToNative.ESDKAlgorithmSuiteId(
        dafnyValue.dtor_algorithmSuiteId()
      )
    );
    return nativeBuilder.build();
  }

  public static NetV4_0_0_RetryPolicy NetV4_0_0_RetryPolicy(
    NetV4__0__0__RetryPolicy dafnyValue
  ) {
    if (dafnyValue.is_FORBID__RETRY()) {
      return NetV4_0_0_RetryPolicy.FORBID_RETRY;
    }
    if (dafnyValue.is_ALLOW__RETRY()) {
      return NetV4_0_0_RetryPolicy.ALLOW_RETRY;
    }
    throw new IllegalArgumentException(
      "No entry of software.amazon.cryptography.encryptionsdk.model.NetV4_0_0_RetryPolicy matches the input : " +
      dafnyValue
    );
  }

  public static ESDK AwsEncryptionSdk(IAwsEncryptionSdkClient dafnyValue) {
    return new ESDK(dafnyValue);
  }
}
