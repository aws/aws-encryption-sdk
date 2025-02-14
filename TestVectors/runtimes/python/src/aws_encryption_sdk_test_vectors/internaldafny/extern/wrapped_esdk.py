import aws_encryption_sdk.streaming_client
import aws_encryption_sdk_test_vectors.internaldafny.generated.WrappedESDK as WrappedESDK
import smithy_dafny_standard_library.internaldafny.generated.Wrappers as Wrappers
import aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.client
import aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.config
import aws_encryption_sdk_dafny.internaldafny.generated.ESDK as ESDK
import aws_encryption_sdk_test_vectors.smithygenerated.aws_cryptography_encryptionsdk.shim as shim
from aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.config import (
    dafny_config_to_smithy_config,
)
import aws_encryption_sdk
from aws_encryption_sdk.identifiers import CommitmentPolicy
from aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.models import (
    EncryptInput,
    EncryptOutput,
    DecryptOutput,
    DecryptInput,
    NetV4_0_0_RetryPolicy,
)
from aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.dafny_to_smithy import (
    aws_cryptography_encryptionsdk_EncryptInput as dafny_to_smithy_EncryptInput,
    aws_cryptography_encryptionsdk_DecryptInput as dafny_to_smithy_DecryptInput,
)
from aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.smithy_to_dafny import (
    aws_cryptography_encryptionsdk_DecryptOutput as smithy_to_dafny_DecryptOutput,
    aws_cryptography_encryptionsdk_EncryptOutput as smithy_to_dafny_EncryptOutput,
)
from aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.errors import (
    _smithy_error_to_dafny_error,
)
from aws_encryption_sdk.materials_managers.mpl.cmm import CryptoMaterialsManagerFromMPL
from aws_encryption_sdk.materials_managers.mpl.materials import _mpl_algorithm_id_to_native_algorithm_id


def _esdk_dafny_commitment_policy_to_native(dafny_commitment_policy):
    if dafny_commitment_policy == "FORBID_ENCRYPT_ALLOW_DECRYPT":
        return CommitmentPolicy.FORBID_ENCRYPT_ALLOW_DECRYPT
    elif dafny_commitment_policy == "REQUIRE_ENCRYPT_ALLOW_DECRYPT":
        return CommitmentPolicy.REQUIRE_ENCRYPT_ALLOW_DECRYPT
    elif dafny_commitment_policy == "REQUIRE_ENCRYPT_REQUIRE_DECRYPT":
        return CommitmentPolicy.REQUIRE_ENCRYPT_REQUIRE_DECRYPT
    else:
        raise ValueError(f"Unsupported CommitmentPolicy: {dafny_commitment_policy}")


