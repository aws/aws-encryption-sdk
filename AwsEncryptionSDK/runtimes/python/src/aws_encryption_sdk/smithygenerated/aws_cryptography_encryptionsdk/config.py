# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
# Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

from aws_encryption_sdk.internaldafny.generated.AwsCryptographyEncryptionSdkTypes import (
    AwsEncryptionSdkConfig_AwsEncryptionSdkConfig as DafnyAwsEncryptionSdkConfig,
)
import aws_encryption_sdk.internaldafny.generated.module_
import aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.dafny_to_smithy
import aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.smithy_to_dafny
from dataclasses import dataclass
from typing import Any, Callable, Dict, Optional, TypeAlias

from .dafnyImplInterface import DafnyImplInterface
from smithy_python._private.retries import SimpleRetryStrategy
from smithy_python.interfaces.retries import RetryStrategy


_ServiceInterceptor = Any


@dataclass(init=False)
class Config:
    """Configuration for AwsEncryptionSdk."""

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


class AwsEncryptionSdkConfig(Config):
    commitment_policy: Optional[str]
    max_encrypted_data_keys: int
    net_v4_0_0_retry_policy: Optional[str]

    def __init__(
        self,
        *,
        commitment_policy: Optional[str] = None,
        max_encrypted_data_keys: int = 0,
        net_v4_0_0_retry_policy: Optional[str] = None,
    ):
        """Constructor for AwsEncryptionSdkConfig.

        :param net_v4_0_0_retry_policy: During Decryption, Allow or
            Forbid ESDK-NET v4.0.0 Behavior if the ESDK Message Header
            fails the Header Authentication check.
        """
        super().__init__()
        self.commitment_policy = commitment_policy
        if (max_encrypted_data_keys is not None) and (max_encrypted_data_keys < 1):
            raise ValueError(
                "max_encrypted_data_keys must be greater than or equal to 1"
            )

        self.max_encrypted_data_keys = max_encrypted_data_keys
        self.net_v4_0_0_retry_policy = net_v4_0_0_retry_policy

    def as_dict(self) -> Dict[str, Any]:
        """Converts the AwsEncryptionSdkConfig to a dictionary."""
        d: Dict[str, Any] = {}

        if self.commitment_policy is not None:
            d["commitment_policy"] = self.commitment_policy

        if self.max_encrypted_data_keys is not None:
            d["max_encrypted_data_keys"] = self.max_encrypted_data_keys

        if self.net_v4_0_0_retry_policy is not None:
            d["net_v4_0_0_retry_policy"] = self.net_v4_0_0_retry_policy

        return d

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "AwsEncryptionSdkConfig":
        """Creates a AwsEncryptionSdkConfig from a dictionary."""
        kwargs: Dict[str, Any] = {}

        if "commitment_policy" in d:
            kwargs["commitment_policy"] = d["commitment_policy"]

        if "max_encrypted_data_keys" in d:
            kwargs["max_encrypted_data_keys"] = d["max_encrypted_data_keys"]

        if "net_v4_0_0_retry_policy" in d:
            kwargs["net_v4_0_0_retry_policy"] = d["net_v4_0_0_retry_policy"]

        return AwsEncryptionSdkConfig(**kwargs)

    def __repr__(self) -> str:
        result = "AwsEncryptionSdkConfig("
        if self.commitment_policy is not None:
            result += f"commitment_policy={repr(self.commitment_policy)}, "

        if self.max_encrypted_data_keys is not None:
            result += f"max_encrypted_data_keys={repr(self.max_encrypted_data_keys)}, "

        if self.net_v4_0_0_retry_policy is not None:
            result += f"net_v4_0_0_retry_policy={repr(self.net_v4_0_0_retry_policy)}"

        return result + ")"

    def __eq__(self, other: Any) -> bool:
        if not isinstance(other, AwsEncryptionSdkConfig):
            return False
        attributes: list[str] = [
            "commitment_policy",
            "max_encrypted_data_keys",
            "net_v4_0_0_retry_policy",
        ]
        return all(getattr(self, a) == getattr(other, a) for a in attributes)


def dafny_config_to_smithy_config(dafny_config) -> AwsEncryptionSdkConfig:
    """Converts the provided Dafny shape for this localService's config into
    the corresponding Smithy-modelled shape."""
    return aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.dafny_to_smithy.aws_cryptography_encryptionsdk_AwsEncryptionSdkConfig(
        dafny_config
    )


def smithy_config_to_dafny_config(smithy_config) -> DafnyAwsEncryptionSdkConfig:
    """Converts the provided Smithy-modelled shape for this localService's
    config into the corresponding Dafny shape."""
    return aws_encryption_sdk.smithygenerated.aws_cryptography_encryptionsdk.smithy_to_dafny.aws_cryptography_encryptionsdk_AwsEncryptionSdkConfig(
        smithy_config
    )
