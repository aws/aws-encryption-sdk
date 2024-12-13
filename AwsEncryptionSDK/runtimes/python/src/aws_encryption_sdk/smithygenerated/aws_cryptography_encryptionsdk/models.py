# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
# Do not modify this file. This file is machine generated, and any changes to it will be overwritten.

import aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.references
from typing import Any, Dict, Optional


class DecryptInput:
    ciphertext: bytes | bytearray
    materials_manager: Optional[
        "aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.references.CryptographicMaterialsManager"
    ]
    keyring: Optional[
        "aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.references.Keyring"
    ]
    encryption_context: Optional[dict[str, str]]

    def __init__(
        self,
        *,
        ciphertext: bytes | bytearray,
        materials_manager: Optional[
            "aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.references.CryptographicMaterialsManager"
        ] = None,
        keyring: Optional[
            "aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.references.Keyring"
        ] = None,
        encryption_context: Optional[dict[str, str]] = None,
    ):
        self.ciphertext = ciphertext
        self.materials_manager = materials_manager
        self.keyring = keyring
        self.encryption_context = encryption_context

    def as_dict(self) -> Dict[str, Any]:
        """Converts the DecryptInput to a dictionary."""
        d: Dict[str, Any] = {
            "ciphertext": self.ciphertext,
        }

        if self.materials_manager is not None:
            d["materials_manager"] = self.materials_manager.as_dict()

        if self.keyring is not None:
            d["keyring"] = self.keyring.as_dict()

        if self.encryption_context is not None:
            d["encryption_context"] = self.encryption_context

        return d

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "DecryptInput":
        """Creates a DecryptInput from a dictionary."""
        from aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.references import (
            CryptographicMaterialsManager,
        )
        from aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.references import (
            Keyring,
        )

        kwargs: Dict[str, Any] = {
            "ciphertext": d["ciphertext"],
        }

        if "materials_manager" in d:
            kwargs["materials_manager"] = CryptographicMaterialsManager.from_dict(
                d["materials_manager"]
            )

        if "keyring" in d:
            kwargs["keyring"] = Keyring.from_dict(d["keyring"])

        if "encryption_context" in d:
            kwargs["encryption_context"] = d["encryption_context"]

        return DecryptInput(**kwargs)

    def __repr__(self) -> str:
        result = "DecryptInput("
        if self.ciphertext is not None:
            result += f"ciphertext={repr(self.ciphertext)}, "

        if self.materials_manager is not None:
            result += f"materials_manager={repr(self.materials_manager)}, "

        if self.keyring is not None:
            result += f"keyring={repr(self.keyring)}, "

        if self.encryption_context is not None:
            result += f"encryption_context={repr(self.encryption_context)}"

        return result + ")"

    def __eq__(self, other: Any) -> bool:
        if not isinstance(other, DecryptInput):
            return False
        attributes: list[str] = [
            "ciphertext",
            "materials_manager",
            "keyring",
            "encryption_context",
        ]
        return all(getattr(self, a) == getattr(other, a) for a in attributes)


class DecryptOutput:
    plaintext: bytes | bytearray
    encryption_context: dict[str, str]
    algorithm_suite_id: str

    def __init__(
        self,
        *,
        plaintext: bytes | bytearray,
        encryption_context: dict[str, str],
        algorithm_suite_id: str,
    ):
        self.plaintext = plaintext
        self.encryption_context = encryption_context
        self.algorithm_suite_id = algorithm_suite_id

    def as_dict(self) -> Dict[str, Any]:
        """Converts the DecryptOutput to a dictionary."""
        return {
            "plaintext": self.plaintext,
            "encryption_context": self.encryption_context,
            "algorithm_suite_id": self.algorithm_suite_id,
        }

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "DecryptOutput":
        """Creates a DecryptOutput from a dictionary."""
        kwargs: Dict[str, Any] = {
            "plaintext": d["plaintext"],
            "encryption_context": d["encryption_context"],
            "algorithm_suite_id": d["algorithm_suite_id"],
        }

        return DecryptOutput(**kwargs)

    def __repr__(self) -> str:
        result = "DecryptOutput("
        if self.plaintext is not None:
            result += f"plaintext={repr(self.plaintext)}, "

        if self.encryption_context is not None:
            result += f"encryption_context={repr(self.encryption_context)}, "

        if self.algorithm_suite_id is not None:
            result += f"algorithm_suite_id={repr(self.algorithm_suite_id)}"

        return result + ")"

    def __eq__(self, other: Any) -> bool:
        if not isinstance(other, DecryptOutput):
            return False
        attributes: list[str] = [
            "plaintext",
            "encryption_context",
            "algorithm_suite_id",
        ]
        return all(getattr(self, a) == getattr(other, a) for a in attributes)


