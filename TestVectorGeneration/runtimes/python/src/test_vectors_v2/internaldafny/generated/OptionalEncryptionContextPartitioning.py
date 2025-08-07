import sys
from typing import Callable, Any, TypeVar, NamedTuple
from math import floor
from itertools import count

import test_vectors_v2.internaldafny.generated.module_ as module_
import _dafny as _dafny
import System_ as System_
import smithy_dafny_standard_library.internaldafny.generated.Wrappers as Wrappers
import smithy_dafny_standard_library.internaldafny.generated.BoundedInts as BoundedInts
import smithy_dafny_standard_library.internaldafny.generated.StandardLibrary_UInt as StandardLibrary_UInt
import smithy_dafny_standard_library.internaldafny.generated.StandardLibrary_MemoryMath as StandardLibrary_MemoryMath
import smithy_dafny_standard_library.internaldafny.generated.StandardLibrary_Sequence as StandardLibrary_Sequence
import smithy_dafny_standard_library.internaldafny.generated.StandardLibrary_String as StandardLibrary_String
import smithy_dafny_standard_library.internaldafny.generated.StandardLibrary as StandardLibrary
import smithy_dafny_standard_library.internaldafny.generated.UTF8 as UTF8
import aws_cryptography_primitives.internaldafny.generated.AwsCryptographyPrimitivesTypes as AwsCryptographyPrimitivesTypes
import aws_cryptography_primitives.internaldafny.generated.ExternRandom as ExternRandom
import aws_cryptography_primitives.internaldafny.generated.Random as Random
import aws_cryptography_primitives.internaldafny.generated.AESEncryption as AESEncryption
import aws_cryptography_primitives.internaldafny.generated.ExternDigest as ExternDigest
import aws_cryptography_primitives.internaldafny.generated.Digest as Digest
import aws_cryptography_primitives.internaldafny.generated.HMAC as HMAC
import aws_cryptography_primitives.internaldafny.generated.WrappedHMAC as WrappedHMAC
import aws_cryptography_primitives.internaldafny.generated.HKDF as HKDF
import aws_cryptography_primitives.internaldafny.generated.WrappedHKDF as WrappedHKDF
import aws_cryptography_primitives.internaldafny.generated.Signature as Signature
import aws_cryptography_primitives.internaldafny.generated.KdfCtr as KdfCtr
import aws_cryptography_primitives.internaldafny.generated.RSAEncryption as RSAEncryption
import aws_cryptography_primitives.internaldafny.generated.ECDH as ECDH
import aws_cryptography_primitives.internaldafny.generated.AwsCryptographyPrimitivesOperations as AwsCryptographyPrimitivesOperations
import aws_cryptography_primitives.internaldafny.generated.AtomicPrimitives as AtomicPrimitives
import aws_cryptography_internal_dynamodb.internaldafny.generated.ComAmazonawsDynamodbTypes as ComAmazonawsDynamodbTypes
import aws_cryptography_internal_kms.internaldafny.generated.ComAmazonawsKmsTypes as ComAmazonawsKmsTypes
import aws_cryptographic_material_providers.internaldafny.generated.AwsCryptographyKeyStoreTypes as AwsCryptographyKeyStoreTypes
import aws_cryptographic_material_providers.internaldafny.generated.AwsCryptographyMaterialProvidersTypes as AwsCryptographyMaterialProvidersTypes
import smithy_dafny_standard_library.internaldafny.generated.Base64 as Base64
import aws_cryptographic_material_providers.internaldafny.generated.AlgorithmSuites as AlgorithmSuites
import aws_cryptographic_material_providers.internaldafny.generated.Materials as Materials
import aws_cryptographic_material_providers.internaldafny.generated.Keyring as Keyring
import smithy_dafny_standard_library.internaldafny.generated.Relations as Relations
import smithy_dafny_standard_library.internaldafny.generated.Seq_MergeSort as Seq_MergeSort
import smithy_dafny_standard_library.internaldafny.generated.Math as Math
import smithy_dafny_standard_library.internaldafny.generated.Seq as Seq
import smithy_dafny_standard_library.internaldafny.generated.Actions as Actions
import smithy_dafny_standard_library.internaldafny.generated.SortedSets as SortedSets
import aws_cryptographic_material_providers.internaldafny.generated.CanonicalEncryptionContext as CanonicalEncryptionContext
import aws_cryptographic_material_providers.internaldafny.generated.MaterialWrapping as MaterialWrapping
import aws_cryptographic_material_providers.internaldafny.generated.IntermediateKeyWrapping as IntermediateKeyWrapping
import aws_cryptographic_material_providers.internaldafny.generated.EdkWrapping as EdkWrapping
import smithy_dafny_standard_library.internaldafny.generated.UUID as UUID
import aws_cryptographic_material_providers.internaldafny.generated.ErrorMessages as ErrorMessages
import aws_cryptographic_material_providers.internaldafny.generated.RawAESKeyring as RawAESKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsArnParsing as AwsArnParsing
import aws_cryptographic_material_providers.internaldafny.generated.Constants as Constants
import aws_cryptographic_material_providers.internaldafny.generated.EcdhEdkWrapping as EcdhEdkWrapping
import aws_cryptographic_material_providers.internaldafny.generated.RawECDHKeyring as RawECDHKeyring
import aws_cryptographic_material_providers.internaldafny.generated.RawRSAKeyring as RawRSAKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsMrkMatchForDecrypt as AwsKmsMrkMatchForDecrypt
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsUtils as AwsKmsUtils
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsKeyring as AwsKmsKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsDiscoveryKeyring as AwsKmsDiscoveryKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsEcdhKeyring as AwsKmsEcdhKeyring
import smithy_dafny_standard_library.internaldafny.generated.DafnyLibraries as DafnyLibraries
import smithy_dafny_standard_library.internaldafny.generated.OsLang as OsLang
import smithy_dafny_standard_library.internaldafny.generated.FileIO as FileIO
import smithy_dafny_standard_library.internaldafny.generated.Time as Time
import aws_cryptographic_material_providers.internaldafny.generated.LocalCMC as LocalCMC
import aws_cryptographic_material_providers.internaldafny.generated.SynchronizedLocalCMC as SynchronizedLocalCMC
import aws_cryptographic_material_providers.internaldafny.generated.StormTracker as StormTracker
import aws_cryptographic_material_providers.internaldafny.generated.StormTrackingCMC as StormTrackingCMC
import aws_cryptographic_material_providers.internaldafny.generated.CacheConstants as CacheConstants
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsHierarchicalKeyring as AwsKmsHierarchicalKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsMrkDiscoveryKeyring as AwsKmsMrkDiscoveryKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsMrkKeyring as AwsKmsMrkKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsRsaKeyring as AwsKmsRsaKeyring
import aws_cryptographic_material_providers.internaldafny.generated.MultiKeyring as MultiKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsMrkAreUnique as AwsKmsMrkAreUnique
import aws_cryptographic_material_providers.internaldafny.generated.StrictMultiKeyring as StrictMultiKeyring
import aws_cryptography_internal_kms.internaldafny.generated.Com_Amazonaws_Kms as Com_Amazonaws_Kms
import aws_cryptography_internal_dynamodb.internaldafny.generated.Com_Amazonaws_Dynamodb as Com_Amazonaws_Dynamodb
import aws_cryptographic_material_providers.internaldafny.generated.DiscoveryMultiKeyring as DiscoveryMultiKeyring
import aws_cryptographic_material_providers.internaldafny.generated.MrkAwareDiscoveryMultiKeyring as MrkAwareDiscoveryMultiKeyring
import aws_cryptographic_material_providers.internaldafny.generated.MrkAwareStrictMultiKeyring as MrkAwareStrictMultiKeyring
import aws_cryptographic_material_providers.internaldafny.generated.CMM as CMM
import aws_cryptographic_material_providers.internaldafny.generated.Defaults as Defaults
import aws_cryptographic_material_providers.internaldafny.generated.Commitment as Commitment
import aws_cryptographic_material_providers.internaldafny.generated.DefaultCMM as DefaultCMM
import aws_cryptographic_material_providers.internaldafny.generated.DefaultClientSupplier as DefaultClientSupplier
import aws_cryptographic_material_providers.internaldafny.generated.Utils as Utils
import aws_cryptographic_material_providers.internaldafny.generated.RequiredEncryptionContextCMM as RequiredEncryptionContextCMM
import aws_cryptographic_material_providers.internaldafny.generated.AwsCryptographyMaterialProvidersOperations as AwsCryptographyMaterialProvidersOperations
import aws_cryptographic_material_providers.internaldafny.generated.MaterialProviders as MaterialProviders
import aws_cryptographic_material_providers.internaldafny.generated.KeyStoreErrorMessages as KeyStoreErrorMessages
import aws_cryptographic_material_providers.internaldafny.generated.KmsArn as KmsArn
import aws_cryptographic_material_providers.internaldafny.generated.Structure as Structure
import aws_cryptographic_material_providers.internaldafny.generated.KMSKeystoreOperations as KMSKeystoreOperations
import aws_cryptographic_material_providers.internaldafny.generated.DDBKeystoreOperations as DDBKeystoreOperations
import aws_cryptographic_material_providers.internaldafny.generated.CreateKeys as CreateKeys
import aws_cryptographic_material_providers.internaldafny.generated.CreateKeyStoreTable as CreateKeyStoreTable
import aws_cryptographic_material_providers.internaldafny.generated.GetKeys as GetKeys
import aws_cryptographic_material_providers.internaldafny.generated.AwsCryptographyKeyStoreOperations as AwsCryptographyKeyStoreOperations
import aws_cryptographic_material_providers.internaldafny.generated.KeyStore as KeyStore
import aws_encryption_sdk_dafny.internaldafny.generated.AwsCryptographyEncryptionSdkTypes as AwsCryptographyEncryptionSdkTypes
import smithy_dafny_standard_library.internaldafny.generated.Streams as Streams
import aws_encryption_sdk_dafny.internaldafny.generated.SerializableTypes as SerializableTypes
import aws_encryption_sdk_dafny.internaldafny.generated.SerializeFunctions as SerializeFunctions
import aws_encryption_sdk_dafny.internaldafny.generated.EncryptionContext as EncryptionContext
import aws_encryption_sdk_dafny.internaldafny.generated.HeaderTypes as HeaderTypes
import aws_encryption_sdk_dafny.internaldafny.generated.SharedHeaderFunctions as SharedHeaderFunctions
import aws_encryption_sdk_dafny.internaldafny.generated.EncryptedDataKeys as EncryptedDataKeys
import aws_encryption_sdk_dafny.internaldafny.generated.V1HeaderBody as V1HeaderBody
import aws_encryption_sdk_dafny.internaldafny.generated.V2HeaderBody as V2HeaderBody
import aws_encryption_sdk_dafny.internaldafny.generated.HeaderAuth as HeaderAuth
import aws_encryption_sdk_dafny.internaldafny.generated.Header as Header
import aws_encryption_sdk_dafny.internaldafny.generated.Frames as Frames
import aws_encryption_sdk_dafny.internaldafny.generated.MessageBody as MessageBody
import aws_encryption_sdk_dafny.internaldafny.generated.KeyDerivation as KeyDerivation
import aws_encryption_sdk_dafny.internaldafny.generated.EncryptDecryptHelpers as EncryptDecryptHelpers
import aws_encryption_sdk_dafny.internaldafny.generated.AwsEncryptionSdkOperations as AwsEncryptionSdkOperations
import aws_encryption_sdk_dafny.internaldafny.generated.ESDK as ESDK
import aws_cryptography_primitives.internaldafny.generated.AesKdfCtr as AesKdfCtr
import smithy_dafny_standard_library.internaldafny.generated.Unicode as Unicode
import smithy_dafny_standard_library.internaldafny.generated.Functions as Functions
import smithy_dafny_standard_library.internaldafny.generated.Utf8EncodingForm as Utf8EncodingForm
import smithy_dafny_standard_library.internaldafny.generated.Utf16EncodingForm as Utf16EncodingForm
import smithy_dafny_standard_library.internaldafny.generated.UnicodeStrings as UnicodeStrings
import smithy_dafny_standard_library.internaldafny.generated.GeneralInternals as GeneralInternals
import smithy_dafny_standard_library.internaldafny.generated.MulInternalsNonlinear as MulInternalsNonlinear
import smithy_dafny_standard_library.internaldafny.generated.MulInternals as MulInternals
import smithy_dafny_standard_library.internaldafny.generated.Mul as Mul
import smithy_dafny_standard_library.internaldafny.generated.ModInternalsNonlinear as ModInternalsNonlinear
import smithy_dafny_standard_library.internaldafny.generated.DivInternalsNonlinear as DivInternalsNonlinear
import smithy_dafny_standard_library.internaldafny.generated.ModInternals as ModInternals
import smithy_dafny_standard_library.internaldafny.generated.DivInternals as DivInternals
import smithy_dafny_standard_library.internaldafny.generated.DivMod as DivMod
import smithy_dafny_standard_library.internaldafny.generated.Power as Power
import smithy_dafny_standard_library.internaldafny.generated.Logarithm as Logarithm
import smithy_dafny_standard_library.internaldafny.generated.StandardLibraryInterop as StandardLibraryInterop
import smithy_dafny_standard_library.internaldafny.generated.Sorting as Sorting
import smithy_dafny_standard_library.internaldafny.generated.HexStrings as HexStrings
import smithy_dafny_standard_library.internaldafny.generated.GetOpt as GetOpt
import smithy_dafny_standard_library.internaldafny.generated.FloatCompare as FloatCompare
import smithy_dafny_standard_library.internaldafny.generated.ConcurrentCall as ConcurrentCall
import smithy_dafny_standard_library.internaldafny.generated.Base64Lemmas as Base64Lemmas
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Views_Core as JSON_Utils_Views_Core
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Views_Writers as JSON_Utils_Views_Writers
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Lexers_Core as JSON_Utils_Lexers_Core
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Lexers_Strings as JSON_Utils_Lexers_Strings
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Cursors as JSON_Utils_Cursors
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Parsers as JSON_Utils_Parsers
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Str_CharStrConversion as JSON_Utils_Str_CharStrConversion
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Str_CharStrEscaping as JSON_Utils_Str_CharStrEscaping
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Str as JSON_Utils_Str
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Seq as JSON_Utils_Seq
import smithy_dafny_standard_library.internaldafny.generated.JSON_Utils_Vectors as JSON_Utils_Vectors
import smithy_dafny_standard_library.internaldafny.generated.JSON_Errors as JSON_Errors
import smithy_dafny_standard_library.internaldafny.generated.JSON_Values as JSON_Values
import smithy_dafny_standard_library.internaldafny.generated.JSON_Spec as JSON_Spec
import smithy_dafny_standard_library.internaldafny.generated.JSON_Grammar as JSON_Grammar
import smithy_dafny_standard_library.internaldafny.generated.JSON_Serializer_ByteStrConversion as JSON_Serializer_ByteStrConversion
import smithy_dafny_standard_library.internaldafny.generated.JSON_Serializer as JSON_Serializer
import smithy_dafny_standard_library.internaldafny.generated.JSON_Deserializer_Uint16StrConversion as JSON_Deserializer_Uint16StrConversion
import smithy_dafny_standard_library.internaldafny.generated.JSON_Deserializer_ByteStrConversion as JSON_Deserializer_ByteStrConversion
import smithy_dafny_standard_library.internaldafny.generated.JSON_Deserializer as JSON_Deserializer
import smithy_dafny_standard_library.internaldafny.generated.JSON_ConcreteSyntax_Spec as JSON_ConcreteSyntax_Spec
import smithy_dafny_standard_library.internaldafny.generated.JSON_ConcreteSyntax_SpecProperties as JSON_ConcreteSyntax_SpecProperties
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Serializer as JSON_ZeroCopy_Serializer
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Core as JSON_ZeroCopy_Deserializer_Core
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Strings as JSON_ZeroCopy_Deserializer_Strings
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Numbers as JSON_ZeroCopy_Deserializer_Numbers
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_ObjectParams as JSON_ZeroCopy_Deserializer_ObjectParams
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Objects as JSON_ZeroCopy_Deserializer_Objects
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_ArrayParams as JSON_ZeroCopy_Deserializer_ArrayParams
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Arrays as JSON_ZeroCopy_Deserializer_Arrays
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Constants as JSON_ZeroCopy_Deserializer_Constants
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_Values as JSON_ZeroCopy_Deserializer_Values
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer_API as JSON_ZeroCopy_Deserializer_API
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_Deserializer as JSON_ZeroCopy_Deserializer
import smithy_dafny_standard_library.internaldafny.generated.JSON_ZeroCopy_API as JSON_ZeroCopy_API
import smithy_dafny_standard_library.internaldafny.generated.JSON_API as JSON_API
import test_vectors_v2.internaldafny.generated.TestVectorGeneration as TestVectorGeneration
import test_vectors_v2.internaldafny.generated.EmptyStringPartition as EmptyStringPartition
import test_vectors_v2.internaldafny.generated.NonemptyStringPartition as NonemptyStringPartition
import test_vectors_v2.internaldafny.generated.StringLengthPartitioning as StringLengthPartitioning
import test_vectors_v2.internaldafny.generated.AsciiPartition as AsciiPartition
import test_vectors_v2.internaldafny.generated.NonAsciiPartition as NonAsciiPartition
import test_vectors_v2.internaldafny.generated.BasicStringPartitioning as BasicStringPartitioning
import test_vectors_v2.internaldafny.generated.OptionStringNone as OptionStringNone
import test_vectors_v2.internaldafny.generated.OptionStringSome as OptionStringSome
import test_vectors_v2.internaldafny.generated.OptionStringPartitioning as OptionStringPartitioning
import test_vectors_v2.internaldafny.generated.Aes128WrappingKeyPartition as Aes128WrappingKeyPartition
import test_vectors_v2.internaldafny.generated.Aes192WrappingKeyPartition as Aes192WrappingKeyPartition
import test_vectors_v2.internaldafny.generated.Aes256WrappingKeyPartition as Aes256WrappingKeyPartition
import test_vectors_v2.internaldafny.generated.ValidAesWrappingKeyPartition as ValidAesWrappingKeyPartition
import test_vectors_v2.internaldafny.generated.InvalidAesWrappingKeyPartition as InvalidAesWrappingKeyPartition
import test_vectors_v2.internaldafny.generated.AesWrappingKeyPartitioningScheme as AesWrappingKeyPartitioningScheme
import test_vectors_v2.internaldafny.generated.WrappingAlgValues as WrappingAlgValues
import test_vectors_v2.internaldafny.generated.InvalidRawAesKeyNamespacePartition as InvalidRawAesKeyNamespacePartition
import test_vectors_v2.internaldafny.generated.ValidRawAesKeyNamespacePartition as ValidRawAesKeyNamespacePartition
import test_vectors_v2.internaldafny.generated.KeyNamespacePartitioningScheme as KeyNamespacePartitioningScheme
import test_vectors_v2.internaldafny.generated.KeyNamePartitioningScheme as KeyNamePartitioningScheme
import test_vectors_v2.internaldafny.generated.InvalidAes128PruningConfiguration as InvalidAes128PruningConfiguration
import test_vectors_v2.internaldafny.generated.InvalidAes192PruningConfiguration as InvalidAes192PruningConfiguration
import test_vectors_v2.internaldafny.generated.InvalidAes256PruningConfiguration as InvalidAes256PruningConfiguration
import test_vectors_v2.internaldafny.generated.CreateRawAesKeyringInput as CreateRawAesKeyringInput
import test_vectors_v2.internaldafny.generated.CreateRawAesKeyringSmokeTest as CreateRawAesKeyringSmokeTest
import test_vectors_v2.internaldafny.generated.CreateRawAesKeyringTestService as CreateRawAesKeyringTestService
import test_vectors_v2.internaldafny.generated.CreateRawAesKeyringInputTests as CreateRawAesKeyringInputTests
import test_vectors_v2.internaldafny.generated.EmptyPlaintextPartitioning as EmptyPlaintextPartitioning
import test_vectors_v2.internaldafny.generated.NonemptyPlaintextPartitioning as NonemptyPlaintextPartitioning
import test_vectors_v2.internaldafny.generated.PlaintextPartitioning as PlaintextPartitioning
import test_vectors_v2.internaldafny.generated.EmptyEncryptionContextPartition as EmptyEncryptionContextPartition
import test_vectors_v2.internaldafny.generated.NomemptyEncryptionContextPartition as NomemptyEncryptionContextPartition
import test_vectors_v2.internaldafny.generated.EncryptionContextPartitioning as EncryptionContextPartitioning
import test_vectors_v2.internaldafny.generated.ValidKeyIdentifierPruningConfiguration as ValidKeyIdentifierPruningConfiguration
import test_vectors_v2.internaldafny.generated.InvalidKeyIdentifierPruningConfiguration as InvalidKeyIdentifierPruningConfiguration
import test_vectors_v2.internaldafny.generated.AwsCryptographyEncryptionSdkTestRoundtripTypes as AwsCryptographyEncryptionSdkTestRoundtripTypes
import test_vectors_v2.internaldafny.generated.KeyringPartitioning as KeyringPartitioning
import test_vectors_v2.internaldafny.generated.ESDKAlgorithmSuitePartitioning as ESDKAlgorithmSuitePartitioning
import test_vectors_v2.internaldafny.generated.MinimumFrameLengthPartition as MinimumFrameLengthPartition
import test_vectors_v2.internaldafny.generated.NonminimumFrameLengthPartition as NonminimumFrameLengthPartition
import test_vectors_v2.internaldafny.generated.FrameLengthPartitioning as FrameLengthPartitioning
import test_vectors_v2.internaldafny.generated.OptionalFrameLengthNonePartitioning as OptionalFrameLengthNonePartitioning
import test_vectors_v2.internaldafny.generated.OptionalFrameLengthSomePartitioning as OptionalFrameLengthSomePartitioning
import test_vectors_v2.internaldafny.generated.OptionalFrameLengthPartitioning as OptionalFrameLengthPartitioning
import test_vectors_v2.internaldafny.generated.OptionNoneEncryptionContextPartitioning as OptionNoneEncryptionContextPartitioning
import test_vectors_v2.internaldafny.generated.OptionSomeEncryptionContextPartitioning as OptionSomeEncryptionContextPartitioning

