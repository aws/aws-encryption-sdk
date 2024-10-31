# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
# Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

from aws_encryption_sdk_python_dafny.internaldafny.generated.AwsCryptographyEncryptionSdkTypes import (
    IAwsEncryptionSdkClient,
)
from typing import Callable, TypeVar, cast

from .config import AwsEncryptionSdkConfig, Config
from .dafny_protocol import DafnyRequest, DafnyResponse
from .plugin import set_config_impl
from smithy_python.exceptions import SmithyRetryException
from smithy_python.interfaces.interceptor import Interceptor, InterceptorContext
from smithy_python.interfaces.retries import RetryErrorInfo, RetryErrorType

from .config import Plugin
from .deserialize import _deserialize_decrypt, _deserialize_encrypt
from .errors import ServiceError
from .models import DecryptInput, DecryptOutput, EncryptInput, EncryptOutput
from .serialize import _serialize_decrypt, _serialize_encrypt


Input = TypeVar("Input")
Output = TypeVar("Output")


class AwsEncryptionSdk:
    """Client for AwsEncryptionSdk.

    :param config: Configuration for the client.
    """

    def __init__(
        self,
        config: AwsEncryptionSdkConfig | None = None,
        dafny_client: IAwsEncryptionSdkClient | None = None,
    ):
        if config is None:
            self._config = Config()
        else:
            self._config = config

        client_plugins: list[Plugin] = [
            set_config_impl,
        ]

        for plugin in client_plugins:
            plugin(self._config)

        if dafny_client is not None:
            self._config.dafnyImplInterface.impl = dafny_client

    def encrypt(self, input: EncryptInput) -> EncryptOutput:
        """Invokes the Encrypt operation.

        :param input: The operation's input.
        """
        return self._execute_operation(
            input=input,
            plugins=[],
            serialize=_serialize_encrypt,
            deserialize=_deserialize_encrypt,
            config=self._config,
            operation_name="Encrypt",
        )

    def decrypt(self, input: DecryptInput) -> DecryptOutput:
        """Invokes the Decrypt operation.

        :param input: The operation's input.
        """
        return self._execute_operation(
            input=input,
            plugins=[],
            serialize=_serialize_decrypt,
            deserialize=_deserialize_decrypt,
            config=self._config,
            operation_name="Decrypt",
        )

    def _execute_operation(
        self,
        input: Input,
        plugins: list[Plugin],
        serialize: Callable[[Input, Config], DafnyRequest],
        deserialize: Callable[[DafnyResponse, Config], Output],
        config: Config,
        operation_name: str,
    ) -> Output:
        try:
            return self._handle_execution(
                input, plugins, serialize, deserialize, config, operation_name
            )
        except Exception as e:
            # Make sure every exception that we throw is an instance of ServiceError so
            # customers can reliably catch everything we throw.
            if not isinstance(e, ServiceError):
                raise ServiceError(e) from e
            raise e

    def _handle_execution(
        self,
        input: Input,
        plugins: list[Plugin],
        serialize: Callable[[Input, Config], DafnyRequest],
        deserialize: Callable[[DafnyResponse, Config], Output],
        config: Config,
        operation_name: str,
    ) -> Output:
        context: InterceptorContext[Input, None, None, None] = InterceptorContext(
            request=input,
            response=None,
            transport_request=None,
            transport_response=None,
        )
        _client_interceptors = config.interceptors
        client_interceptors = cast(
            list[Interceptor[Input, Output, DafnyRequest, DafnyResponse]],
            _client_interceptors,
        )
        interceptors = client_interceptors

        try:
            # Step 1a: Invoke read_before_execution on client-level interceptors
            for interceptor in client_interceptors:
                interceptor.read_before_execution(context)

            # Step 1b: Run operation-level plugins
            for plugin in plugins:
                plugin(config)

            _client_interceptors = config.interceptors
            interceptors = cast(
                list[Interceptor[Input, Output, DafnyRequest, DafnyResponse]],
                _client_interceptors,
            )

            # Step 1c: Invoke the read_before_execution hooks on newly added
            # interceptors.
            for interceptor in interceptors:
                if interceptor not in client_interceptors:
                    interceptor.read_before_execution(context)

            # Step 2: Invoke the modify_before_serialization hooks
            for interceptor in interceptors:
                context._request = interceptor.modify_before_serialization(context)

            # Step 3: Invoke the read_before_serialization hooks
            for interceptor in interceptors:
                interceptor.read_before_serialization(context)

            # Step 4: Serialize the request
            context_with_transport_request = cast(
                InterceptorContext[Input, None, DafnyRequest, None], context
            )
            context_with_transport_request._transport_request = serialize(
                context_with_transport_request.request, config
            )

            # Step 5: Invoke read_after_serialization
            for interceptor in interceptors:
                interceptor.read_after_serialization(context_with_transport_request)

            # Step 6: Invoke modify_before_retry_loop
            for interceptor in interceptors:
                context_with_transport_request._transport_request = (
                    interceptor.modify_before_retry_loop(context_with_transport_request)
                )

            # Step 7: Acquire the retry token.
            retry_strategy = config.retry_strategy
            retry_token = retry_strategy.acquire_initial_retry_token()

            while True:
                # Make an attempt, creating a copy of the context so we don't pass
                # around old data.
                context_with_response = self._handle_attempt(
                    deserialize,
                    interceptors,
                    context_with_transport_request.copy(),
                    config,
                    operation_name,
                )

                # We perform this type-ignored re-assignment because `context` needs
                # to point at the latest context so it can be generically handled
                # later on. This is only an issue here because we've created a copy,
                # so we're no longer simply pointing at the same object in memory
                # with different names and type hints. It is possible to address this
                # without having to fall back to the type ignore, but it would impose
                # unnecessary runtime costs.
                context = context_with_response  # type: ignore

                if isinstance(context_with_response.response, Exception):
                    # Step 7u: Reacquire retry token if the attempt failed
                    try:
                        retry_token = retry_strategy.refresh_retry_token_for_retry(
                            token_to_renew=retry_token,
                            error_info=RetryErrorInfo(
                                # TODO: Determine the error type.
                                error_type=RetryErrorType.CLIENT_ERROR,
                            ),
                        )
                    except SmithyRetryException:
                        raise context_with_response.response
                else:
                    # Step 8: Invoke record_success
                    retry_strategy.record_success(token=retry_token)
                    break
        except Exception as e:
            context._response = e

        # At this point, the context's request will have been definitively set, and
        # The response will be set either with the modeled output or an exception. The
        # transport_request and transport_response may be set or None.
        execution_context = cast(
            InterceptorContext[
                Input, Output, DafnyRequest | None, DafnyResponse | None
            ],
            context,
        )
        return self._finalize_execution(interceptors, execution_context)

    def _handle_attempt(
        self,
        deserialize: Callable[[DafnyResponse, Config], Output],
        interceptors: list[Interceptor[Input, Output, DafnyRequest, DafnyResponse]],
        context: InterceptorContext[Input, None, DafnyRequest, None],
        config: Config,
        operation_name: str,
    ) -> InterceptorContext[Input, Output, DafnyRequest, DafnyResponse | None]:
        try:
            # Step 7a: Invoke read_before_attempt
            for interceptor in interceptors:
                interceptor.read_before_attempt(context)

            # Step 7m: Involve client Dafny impl
            if config.dafnyImplInterface.impl is None:
                raise Exception("No impl found on the operation config.")

            context_with_response = cast(
                InterceptorContext[Input, None, DafnyRequest, DafnyResponse], context
            )

            context_with_response._transport_response = (
                config.dafnyImplInterface.handle_request(
                    input=context_with_response.transport_request
                )
            )

            # Step 7n: Invoke read_after_transmit
            for interceptor in interceptors:
                interceptor.read_after_transmit(context_with_response)

            # Step 7o: Invoke modify_before_deserialization
            for interceptor in interceptors:
                context_with_response._transport_response = (
                    interceptor.modify_before_deserialization(context_with_response)
                )

            # Step 7p: Invoke read_before_deserialization
            for interceptor in interceptors:
                interceptor.read_before_deserialization(context_with_response)

            # Step 7q: deserialize
            context_with_output = cast(
                InterceptorContext[Input, Output, DafnyRequest, DafnyResponse],
                context_with_response,
            )
            context_with_output._response = deserialize(
                context_with_output._transport_response, config
            )

            # Step 7r: Invoke read_after_deserialization
            for interceptor in interceptors:
                interceptor.read_after_deserialization(context_with_output)
        except Exception as e:
            context._response = e

        # At this point, the context's request and transport_request have definitively been set,
        # the response is either set or an exception, and the transport_resposne is either set or
        # None. This will also be true after _finalize_attempt because there is no opportunity
        # there to set the transport_response.
        attempt_context = cast(
            InterceptorContext[Input, Output, DafnyRequest, DafnyResponse | None],
            context,
        )
        return self._finalize_attempt(interceptors, attempt_context)

    def _finalize_attempt(
        self,
        interceptors: list[Interceptor[Input, Output, DafnyRequest, DafnyResponse]],
        context: InterceptorContext[Input, Output, DafnyRequest, DafnyResponse | None],
    ) -> InterceptorContext[Input, Output, DafnyRequest, DafnyResponse | None]:
        # Step 7s: Invoke modify_before_attempt_completion
        try:
            for interceptor in interceptors:
                context._response = interceptor.modify_before_attempt_completion(
                    context
                )
        except Exception as e:
            context._response = e

        # Step 7t: Invoke read_after_attempt
        for interceptor in interceptors:
            try:
                interceptor.read_after_attempt(context)
            except Exception as e:
                context._response = e

        return context

    def _finalize_execution(
        self,
        interceptors: list[Interceptor[Input, Output, DafnyRequest, DafnyResponse]],
        context: InterceptorContext[
            Input, Output, DafnyRequest | None, DafnyResponse | None
        ],
    ) -> Output:
        try:
            # Step 9: Invoke modify_before_completion
            for interceptor in interceptors:
                context._response = interceptor.modify_before_completion(context)

        except Exception as e:
            context._response = e

        # Step 11: Invoke read_after_execution
        for interceptor in interceptors:
            try:
                interceptor.read_after_execution(context)
            except Exception as e:
                context._response = e

        # Step 12: Return / throw
        if isinstance(context.response, Exception):
            raise context.response

        # We may want to add some aspects of this context to the output types so we can
        # return it to the end-users.
        return context.response
