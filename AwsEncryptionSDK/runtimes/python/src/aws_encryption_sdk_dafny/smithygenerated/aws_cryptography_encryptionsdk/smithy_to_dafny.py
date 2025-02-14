# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
# Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

from _dafny import Map, Seq
import aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.smithy_to_dafny
from aws_encryption_sdk_dafny.internaldafny.generated.AwsCryptographyEncryptionSdkTypes import (
    AwsEncryptionSdkConfig_AwsEncryptionSdkConfig as DafnyAwsEncryptionSdkConfig,
    DecryptInput_DecryptInput as DafnyDecryptInput,
    DecryptOutput_DecryptOutput as DafnyDecryptOutput,
    EncryptInput_EncryptInput as DafnyEncryptInput,
    EncryptOutput_EncryptOutput as DafnyEncryptOutput,
    NetV4__0__0__RetryPolicy_ALLOW__RETRY,
    NetV4__0__0__RetryPolicy_FORBID__RETRY,
)
import aws_encryption_sdk_dafny.internaldafny.generated.module_
import aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.smithy_to_dafny
from smithy_dafny_standard_library.internaldafny.generated.Wrappers import (
    Option_None,
    Option_Some,
)


def aws_cryptography_encryptionsdk_EncryptInput(native_input):
    return DafnyEncryptInput(
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
        materialsManager=(
            (
                Option_Some(
                    aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.smithy_to_dafny.aws_cryptography_materialproviders_CryptographicMaterialsManagerReference(
                        native_input.materials_manager
                    )
                )
            )
            if (
                (native_input.materials_manager is not None)
                and (
                    aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.smithy_to_dafny.aws_cryptography_materialproviders_CryptographicMaterialsManagerReference(
                        native_input.materials_manager
                    )
                    is not None
                )
            )
            else (Option_None())
        ),
        keyring=(
            (
                Option_Some(
                    aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.smithy_to_dafny.aws_cryptography_materialproviders_KeyringReference(
                        native_input.keyring
                    )
                )
            )
            if (
                (native_input.keyring is not None)
                and (
                    aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.smithy_to_dafny.aws_cryptography_materialproviders_KeyringReference(
                        native_input.keyring
                    )
                    is not None
                )
            )
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


def aws_cryptography_encryptionsdk_DecryptInput(native_input):
    return DafnyDecryptInput(
        ciphertext=Seq(native_input.ciphertext),
        materialsManager=(
            (
                Option_Some(
                    aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.smithy_to_dafny.aws_cryptography_materialproviders_CryptographicMaterialsManagerReference(
                        native_input.materials_manager
                    )
                )
            )
            if (
                (native_input.materials_manager is not None)
                and (
                    aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.smithy_to_dafny.aws_cryptography_materialproviders_CryptographicMaterialsManagerReference(
                        native_input.materials_manager
                    )
                    is not None
                )
            )
            else (Option_None())
        ),
        keyring=(
            (
                Option_Some(
                    aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.smithy_to_dafny.aws_cryptography_materialproviders_KeyringReference(
                        native_input.keyring
                    )
                )
            )
            if (
                (native_input.keyring is not None)
                and (
                    aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.smithy_to_dafny.aws_cryptography_materialproviders_KeyringReference(
                        native_input.keyring
                    )
                    is not None
                )
            )
            else (Option_None())
        ),
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
    )


def aws_cryptography_encryptionsdk_EncryptOutput(native_input):
    return DafnyEncryptOutput(
        ciphertext=Seq(native_input.ciphertext),
        encryptionContext=Map(
            {
                Seq(key.encode("utf-8")): Seq(value.encode("utf-8"))
                for (key, value) in native_input.encryption_context.items()
            }
        ),
        algorithmSuiteId=aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.smithy_to_dafny.aws_cryptography_materialproviders_ESDKAlgorithmSuiteId(
            native_input.algorithm_suite_id
        ),
    )


def aws_cryptography_encryptionsdk_DecryptOutput(native_input):
    return DafnyDecryptOutput(
        plaintext=Seq(native_input.plaintext),
        encryptionContext=Map(
            {
                Seq(key.encode("utf-8")): Seq(value.encode("utf-8"))
                for (key, value) in native_input.encryption_context.items()
            }
        ),
        algorithmSuiteId=aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.smithy_to_dafny.aws_cryptography_materialproviders_ESDKAlgorithmSuiteId(
            native_input.algorithm_suite_id
        ),
    )


def aws_cryptography_encryptionsdk_AtomicPrimitivesReference(native_input):
    return native_input._config.dafnyImplInterface.impl


def aws_cryptography_encryptionsdk_NetV4_0_0_RetryPolicy(native_input):
    if native_input == "FORBID_RETRY":
        return NetV4__0__0__RetryPolicy_FORBID__RETRY()

    elif native_input == "ALLOW_RETRY":
        return NetV4__0__0__RetryPolicy_ALLOW__RETRY()

    else:
        raise ValueError(f"No recognized enum value in enum type: {native_input=}")


def aws_cryptography_encryptionsdk_AwsEncryptionSdkConfig(native_input):
    return DafnyAwsEncryptionSdkConfig(
        commitmentPolicy=(
            (
                Option_Some(
                    aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.smithy_to_dafny.aws_cryptography_materialproviders_ESDKCommitmentPolicy(
                        native_input.commitment_policy
                    )
                )
            )
            if (native_input.commitment_policy is not None)
            else (Option_None())
        ),
        maxEncryptedDataKeys=(
            (Option_Some(native_input.max_encrypted_data_keys))
            if (native_input.max_encrypted_data_keys is not None)
            else (Option_None())
        ),
        netV4_0_0_RetryPolicy=(
            (
                Option_Some(
                    aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.smithy_to_dafny.aws_cryptography_encryptionsdk_NetV4_0_0_RetryPolicy(
                        native_input.net_v4_0_0_retry_policy
                    )
                )
            )
            if (native_input.net_v4_0_0_retry_policy is not None)
            else (Option_None())
        ),
    )


def aws_cryptography_encryptionsdk_MaterialProvidersReference(native_input):
    return native_input._config.dafnyImplInterface.impl