class EncryptInput:
    plaintext: bytes | bytearray
    encryption_context: Optional[dict[str, str]]
    materials_manager: Optional[
        "aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.references.CryptographicMaterialsManager"
    ]
    keyring: Optional[
        "aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.references.Keyring"
    ]
    algorithm_suite_id: Optional[str]
    frame_length: int

    def __init__(
        self,
        *,
        plaintext: bytes | bytearray,
        encryption_context: Optional[dict[str, str]] = None,
        materials_manager: Optional[
            "aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.references.CryptographicMaterialsManager"
        ] = None,
        keyring: Optional[
            "aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.references.Keyring"
        ] = None,
        algorithm_suite_id: Optional[str] = None,
        frame_length: int = 0,
    ):
        self.plaintext = plaintext
        self.encryption_context = encryption_context
        self.materials_manager = materials_manager
        self.keyring = keyring
        self.algorithm_suite_id = algorithm_suite_id
        if (frame_length is not None) and (frame_length < 1):
            raise ValueError("frame_length must be greater than or equal to 1")

        if (frame_length is not None) and (frame_length > 4294967296):
            raise ValueError("frame_length must be less than or equal to 4294967296")

        self.frame_length = frame_length

    def as_dict(self) -> Dict[str, Any]:
        """Converts the EncryptInput to a dictionary."""
        d: Dict[str, Any] = {
            "plaintext": self.plaintext,
        }

        if self.encryption_context is not None:
            d["encryption_context"] = self.encryption_context

        if self.materials_manager is not None:
            d["materials_manager"] = self.materials_manager.as_dict()

        if self.keyring is not None:
            d["keyring"] = self.keyring.as_dict()

        if self.algorithm_suite_id is not None:
            d["algorithm_suite_id"] = self.algorithm_suite_id

        if self.frame_length is not None:
            d["frame_length"] = self.frame_length

        return d

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "EncryptInput":
        """Creates a EncryptInput from a dictionary."""
        from aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.references import (
            CryptographicMaterialsManager,
        )
        from aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.references import (
            Keyring,
        )

        kwargs: Dict[str, Any] = {
            "plaintext": d["plaintext"],
        }

        if "encryption_context" in d:
            kwargs["encryption_context"] = d["encryption_context"]

        if "materials_manager" in d:
            kwargs["materials_manager"] = CryptographicMaterialsManager.from_dict(
                d["materials_manager"]
            )

        if "keyring" in d:
            kwargs["keyring"] = Keyring.from_dict(d["keyring"])

        if "algorithm_suite_id" in d:
            kwargs["algorithm_suite_id"] = d["algorithm_suite_id"]

        if "frame_length" in d:
            kwargs["frame_length"] = d["frame_length"]

        return EncryptInput(**kwargs)

    def __repr__(self) -> str:
        result = "EncryptInput("
        if self.plaintext is not None:
            result += f"plaintext={repr(self.plaintext)}, "

        if self.encryption_context is not None:
            result += f"encryption_context={repr(self.encryption_context)}, "

        if self.materials_manager is not None:
            result += f"materials_manager={repr(self.materials_manager)}, "

        if self.keyring is not None:
            result += f"keyring={repr(self.keyring)}, "

        if self.algorithm_suite_id is not None:
            result += f"algorithm_suite_id={repr(self.algorithm_suite_id)}, "

        if self.frame_length is not None:
            result += f"frame_length={repr(self.frame_length)}"

        return result + ")"

    def __eq__(self, other: Any) -> bool:
        if not isinstance(other, EncryptInput):
            return False
        attributes: list[str] = [
            "plaintext",
            "encryption_context",
            "materials_manager",
            "keyring",
            "algorithm_suite_id",
            "frame_length",
        ]
        return all(getattr(self, a) == getattr(other, a) for a in attributes)


class EncryptOutput:
    ciphertext: bytes | bytearray
    encryption_context: dict[str, str]
    algorithm_suite_id: str

    def __init__(
        self,
        *,
        ciphertext: bytes | bytearray,
        encryption_context: dict[str, str],
        algorithm_suite_id: str,
    ):
        self.ciphertext = ciphertext
        self.encryption_context = encryption_context
        self.algorithm_suite_id = algorithm_suite_id

    def as_dict(self) -> Dict[str, Any]:
        """Converts the EncryptOutput to a dictionary."""
        return {
            "ciphertext": self.ciphertext,
            "encryption_context": self.encryption_context,
            "algorithm_suite_id": self.algorithm_suite_id,
        }

    @staticmethod
    def from_dict(d: Dict[str, Any]) -> "EncryptOutput":
        """Creates a EncryptOutput from a dictionary."""
        kwargs: Dict[str, Any] = {
            "ciphertext": d["ciphertext"],
            "encryption_context": d["encryption_context"],
            "algorithm_suite_id": d["algorithm_suite_id"],
        }

        return EncryptOutput(**kwargs)

    def __repr__(self) -> str:
        result = "EncryptOutput("
        if self.ciphertext is not None:
            result += f"ciphertext={repr(self.ciphertext)}, "

        if self.encryption_context is not None:
            result += f"encryption_context={repr(self.encryption_context)}, "

        if self.algorithm_suite_id is not None:
            result += f"algorithm_suite_id={repr(self.algorithm_suite_id)}"

        return result + ")"

    def __eq__(self, other: Any) -> bool:
        if not isinstance(other, EncryptOutput):
            return False
        attributes: list[str] = [
            "ciphertext",
            "encryption_context",
            "algorithm_suite_id",
        ]
        return all(getattr(self, a) == getattr(other, a) for a in attributes)


class Unit:
    pass
