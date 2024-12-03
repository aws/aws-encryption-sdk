# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
# Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

from .config import (
    Config,
    Plugin,
    smithy_config_to_dafny_config,
    AwsEncryptionSdkConfig,
)
from smithy_python.interfaces.retries import RetryStrategy
from smithy_python.exceptions import SmithyRetryException
from .dafnyImplInterface import DafnyImplInterface


def set_config_impl(config: Config):
    """Set the Dafny-compiled implementation in the Smithy-Python client Config
    and load our custom NoRetriesStrategy."""
    config.dafnyImplInterface = DafnyImplInterface()
    if isinstance(config, AwsEncryptionSdkConfig):
        from aws_encryption_sdk.internaldafny.generated.ESDK import default__

        config.dafnyImplInterface.impl = default__.ESDK(
            smithy_config_to_dafny_config(config)
        ).value
    config.retry_strategy = NoRetriesStrategy()


class ZeroRetryDelayToken:
    """Placeholder class required by Smithy-Python client implementation.

    Do not wait to retry.
    """

    retry_delay = 0


class NoRetriesStrategy(RetryStrategy):
    """Placeholder class required by Smithy-Python client implementation.

    Do not retry calling Dafny code.
    """

    def acquire_initial_retry_token(self):
        return ZeroRetryDelayToken()

    def refresh_retry_token_for_retry(self, token_to_renew, error_info):
        # Do not retry
        raise SmithyRetryException()