# Module: OptionalEncryptionContextPartitioning

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def PartitionMemberships(x):
        return _dafny.Seq([OptionNoneEncryptionContextPartitioning.default__.IsValidMember(x), OptionSomeEncryptionContextPartitioning.default__.IsValidMember(x)])

    @staticmethod
    def RepresentativeValues():
        output: _dafny.Seq = _dafny.Seq({})
        d_0_values1_: _dafny.Seq
        out0_: _dafny.Seq
        out0_ = OptionNoneEncryptionContextPartitioning.default__.GetValues()
        d_0_values1_ = out0_
        d_1_values2_: _dafny.Seq
        out1_: _dafny.Seq
        out1_ = OptionSomeEncryptionContextPartitioning.default__.GetValues()
        d_1_values2_ = out1_
        output = (d_0_values1_) + (d_1_values2_)
        return output

    @staticmethod
    def EncryptionContextToJson(key, m):
        def lambda0_(d_1_a_, d_2_b_):
            return (d_1_a_) < (d_2_b_)

        d_0_keys_ = SortedSets.default__.SetToOrderedSequence2((m).keys, lambda0_)
        def lambda1_(d_4_m_):
            def lambda2_(d_5_k_):
                def iife0_(_pat_let0_0):
                    def iife1_(d_6_valueOrError1_):
                        def iife2_(_pat_let1_0):
                            def iife3_(d_7_key_):
                                def iife4_(_pat_let2_0):
                                    def iife5_(d_8_valueOrError2_):
                                        def iife6_(_pat_let3_0):
                                            def iife7_(d_9_value_):
                                                return Wrappers.Result_Success((d_7_key_, JSON_Values.JSON_String(d_9_value_)))
                                            return iife7_(_pat_let3_0)
                                        return ((d_8_valueOrError2_).PropagateFailure() if (d_8_valueOrError2_).IsFailure() else iife6_((d_8_valueOrError2_).Extract()))
                                    return iife5_(_pat_let2_0)
                                return iife4_(UTF8.default__.Decode((d_4_m_)[d_5_k_]))
                            return iife3_(_pat_let1_0)
                        return ((d_6_valueOrError1_).PropagateFailure() if (d_6_valueOrError1_).IsFailure() else iife2_((d_6_valueOrError1_).Extract()))
                    return iife1_(_pat_let0_0)
                return iife0_(UTF8.default__.Decode(d_5_k_))

            return lambda2_

        d_3_valueOrError0_ = Seq.default__.MapWithResult(lambda1_(m), d_0_keys_)
        if (d_3_valueOrError0_).IsFailure():
            return (d_3_valueOrError0_).PropagateFailure()
        elif True:
            d_10_pairsBytes_ = (d_3_valueOrError0_).Extract()
            return Wrappers.Result_Success(_dafny.Seq([(key, JSON_Values.JSON_Object(d_10_pairsBytes_))]))

    @staticmethod
    def ToJSON(x):
        if (x).is_None:
            return JSON_Values.JSON_Object(_dafny.Seq([(_dafny.Seq("None"), JSON_Values.JSON_Null())]))
        elif True:
            source0_ = default__.EncryptionContextToJson(_dafny.Seq("Some"), (x).value)
            if True:
                if source0_.is_Success:
                    d_0_pairs_ = source0_.value
                    return JSON_Values.JSON_Object(d_0_pairs_)
            if True:
                d_1_err_ = source0_.error
                return JSON_Values.JSON_Object(_dafny.Seq([(_dafny.Seq("Error"), JSON_Values.JSON_String(d_1_err_))]))

    @staticmethod
    def CountTrues(partitions):
        d_0___accumulator_ = 0
        while True:
            with _dafny.label():
                if (len(partitions)) == (0):
                    return (0) + (d_0___accumulator_)
                elif (partitions)[0]:
                    d_0___accumulator_ = (d_0___accumulator_) + (1)
                    in0_ = _dafny.Seq((partitions)[1::])
                    partitions = in0_
                    raise _dafny.TailCall()
                elif True:
                    in1_ = _dafny.Seq((partitions)[1::])
                    partitions = in1_
                    raise _dafny.TailCall()
                break

    @staticmethod
    def IsInExactlyOnePartition(x):
        d_0_partitionList_ = default__.PartitionMemberships(x)
        return (default__.CountTrues(d_0_partitionList_)) == (1)

    @staticmethod
    def ValuesExhaustPartitions(x):
        def lambda0_(forall_var_0_):
            d_0_i_: int = forall_var_0_
            return not (((0) <= (d_0_i_)) and ((d_0_i_) < (len(x)))) or (default__.IsInExactlyOnePartition((x)[d_0_i_]))

        def lambda1_(forall_var_1_):
            def lambda2_(exists_var_0_):
                d_2_j_: int = exists_var_0_
                return (((0) <= (d_2_j_)) and ((d_2_j_) < (len(x)))) and ((default__.PartitionMemberships((x)[d_2_j_]))[d_1_p_])

            d_1_p_: int = forall_var_1_
            return not (((0) <= (d_1_p_)) and ((d_1_p_) < (len(default__.PartitionMemberships((x)[0]))))) or (_dafny.quantifier(_dafny.IntegerRange(0, len(x)), False, lambda2_))

        return (_dafny.quantifier(_dafny.IntegerRange(0, len(x)), True, lambda0_)) and (_dafny.quantifier(_dafny.IntegerRange(0, len(default__.PartitionMemberships((x)[0]))), True, lambda1_))

    @staticmethod
    def IsValidMember(x):
        return (True) and (default__.IsInExactlyOnePartition(x))

    @staticmethod
    def GenerateJSON():
        outputJSON: JSON_Values.JSON = JSON_Values.JSON.default()()
        d_0_values_: _dafny.Seq
        out0_: _dafny.Seq
        out0_ = default__.GetValues()
        d_0_values_ = out0_
        d_1_out_: _dafny.Seq
        d_1_out_ = _dafny.Seq([])
        hi0_ = len(d_0_values_)
        for d_2_i_ in range(0, hi0_):
            d_3_value_: Wrappers.Option
            d_3_value_ = (d_0_values_)[d_2_i_]
            d_4_json_: JSON_Values.JSON
            d_4_json_ = default__.ToJSON(d_3_value_)
            d_1_out_ = (d_1_out_) + (_dafny.Seq([d_4_json_]))
        outputJSON = JSON_Values.JSON_Array(d_1_out_)
        return outputJSON

    @staticmethod
    def GetValues():
        output: _dafny.Seq = _dafny.Seq({})
        d_0_raw_: _dafny.Seq
        out0_: _dafny.Seq
        out0_ = default__.RepresentativeValues()
        d_0_raw_ = out0_
        d_1_seen_: _dafny.Set
        d_1_seen_ = _dafny.Set({})
        output = _dafny.Seq([])
        hi0_ = len(d_0_raw_)
        for d_2_i_ in range(0, hi0_):
            if ((d_0_raw_)[d_2_i_]) not in (d_1_seen_):
                output = (output) + (_dafny.Seq([(d_0_raw_)[d_2_i_]]))
                d_1_seen_ = (d_1_seen_) | (_dafny.Set({(d_0_raw_)[d_2_i_]}))
        return output

    @staticmethod
    def SomeValue():
        output: Wrappers.Option = Wrappers.Option.default()()
        d_0_values_: _dafny.Seq
        out0_: _dafny.Seq
        out0_ = default__.GetValues()
        d_0_values_ = out0_
        output = (d_0_values_)[0]
        return output

    @staticmethod
    def PruningConfigurationList(x):
        output: _dafny.Seq = _dafny.Seq({})
        output = _dafny.Seq([])
        return output
        return output

    @staticmethod
    def MaybeReplaceWithRepresentativeValue(x):
        output: Wrappers.Option = Wrappers.Option.default()()
        d_0_pruningConfigurationList_: _dafny.Seq
        out0_: _dafny.Seq
        out0_ = default__.PruningConfigurationList(x)
        d_0_pruningConfigurationList_ = out0_
        output = x
        hi0_ = len(d_0_pruningConfigurationList_)
        for d_1_i_ in range(0, hi0_):
            if ((d_0_pruningConfigurationList_)[d_1_i_])[0]:
                output = ((d_0_pruningConfigurationList_)[d_1_i_])[1]
                output = output
                return output
        return output

    @staticmethod
    def AddIfNotPruned(x, vals):
        output: _dafny.Seq = _dafny.Seq({})
        d_0_replace_: Wrappers.Option
        out0_: Wrappers.Option
        out0_ = default__.MaybeReplaceWithRepresentativeValue(x)
        d_0_replace_ = out0_
        if (x) == (d_0_replace_):
            output = (vals) + (_dafny.Seq([x]))
            return output
        elif True:
            output = vals
            return output
        return output

    @_dafny.classproperty
    def PartitionCount(instance):
        return 2
