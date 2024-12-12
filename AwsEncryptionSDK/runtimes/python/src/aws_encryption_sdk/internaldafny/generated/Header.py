import sys
from typing import Callable, Any, TypeVar, NamedTuple
from math import floor
from itertools import count

import aws_encryption_sdk.internaldafny.generated.module_ as module_
import _dafny as _dafny
import System_ as System_
import smithy_dafny_standard_library.internaldafny.generated.Wrappers as Wrappers
import smithy_dafny_standard_library.internaldafny.generated.BoundedInts as BoundedInts
import smithy_dafny_standard_library.internaldafny.generated.StandardLibrary_UInt as StandardLibrary_UInt
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
import aws_cryptographic_material_providers.internaldafny.generated.MultiKeyring as MultiKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsArnParsing as AwsArnParsing
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsMrkAreUnique as AwsKmsMrkAreUnique
import smithy_dafny_standard_library.internaldafny.generated.Actions as Actions
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsMrkMatchForDecrypt as AwsKmsMrkMatchForDecrypt
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsUtils as AwsKmsUtils
import aws_cryptographic_material_providers.internaldafny.generated.Constants as Constants
import smithy_dafny_standard_library.internaldafny.generated.UUID as UUID
import aws_cryptographic_material_providers.internaldafny.generated.MaterialWrapping as MaterialWrapping
import smithy_dafny_standard_library.internaldafny.generated.SortedSets as SortedSets
import aws_cryptographic_material_providers.internaldafny.generated.CanonicalEncryptionContext as CanonicalEncryptionContext
import aws_cryptographic_material_providers.internaldafny.generated.IntermediateKeyWrapping as IntermediateKeyWrapping
import aws_cryptographic_material_providers.internaldafny.generated.EdkWrapping as EdkWrapping
import aws_cryptographic_material_providers.internaldafny.generated.ErrorMessages as ErrorMessages
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsKeyring as AwsKmsKeyring
import aws_cryptographic_material_providers.internaldafny.generated.StrictMultiKeyring as StrictMultiKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsDiscoveryKeyring as AwsKmsDiscoveryKeyring
import aws_cryptography_internal_kms.internaldafny.generated.Com_Amazonaws_Kms as Com_Amazonaws_Kms
import aws_cryptography_internal_dynamodb.internaldafny.generated.Com_Amazonaws_Dynamodb as Com_Amazonaws_Dynamodb
import aws_cryptographic_material_providers.internaldafny.generated.DiscoveryMultiKeyring as DiscoveryMultiKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsMrkDiscoveryKeyring as AwsKmsMrkDiscoveryKeyring
import aws_cryptographic_material_providers.internaldafny.generated.MrkAwareDiscoveryMultiKeyring as MrkAwareDiscoveryMultiKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsMrkKeyring as AwsKmsMrkKeyring
import aws_cryptographic_material_providers.internaldafny.generated.MrkAwareStrictMultiKeyring as MrkAwareStrictMultiKeyring
import smithy_dafny_standard_library.internaldafny.generated.DafnyLibraries as DafnyLibraries
import smithy_dafny_standard_library.internaldafny.generated.Time as Time
import aws_cryptographic_material_providers.internaldafny.generated.LocalCMC as LocalCMC
import aws_cryptographic_material_providers.internaldafny.generated.SynchronizedLocalCMC as SynchronizedLocalCMC
import aws_cryptographic_material_providers.internaldafny.generated.StormTracker as StormTracker
import aws_cryptographic_material_providers.internaldafny.generated.StormTrackingCMC as StormTrackingCMC
import aws_cryptographic_material_providers.internaldafny.generated.CacheConstants as CacheConstants
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsHierarchicalKeyring as AwsKmsHierarchicalKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsRsaKeyring as AwsKmsRsaKeyring
import aws_cryptographic_material_providers.internaldafny.generated.EcdhEdkWrapping as EcdhEdkWrapping
import aws_cryptographic_material_providers.internaldafny.generated.RawECDHKeyring as RawECDHKeyring
import aws_cryptographic_material_providers.internaldafny.generated.AwsKmsEcdhKeyring as AwsKmsEcdhKeyring
import aws_cryptographic_material_providers.internaldafny.generated.RawAESKeyring as RawAESKeyring
import aws_cryptographic_material_providers.internaldafny.generated.RawRSAKeyring as RawRSAKeyring
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
import aws_cryptography_primitives.internaldafny.generated.AesKdfCtr as AesKdfCtr
import smithy_dafny_standard_library.internaldafny.generated.Unicode as Unicode
import smithy_dafny_standard_library.internaldafny.generated.Functions as Functions
import smithy_dafny_standard_library.internaldafny.generated.Utf8EncodingForm as Utf8EncodingForm
import smithy_dafny_standard_library.internaldafny.generated.Utf16EncodingForm as Utf16EncodingForm
import smithy_dafny_standard_library.internaldafny.generated.UnicodeStrings as UnicodeStrings
import smithy_dafny_standard_library.internaldafny.generated.FileIO as FileIO
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
import smithy_dafny_standard_library.internaldafny.generated.Streams as Streams
import smithy_dafny_standard_library.internaldafny.generated.Sorting as Sorting
import smithy_dafny_standard_library.internaldafny.generated.HexStrings as HexStrings
import smithy_dafny_standard_library.internaldafny.generated.GetOpt as GetOpt
import smithy_dafny_standard_library.internaldafny.generated.FloatCompare as FloatCompare
import smithy_dafny_standard_library.internaldafny.generated.ConcurrentCall as ConcurrentCall
import smithy_dafny_standard_library.internaldafny.generated.Base64Lemmas as Base64Lemmas
import aws_encryption_sdk.internaldafny.generated.AwsCryptographyEncryptionSdkTypes as AwsCryptographyEncryptionSdkTypes
import aws_encryption_sdk.internaldafny.generated.SerializableTypes as SerializableTypes
import aws_encryption_sdk.internaldafny.generated.SerializeFunctions as SerializeFunctions
import aws_encryption_sdk.internaldafny.generated.EncryptionContext as EncryptionContext
import aws_encryption_sdk.internaldafny.generated.HeaderTypes as HeaderTypes
import aws_encryption_sdk.internaldafny.generated.SharedHeaderFunctions as SharedHeaderFunctions
import aws_encryption_sdk.internaldafny.generated.EncryptedDataKeys as EncryptedDataKeys
import aws_encryption_sdk.internaldafny.generated.V1HeaderBody as V1HeaderBody
import aws_encryption_sdk.internaldafny.generated.V2HeaderBody as V2HeaderBody
import aws_encryption_sdk.internaldafny.generated.HeaderAuth as HeaderAuth

