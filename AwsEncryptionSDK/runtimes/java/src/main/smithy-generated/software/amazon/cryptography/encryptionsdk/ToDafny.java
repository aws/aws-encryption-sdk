// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
package software.amazon.cryptography.encryptionsdk;

import Wrappers_Compile.Option;
import dafny.DafnyMap;
import dafny.DafnySequence;
import dafny.TypeDescriptor;
import java.lang.Byte;
import java.lang.Character;
import java.lang.Long;
import java.lang.RuntimeException;
import java.util.Objects;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.AwsEncryptionSdkConfig;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.DecryptInput;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.DecryptOutput;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.EncryptInput;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.EncryptOutput;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.Error;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_AwsEncryptionSdkException;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.IAwsEncryptionSdkClient;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.NetV4__0__0__RetryPolicy;
import software.amazon.cryptography.encryptionsdk.model.AwsEncryptionSdkException;
import software.amazon.cryptography.encryptionsdk.model.CollectionOfErrors;
import software.amazon.cryptography.encryptionsdk.model.NetV4_0_0_RetryPolicy;
import software.amazon.cryptography.encryptionsdk.model.OpaqueError;
import software.amazon.cryptography.materialproviders.internaldafny.types.ESDKAlgorithmSuiteId;
import software.amazon.cryptography.materialproviders.internaldafny.types.ESDKCommitmentPolicy;
import software.amazon.cryptography.materialproviders.internaldafny.types.ICryptographicMaterialsManager;
import software.amazon.cryptography.materialproviders.internaldafny.types.IKeyring;

public class ToDafny {

  public static Error Error(RuntimeException nativeValue) {
    if (nativeValue instanceof AwsEncryptionSdkException) {
      return ToDafny.Error((AwsEncryptionSdkException) nativeValue);
    }
    if (nativeValue instanceof OpaqueError) {
      return ToDafny.Error((OpaqueError) nativeValue);
    }
    if (nativeValue instanceof CollectionOfErrors) {
      return ToDafny.Error((CollectionOfErrors) nativeValue);
    }
    return Error.create_Opaque(
      nativeValue,
      dafny.DafnySequence.asString(
        java.util.Objects.nonNull(nativeValue.getMessage())
          ? nativeValue.getMessage()
          : ""
      )
    );
  }

  public static Error Error(OpaqueError nativeValue) {
    return Error.create_Opaque(
      nativeValue.obj(),
      dafny.DafnySequence.asString(
        java.util.Objects.nonNull(nativeValue.altText())
          ? nativeValue.altText()
          : ""
      )
    );
  }

  public static Error Error(CollectionOfErrors nativeValue) {
    DafnySequence<? extends Error> list =
      software.amazon.smithy.dafny.conversion.ToDafny.Aggregate.GenericToSequence(
        nativeValue.list(),
        ToDafny::Error,
        Error._typeDescriptor()
      );
    DafnySequence<? extends Character> message =
      software.amazon.smithy.dafny.conversion.ToDafny.Simple.CharacterSequence(
        nativeValue.getMessage()
      );
    return Error.create_CollectionOfErrors(list, message);
  }

  public static AwsEncryptionSdkConfig AwsEncryptionSdkConfig(
    software.amazon.cryptography.encryptionsdk.model.AwsEncryptionSdkConfig nativeValue
  ) {
    Option<ESDKCommitmentPolicy> commitmentPolicy;
    commitmentPolicy =
      Objects.nonNull(nativeValue.commitmentPolicy())
        ? Option.create_Some(
        ESDKCommitmentPolicy._typeDescriptor(),
        software.amazon.cryptography.materialproviders.ToDafny.ESDKCommitmentPolicy(
          nativeValue.commitmentPolicy()
        )
      )
        : Option.create_None(ESDKCommitmentPolicy._typeDescriptor());
    Option<Long> maxEncryptedDataKeys;
    maxEncryptedDataKeys =
      Objects.nonNull(nativeValue.maxEncryptedDataKeys())
        ? Option.create_Some(
        TypeDescriptor.LONG,
        (nativeValue.maxEncryptedDataKeys())
      )
        : Option.create_None(TypeDescriptor.LONG);
    Option<NetV4__0__0__RetryPolicy> netV4_0_0_RetryPolicy;
    netV4_0_0_RetryPolicy =
      Objects.nonNull(nativeValue.netV4_0_0_RetryPolicy())
        ? Option.create_Some(
        NetV4__0__0__RetryPolicy._typeDescriptor(),
        ToDafny.NetV4_0_0_RetryPolicy(nativeValue.netV4_0_0_RetryPolicy())
      )
        : Option.create_None(NetV4__0__0__RetryPolicy._typeDescriptor());
    return new AwsEncryptionSdkConfig(
      commitmentPolicy,
      maxEncryptedDataKeys,
      netV4_0_0_RetryPolicy
    );
  }

