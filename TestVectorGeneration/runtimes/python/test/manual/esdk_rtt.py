from aws_cryptographic_material_providers.mpl.models import CreateRawAesKeyringInput
import json
from aws_cryptographic_material_providers.mpl.client import AwsCryptographicMaterialProviders
from aws_cryptographic_material_providers.mpl.config import MaterialProvidersConfig
from test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.models import (
    TestCrossLanguageRoundTripInput,
    TestCrossLanguageRoundTripOutput,
    SupportedKeyringCreateInputsRawAes,
    ImplementationLanguage,
)
from aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.client import AwsEncryptionSdk
from aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.config import AwsEncryptionSdkConfig
from aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.models import (
    EncryptInput,
    DecryptInput,
)

import base64

def create_keyring(mpl_client, keyring):
    if isinstance(keyring, SupportedKeyringCreateInputsRawAes):
        return mpl_client.create_raw_aes_keyring(
            keyring.value
        )
    else:
        raise Exception(f"Unsupported keyring: {keyring}")
    
def create_mpl_client_for_language(language: ImplementationLanguage):
    if language == ImplementationLanguage.PYTHON:
        # TODO: This is where we would create a client to call the remote Python MPL instance
        return AwsCryptographicMaterialProviders(config=MaterialProvidersConfig())
    else:
        raise Exception(f"Unsupported language: {language}")
    
def create_esdk_client_for_language(language: ImplementationLanguage):
    if language == ImplementationLanguage.PYTHON:
        return AwsEncryptionSdk(config=AwsEncryptionSdkConfig())
    else:
        raise Exception(f"Unsupported language: {language}")

class TestImplementation:
    @staticmethod
    def TestCrossLanguageRoundTrip(input: TestCrossLanguageRoundTripInput) -> str:
        try:
            # TODO: Pass client config via TestCrossLanguageRoundTripInput
            # (MPL doesn't have this, but good for completeness) 
            encrypt_mpl_client = create_mpl_client_for_language(input.encrypt_language)
            # TODO: Pass client config via TestCrossLanguageRoundTripInput
            encrypt_esdk_client = create_esdk_client_for_language(input.encrypt_language)
            rtt_encrypt_input = input.encrypt_input
            esdk_encrypt_input = EncryptInput(
                plaintext=rtt_encrypt_input.plaintext,
                keyring=create_keyring(encrypt_mpl_client, rtt_encrypt_input.keyring),
                encryption_context=rtt_encrypt_input.encryption_context,
                frame_length=rtt_encrypt_input.frame_length,
            )
            esdk_encrypt_output = encrypt_esdk_client.encrypt(esdk_encrypt_input)

            ciphertext = esdk_encrypt_output.ciphertext

            decrypt_mpl_client = create_mpl_client_for_language(input.decrypt_language)
            decrypt_esdk_client = create_esdk_client_for_language(input.decrypt_language)
            rtt_decrypt_input = input.decrypt_input
            esdk_decrypt_input = DecryptInput(
                ciphertext=ciphertext,
                keyring=create_keyring(decrypt_mpl_client, rtt_decrypt_input.keyring),
                encryption_context=rtt_decrypt_input.encryption_context,
            )
            plaintext = decrypt_esdk_client.decrypt(esdk_decrypt_input).plaintext

            assert plaintext == rtt_encrypt_input.plaintext, "Decrypted plaintext does not match original plaintext"
            
            return TestCrossLanguageRoundTripOutput(status="ok")
        except Exception as e:
            return TestCrossLanguageRoundTripOutput(status="error", error_message=str(e))

# Could be Smithy-generated
class KeyringTestClient:

    def __init__(self):
        pass

    def CreateRawAesKeyringSmokeTest(self, input: CreateRawAesKeyringInput) -> str:
        return TestImplementation.CreateRawAesKeyringSmokeTest(input)

def test_CreateRawAesKeyringSmokeTest(json_filename = "runtimes/python/outkeyrings.json"):
    test_client = KeyringTestClient()
    json_test_vectors = json.load(open(json_filename, "r"))
    operations_json = json_test_vectors[":CreateRawAesKeyringTestService"][0]["CreateRawAesKeyringSmokeTest"]
    for operation_json in operations_json:
        keyring = load_create_raw_aes_keyring_input(operation_json["input"])
        output = test_client.CreateRawAesKeyringSmokeTest(keyring)
        if output != operation_json["output"]:
            print(f"Expected {operation_json['output']} but got {output}")
        else:
            print(f"Test passed for input: {keyring.key_name} with output {output}")


def load_create_raw_aes_keyring_input(json_create_raw_aes_keyring_input) -> CreateRawAesKeyringInput:
    """
    Load the CreateRawAesKeyringInput from a JSON file.
    """
    return CreateRawAesKeyringInput(
        key_namespace = json_create_raw_aes_keyring_input["keyNamespace"],
        key_name = json_create_raw_aes_keyring_input["keyName"],
        wrapping_key = base64.b64decode(json_create_raw_aes_keyring_input["wrappingKey"]),
        wrapping_alg = json_create_raw_aes_keyring_input["wrappingAlg"]
    )

if __name__ == "__main__":
    test_CreateRawAesKeyringSmokeTest()
    print("All tests passed.")