# Module: Header

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def HeaderVersionSupportsCommitment_q(suite, body):
        return (not (((suite).commitment).is_HKDF) or (((body).is_V2HeaderBody) and ((len((body).suiteData)) == ((((suite).commitment).HKDF).outputKeyLength)))) and (not (not(((suite).commitment).is_HKDF)) or ((True) and ((body).is_V1HeaderBody)))

    @staticmethod
    def ReadHeaderBody(buffer, maxEdks, mpl):
        d_0_valueOrError0_ = SharedHeaderFunctions.default__.ReadMessageFormatVersion(buffer)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_version_ = (d_0_valueOrError0_).Extract()
            def lambda0_():
                source0_ = (d_1_version_).data
                if True:
                    if source0_.is_V1:
                        d_3_valueOrError2_ = V1HeaderBody.default__.ReadV1HeaderBody(buffer, maxEdks, mpl)
                        if (d_3_valueOrError2_).IsFailure():
                            return (d_3_valueOrError2_).PropagateFailure()
                        elif True:
                            d_4_b_ = (d_3_valueOrError2_).Extract()
                            d_5_body_ = (d_4_b_).data
                            return Wrappers.Result_Success((d_5_body_, (d_4_b_).tail))
                if True:
                    d_6_valueOrError3_ = V2HeaderBody.default__.ReadV2HeaderBody(buffer, maxEdks, mpl)
                    if (d_6_valueOrError3_).IsFailure():
                        return (d_6_valueOrError3_).PropagateFailure()
                    elif True:
                        d_7_b_ = (d_6_valueOrError3_).Extract()
                        d_8_body_ = (d_7_b_).data
                        return Wrappers.Result_Success((d_8_body_, (d_7_b_).tail))

            d_2_valueOrError1_ = lambda0_()
            if (d_2_valueOrError1_).IsFailure():
                return (d_2_valueOrError1_).PropagateFailure()
            elif True:
                let_tmp_rhs0_ = (d_2_valueOrError1_).Extract()
                d_9_body_ = let_tmp_rhs0_[0]
                d_10_tail_ = let_tmp_rhs0_[1]
                d_11_valueOrError4_ = Wrappers.default__.Need((((d_9_body_).contentType).is_Framed) == (((d_9_body_).frameLength) > (0)), SerializeFunctions.ReadProblems_Error(_dafny.Seq("Frame length must be positive if content is framed")))
                if (d_11_valueOrError4_).IsFailure():
                    return (d_11_valueOrError4_).PropagateFailure()
                elif True:
                    d_12_valueOrError5_ = Wrappers.default__.Need((((d_9_body_).contentType).is_NonFramed) == (((d_9_body_).frameLength) == (0)), SerializeFunctions.ReadProblems_Error(_dafny.Seq("Frame length must be zero if content is non-framed")))
                    if (d_12_valueOrError5_).IsFailure():
                        return (d_12_valueOrError5_).PropagateFailure()
                    elif True:
                        return Wrappers.Result_Success(SerializeFunctions.SuccessfulRead_SuccessfulRead(d_9_body_, d_10_tail_))

    @staticmethod
    def WriteHeaderBody(body):
        source0_ = body
        if True:
            if source0_.is_V1HeaderBody:
                return V1HeaderBody.default__.WriteV1HeaderBody(body)
        if True:
            return V2HeaderBody.default__.WriteV2HeaderBody(body)


class HeaderInfo:
    @classmethod
    def default(cls, ):
        return lambda: HeaderInfo_HeaderInfo(HeaderTypes.HeaderBody.default()(), _dafny.Seq({}), _dafny.Map({}), AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo.default()(), HeaderTypes.HeaderAuth.default()())
    def __ne__(self, __o: object) -> bool:
        return not self.__eq__(__o)
    @property
    def is_HeaderInfo(self) -> bool:
        return isinstance(self, HeaderInfo_HeaderInfo)

class HeaderInfo_HeaderInfo(HeaderInfo, NamedTuple('HeaderInfo', [('body', Any), ('rawHeader', Any), ('encryptionContext', Any), ('suite', Any), ('headerAuth', Any)])):
    def __dafnystr__(self) -> str:
        return f'Header.HeaderInfo.HeaderInfo({_dafny.string_of(self.body)}, {_dafny.string_of(self.rawHeader)}, {_dafny.string_of(self.encryptionContext)}, {_dafny.string_of(self.suite)}, {_dafny.string_of(self.headerAuth)})'
    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, HeaderInfo_HeaderInfo) and self.body == __o.body and self.rawHeader == __o.rawHeader and self.encryptionContext == __o.encryptionContext and self.suite == __o.suite and self.headerAuth == __o.headerAuth
    def __hash__(self) -> int:
        return super().__hash__()


class Header:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return HeaderInfo.default()()
