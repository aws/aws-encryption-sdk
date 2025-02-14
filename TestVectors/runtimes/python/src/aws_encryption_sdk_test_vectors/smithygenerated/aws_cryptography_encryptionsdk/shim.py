# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
# Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

from aws_encryption_sdk_dafny.internaldafny.generated.AwsCryptographyEncryptionSdkTypes import (
    DecryptInput_DecryptInput as DafnyDecryptInput,
    DecryptOutput_DecryptOutput as DafnyDecryptOutput,
    EncryptInput_EncryptInput as DafnyEncryptInput,
    EncryptOutput_EncryptOutput as DafnyEncryptOutput,
)
import aws_encryption_sdk_dafny.internaldafny.generated.module_
import aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.dafny_to_smithy
import aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.errors
from aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.errors import (
    CollectionOfErrors,
    OpaqueError,
    ServiceError,
    _smithy_error_to_dafny_error,
)
import aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.models
import aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.smithy_to_dafny
from typing import Any


import smithy_dafny_standard_library.internaldafny.generated.Wrappers as Wrappers
import aws_encryption_sdk_dafny.internaldafny.generated.AwsCryptographyEncryptionSdkTypes
import aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.client as client_impl


class ESDKShim(
    aws_encryption_sdk_dafny.internaldafny.generated.AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClient
):
    def __init__(self, _impl: client_impl):
        self._impl = _impl

    def Encrypt(self, input):
        try:
            smithy_client_request: (
                aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.models.EncryptInput
            ) = aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.dafny_to_smithy.aws_cryptography_encryptionsdk_EncryptInput(
                input
            )
            smithy_client_response = self._impl.encrypt(smithy_client_request)
            return Wrappers.Result_Success(
                aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.smithy_to_dafny.aws_cryptography_encryptionsdk_EncryptOutput(
                    smithy_client_response
                )
            )
        except Exception as e:
            return Wrappers.Result_Failure(_smithy_error_to_dafny_error(e))

    def Decrypt(self, input):
        try:
            smithy_client_request: (
                aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.models.DecryptInput
            ) = aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.dafny_to_smithy.aws_cryptography_encryptionsdk_DecryptInput(
                input
            )
            smithy_client_response = self._impl.decrypt(smithy_client_request)
            return Wrappers.Result_Success(
                aws_encryption_sdk_dafny.smithygenerated.aws_cryptography_encryptionsdk.smithy_to_dafny.aws_cryptography_encryptionsdk_DecryptOutput(
                    smithy_client_response
                )
            )
        except Exception as e:
            return Wrappers.Result_Failure(_smithy_error_to_dafny_error(e))
