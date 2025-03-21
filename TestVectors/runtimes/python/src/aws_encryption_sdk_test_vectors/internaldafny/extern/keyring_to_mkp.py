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
    DiscoveryAwsKmsMasterKeyProvider,
    DiscoveryFilter
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
    def wrapping_algorithm(self):
        return self._wrapping_algorithm
    
    @property
    def encryptionKeyType(self):
        return self._encryptionKeyType
    
    @provider_id.setter
    def provider_id(self, value):
        self._provider_id = value
    
    @static_keys.setter
    def static_keys(self, static_key_dict):
        self._static_keys = static_key_dict
        
    @wrapping_algorithm.setter
    def wrapping_algorithm(self, wrapping_algorithm):
        self._wrapping_algorithm = wrapping_algorithm

    @encryptionKeyType.setter
    def encryptionKeyType(self, encryptionKeyType):
        self._encryptionKeyType = encryptionKeyType

    def _get_raw_key(self, key_id):
        """Returns a static, symmetric key for the specified key ID.

        :param str key_id: Key ID
        :returns: Wrapping key that contains the specified static key
        :rtype: :class:`aws_encryption_sdk.internal.crypto.WrappingKey`
        """
        key_id_str = to_str(key_id)
        static_key = self._static_keys[key_id_str] # add_master_key changes it to bytes and we have to use internal to_str
        return WrappingKey(
            wrapping_algorithm=self._wrapping_algorithm,
            wrapping_key=static_key,
            wrapping_key_type=self._encryptionKeyType,
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
    key_provider.encryptionKeyType = EncryptionKeyType.SYMMETRIC
    key_provider.wrapping_algorithm = WrappingAlgorithm.AES_256_GCM_IV12_TAG16_NO_PADDING
    key_provider.add_master_key(key_name)

    return key_provider

def create_raw_rsa_key_provider(key_name, key_namespace, public_key, private_key, wrapping_algorithm):
    # Create a Raw RSA master key provider.

    # The key name in the Raw keyrings is equivalent to the Key ID field
    # in the Raw Master Key Providers
    if private_key is not None:
        static_key_map = {
            key_name: private_key
        }
    elif public_key is not None:
        static_key_map = {
            key_name: public_key
        }
    else:
        raise ValueError("Either public or private key must be provided")
    key_provider = StaticMasterKeyProvider()
    key_provider.static_keys = static_key_map
    key_provider.provider_id = key_namespace
   
    key_provider.wrapping_algorithm = wrapping_algorithm
    if private_key is not None:
        key_provider.encryptionKeyType = EncryptionKeyType.PRIVATE
        key_provider.add_master_key(key_name)
    elif public_key is not None:
        key_provider.encryptionKeyType = EncryptionKeyType.PUBLIC
        key_provider.add_master_key(key_name)
    else:
        # This redundant else block will never be reached but still keeping it to be "extra safe".
        raise ValueError("Either public or private key must be provided")
    return key_provider

def materials_manager_to_mkp(materials_manager):
    """
    Fetches the underlying keyring from materials manager and converts it to a Master Key Provider (MKP) if it's a DefaultCMM.

    Args:
        materials_manager: The cryptographic materials manager to convert.

    Returns:
        MasterKeyProvider or None: Returns None if the materials_manager is not a DefaultCMM.
                                  Otherwise, returns the result of converting the manager's
                                  keyring to a Master Key Provider using keyring_to_mkp.

    Note:
        The calling function is expected to check for None and handle the case appropriately if needed.
    """
    if not isinstance(materials_manager, DefaultCMM):
        return None
    return keyring_to_mkp(materials_manager.keyring)

def keyring_to_mkp(keyring):
    """
    Converts a keyring to a Master Key Provider (MKP), with specific exclusions for keyring without equivalent MKP type.

    Args:
        keyring: The keyring object to be converted to a Master Key Provider.

    Returns:
        MasterKeyProvider or None: Returns None if the keyring is of an unconvertible type
                                  (RawEcdhKeyring, AwsKmsEcdhKeyring, AwsKmsHierarchicalKeyring,
                                  or AwsKmsRsaKeyring). Otherwise, returns the converted
                                  Master Key Provider.
    
    Note:
        The calling function is expected to check for None and handle the case appropriately if needed.
    """
    keyring_to_mkp_NOT_convertable_types = (RawEcdhKeyring, AwsKmsEcdhKeyring, AwsKmsHierarchicalKeyring, AwsKmsRsaKeyring)
    if isinstance(keyring, keyring_to_mkp_NOT_convertable_types):
        return None
    if (isinstance(keyring, RawAESKeyring)):
        mkp_key_id = bytes(keyring.wrappingKey)
        # The key name in the Raw keyrings is equivalent to the Key ID field
        # in the Raw Master Key Providers
        mkp_key = bytes(keyring.keyName.Elements).decode('utf-8')
        mkpProvider_id = bytes(keyring.keyNamespace.Elements).decode('utf-8')
        return create_raw_aes_key_provider(mkp_key, mkpProvider_id, mkp_key_id)
    
    if (isinstance(keyring, RawRSAKeyring)):
        public_key = keyring.publicKey.UnwrapOr(None)
        private_key = keyring.privateKey.UnwrapOr(None)
        if public_key is not None:
            public_key = bytes(public_key)
        if private_key is not None:
            private_key = bytes(private_key)
        # The key name in the Raw keyrings is equivalent to the Key ID field
        # in the Raw Master Key Providers
        mkp_key_name = bytes(keyring.keyName.Elements).decode('utf-8')
        mkpProvider_id = bytes(keyring.keyNamespace.Elements).decode('utf-8')
        return create_raw_rsa_key_provider(mkp_key_name, mkpProvider_id, public_key, private_key, get_rsa_wrapping_alg(keyring.paddingScheme))
    
    if (isinstance(keyring, MultiKeyring)):
        child_master_key_providers = []
        generator_keyring_mkp = None
        if keyring.generatorKeyring.is_Some:
            generator_keyring_mkp = keyring_to_mkp(keyring.generatorKeyring.UnwrapOr(None))
        for child_keyring in keyring.childKeyrings:
            child_keyring = keyring_to_mkp(child_keyring)
            # keyring_to_mkp will return None if there is no equivalent keyring type in MKP
            if (child_keyring is not None):
                child_master_key_providers.append(child_keyring)

        if generator_keyring_mkp is not None:
            generator_keyring_mkp.add_master_key_providers_from_list(child_master_key_providers)
            return generator_keyring_mkp
        elif len(child_master_key_providers) != 0:
            child_mkp = child_master_key_providers.pop(0)
            child_mkp.add_master_key_providers_from_list(child_master_key_providers)
            return child_mkp
        else:
            raise ValueError("Both generator and child keyrings are not present in MultiKeyring.")
    
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
            native_discovery_filter = aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.dafny_to_smithy.aws_cryptography_materialproviders_DiscoveryFilter(
                keyring.discoveryFilter.UnwrapOr(None)
            )
            discovery_filter = DiscoveryFilter(
                account_ids=native_discovery_filter.account_ids,
                partition=native_discovery_filter.partition
            )
            kwargs = dict(discovery_filter=discovery_filter)
            return DiscoveryAwsKmsMasterKeyProvider(**kwargs)
        return DiscoveryAwsKmsMasterKeyProvider()
    
    if (isinstance(keyring, AwsKmsMrkDiscoveryKeyring)):
        region = string_to_native(keyring.region)
        if keyring.discoveryFilter.is_Some:
            native_discovery_filter = aws_cryptographic_material_providers.smithygenerated.aws_cryptography_materialproviders.dafny_to_smithy.aws_cryptography_materialproviders_DiscoveryFilter(
                keyring.discoveryFilter.UnwrapOr(None)
            )
            discovery_filter = DiscoveryFilter(
                account_ids=native_discovery_filter.account_ids,
                partition=native_discovery_filter.partition
            )
            kwargs = dict(discovery_filter=discovery_filter, discovery_region=region)
            return MRKAwareDiscoveryAwsKmsMasterKeyProvider(**kwargs)
        kwargs = dict(discovery_region=region)
        return MRKAwareDiscoveryAwsKmsMasterKeyProvider(**kwargs)
    raise ValueError("No keyring matched to convert to MKP. Input keyring type: "+ str(type(keyring)))
    

def string_to_native(arn):
    return b"".join(
        ord(c).to_bytes(2, "big") for c in arn
    ).decode("utf-16-be")

def get_rsa_wrapping_alg(rsa_padding_mode):
    if (rsa_padding_mode.is_PKCS1):
        return WrappingAlgorithm.RSA_PKCS1
    elif (rsa_padding_mode.is_OAEP__SHA1):
        return WrappingAlgorithm.RSA_OAEP_SHA1_MGF1
    elif (rsa_padding_mode.is_OAEP__SHA256):
        return WrappingAlgorithm.RSA_OAEP_SHA256_MGF1
    elif (rsa_padding_mode.is_OAEP__SHA384):
        return WrappingAlgorithm.RSA_OAEP_SHA384_MGF1
    elif (rsa_padding_mode.is_OAEP__SHA512):
        return WrappingAlgorithm.RSA_OAEP_SHA512_MGF1
    else:
        raise ValueError("Unsupported RSA padding scheme: " + str(rsa_padding_mode))