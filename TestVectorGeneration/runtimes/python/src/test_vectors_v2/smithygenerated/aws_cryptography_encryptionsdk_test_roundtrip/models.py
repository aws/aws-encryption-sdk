# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
# Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

from typing import Any, Dict, Optional, Union

from aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.models import (
    CreateRawAesKeyringInput,
)


class SupportedKeyringCreateInputsRawAes:
    """Inputs for creating a Raw AES Keyring."""

    def __init__(self, value: CreateRawAesKeyringInput):
        self.value = value

    def as_dict(self) -> Dict[str, Any]:
        return {"RawAes": self.value.as_dict()}

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "SupportedKeyringCreateInputsRawAes":
        if len(d) != 1:
            raise TypeError(f"Unions may have exactly 1 value, but found {len(d)}")

        return SupportedKeyringCreateInputsRawAes(
            CreateRawAesKeyringInput.from_dict(d["RawAes"])
        )

    def __repr__(self) -> str:
        return f"SupportedKeyringCreateInputsRawAes(value=repr(self.value))"

    def __eq__(self, other: Any) -> bool:
        if not isinstance(other, SupportedKeyringCreateInputsRawAes):
            return False
        return self.value == other.value


class SupportedKeyringCreateInputsUnknown:
    """Represents an unknown variant.

    If you receive this value, you will need to update your library to
    receive the parsed value.

    This value may not be deliberately sent.
    """

    def __init__(self, tag: str):
        self.tag = tag

    def as_dict(self) -> Dict[str, Any]:
        return {"SDK_UNKNOWN_MEMBER": {"name": self.tag}}

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "SupportedKeyringCreateInputsUnknown":
        if len(d) != 1:
            raise TypeError(f"Unions may have exactly 1 value, but found {len(d)}")
        return SupportedKeyringCreateInputsUnknown(d["SDK_UNKNOWN_MEMBER"]["name"])

    def __repr__(self) -> str:
        return f"SupportedKeyringCreateInputsUnknown(tag={self.tag})"


SupportedKeyringCreateInputs = Union[
    SupportedKeyringCreateInputsRawAes, SupportedKeyringCreateInputsUnknown
]


def _supported_keyring_create_inputs_from_dict(
    d: Dict[str, Any],
) -> SupportedKeyringCreateInputs:
    if "RawAes" in d:
        return SupportedKeyringCreateInputsRawAes.from_dict(d)

    raise TypeError(f"Unions may have exactly 1 value, but found {len(d)}")


class RoundTripDecryptInput:
    encryption_context: Optional[dict[str, str]]
    keyring: Optional[SupportedKeyringCreateInputs]

    def __init__(
        self,
        *,
        encryption_context: Optional[dict[str, str]] = None,
        keyring: Optional[SupportedKeyringCreateInputs] = None,
    ):
        self.encryption_context = encryption_context
        self.keyring = keyring

    def as_dict(self) -> Dict[str, Any]:
        """Converts the RoundTripDecryptInput to a dictionary."""
        d: Dict[str, Any] = {}

        if self.encryption_context is not None:
            d["encryption_context"] = self.encryption_context

        if self.keyring is not None:
            d["keyring"] = self.keyring.as_dict()

        return d

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "RoundTripDecryptInput":
        """Creates a RoundTripDecryptInput from a dictionary."""
        kwargs: Dict[str, Any] = {}

        if "encryption_context" in d:
            kwargs["encryption_context"] = d["encryption_context"]

        if "keyring" in d:
            kwargs["keyring"] = (
                _supported_keyring_create_inputs_from_dict(d["keyring"]),
            )

        return RoundTripDecryptInput(**kwargs)

    def __repr__(self) -> str:
        result = "RoundTripDecryptInput("
        if self.encryption_context is not None:
            result += f"encryption_context={repr(self.encryption_context)}, "

        if self.keyring is not None:
            result += f"keyring={repr(self.keyring)}"

        return result + ")"

    def __eq__(self, other: Any) -> bool:
        if not isinstance(other, RoundTripDecryptInput):
            return False
        attributes: list[str] = [
            "encryption_context",
            "keyring",
        ]
        return all(getattr(self, a) == getattr(other, a) for a in attributes)


class ImplementationLanguage:
    JAVA = "JAVA"

    PYTHON = "PYTHON"

    DOTNET = "DOTNET"

    GO = "GO"

    JAVASCRIPT = "JAVASCRIPT"

    RUST = "RUST"

    # This set contains every possible value known at the time this was generated. New
    # values may be added in the future.
    values = frozenset({"JAVA", "PYTHON", "DOTNET", "GO", "JAVASCRIPT", "RUST"})


