# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
# Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

from dataclasses import dataclass
from test_vectors_v2.internaldafny.generated.AwsCryptographyEncryptionSdkTestRoundtripTypes import (
    ESDKTestRoundTripConfig_ESDKTestRoundTripConfig as DafnyESDKTestRoundTripConfig,
)
import test_vectors_v2.internaldafny.generated.module_
import test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.dafny_to_smithy
import test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.smithy_to_dafny
from typing import Any, Callable, Dict, TypeAlias

from .dafnyImplInterface import DafnyImplInterface
from smithy_python._private.retries import SimpleRetryStrategy
from smithy_python.interfaces.retries import RetryStrategy


_ServiceInterceptor = Any


@dataclass(init=False)
class Config:
    """Configuration for ESDKTestRoundTripService."""

    interceptors: list[_ServiceInterceptor]
    retry_strategy: RetryStrategy
    dafnyImplInterface: DafnyImplInterface | None

    def __init__(
        self,
        *,
        interceptors: list[_ServiceInterceptor] | None = None,
        retry_strategy: RetryStrategy | None = None,
        dafnyImplInterface: DafnyImplInterface | None = None,
    ):
        """Constructor.

        :param interceptors: The list of interceptors, which are hooks
            that are called during the execution of a request.
        :param retry_strategy: The retry strategy for issuing retry
            tokens and computing retry delays.
        :param dafnyImplInterface:
        """
        self.interceptors = interceptors or []
        self.retry_strategy = retry_strategy or SimpleRetryStrategy()
        self.dafnyImplInterface = dafnyImplInterface


# A callable that allows customizing the config object on each request.
Plugin: TypeAlias = Callable[[Config], None]


class ESDKTestRoundTripConfig(Config):
    def __init__(
        self,
    ):
        """Constructor for ESDKTestRoundTripConfig."""
        super().__init__()

    def as_dict(self) -> Dict[str, Any]:
        """Converts the ESDKTestRoundTripConfig to a dictionary."""
        return {}

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "ESDKTestRoundTripConfig":
        """Creates a ESDKTestRoundTripConfig from a dictionary."""
        return ESDKTestRoundTripConfig()

    def __repr__(self) -> str:
        result = "ESDKTestRoundTripConfig("

        return result + ")"

    def __eq__(self, other: Any) -> bool:
        return isinstance(other, ESDKTestRoundTripConfig)


def dafny_config_to_smithy_config(dafny_config) -> ESDKTestRoundTripConfig:
    """Converts the provided Dafny shape for this localService's config into
    the corresponding Smithy-modelled shape."""
    return test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.dafny_to_smithy.aws_cryptography_encryptionsdk_test_roundtrip_ESDKTestRoundTripConfig(
        dafny_config
    )


def smithy_config_to_dafny_config(smithy_config) -> DafnyESDKTestRoundTripConfig:
    """Converts the provided Smithy-modelled shape for this localService's
    config into the corresponding Dafny shape."""
    return test_vectors_v2.smithygenerated.aws_cryptography_encryptionsdk_test_roundtrip.smithy_to_dafny.aws_cryptography_encryptionsdk_test_roundtrip_ESDKTestRoundTripConfig(
        smithy_config
    )
