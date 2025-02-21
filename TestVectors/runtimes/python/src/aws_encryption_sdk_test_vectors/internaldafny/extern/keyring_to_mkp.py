# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

from aws_encryption_sdk.key_providers.raw import RawMasterKeyProvider
from aws_encryption_sdk.identifiers import EncryptionKeyType, WrappingAlgorithm
from aws_encryption_sdk.internal.crypto.wrapping_keys import WrappingKey
from aws_encryption_sdk.internal.str_ops import to_str
from aws_cryptographic_material_providers.internaldafny.generated.RawAESKeyring import RawAESKeyring
from aws_cryptographic_material_providers.internaldafny.generated.AwsKmsKeyring import AwsKmsKeyring
from aws_cryptographic_material_providers.internaldafny.generated.AwsKmsMrkKeyring import AwsKmsMrkKeyring
from aws_cryptographic_material_providers.internaldafny.generated.AwsKmsDiscoveryKeyring import AwsKmsDiscoveryKeyring
from aws_cryptographic_material_providers.internaldafny.generated.AwsKmsMrkDiscoveryKeyring import AwsKmsMrkDiscoveryKeyring
from aws_encryption_sdk.key_providers.kms import (
    MRKAwareDiscoveryAwsKmsMasterKeyProvider,
    MRKAwareStrictAwsKmsMasterKeyProvider,
)
from aws_cryptographic_material_providers.internaldafny.generated.DefaultCMM import DefaultCMM
from aws_cryptographic_material_providers.internaldafny.generated.MultiKeyring import MultiKeyring
from aws_cryptographic_material_providers.internaldafny.generated.RawRSAKeyring import RawRSAKeyring
from aws_cryptographic_material_providers.internaldafny.generated.RawECDHKeyring import RawEcdhKeyring
from aws_cryptographic_material_providers.internaldafny.generated.AwsKmsHierarchicalKeyring import AwsKmsHierarchicalKeyring
from aws_cryptographic_material_providers.internaldafny.generated.AwsKmsRsaKeyring import AwsKmsRsaKeyring
from aws_cryptographic_material_providers.internaldafny.generated.AwsKmsEcdhKeyring import AwsKmsEcdhKeyring
import aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.dafny_to_smithy
import aws_encryption_sdk
class StaticMasterKeyProvider(RawMasterKeyProvider):
    """Generates 256-bit keys for each unique key ID."""

    def __init__(self, **kwargs):  # pylint: disable=unused-argument
        """Initialize empty map of keys."""
        self._static_keys = {}
        self.provider_id = ""

    @property
    def static_keys(self):
        return self._static_keys
    
    # The key namespace in the Raw keyrings is equivalent to Provider ID (or Provider) field
    # in the Raw Master Key Providers
    @property
    def provider_id(self):
        return self._provider_id
    
    @property
    def wrapping_key_type(self):
        return self._wrapping_key_type
    
    @property
    def wrapping_algorithm(self):
        return self._wrapping_algorithm
    
    @provider_id.setter
    def provider_id(self, value):
        self._provider_id = value
    
    @static_keys.setter
    def static_keys(self, static_key_dict):
        self._static_keys = static_key_dict
        
    @wrapping_key_type.setter
    def wrapping_key_type(self, wrapping_key_type):
        self._wrapping_key_type = wrapping_key_type

    @wrapping_key_type.setter
    def wrapping_algorithm(self, wrapping_algorithm):
        self._wrapping_algorithm = wrapping_algorithm

    def _get_raw_key(self, key_id):
        """Returns a static, symmetric key for the specified key ID.

        :param str key_id: Key ID
        :returns: Wrapping key that contains the specified static key
        :rtype: :class:`aws_encryption_sdk.internal.crypto.WrappingKey`
        """
        static_key = self._static_keys[to_str(key_id)] # add_master_key changes it to bytes and we have to use internal to_str
        return WrappingKey(
            wrapping_algorithm=self._wrapping_algorithm,
            wrapping_key=static_key,
            wrapping_key_type=self._wrapping_key_type,
        )
    
