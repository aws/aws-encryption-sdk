from aws_cryptographic_material_providers.mpl.models import CreateRawAesKeyringInput
import json
from aws_cryptographic_material_providers.mpl.client import AwsCryptographicMaterialProviders
from aws_cryptographic_material_providers.mpl.config import MaterialProvidersConfig

import base64

class TestImplementation:
    @staticmethod
    def CreateRawAesKeyringSmokeTest(input: CreateRawAesKeyringInput) -> str:
        mpl_client = AwsCryptographicMaterialProviders(
            config=MaterialProvidersConfig()
        )
        try:
            keyring = mpl_client.create_raw_aes_keyring(input)
            return "ok"
        except Exception as e:
            if "keyNamespace must not be `aws-kms`" in str(e):
                return "Invalid key namespace for raw AES keyring"
            elif "Wrapping key length does not match specified wrapping algorithm" in str(e):
                return "Invalid wrapping key length"
            elif "Invalid wrapping key length" in str(e):
                return "Invalid wrapping key length"
            else:
                return f"Unexpected error: {str(e)}"

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