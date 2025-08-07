# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
# Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

import aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.dafny_to_smithy
from test_vectors_v2.internaldafny.generated.AwsCryptographyEncryptionSdkTestRoundtripTypes import (
    ImplementationLanguage_DOTNET,
    ImplementationLanguage_GO,
    ImplementationLanguage_JAVA,
    ImplementationLanguage_JAVASCRIPT,
    ImplementationLanguage_PYTHON,
    ImplementationLanguage_RUST,
    SupportedKeyringCreateInputs_RawAes,
)
import test_vectors_v2.internaldafny.generated.module_
import test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.dafny_to_smithy
import test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.models


def aws_cryptography_encryptionsdk_test_roundtrip_ImplementationLanguage(dafny_input):
    if isinstance(dafny_input, ImplementationLanguage_JAVA):
        return "JAVA"

    elif isinstance(dafny_input, ImplementationLanguage_PYTHON):
        return "PYTHON"

    elif isinstance(dafny_input, ImplementationLanguage_DOTNET):
        return "DOTNET"

    elif isinstance(dafny_input, ImplementationLanguage_GO):
        return "GO"

    elif isinstance(dafny_input, ImplementationLanguage_JAVASCRIPT):
        return "JAVASCRIPT"

    elif isinstance(dafny_input, ImplementationLanguage_RUST):
        return "RUST"

    else:
        raise ValueError(f"No recognized enum value in enum type: {dafny_input=}")


def aws_cryptography_encryptionsdk_test_roundtrip_RoundTripEncryptInput(dafny_input):
    return test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.models.RoundTripEncryptInput(
        plaintext=bytes(dafny_input.plaintext),
        encryption_context=(
            (
                {
                    bytes(key.Elements)
                    .decode("utf-8"): bytes(value.Elements)
                    .decode("utf-8")
                    for (key, value) in dafny_input.encryptionContext.value.items
                }
            )
            if (dafny_input.encryptionContext.is_Some)
            else None
        ),
        keyring=(
            (
                test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.dafny_to_smithy.aws_cryptography_encryptionsdk_test_roundtrip_SupportedKeyringCreateInputs(
                    dafny_input.keyring.value
                )
            )
            if (dafny_input.keyring.is_Some)
            else None
        ),
        algorithm_suite_id=(
            (
                aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.dafny_to_smithy.aws_cryptography_materialproviders_ESDKAlgorithmSuiteId(
                    dafny_input.algorithmSuiteId.value
                )
            )
            if (dafny_input.algorithmSuiteId.is_Some)
            else None
        ),
        frame_length=(
            (dafny_input.frameLength.value)
            if (dafny_input.frameLength.is_Some)
            else None
        ),
    )


def aws_cryptography_encryptionsdk_test_roundtrip_SupportedKeyringCreateInputs(
    dafny_input,
):
    # Convert SupportedKeyringCreateInputs
    if isinstance(dafny_input, SupportedKeyringCreateInputs_RawAes):
        SupportedKeyringCreateInputs_union_value = test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.models.SupportedKeyringCreateInputsRawAes(
            aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.dafny_to_smithy.aws_cryptography_materialproviders_CreateRawAesKeyringInput(
                dafny_input.RawAes
            )
        )
    else:
        raise ValueError("No recognized union value in union type: " + str(dafny_input))

    return SupportedKeyringCreateInputs_union_value


def aws_cryptography_encryptionsdk_test_roundtrip_RoundTripDecryptInput(dafny_input):
    return test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.models.RoundTripDecryptInput(
        encryption_context=(
            (
                {
                    bytes(key.Elements)
                    .decode("utf-8"): bytes(value.Elements)
                    .decode("utf-8")
                    for (key, value) in dafny_input.encryptionContext.value.items
                }
            )
            if (dafny_input.encryptionContext.is_Some)
            else None
        ),
        keyring=(
            (
                test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.dafny_to_smithy.aws_cryptography_encryptionsdk_test_roundtrip_SupportedKeyringCreateInputs(
                    dafny_input.keyring.value
                )
            )
            if (dafny_input.keyring.is_Some)
            else None
        ),
    )


def aws_cryptography_encryptionsdk_test_roundtrip_TestCrossLanguageRoundTripInput(
    dafny_input,
):
    return test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.models.TestCrossLanguageRoundTripInput(
        encrypt_language=test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.dafny_to_smithy.aws_cryptography_encryptionsdk_test_roundtrip_ImplementationLanguage(
            dafny_input.encryptLanguage
        ),
        decrypt_language=test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.dafny_to_smithy.aws_cryptography_encryptionsdk_test_roundtrip_ImplementationLanguage(
            dafny_input.decryptLanguage
        ),
        encrypt_input=test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.dafny_to_smithy.aws_cryptography_encryptionsdk_test_roundtrip_RoundTripEncryptInput(
            dafny_input.encryptInput
        ),
        decrypt_input=test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.dafny_to_smithy.aws_cryptography_encryptionsdk_test_roundtrip_RoundTripDecryptInput(
            dafny_input.decryptInput
        ),
    )


def aws_cryptography_encryptionsdk_test_roundtrip_TestCrossLanguageRoundTripOutput(
    dafny_input,
):
    return test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.models.TestCrossLanguageRoundTripOutput(
        status=b"".join(ord(c).to_bytes(2, "big") for c in dafny_input.status).decode(
            "utf-16-be"
        ),
    )


def aws_cryptography_encryptionsdk_test_roundtrip_ESDKTestRoundTripConfig(dafny_input):
    # Deferred import of .config to avoid circular dependency
    import test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.config

    return (
        test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.config.ESDKTestRoundTripConfig()
    )
