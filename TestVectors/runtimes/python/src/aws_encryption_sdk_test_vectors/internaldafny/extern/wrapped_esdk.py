import aws_encryption_sdk_test_vectors.internaldafny.generated.WrappedESDK as WrappedESDK
import smithy_dafny_standard_library.internaldafny.generated.Wrappers as Wrappers
import aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.client
import aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.config
import aws_encryption_sdk.internaldafny.generated.EncryptionSdk as EncryptionSdk
import aws_encryption_sdk_test_vectors.smithygenerated.aws_cryptography_encryptionsdk.shim as shim

class default__(WrappedESDK.default__):

  @staticmethod
  def WrappedESDK(config):  
    smithy_client = aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.client.AwsEncryptionSdk(
      dafny_client=EncryptionSdk.default__.ESDK(config).value
    )
    wrapped_client = shim.ESDKShim(smithy_client)
    return Wrappers.Result_Success(wrapped_client)

WrappedESDK.default__ = default__