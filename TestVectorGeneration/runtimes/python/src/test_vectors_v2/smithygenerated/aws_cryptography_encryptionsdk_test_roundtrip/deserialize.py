# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
# Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

import _dafny
from test_vectors_v2.internaldafny.generated.AwsCryptographyEncryptionSdkTestRoundtripTypes import (
    Error,
    TestCrossLanguageRoundTripOutput_TestCrossLanguageRoundTripOutput as DafnyTestCrossLanguageRoundTripOutput,
)
import test_vectors_v2.internaldafny.generated.module_
import test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.dafny_to_smithy
from typing import Any

from .dafny_protocol import DafnyResponse
from .errors import CollectionOfErrors, OpaqueError, ServiceError

from .config import Config


def _deserialize_test_cross_language_round_trip(input: DafnyResponse, config: Config):

    if input.IsFailure():
        return _deserialize_error(input.error)
    return test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.dafny_to_smithy.aws_cryptography_encryptionsdk_test_roundtrip_TestCrossLanguageRoundTripOutput(
        input.value
    )


def _deserialize_error(error: Error) -> ServiceError:
    if error.is_Opaque:
        return OpaqueError(obj=error.obj)
    elif error.is_OpaqueWithText:
        return OpaqueErrorWithText(obj=error.obj, obj_message=error.objMessage)
    elif error.is_CollectionOfErrors:
        return CollectionOfErrors(
            message=_dafny.string_of(error.message),
            list=[_deserialize_error(dafny_e) for dafny_e in error.list],
        )
    else:
        return OpaqueError(obj=error)