class DafnyESDKToNativeESDKShim:

    def __init__(self, native_esdk):
        self.native_esdk = native_esdk

    def Encrypt(self, dafny_encrypt_input):
        try:
          native_encrypt_input = dafny_to_smithy_EncryptInput(dafny_encrypt_input)

          if native_encrypt_input.algorithm_suite_id is None:
            if native_encrypt_input.materials_manager is not None:
                native_esdk_ciphertext, native_esdk_header = self.native_esdk.encrypt(
                    source=native_encrypt_input.plaintext,
                    materials_manager=native_encrypt_input.materials_manager,
                    encryption_context=native_encrypt_input.encryption_context,
                )
            else:
                native_esdk_ciphertext, native_esdk_header = self.native_esdk.encrypt(
                    source=native_encrypt_input.plaintext,
                    materials_manager=native_encrypt_input.keyring,
                    encryption_context=native_encrypt_input.encryption_context,
                )
          else: 
              if native_encrypt_input.materials_manager is not None:
                native_esdk_ciphertext, native_esdk_header = self.native_esdk.encrypt(
                    source=native_encrypt_input.plaintext,
                    materials_manager=native_encrypt_input.materials_manager,
                    encryption_context=native_encrypt_input.encryption_context,
                    algorithm = _mpl_algorithm_id_to_native_algorithm_id(native_encrypt_input.algorithm_suite_id)
                )
              else:
                native_esdk_ciphertext, native_esdk_header = self.native_esdk.encrypt(
                    source=native_encrypt_input.plaintext,
                    materials_manager=native_encrypt_input.keyring,
                    encryption_context=native_encrypt_input.encryption_context,
                    algorithm = _mpl_algorithm_id_to_native_algorithm_id(native_encrypt_input.algorithm_suite_id)
                )

          dafny_esdk_native_encrypt_output = EncryptOutput(
              ciphertext=native_esdk_ciphertext,
              encryption_context=native_esdk_header.encryption_context,
              algorithm_suite_id=CryptoMaterialsManagerFromMPL._native_algorithm_id_to_mpl_algorithm_id(
                  native_esdk_header.algorithm.algorithm_id
              ).value,
          )

          dafny_esdk_dafny_encrypt_output = smithy_to_dafny_EncryptOutput(
              dafny_esdk_native_encrypt_output
          )

          return Wrappers.Result_Success(dafny_esdk_dafny_encrypt_output)
        except Exception as e:
          return Wrappers.Result_Failure(_smithy_error_to_dafny_error(e))

    def Decrypt(self, dafny_decrypt_input):
        
        try:

          native_decrypt_input = dafny_to_smithy_DecryptInput(dafny_decrypt_input)

          if native_decrypt_input.materials_manager is not None:
              native_esdk_plaintext, native_esdk_header = self.native_esdk.encrypt(
                  source=native_decrypt_input.ciphertext,
                  materials_manager=native_decrypt_input.materials_manager,
                  encryption_context=native_decrypt_input.encryption_context,
              )
          else:
              native_esdk_plaintext, native_esdk_header = self.native_esdk.encrypt(
                  source=native_decrypt_input.ciphertext,
                  materials_manager=native_decrypt_input.keyring,
                  encryption_context=native_decrypt_input.encryption_context,
              )

          dafny_esdk_native_decrypt_output = DecryptOutput(
              plaintext=native_esdk_plaintext,
              encryption_context=native_esdk_header.encryption_context,
              algorithm_suite_id=CryptoMaterialsManagerFromMPL._native_algorithm_id_to_mpl_algorithm_id(
                  native_esdk_header.algorithm.algorithm_id
              ).value,
          )

          dafny_esdk_dafny_decrypt_output = smithy_to_dafny_DecryptOutput(
              dafny_esdk_native_decrypt_output
          )

          return Wrappers.Result_Success(dafny_esdk_dafny_decrypt_output)
        except Exception as e:
            return Wrappers.Result_Failure(_smithy_error_to_dafny_error(e))


class default__(WrappedESDK.default__):

    # Dafny-generated ESDK. Not launched right now.
    # @staticmethod
    # def WrappedESDK(config):
    #   smithy_client = aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.client.AwsEncryptionSdk(
    #     dafny_client=ESDK.default__.ESDK(config).value
    #   )
    #   wrapped_client = shim.ESDKShim(smithy_client)
    #   return Wrappers.Result_Success(wrapped_client)

    @staticmethod
    # Wrapper for the native ESDK-Python.
    def WrappedESDK(dafny_config):
        native_config = dafny_config_to_smithy_config(dafny_config)

        # TODO deny net 4.0.0 allow retry

        if native_config.net_v4_0_0_retry_policy == NetV4_0_0_RetryPolicy.ALLOW_RETRY:
            raise ValueError("net 4.0.0 retry policy is not supported")

        commitment_policy = _esdk_dafny_commitment_policy_to_native(
            native_config.commitment_policy
        )

        max_edks = (
            1
            if native_config.max_encrypted_data_keys == 0
            else native_config.max_encrypted_data_keys
        )

        native_esdk = aws_encryption_sdk.EncryptionSDKClient(
            commitment_policy=commitment_policy,
            max_encrypted_data_keys=max_edks,
        )

        dafny_wrapped_esdk = DafnyESDKToNativeESDKShim(native_esdk)

        return Wrappers.Result_Success(dafny_wrapped_esdk)


WrappedESDK.default__ = default__