class RoundTripEncryptInput:
    plaintext: bytes | bytearray
    encryption_context: Optional[dict[str, str]]
    keyring: Optional[SupportedKeyringCreateInputs]
    algorithm_suite_id: Optional[str]
    frame_length: int

    def __init__(
        self,
        *,
        plaintext: bytes | bytearray,
        encryption_context: Optional[dict[str, str]] = None,
        keyring: Optional[SupportedKeyringCreateInputs] = None,
        algorithm_suite_id: Optional[str] = None,
        frame_length: int = 0,
    ):
        self.plaintext = plaintext
        self.encryption_context = encryption_context
        self.keyring = keyring
        self.algorithm_suite_id = algorithm_suite_id
        if (frame_length is not None) and (frame_length < 1):
            raise ValueError("frame_length must be greater than or equal to 1")

        if (frame_length is not None) and (frame_length > 4294967296):
            raise ValueError("frame_length must be less than or equal to 4294967296")

        self.frame_length = frame_length

    def as_dict(self) -> Dict[str, Any]:
        """Converts the RoundTripEncryptInput to a dictionary."""
        d: Dict[str, Any] = {
            "plaintext": self.plaintext,
        }

        if self.encryption_context is not None:
            d["encryption_context"] = self.encryption_context

        if self.keyring is not None:
            d["keyring"] = self.keyring.as_dict()

        if self.algorithm_suite_id is not None:
            d["algorithm_suite_id"] = self.algorithm_suite_id

        if self.frame_length is not None:
            d["frame_length"] = self.frame_length

        return d

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "RoundTripEncryptInput":
        """Creates a RoundTripEncryptInput from a dictionary."""
        kwargs: Dict[str, Any] = {
            "plaintext": d["plaintext"],
        }

        if "encryption_context" in d:
            kwargs["encryption_context"] = d["encryption_context"]

        if "keyring" in d:
            kwargs["keyring"] = (
                _supported_keyring_create_inputs_from_dict(d["keyring"]),
            )

        if "algorithm_suite_id" in d:
            kwargs["algorithm_suite_id"] = d["algorithm_suite_id"]

        if "frame_length" in d:
            kwargs["frame_length"] = d["frame_length"]

        return RoundTripEncryptInput(**kwargs)

    def __repr__(self) -> str:
        result = "RoundTripEncryptInput("
        if self.plaintext is not None:
            result += f"plaintext={repr(self.plaintext)}, "

        if self.encryption_context is not None:
            result += f"encryption_context={repr(self.encryption_context)}, "

        if self.keyring is not None:
            result += f"keyring={repr(self.keyring)}, "

        if self.algorithm_suite_id is not None:
            result += f"algorithm_suite_id={repr(self.algorithm_suite_id)}, "

        if self.frame_length is not None:
            result += f"frame_length={repr(self.frame_length)}"

        return result + ")"

    def __eq__(self, other: Any) -> bool:
        if not isinstance(other, RoundTripEncryptInput):
            return False
        attributes: list[str] = [
            "plaintext",
            "encryption_context",
            "keyring",
            "algorithm_suite_id",
            "frame_length",
        ]
        return all(getattr(self, a) == getattr(other, a) for a in attributes)


class TestCrossLanguageRoundTripInput:
    encrypt_language: str
    decrypt_language: str
    encrypt_input: RoundTripEncryptInput
    decrypt_input: RoundTripDecryptInput

    def __init__(
        self,
        *,
        encrypt_language: str,
        decrypt_language: str,
        encrypt_input: RoundTripEncryptInput,
        decrypt_input: RoundTripDecryptInput,
    ):
        self.encrypt_language = encrypt_language
        self.decrypt_language = decrypt_language
        self.encrypt_input = encrypt_input
        self.decrypt_input = decrypt_input

    def as_dict(self) -> Dict[str, Any]:
        """Converts the TestCrossLanguageRoundTripInput to a dictionary."""
        return {
            "encrypt_language": self.encrypt_language,
            "decrypt_language": self.decrypt_language,
            "encrypt_input": self.encrypt_input.as_dict(),
            "decrypt_input": self.decrypt_input.as_dict(),
        }

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "TestCrossLanguageRoundTripInput":
        """Creates a TestCrossLanguageRoundTripInput from a dictionary."""
        kwargs: Dict[str, Any] = {
            "encrypt_language": d["encrypt_language"],
            "decrypt_language": d["decrypt_language"],
            "encrypt_input": RoundTripEncryptInput.from_dict(d["encrypt_input"]),
            "decrypt_input": RoundTripDecryptInput.from_dict(d["decrypt_input"]),
        }

        return TestCrossLanguageRoundTripInput(**kwargs)

    def __repr__(self) -> str:
        result = "TestCrossLanguageRoundTripInput("
        if self.encrypt_language is not None:
            result += f"encrypt_language={repr(self.encrypt_language)}, "

        if self.decrypt_language is not None:
            result += f"decrypt_language={repr(self.decrypt_language)}, "

        if self.encrypt_input is not None:
            result += f"encrypt_input={repr(self.encrypt_input)}, "

        if self.decrypt_input is not None:
            result += f"decrypt_input={repr(self.decrypt_input)}"

        return result + ")"

    def __eq__(self, other: Any) -> bool:
        if not isinstance(other, TestCrossLanguageRoundTripInput):
            return False
        attributes: list[str] = [
            "encrypt_language",
            "decrypt_language",
            "encrypt_input",
            "decrypt_input",
        ]
        return all(getattr(self, a) == getattr(other, a) for a in attributes)


class TestCrossLanguageRoundTripOutput:
    status: str

    def __init__(
        self,
        *,
        status: str,
    ):
        self.status = status

    def as_dict(self) -> Dict[str, Any]:
        """Converts the TestCrossLanguageRoundTripOutput to a dictionary."""
        return {
            "status": self.status,
        }

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "TestCrossLanguageRoundTripOutput":
        """Creates a TestCrossLanguageRoundTripOutput from a dictionary."""
        kwargs: Dict[str, Any] = {
            "status": d["status"],
        }

        return TestCrossLanguageRoundTripOutput(**kwargs)

    def __repr__(self) -> str:
        result = "TestCrossLanguageRoundTripOutput("
        if self.status is not None:
            result += f"status={repr(self.status)}"

        return result + ")"

    def __eq__(self, other: Any) -> bool:
        if not isinstance(other, TestCrossLanguageRoundTripOutput):
            return False
        attributes: list[str] = [
            "status",
        ]
        return all(getattr(self, a) == getattr(other, a) for a in attributes)


class Unit:
    pass