  public static DecryptInput DecryptInput(
    software.amazon.cryptography.encryptionsdk.model.DecryptInput nativeValue
  ) {
    DafnySequence<? extends Byte> ciphertext;
    ciphertext =
      software.amazon.smithy.dafny.conversion.ToDafny.Simple.ByteSequence(
        nativeValue.ciphertext()
      );
    Option<ICryptographicMaterialsManager> materialsManager;
    materialsManager =
      Objects.nonNull(nativeValue.materialsManager())
        ? Option.create_Some(
        TypeDescriptor.reference(ICryptographicMaterialsManager.class),
        software.amazon.cryptography.materialproviders.ToDafny.CryptographicMaterialsManager(
          nativeValue.materialsManager()
        )
      )
        : Option.create_None(
          TypeDescriptor.reference(ICryptographicMaterialsManager.class)
        );
    Option<IKeyring> keyring;
    keyring =
      Objects.nonNull(nativeValue.keyring())
        ? Option.create_Some(
        TypeDescriptor.reference(IKeyring.class),
        software.amazon.cryptography.materialproviders.ToDafny.Keyring(
          nativeValue.keyring()
        )
      )
        : Option.create_None(TypeDescriptor.reference(IKeyring.class));
    Option<
      DafnyMap<
        ? extends DafnySequence<? extends Byte>,
        ? extends DafnySequence<? extends Byte>
        >
      > encryptionContext;
    encryptionContext =
      (Objects.nonNull(nativeValue.encryptionContext()) &&
        nativeValue.encryptionContext().size() > 0)
        ? Option.create_Some(
        DafnyMap._typeDescriptor(
          DafnySequence._typeDescriptor(TypeDescriptor.BYTE),
          DafnySequence._typeDescriptor(TypeDescriptor.BYTE)
        ),
        software.amazon.cryptography.materialproviders.ToDafny.EncryptionContext(
          nativeValue.encryptionContext()
        )
      )
        : Option.create_None(
          DafnyMap._typeDescriptor(
            DafnySequence._typeDescriptor(TypeDescriptor.BYTE),
            DafnySequence._typeDescriptor(TypeDescriptor.BYTE)
          )
        );
    return new DecryptInput(
      ciphertext,
      materialsManager,
      keyring,
      encryptionContext
    );
  }

  public static DecryptOutput DecryptOutput(
    software.amazon.cryptography.encryptionsdk.model.DecryptOutput nativeValue
  ) {
    DafnySequence<? extends Byte> plaintext;
    plaintext =
      software.amazon.smithy.dafny.conversion.ToDafny.Simple.ByteSequence(
        nativeValue.plaintext()
      );
    DafnyMap<
      ? extends DafnySequence<? extends Byte>,
      ? extends DafnySequence<? extends Byte>
      > encryptionContext;
    encryptionContext =
      software.amazon.cryptography.materialproviders.ToDafny.EncryptionContext(
        nativeValue.encryptionContext()
      );
    ESDKAlgorithmSuiteId algorithmSuiteId;
    algorithmSuiteId =
      software.amazon.cryptography.materialproviders.ToDafny.ESDKAlgorithmSuiteId(
        nativeValue.algorithmSuiteId()
      );
    return new DecryptOutput(plaintext, encryptionContext, algorithmSuiteId);
  }

