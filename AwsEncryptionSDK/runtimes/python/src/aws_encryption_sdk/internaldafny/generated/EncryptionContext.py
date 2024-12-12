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

# Module: EncryptionContext

class default__:
    def  __init__(self):
        pass

    @staticmethod
    def GetCanonicalEncryptionContext(encryptionContext):
        return SerializableTypes.default__.GetCanonicalLinearPairs(encryptionContext)

    @staticmethod
    def GetEncryptionContext(canonicalEncryptionContext):
        def iife0_():
            coll0_ = _dafny.Map()
            compr_0_: int
            for compr_0_ in _dafny.IntegerRange(0, len(canonicalEncryptionContext)):
                d_0_i_: int = compr_0_
                if ((0) <= (d_0_i_)) and ((d_0_i_) < (len(canonicalEncryptionContext))):
                    coll0_[((canonicalEncryptionContext)[d_0_i_]).key] = ((canonicalEncryptionContext)[d_0_i_]).value
            return _dafny.Map(coll0_)
        return iife0_()
        

    @staticmethod
    def WriteAADSection(ec):
        if (len(ec)) == (0):
            return SerializeFunctions.default__.WriteUint16(0)
        elif True:
            d_0_aad_ = default__.WriteAAD(ec)
            return (SerializeFunctions.default__.WriteUint16(len(d_0_aad_))) + (d_0_aad_)

    @staticmethod
    def WriteEmptyEcOrWriteAAD(ec):
        if (len(ec)) == (0):
            return _dafny.Seq([])
        elif True:
            return default__.WriteAAD(ec)

    @staticmethod
    def WriteAAD(ec):
        return (SerializeFunctions.default__.WriteUint16(len(ec))) + (default__.WriteAADPairs(ec))

    @staticmethod
    def WriteAADPairs(ec):
        d_0___accumulator_ = _dafny.Seq([])
        while True:
            with _dafny.label():
                if (len(ec)) == (0):
                    return (_dafny.Seq([])) + (d_0___accumulator_)
                elif True:
                    d_0___accumulator_ = (default__.WriteAADPair(Seq.default__.Last(ec))) + (d_0___accumulator_)
                    in0_ = Seq.default__.DropLast(ec)
                    ec = in0_
                    raise _dafny.TailCall()
                break

    @staticmethod
    def WriteAADPair(pair):
        return (SerializeFunctions.default__.WriteShortLengthSeq((pair).key)) + (SerializeFunctions.default__.WriteShortLengthSeq((pair).value))

    @staticmethod
    def ReadAADPair(buffer):
        d_0_valueOrError0_ = SerializeFunctions.default__.ReadShortLengthSeq(buffer)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            let_tmp_rhs0_ = (d_0_valueOrError0_).Extract()
            d_1_key_ = let_tmp_rhs0_.data
            d_2_keyEnd_ = let_tmp_rhs0_.tail
            d_3_valueOrError1_ = Wrappers.default__.Need(UTF8.default__.ValidUTF8Seq(d_1_key_), SerializeFunctions.ReadProblems_Error(_dafny.Seq("Invalid Encryption Context key")))
            if (d_3_valueOrError1_).IsFailure():
                return (d_3_valueOrError1_).PropagateFailure()
            elif True:
                d_4_valueOrError2_ = SerializeFunctions.default__.ReadShortLengthSeq(d_2_keyEnd_)
                if (d_4_valueOrError2_).IsFailure():
                    return (d_4_valueOrError2_).PropagateFailure()
                elif True:
                    let_tmp_rhs1_ = (d_4_valueOrError2_).Extract()
                    d_5_value_ = let_tmp_rhs1_.data
                    d_6_tail_ = let_tmp_rhs1_.tail
                    d_7_valueOrError3_ = Wrappers.default__.Need(UTF8.default__.ValidUTF8Seq(d_5_value_), SerializeFunctions.ReadProblems_Error(_dafny.Seq("Invalid Encryption Context value")))
                    if (d_7_valueOrError3_).IsFailure():
                        return (d_7_valueOrError3_).PropagateFailure()
                    elif True:
                        d_8_pair_ = SerializableTypes.Pair_Pair(d_1_key_, d_5_value_)
                        return Wrappers.Result_Success(SerializeFunctions.SuccessfulRead_SuccessfulRead(d_8_pair_, d_6_tail_))

    @staticmethod
    def ReadAADPairs(buffer, accumulator, keys, count, nextPair):
        while True:
            with _dafny.label():
                if (count) > (len(accumulator)):
                    d_0_valueOrError0_ = default__.ReadAADPair(nextPair)
                    if (d_0_valueOrError0_).IsFailure():
                        return (d_0_valueOrError0_).PropagateFailure()
                    elif True:
                        let_tmp_rhs0_ = (d_0_valueOrError0_).Extract()
                        d_1_pair_ = let_tmp_rhs0_.data
                        d_2_newPos_ = let_tmp_rhs0_.tail
                        d_3_valueOrError1_ = Wrappers.default__.Need(((d_1_pair_).key) not in (keys), SerializeFunctions.ReadProblems_Error(_dafny.Seq("Duplicate Encryption Context key value.")))
                        if (d_3_valueOrError1_).IsFailure():
                            return (d_3_valueOrError1_).PropagateFailure()
                        elif True:
                            d_4_valueOrError2_ = Wrappers.default__.Need((((d_2_newPos_).start) - ((buffer).start)) < (SerializableTypes.default__.ESDK__CANONICAL__ENCRYPTION__CONTEXT__MAX__LENGTH), SerializeFunctions.ReadProblems_Error(_dafny.Seq("Encryption Context exceeds maximum length.")))
                            if (d_4_valueOrError2_).IsFailure():
                                return (d_4_valueOrError2_).PropagateFailure()
                            elif True:
                                d_5_nextAcc_ = (accumulator) + (_dafny.Seq([d_1_pair_]))
                                d_6_nextKeys_ = (keys) | (default__.KeysToSet(_dafny.Seq([d_1_pair_])))
                                in0_ = buffer
                                in1_ = d_5_nextAcc_
                                in2_ = d_6_nextKeys_
                                in3_ = count
                                in4_ = d_2_newPos_
                                buffer = in0_
                                accumulator = in1_
                                keys = in2_
                                count = in3_
                                nextPair = in4_
                                raise _dafny.TailCall()
                elif True:
                    return Wrappers.Result_Success(SerializeFunctions.SuccessfulRead_SuccessfulRead(accumulator, nextPair))
                break

    @staticmethod
    def ReadAAD(buffer):
        d_0_valueOrError0_ = SerializeFunctions.default__.ReadUInt16(buffer)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            let_tmp_rhs0_ = (d_0_valueOrError0_).Extract()
            d_1_count_ = let_tmp_rhs0_.data
            d_2_ecPos_ = let_tmp_rhs0_.tail
            if (d_1_count_) == (0):
                d_3_edks_ = _dafny.Seq([])
                return Wrappers.Result_Success(SerializeFunctions.SuccessfulRead_SuccessfulRead(d_3_edks_, d_2_ecPos_))
            elif True:
                d_4_accumulator_ = _dafny.Seq([])
                d_5_keys_ = default__.KeysToSet(d_4_accumulator_)
                d_6_valueOrError1_ = default__.ReadAADPairs(d_2_ecPos_, d_4_accumulator_, d_5_keys_, d_1_count_, d_2_ecPos_)
                if (d_6_valueOrError1_).IsFailure():
                    return (d_6_valueOrError1_).PropagateFailure()
                elif True:
                    let_tmp_rhs1_ = (d_6_valueOrError1_).Extract()
                    d_7_pairs_ = let_tmp_rhs1_.data
                    d_8_tail_ = let_tmp_rhs1_.tail
                    return Wrappers.Result_Success(SerializeFunctions.SuccessfulRead_SuccessfulRead(d_7_pairs_, d_8_tail_))

    @staticmethod
    def ReadAADSection(buffer):
        d_0_valueOrError0_ = SerializeFunctions.default__.ReadUInt16(buffer)
        if (d_0_valueOrError0_).IsFailure():
            return (d_0_valueOrError0_).PropagateFailure()
        elif True:
            d_1_length_ = (d_0_valueOrError0_).Extract()
            if ((d_1_length_).data) == (0):
                d_2_empty_ = _dafny.Seq([])
                return Wrappers.Result_Success(SerializeFunctions.SuccessfulRead_SuccessfulRead(d_2_empty_, (d_1_length_).tail))
            elif True:
                d_3_valueOrError1_ = Wrappers.default__.Need(((((d_1_length_).tail).start) + ((d_1_length_).data)) <= (len(((d_1_length_).tail).bytes)), SerializeFunctions.ReadProblems_MoreNeeded((((d_1_length_).tail).start) + ((d_1_length_).data)))
                if (d_3_valueOrError1_).IsFailure():
                    return (d_3_valueOrError1_).PropagateFailure()
                elif True:
                    d_4_valueOrError2_ = SerializeFunctions.default__.ReadUInt16((d_1_length_).tail)
                    if (d_4_valueOrError2_).IsFailure():
                        return (d_4_valueOrError2_).PropagateFailure()
                    elif True:
                        d_5_verifyCount_ = (d_4_valueOrError2_).Extract()
                        if ((d_1_length_).data) == (2):
                            d_6_valueOrError3_ = Wrappers.default__.Need(((d_5_verifyCount_).data) == (0), SerializeFunctions.ReadProblems_Error(_dafny.Seq("Encryption Context pairs count can not exceed byte length")))
                            if (d_6_valueOrError3_).IsFailure():
                                return (d_6_valueOrError3_).PropagateFailure()
                            elif True:
                                d_7_empty_ = _dafny.Seq([])
                                return Wrappers.Result_Success(SerializeFunctions.SuccessfulRead_SuccessfulRead(d_7_empty_, (d_5_verifyCount_).tail))
                        elif True:
                            d_8_valueOrError4_ = Wrappers.default__.Need((0) < ((d_5_verifyCount_).data), SerializeFunctions.ReadProblems_Error(_dafny.Seq("Encryption Context byte length exceeds pairs count.")))
                            if (d_8_valueOrError4_).IsFailure():
                                return (d_8_valueOrError4_).PropagateFailure()
                            elif True:
                                d_9_valueOrError5_ = default__.ReadAAD((d_1_length_).tail)
                                if (d_9_valueOrError5_).IsFailure():
                                    return (d_9_valueOrError5_).PropagateFailure()
                                elif True:
                                    d_10_aad_ = (d_9_valueOrError5_).Extract()
                                    d_11_valueOrError6_ = Wrappers.default__.Need(((((d_10_aad_).tail).start) - (((d_1_length_).tail).start)) == ((d_1_length_).data), SerializeFunctions.ReadProblems_Error(_dafny.Seq("AAD Length did not match stored length.")))
                                    if (d_11_valueOrError6_).IsFailure():
                                        return (d_11_valueOrError6_).PropagateFailure()
                                    elif True:
                                        return Wrappers.Result_Success(d_10_aad_)

    @staticmethod
    def KeysToSet(pairs):
        def iife0_():
            coll0_ = _dafny.Set()
            compr_0_: SerializableTypes.Pair
            for compr_0_ in (pairs).Elements:
                d_0_p_: SerializableTypes.Pair = compr_0_
                if (d_0_p_) in (pairs):
                    coll0_ = coll0_.union(_dafny.Set([(d_0_p_).key]))
            return _dafny.Set(coll0_)
        return iife0_()
        


class ESDKEncryptionContextPair:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return SerializableTypes.Pair.default(UTF8.ValidUTF8Bytes.default, UTF8.ValidUTF8Bytes.default)()
    def _Is(source__):
        d_0_p_: SerializableTypes.Pair = source__
        return (((StandardLibrary_UInt.default__.HasUint16Len((d_0_p_).key)) and (UTF8.default__.ValidUTF8Seq((d_0_p_).key))) and (StandardLibrary_UInt.default__.HasUint16Len((d_0_p_).value))) and (UTF8.default__.ValidUTF8Seq((d_0_p_).value))

class ESDKCanonicalEncryptionContext:
    def  __init__(self):
        pass

    @staticmethod
    def default():
        return _dafny.Seq({})
