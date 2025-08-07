# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
# Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

from _dafny import Map, Seq
import aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.smithy_to_dafny
from smithy_dafny_standard_library.internaldafny.generated.Wrappers import (
    Option_None,
    Option_Some,
)
from test_vectors_v2.internaldafny.generated.AwsCryptographyEncryptionSdkTestRoundtripTypes import (
    ESDKTestRoundTripConfig_ESDKTestRoundTripConfig as DafnyESDKTestRoundTripConfig,
    ImplementationLanguage_DOTNET,
    ImplementationLanguage_GO,
    ImplementationLanguage_JAVA,
    ImplementationLanguage_JAVASCRIPT,
    ImplementationLanguage_PYTHON,
    ImplementationLanguage_RUST,
    RoundTripDecryptInput_RoundTripDecryptInput as DafnyRoundTripDecryptInput,
    RoundTripEncryptInput_RoundTripEncryptInput as DafnyRoundTripEncryptInput,
    SupportedKeyringCreateInputs_RawAes,
    TestCrossLanguageRoundTripInput_TestCrossLanguageRoundTripInput as DafnyTestCrossLanguageRoundTripInput,
    TestCrossLanguageRoundTripOutput_TestCrossLanguageRoundTripOutput as DafnyTestCrossLanguageRoundTripOutput,
)
import test_vectors_v2.internaldafny.generated.module_
import test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.models
import test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.smithy_to_dafny


def aws_cryptography_encryptionsdk_test_roundtrip_TestCrossLanguageRoundTripInput(
    native_input,
):
    return DafnyTestCrossLanguageRoundTripInput(
        encryptLanguage=test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.smithy_to_dafny.aws_cryptography_encryptionsdk_test_roundtrip_ImplementationLanguage(
            native_input.encrypt_language
        ),
        decryptLanguage=test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.smithy_to_dafny.aws_cryptography_encryptionsdk_test_roundtrip_ImplementationLanguage(
            native_input.decrypt_language
        ),
        encryptInput=test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.smithy_to_dafny.aws_cryptography_encryptionsdk_test_roundtrip_RoundTripEncryptInput(
            native_input.encrypt_input
        ),
        decryptInput=test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.smithy_to_dafny.aws_cryptography_encryptionsdk_test_roundtrip_RoundTripDecryptInput(
            native_input.decrypt_input
        ),
    )


def aws_cryptography_encryptionsdk_test_roundtrip_ImplementationLanguage(native_input):
    if native_input == "JAVA":
        return ImplementationLanguage_JAVA()

    elif native_input == "PYTHON":
        return ImplementationLanguage_PYTHON()

    elif native_input == "DOTNET":
        return ImplementationLanguage_DOTNET()

    elif native_input == "GO":
        return ImplementationLanguage_GO()

    elif native_input == "JAVASCRIPT":
        return ImplementationLanguage_JAVASCRIPT()

    elif native_input == "RUST":
        return ImplementationLanguage_RUST()

    else:
        raise ValueError(f"No recognized enum value in enum type: {native_input=}")


def aws_cryptography_encryptionsdk_test_roundtrip_RoundTripEncryptInput(native_input):
    return DafnyRoundTripEncryptInput(
        plaintext=Seq(native_input.plaintext),
        encryptionContext=(
            (
                Option_Some(
                    Map(
                        {
                            Seq(key.encode("utf-8")): Seq(value.encode("utf-8"))
                            for (key, value) in native_input.encryption_context.items()
                        }
                    )
                )
            )
            if (native_input.encryption_context is not None)
            else (Option_None())
        ),
        keyring=(
            (
                Option_Some(
                    test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.smithy_to_dafny.aws_cryptography_encryptionsdk_test_roundtrip_SupportedKeyringCreateInputs(
                        native_input.keyring
                    )
                )
            )
            if (native_input.keyring is not None)
            else (Option_None())
        ),
        algorithmSuiteId=(
            (
                Option_Some(
                    aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.smithy_to_dafny.aws_cryptography_materialproviders_ESDKAlgorithmSuiteId(
                        native_input.algorithm_suite_id
                    )
                )
            )
            if (native_input.algorithm_suite_id is not None)
            else (Option_None())
        ),
        frameLength=(
            (Option_Some(native_input.frame_length))
            if (native_input.frame_length is not None)
            else (Option_None())
        ),
    )


def aws_cryptography_encryptionsdk_test_roundtrip_SupportedKeyringCreateInputs(
    native_input,
):
    if isinstance(
        native_input,
        test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.models.SupportedKeyringCreateInputsRawAes,
    ):
        SupportedKeyringCreateInputs_union_value = SupportedKeyringCreateInputs_RawAes(
            aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.smithy_to_dafny.aws_cryptography_materialproviders_CreateRawAesKeyringInput(
                native_input.value
            )
        )
    else:
        raise ValueError(
            "No recognized union value in union type: " + str(native_input)
        )

    return SupportedKeyringCreateInputs_union_value


def aws_cryptography_encryptionsdk_test_roundtrip_RoundTripDecryptInput(native_input):
    return DafnyRoundTripDecryptInput(
        encryptionContext=(
            (
                Option_Some(
                    Map(
                        {
                            Seq(key.encode("utf-8")): Seq(value.encode("utf-8"))
                            for (key, value) in native_input.encryption_context.items()
                        }
                    )
                )
            )
            if (native_input.encryption_context is not None)
            else (Option_None())
        ),
        keyring=(
            (
                Option_Some(
                    test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.smithy_to_dafny.aws_cryptography_encryptionsdk_test_roundtrip_SupportedKeyringCreateInputs(
                        native_input.keyring
                    )
                )
            )
            if (native_input.keyring is not None)
            else (Option_None())
        ),
    )


def aws_cryptography_encryptionsdk_test_roundtrip_TestCrossLanguageRoundTripOutput(
    native_input,
):
    return DafnyTestCrossLanguageRoundTripOutput(
        status=Seq(
            "".join(
                [
                    chr(int.from_bytes(pair, "big"))
                    for pair in zip(
                        *[iter(native_input.status.encode("utf-16-be"))] * 2
                    )
                ]
            )
        ),
    )


def aws_cryptography_encryptionsdk_test_roundtrip_ESDKTestRoundTripConfig(native_input):
    return DafnyESDKTestRoundTripConfig()
