# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
# Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

import _dafny
from aws_encryption_sdk.internaldafny.generated.AwsCryptographyEncryptionSdkTypes import (
    DecryptOutput_DecryptOutput as DafnyDecryptOutput,
    EncryptOutput_EncryptOutput as DafnyEncryptOutput,
    Error,
    Error_AwsEncryptionSdkException,
)
import aws_encryption_sdk.internaldafny.generated.module_
import aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.dafny_to_smithy
from typing import Any

from .dafny_protocol import DafnyResponse
from .errors import (
    AwsCryptographicMaterialProviders,
    AwsCryptographicPrimitives,
    AwsEncryptionSdkException,
    CollectionOfErrors,
    OpaqueError,
    ServiceError,
)
from aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.deserialize import (
    _deserialize_error as aws_cryptography_materialproviders_deserialize_error,
)
from aws_cryptography_primitives.smithygenerated.aws_cryptography_primitives.deserialize import (
    _deserialize_error as aws_cryptography_primitives_deserialize_error,
)

from .config import Config


def _deserialize_encrypt(input: DafnyResponse, config: Config):

    if input.IsFailure():
        return _deserialize_error(input.error)
    return aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.dafny_to_smithy.aws_cryptography_encryptionsdk_EncryptOutput(
        input.value
    )


def _deserialize_decrypt(input: DafnyResponse, config: Config):

    if input.IsFailure():
        return _deserialize_error(input.error)
    return aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.dafny_to_smithy.aws_cryptography_encryptionsdk_DecryptOutput(
        input.value
    )


def _deserialize_error(error: Error) -> ServiceError:
    if error.is_Opaque:
        return OpaqueError(obj=error.obj)
    elif error.is_CollectionOfErrors:
        return CollectionOfErrors(
            message=_dafny.string_of(error.message),
            list=[_deserialize_error(dafny_e) for dafny_e in error.list],
        )
    elif error.is_AwsEncryptionSdkException:
        return AwsEncryptionSdkException(message=_dafny.string_of(error.message))
    elif error.is_AwsCryptographyPrimitives:
        return AwsCryptographicPrimitives(
            aws_cryptography_primitives_deserialize_error(
                error.AwsCryptographyPrimitives
            )
        )
    elif error.is_AwsCryptographyMaterialProviders:
        return AwsCryptographicMaterialProviders(
            aws_cryptography_materialproviders_deserialize_error(
                error.AwsCryptographyMaterialProviders
            )
        )
    else:
        return OpaqueError(obj=error)
