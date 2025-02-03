package software.amazon.cryptography.encryptionsdk.internaldafny.wrapped;

import Wrappers_Compile.Result;
import com.amazonaws.encryptionsdk.AwsCrypto;
import com.amazonaws.encryptionsdk.CommitmentPolicy;
import com.amazonaws.encryptionsdk.CryptoAlgorithm;
import software.amazon.cryptography.encryptionsdk.ToNative;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.AwsEncryptionSdkConfig;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.Error;
import software.amazon.cryptography.encryptionsdk.internaldafny.types.IAwsEncryptionSdkClient;
import software.amazon.cryptography.encryptionsdk.model.NetV4_0_0_RetryPolicy;
import software.amazon.cryptography.encryptionsdk.wrapped.TestESDK;
import software.amazon.cryptography.materialproviders.model.ESDKCommitmentPolicy;

public class __default extends _ExternBase___default {

  public static Result<IAwsEncryptionSdkClient, Error> WrappedESDK(
    AwsEncryptionSdkConfig config
  ) {
    software.amazon.cryptography.encryptionsdk.model.AwsEncryptionSdkConfig wrappedConfig =
      ToNative.AwsEncryptionSdkConfig(config);
    if (
      wrappedConfig.netV4_0_0_RetryPolicy() == NetV4_0_0_RetryPolicy.ALLOW_RETRY
    ) {
      throw new IllegalArgumentException(
        "Native AWS Encryption SDK for Java does not support NetV4_0_0_RetryPolicy.ALLOW_RETRY"
      );
    }
    CommitmentPolicy commitmentPolicy = _esdkDafnyCommitmentPolicyToNative(
      wrappedConfig.commitmentPolicy()
    );

    int maxEncryptedDataKeys = wrappedConfig.maxEncryptedDataKeys() == 0
      ? 1
      : (int) wrappedConfig.maxEncryptedDataKeys();
    final AwsCrypto awsCrypto;

    if (wrappedConfig.maxEncryptedDataKeys() == 0) {
      awsCrypto =
        AwsCrypto.builder().withCommitmentPolicy(commitmentPolicy).build();
    } else {
      awsCrypto =
        AwsCrypto
          .builder()
          .withCommitmentPolicy(commitmentPolicy)
          .withMaxEncryptedDataKeys(maxEncryptedDataKeys)
          .build();
    }
    TestESDK wrappedEsdk = TestESDK.builder().impl(awsCrypto).build();
    return software.amazon.cryptography.encryptionsdk.internaldafny._ExternBase___default.CreateSuccessOfClient(
      wrappedEsdk
    );
  }

  private static CommitmentPolicy _esdkDafnyCommitmentPolicyToNative(
    ESDKCommitmentPolicy esdkCommitmentPolicy
  ) {
    switch (esdkCommitmentPolicy) {
      case FORBID_ENCRYPT_ALLOW_DECRYPT:
        return CommitmentPolicy.ForbidEncryptAllowDecrypt;
      case REQUIRE_ENCRYPT_ALLOW_DECRYPT:
        return CommitmentPolicy.RequireEncryptAllowDecrypt;
      case REQUIRE_ENCRYPT_REQUIRE_DECRYPT:
        return CommitmentPolicy.RequireEncryptRequireDecrypt;
      default:
        throw new IllegalArgumentException(
          "Unsupported CommitmentPolicy: " + esdkCommitmentPolicy
        );
    }
  }
}
