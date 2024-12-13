# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
# Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

import aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.dafny_to_smithy
from aws_encryption_sdk.internaldafny.generated.AwsCryptographyEncryptionSdkTypes import (
    NetV4_0_0_RetryPolicy_ALLOW__RETRY,
    NetV4_0_0_RetryPolicy_FORBID__RETRY,
)
import aws_encryption_sdk.internaldafny.generated.module_
import aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.dafny_to_smithy
import aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.models


def aws_cryptography_encryptionsdk_EncryptInput(dafny_input):
    return aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.models.EncryptInput(
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
        materials_manager=(
            (
                aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.dafny_to_smithy.aws_cryptography_materialproviders_CryptographicMaterialsManagerReference(
                    dafny_input.materialsManager.UnwrapOr(None)
                )
            )
            if (dafny_input.materialsManager.UnwrapOr(None) is not None)
            else None
        ),
        keyring=(
            (
                aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.dafny_to_smithy.aws_cryptography_materialproviders_KeyringReference(
                    dafny_input.keyring.UnwrapOr(None)
                )
            )
            if (dafny_input.keyring.UnwrapOr(None) is not None)
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


def aws_cryptography_encryptionsdk_DecryptInput(dafny_input):
    return aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.models.DecryptInput(
        ciphertext=bytes(dafny_input.ciphertext),
        materials_manager=(
            (
                aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.dafny_to_smithy.aws_cryptography_materialproviders_CryptographicMaterialsManagerReference(
                    dafny_input.materialsManager.UnwrapOr(None)
                )
            )
            if (dafny_input.materialsManager.UnwrapOr(None) is not None)
            else None
        ),
        keyring=(
            (
                aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.dafny_to_smithy.aws_cryptography_materialproviders_KeyringReference(
                    dafny_input.keyring.UnwrapOr(None)
                )
            )
            if (dafny_input.keyring.UnwrapOr(None) is not None)
            else None
        ),
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
    )


def aws_cryptography_encryptionsdk_EncryptOutput(dafny_input):
    return aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.models.EncryptOutput(
        ciphertext=bytes(dafny_input.ciphertext),
        encryption_context={
            bytes(key.Elements).decode("utf-8"): bytes(value.Elements).decode("utf-8")
            for (key, value) in dafny_input.encryptionContext.items
        },
        algorithm_suite_id=aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.dafny_to_smithy.aws_cryptography_materialproviders_ESDKAlgorithmSuiteId(
            dafny_input.algorithmSuiteId
        ),
    )


def aws_cryptography_encryptionsdk_DecryptOutput(dafny_input):
    return aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.models.DecryptOutput(
        plaintext=bytes(dafny_input.plaintext),
        encryption_context={
            bytes(key.Elements).decode("utf-8"): bytes(value.Elements).decode("utf-8")
            for (key, value) in dafny_input.encryptionContext.items
        },
        algorithm_suite_id=aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.dafny_to_smithy.aws_cryptography_materialproviders_ESDKAlgorithmSuiteId(
            dafny_input.algorithmSuiteId
        ),
    )


def aws_cryptography_encryptionsdk_NetV4_0_0_RetryPolicy(dafny_input):
    if isinstance(dafny_input, NetV4_0_0_RetryPolicy_FORBID__RETRY):
        return "FORBID_RETRY"

    elif isinstance(dafny_input, NetV4_0_0_RetryPolicy_ALLOW__RETRY):
        return "ALLOW_RETRY"

    else:
        raise ValueError(f"No recognized enum value in enum type: {dafny_input=}")


def aws_cryptography_encryptionsdk_AwsEncryptionSdkConfig(dafny_input):
    # Deferred import of .config to avoid circular dependency
    import aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.config

    return aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.config.AwsEncryptionSdkConfig(
        commitment_policy=(
            (
                aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.dafny_to_smithy.aws_cryptography_materialproviders_ESDKCommitmentPolicy(
                    dafny_input.commitmentPolicy.value
                )
            )
            if (dafny_input.commitmentPolicy.is_Some)
            else None
        ),
        max_encrypted_data_keys=(
            (dafny_input.maxEncryptedDataKeys.value)
            if (dafny_input.maxEncryptedDataKeys.is_Some)
            else None
        ),
        net_v4_0_0_retry_policy=(
            (
                aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.dafny_to_smithy.aws_cryptography_encryptionsdk_NetV4_0_0_RetryPolicy(
                    dafny_input.netV4_0_0_RetryPolicy.value
                )
            )
            if (dafny_input.netV4_0_0_RetryPolicy.is_Some)
            else None
        ),
    )
