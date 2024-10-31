# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
# Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

import aws_encryption_sdk_python_dafny.smithygenerated.aws_cryptography_encryptionsdk.smithy_to_dafny

from .dafny_protocol import DafnyRequest

from .config import Config


def _serialize_encrypt(input, config: Config) -> DafnyRequest:
    return DafnyRequest(
        operation_name="Encrypt",
        dafny_operation_input=aws_encryption_sdk_python_dafny.smithygenerated.aws_cryptography_encryptionsdk.smithy_to_dafny.aws_cryptography_encryptionsdk_EncryptInput(
            input
        ),
    )


def _serialize_decrypt(input, config: Config) -> DafnyRequest:
    return DafnyRequest(
        operation_name="Decrypt",
        dafny_operation_input=aws_encryption_sdk_python_dafny.smithygenerated.aws_cryptography_encryptionsdk.smithy_to_dafny.aws_cryptography_encryptionsdk_DecryptInput(
            input
        ),
    )