def create_raw_aes_key_provider(key_name, key_namespace, key):
    # Create a Raw AES master key provider.

    # The key name in the Raw keyrings is equivalent to the Key ID field
    # in the Raw Master Key Providers
    key_id = key_name
    static_key_map = {key_id: key} 
    key_provider = StaticMasterKeyProvider()
    key_provider.static_keys = static_key_map
    key_provider.provider_id = key_namespace
    key_provider.wrapping_key_type = EncryptionKeyType.SYMMETRIC
    key_provider.wrapping_algorithm = WrappingAlgorithm.AES_256_GCM_IV12_TAG16_NO_PADDING
    key_provider.add_master_key(key_name)

    return key_provider

def materials_manager_to_mkp(materials_manager):
    if not isinstance(materials_manager, DefaultCMM):
        return None
    return keyring_to_mkp(materials_manager.keyring)

def keyring_to_mkp(keyring):
    # TODO: Support Multikey and Raw rsa. 
    keyring_to_mkp_NOT_convertable_types = (MultiKeyring, RawRSAKeyring, RawEcdhKeyring, AwsKmsEcdhKeyring, AwsKmsHierarchicalKeyring, AwsKmsRsaKeyring)
    if isinstance(keyring, keyring_to_mkp_NOT_convertable_types):
        return None
    if (isinstance(keyring, RawAESKeyring)):
        mkp_key_id = bytes(keyring.wrappingKey)
        # The key name in the Raw keyrings is equivalent to the Key ID field
        # in the Raw Master Key Providers
        mkp_key = bytes(keyring.keyName.Elements).decode('utf-8')
        mkpProvider_id = bytes(keyring.keyNamespace.Elements).decode('utf-8')
        return create_raw_aes_key_provider(mkp_key, mkpProvider_id, mkp_key_id)
    
    if (isinstance(keyring, AwsKmsKeyring)):
        aws_kms_arn = string_to_native(keyring.awsKmsKey)
        return aws_encryption_sdk.StrictAwsKmsMasterKeyProvider(key_ids=[
            aws_kms_arn,
        ])
    
    if (isinstance(keyring, AwsKmsMrkKeyring)):
        aws_kms_arn = string_to_native(keyring.awsKmsKey)
        kwargs = dict(key_ids=[aws_kms_arn])
        return MRKAwareStrictAwsKmsMasterKeyProvider(**kwargs)
    
    if (isinstance(keyring, AwsKmsDiscoveryKeyring)):
        if keyring.discoveryFilter.is_Some:
            discovery_filter = aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.dafny_to_smithy.aws_cryptography_materialproviders_DiscoveryFilter(
                keyring.discoveryFilter.UnwrapOr(None)
            )
            kwargs = dict(discovery_filter=discovery_filter)
            aws_encryption_sdk.DiscoveryAwsKmsMasterKeyProvider(**kwargs)
        return aws_encryption_sdk.DiscoveryAwsKmsMasterKeyProvider()
    
    if (isinstance(keyring, AwsKmsMrkDiscoveryKeyring)):
        region = string_to_native(keyring.region)
        if keyring.discoveryFilter.is_Some:
            discovery_filter = aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.dafny_to_smithy.aws_cryptography_materialproviders_DiscoveryFilter(
                keyring.discoveryFilter.UnwrapOr(None)
            )
            kwargs = dict(discovery_filter=discovery_filter, discovery_region=region)
            aws_encryption_sdk.MRKAwareDiscoveryAwsKmsMasterKeyProvider(**kwargs)
        kwargs = dict(discovery_region=region)
        return MRKAwareDiscoveryAwsKmsMasterKeyProvider(**kwargs)
    raise ValueError("No keyring matched to convert to MKP. Input keyring type: "+ str(type(keyring)))
    

def string_to_native(arn):
    return b"".join(
        ord(c).to_bytes(2, "big") for c in arn
    ).decode("utf-16-be")