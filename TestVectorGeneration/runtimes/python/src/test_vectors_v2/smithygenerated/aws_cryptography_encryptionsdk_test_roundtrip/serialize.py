# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
# Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

import test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.smithy_to_dafny

from .dafny_protocol import DafnyRequest

from .config import Config


def _serialize_test_cross_language_round_trip(input, config: Config) -> DafnyRequest:
    return DafnyRequest(
        operation_name="TestCrossLanguageRoundTrip",
        dafny_operation_input=test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.smithy_to_dafny.aws_cryptography_encryptionsdk_test_roundtrip_TestCrossLanguageRoundTripInput(
            input
        ),
    )