  public static EncryptInput EncryptInput(
    software.amazon.cryptography.encryptionsdk.model.EncryptInput nativeValue
  ) {
    DafnySequence<? extends Byte> plaintext;
    plaintext =
      software.amazon.smithy.dafny.conversion.ToDafny.Simple.ByteSequence(
        nativeValue.plaintext()
      );
    Option<
      DafnyMap<
        ? extends DafnySequence<? extends Byte>,
        ? extends DafnySequence<? extends Byte>
        >
      > encryptionContext;
    encryptionContext =
      (Objects.nonNull(nativeValue.encryptionContext()) &&
        nativeValue.encryptionContext().size() > 0)
        ? Option.create_Some(
        DafnyMap._typeDescriptor(
          DafnySequence._typeDescriptor(TypeDescriptor.BYTE),
          DafnySequence._typeDescriptor(TypeDescriptor.BYTE)
        ),
        software.amazon.cryptography.materialproviders.ToDafny.EncryptionContext(
          nativeValue.encryptionContext()
        )
      )
        : Option.create_None(
          DafnyMap._typeDescriptor(
            DafnySequence._typeDescriptor(TypeDescriptor.BYTE),
            DafnySequence._typeDescriptor(TypeDescriptor.BYTE)
          )
        );
    Option<ICryptographicMaterialsManager> materialsManager;
    materialsManager =
      Objects.nonNull(nativeValue.materialsManager())
        ? Option.create_Some(
        TypeDescriptor.reference(ICryptographicMaterialsManager.class),
        software.amazon.cryptography.materialproviders.ToDafny.CryptographicMaterialsManager(
          nativeValue.materialsManager()
        )
      )
        : Option.create_None(
          TypeDescriptor.reference(ICryptographicMaterialsManager.class)
        );
    Option<IKeyring> keyring;
    keyring =
      Objects.nonNull(nativeValue.keyring())
        ? Option.create_Some(
        TypeDescriptor.reference(IKeyring.class),
        software.amazon.cryptography.materialproviders.ToDafny.Keyring(
          nativeValue.keyring()
        )
      )
        : Option.create_None(TypeDescriptor.reference(IKeyring.class));
    Option<ESDKAlgorithmSuiteId> algorithmSuiteId;
    algorithmSuiteId =
      Objects.nonNull(nativeValue.algorithmSuiteId())
        ? Option.create_Some(
        ESDKAlgorithmSuiteId._typeDescriptor(),
        software.amazon.cryptography.materialproviders.ToDafny.ESDKAlgorithmSuiteId(
          nativeValue.algorithmSuiteId()
        )
      )
        : Option.create_None(ESDKAlgorithmSuiteId._typeDescriptor());
    Option<Long> frameLength;
    frameLength =
      Objects.nonNull(nativeValue.frameLength())
        ? Option.create_Some(TypeDescriptor.LONG, (nativeValue.frameLength()))
        : Option.create_None(TypeDescriptor.LONG);
    return new EncryptInput(
      plaintext,
      encryptionContext,
      materialsManager,
      keyring,
      algorithmSuiteId,
      frameLength
    );
  }

  public static EncryptOutput EncryptOutput(
    software.amazon.cryptography.encryptionsdk.model.EncryptOutput nativeValue
  ) {
    DafnySequence<? extends Byte> ciphertext;
    ciphertext =
      software.amazon.smithy.dafny.conversion.ToDafny.Simple.ByteSequence(
        nativeValue.ciphertext()
      );
    DafnyMap<
      ? extends DafnySequence<? extends Byte>,
      ? extends DafnySequence<? extends Byte>
      > encryptionContext;
    encryptionContext =
      software.amazon.cryptography.materialproviders.ToDafny.EncryptionContext(
        nativeValue.encryptionContext()
      );
    ESDKAlgorithmSuiteId algorithmSuiteId;
    algorithmSuiteId =
      software.amazon.cryptography.materialproviders.ToDafny.ESDKAlgorithmSuiteId(
        nativeValue.algorithmSuiteId()
      );
    return new EncryptOutput(ciphertext, encryptionContext, algorithmSuiteId);
  }

  public static Error Error(AwsEncryptionSdkException nativeValue) {
    DafnySequence<? extends Character> message;
    message =
      software.amazon.smithy.dafny.conversion.ToDafny.Simple.CharacterSequence(
        nativeValue.message()
      );
    return new Error_AwsEncryptionSdkException(message);
  }

  public static NetV4__0__0__RetryPolicy NetV4_0_0_RetryPolicy(
    NetV4_0_0_RetryPolicy nativeValue
  ) {
    switch (nativeValue) {
      case FORBID_RETRY:
      {
        return NetV4__0__0__RetryPolicy.create_FORBID__RETRY();
      }
      case ALLOW_RETRY:
      {
        return NetV4__0__0__RetryPolicy.create_ALLOW__RETRY();
      }
      default:
      {
        throw new RuntimeException(
          "Cannot convert " +
            nativeValue +
            " to software.amazon.cryptography.encryptionsdk.internaldafny.types.NetV4__0__0__RetryPolicy."
        );
      }
    }
  }

  public static IAwsEncryptionSdkClient AwsEncryptionSdk(ESDK nativeValue) {
    return nativeValue.impl();
  }
}
