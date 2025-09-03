#!/usr/bin/env python3
"""
Core benchmark module for ESDK Python benchmark
"""

import logging
import multiprocessing
import secrets
import sys

import psutil
from aws_cryptographic_material_providers.mpl import AwsCryptographicMaterialProviders
from aws_cryptographic_material_providers.mpl.config import MaterialProvidersConfig
from aws_cryptographic_material_providers.mpl.models import (
    AesWrappingAlg,
    CreateRawAesKeyringInput,
)
from aws_encryption_sdk import EncryptionSDKClient, CommitmentPolicy
from config import load_config


class ESDKBenchmark:
    """Main benchmark class for ESDK Python performance testing"""

    def __init__(self, config_path: str = "../../config/test-scenarios.yaml"):
        self.config = load_config(config_path)
        self.results = []

        self._setup_logging()
        self._setup_esdk()
        self._setup_system_info()

    def _setup_system_info(self):
        """Initialize system information"""
        self.cpu_count = multiprocessing.cpu_count()
        self.total_memory_gb = psutil.virtual_memory().total / (1024**3)

        self.logger.info(
            f"Initialized ESDK Benchmark - CPU cores: {self.cpu_count}, "
            f"Memory: {self.total_memory_gb:.1f}GB"
        )

    def _setup_logging(self):
        """Setup logging configuration"""
        logging.basicConfig(
            level=logging.INFO,
            format="%(message)s",
            handlers=[logging.StreamHandler(sys.stdout)],
        )
        # Suppress AWS SDK logging
        logging.getLogger("aws_encryption_sdk").setLevel(logging.WARNING)
        logging.getLogger("botocore").setLevel(logging.WARNING)
        logging.getLogger("boto3").setLevel(logging.WARNING)

        self.logger = logging.getLogger(__name__)

    def _setup_esdk(self):
        """Initialize ESDK client and raw AES keyring"""
        try:
            self.keyring = self._create_keyring()
            self.esdk_client = self._create_client()
            self.logger.info("ESDK client initialized successfully")
        except Exception as e:
            self.logger.error(f"Failed to initialize ESDK: {e}")
            raise

    def _create_keyring(self):
        """Create raw AES keyring"""
        static_key = secrets.token_bytes(32)
        mat_prov = AwsCryptographicMaterialProviders(config=MaterialProvidersConfig())

        keyring_input = CreateRawAesKeyringInput(
            key_namespace="esdk-performance-test",
            key_name="test-aes-256-key",
            wrapping_key=static_key,
            wrapping_alg=AesWrappingAlg.ALG_AES256_GCM_IV12_TAG16,
        )

        return mat_prov.create_raw_aes_keyring(input=keyring_input)

    def _create_client(self):
        """Create ESDK client"""
        return EncryptionSDKClient(
            commitment_policy=CommitmentPolicy.REQUIRE_ENCRYPT_REQUIRE_DECRYPT
        )

    def should_run_test_type(self, test_type: str, is_quick_mode: bool = False) -> bool:
        """Determine if a test type should be run based on configuration"""
        if is_quick_mode:
            quick_config = self.config.get("quick_config")
            if quick_config and "test_types" in quick_config:
                return test_type in quick_config["test_types"]
        return True
