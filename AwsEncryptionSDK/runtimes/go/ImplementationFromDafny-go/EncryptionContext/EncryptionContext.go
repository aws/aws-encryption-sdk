// Package EncryptionContext
// Dafny module EncryptionContext compiled into Go

package EncryptionContext

import (
	os "os"

	m_ComAmazonawsDynamodbTypes "github.com/aws/aws-cryptographic-material-providers-library/dynamodb/ComAmazonawsDynamodbTypes"
	m_Com_Amazonaws_Dynamodb "github.com/aws/aws-cryptographic-material-providers-library/dynamodb/Com_Amazonaws_Dynamodb"
	m_ComAmazonawsKmsTypes "github.com/aws/aws-cryptographic-material-providers-library/kms/ComAmazonawsKmsTypes"
	m_Com_Amazonaws_Kms "github.com/aws/aws-cryptographic-material-providers-library/kms/Com_Amazonaws_Kms"
	m_AlgorithmSuites "github.com/aws/aws-cryptographic-material-providers-library/mpl/AlgorithmSuites"
	m_AwsArnParsing "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsArnParsing"
	m_AwsCryptographyKeyStoreOperations "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsCryptographyKeyStoreOperations"
	m_AwsCryptographyKeyStoreTypes "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsCryptographyKeyStoreTypes"
	m_AwsCryptographyMaterialProvidersOperations "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsCryptographyMaterialProvidersOperations"
	m_AwsCryptographyMaterialProvidersTypes "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsCryptographyMaterialProvidersTypes"
	m_AwsKmsDiscoveryKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsDiscoveryKeyring"
	m_AwsKmsEcdhKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsEcdhKeyring"
	m_AwsKmsHierarchicalKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsHierarchicalKeyring"
	m_AwsKmsKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsKeyring"
	m_AwsKmsMrkAreUnique "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsMrkAreUnique"
	m_AwsKmsMrkDiscoveryKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsMrkDiscoveryKeyring"
	m_AwsKmsMrkKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsMrkKeyring"
	m_AwsKmsMrkMatchForDecrypt "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsMrkMatchForDecrypt"
	m_AwsKmsRsaKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsRsaKeyring"
	m_AwsKmsUtils "github.com/aws/aws-cryptographic-material-providers-library/mpl/AwsKmsUtils"
	m_CMM "github.com/aws/aws-cryptographic-material-providers-library/mpl/CMM"
	m_CacheConstants "github.com/aws/aws-cryptographic-material-providers-library/mpl/CacheConstants"
	m_CanonicalEncryptionContext "github.com/aws/aws-cryptographic-material-providers-library/mpl/CanonicalEncryptionContext"
	m_Commitment "github.com/aws/aws-cryptographic-material-providers-library/mpl/Commitment"
	m_Constants "github.com/aws/aws-cryptographic-material-providers-library/mpl/Constants"
	m_CreateKeyStoreTable "github.com/aws/aws-cryptographic-material-providers-library/mpl/CreateKeyStoreTable"
	m_CreateKeys "github.com/aws/aws-cryptographic-material-providers-library/mpl/CreateKeys"
	m_DDBKeystoreOperations "github.com/aws/aws-cryptographic-material-providers-library/mpl/DDBKeystoreOperations"
	m_DefaultCMM "github.com/aws/aws-cryptographic-material-providers-library/mpl/DefaultCMM"
	m_DefaultClientSupplier "github.com/aws/aws-cryptographic-material-providers-library/mpl/DefaultClientSupplier"
	m_Defaults "github.com/aws/aws-cryptographic-material-providers-library/mpl/Defaults"
	m_DiscoveryMultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/DiscoveryMultiKeyring"
	m_EcdhEdkWrapping "github.com/aws/aws-cryptographic-material-providers-library/mpl/EcdhEdkWrapping"
	m_EdkWrapping "github.com/aws/aws-cryptographic-material-providers-library/mpl/EdkWrapping"
	m_ErrorMessages "github.com/aws/aws-cryptographic-material-providers-library/mpl/ErrorMessages"
	m_GetKeys "github.com/aws/aws-cryptographic-material-providers-library/mpl/GetKeys"
	m_IntermediateKeyWrapping "github.com/aws/aws-cryptographic-material-providers-library/mpl/IntermediateKeyWrapping"
	m_KMSKeystoreOperations "github.com/aws/aws-cryptographic-material-providers-library/mpl/KMSKeystoreOperations"
	m_KeyStore "github.com/aws/aws-cryptographic-material-providers-library/mpl/KeyStore"
	m_KeyStoreErrorMessages "github.com/aws/aws-cryptographic-material-providers-library/mpl/KeyStoreErrorMessages"
	m_Keyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/Keyring"
	m_KmsArn "github.com/aws/aws-cryptographic-material-providers-library/mpl/KmsArn"
	m_LocalCMC "github.com/aws/aws-cryptographic-material-providers-library/mpl/LocalCMC"
	m_MaterialProviders "github.com/aws/aws-cryptographic-material-providers-library/mpl/MaterialProviders"
	m_MaterialWrapping "github.com/aws/aws-cryptographic-material-providers-library/mpl/MaterialWrapping"
	m_Materials "github.com/aws/aws-cryptographic-material-providers-library/mpl/Materials"
	m_MrkAwareDiscoveryMultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/MrkAwareDiscoveryMultiKeyring"
	m_MrkAwareStrictMultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/MrkAwareStrictMultiKeyring"
	m_MultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/MultiKeyring"
	m_RawAESKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/RawAESKeyring"
	m_RawECDHKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/RawECDHKeyring"
	m_RawRSAKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/RawRSAKeyring"
	m_RequiredEncryptionContextCMM "github.com/aws/aws-cryptographic-material-providers-library/mpl/RequiredEncryptionContextCMM"
	m_StormTracker "github.com/aws/aws-cryptographic-material-providers-library/mpl/StormTracker"
	m_StormTrackingCMC "github.com/aws/aws-cryptographic-material-providers-library/mpl/StormTrackingCMC"
	m_StrictMultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/mpl/StrictMultiKeyring"
	m_Structure "github.com/aws/aws-cryptographic-material-providers-library/mpl/Structure"
	m_SynchronizedLocalCMC "github.com/aws/aws-cryptographic-material-providers-library/mpl/SynchronizedLocalCMC"
	m_Utils "github.com/aws/aws-cryptographic-material-providers-library/mpl/Utils"
	m_AESEncryption "github.com/aws/aws-cryptographic-material-providers-library/primitives/AESEncryption"
	m_AtomicPrimitives "github.com/aws/aws-cryptographic-material-providers-library/primitives/AtomicPrimitives"
	m_AwsCryptographyPrimitivesOperations "github.com/aws/aws-cryptographic-material-providers-library/primitives/AwsCryptographyPrimitivesOperations"
	m_AwsCryptographyPrimitivesTypes "github.com/aws/aws-cryptographic-material-providers-library/primitives/AwsCryptographyPrimitivesTypes"
	m_Digest "github.com/aws/aws-cryptographic-material-providers-library/primitives/Digest"
	m_ECDH "github.com/aws/aws-cryptographic-material-providers-library/primitives/ECDH"
	m_HKDF "github.com/aws/aws-cryptographic-material-providers-library/primitives/HKDF"
	m_HMAC "github.com/aws/aws-cryptographic-material-providers-library/primitives/HMAC"
	m_KdfCtr "github.com/aws/aws-cryptographic-material-providers-library/primitives/KdfCtr"
	m_RSAEncryption "github.com/aws/aws-cryptographic-material-providers-library/primitives/RSAEncryption"
	m_Random "github.com/aws/aws-cryptographic-material-providers-library/primitives/Random"
	m_Signature "github.com/aws/aws-cryptographic-material-providers-library/primitives/Signature"
	m_WrappedHKDF "github.com/aws/aws-cryptographic-material-providers-library/primitives/WrappedHKDF"
	m_WrappedHMAC "github.com/aws/aws-cryptographic-material-providers-library/primitives/WrappedHMAC"
	m_AwsCryptographyEncryptionSdkTypes "github.com/aws/aws-encryption-sdk/AwsCryptographyEncryptionSdkTypes"
	m_SerializableTypes "github.com/aws/aws-encryption-sdk/SerializableTypes"
	m_SerializeFunctions "github.com/aws/aws-encryption-sdk/SerializeFunctions"
	m__System "github.com/dafny-lang/DafnyRuntimeGo/v4/System_"
	_dafny "github.com/dafny-lang/DafnyRuntimeGo/v4/dafny"
	m_Actions "github.com/dafny-lang/DafnyStandardLibGo/Actions"
	m_Base64 "github.com/dafny-lang/DafnyStandardLibGo/Base64"
	m_Base64Lemmas "github.com/dafny-lang/DafnyStandardLibGo/Base64Lemmas"
	m_BoundedInts "github.com/dafny-lang/DafnyStandardLibGo/BoundedInts"
	m_DivInternals "github.com/dafny-lang/DafnyStandardLibGo/DivInternals"
	m_DivInternalsNonlinear "github.com/dafny-lang/DafnyStandardLibGo/DivInternalsNonlinear"
	m_DivMod "github.com/dafny-lang/DafnyStandardLibGo/DivMod"
	m_FileIO "github.com/dafny-lang/DafnyStandardLibGo/FileIO"
	m_FloatCompare "github.com/dafny-lang/DafnyStandardLibGo/FloatCompare"
	m_Functions "github.com/dafny-lang/DafnyStandardLibGo/Functions"
	m_GeneralInternals "github.com/dafny-lang/DafnyStandardLibGo/GeneralInternals"
	m_GetOpt "github.com/dafny-lang/DafnyStandardLibGo/GetOpt"
	m_HexStrings "github.com/dafny-lang/DafnyStandardLibGo/HexStrings"
	m_Logarithm "github.com/dafny-lang/DafnyStandardLibGo/Logarithm"
	m__Math "github.com/dafny-lang/DafnyStandardLibGo/Math_"
	m_ModInternals "github.com/dafny-lang/DafnyStandardLibGo/ModInternals"
	m_ModInternalsNonlinear "github.com/dafny-lang/DafnyStandardLibGo/ModInternalsNonlinear"
	m_Mul "github.com/dafny-lang/DafnyStandardLibGo/Mul"
	m_MulInternals "github.com/dafny-lang/DafnyStandardLibGo/MulInternals"
	m_MulInternalsNonlinear "github.com/dafny-lang/DafnyStandardLibGo/MulInternalsNonlinear"
	m_Power "github.com/dafny-lang/DafnyStandardLibGo/Power"
	m_Relations "github.com/dafny-lang/DafnyStandardLibGo/Relations"
	m_Seq "github.com/dafny-lang/DafnyStandardLibGo/Seq"
	m_Seq_MergeSort "github.com/dafny-lang/DafnyStandardLibGo/Seq_MergeSort"
	m_Sorting "github.com/dafny-lang/DafnyStandardLibGo/Sorting"
	m_StandardLibrary "github.com/dafny-lang/DafnyStandardLibGo/StandardLibrary"
	m_StandardLibraryInterop "github.com/dafny-lang/DafnyStandardLibGo/StandardLibraryInterop"
	m_StandardLibrary_Sequence "github.com/dafny-lang/DafnyStandardLibGo/StandardLibrary_Sequence"
	m_StandardLibrary_String "github.com/dafny-lang/DafnyStandardLibGo/StandardLibrary_String"
	m_StandardLibrary_UInt "github.com/dafny-lang/DafnyStandardLibGo/StandardLibrary_UInt"
	m_Streams "github.com/dafny-lang/DafnyStandardLibGo/Streams"
	m_UTF8 "github.com/dafny-lang/DafnyStandardLibGo/UTF8"
	m_UnicodeStrings "github.com/dafny-lang/DafnyStandardLibGo/UnicodeStrings"
	m__Unicode "github.com/dafny-lang/DafnyStandardLibGo/Unicode_"
	m_Utf16EncodingForm "github.com/dafny-lang/DafnyStandardLibGo/Utf16EncodingForm"
	m_Utf8EncodingForm "github.com/dafny-lang/DafnyStandardLibGo/Utf8EncodingForm"
	m_Wrappers "github.com/dafny-lang/DafnyStandardLibGo/Wrappers"
)

var _ = os.Args
var _ _dafny.Dummy__
var _ m__System.Dummy__
var _ m_Wrappers.Dummy__
var _ m_BoundedInts.Dummy__
var _ m_StandardLibrary_UInt.Dummy__
var _ m_StandardLibrary_Sequence.Dummy__
var _ m_StandardLibrary_String.Dummy__
var _ m_StandardLibrary.Dummy__
var _ m_AwsCryptographyPrimitivesTypes.Dummy__
var _ m_Random.Dummy__
var _ m_AESEncryption.Dummy__
var _ m_Digest.Dummy__
var _ m_HMAC.Dummy__
var _ m_WrappedHMAC.Dummy__
var _ m_HKDF.Dummy__
var _ m_WrappedHKDF.Dummy__
var _ m_Signature.Dummy__
var _ m_KdfCtr.Dummy__
var _ m_RSAEncryption.Dummy__
var _ m_ECDH.Dummy__
var _ m_AwsCryptographyPrimitivesOperations.Dummy__
var _ m_AtomicPrimitives.Dummy__
var _ m_ComAmazonawsDynamodbTypes.Dummy__
var _ m_ComAmazonawsKmsTypes.Dummy__
var _ m_AwsCryptographyKeyStoreTypes.Dummy__
var _ m_AwsCryptographyMaterialProvidersTypes.Dummy__
var _ m_Base64.Dummy__
var _ m_AlgorithmSuites.Dummy__
var _ m_Materials.Dummy__
var _ m_Keyring.Dummy__
var _ m_Relations.Dummy__
var _ m_Seq_MergeSort.Dummy__
var _ m__Math.Dummy__
var _ m_Seq.Dummy__
var _ m_MultiKeyring.Dummy__
var _ m_AwsArnParsing.Dummy__
var _ m_AwsKmsMrkAreUnique.Dummy__
var _ m_Actions.Dummy__
var _ m_AwsKmsMrkMatchForDecrypt.Dummy__
var _ m_AwsKmsUtils.Dummy__
var _ m_Constants.Dummy__
var _ m_MaterialWrapping.Dummy__
var _ m_CanonicalEncryptionContext.Dummy__
var _ m_IntermediateKeyWrapping.Dummy__
var _ m_EdkWrapping.Dummy__
var _ m_ErrorMessages.Dummy__
var _ m_AwsKmsKeyring.Dummy__
var _ m_StrictMultiKeyring.Dummy__
var _ m_AwsKmsDiscoveryKeyring.Dummy__
var _ m_Com_Amazonaws_Kms.Dummy__
var _ m_Com_Amazonaws_Dynamodb.Dummy__
var _ m_DiscoveryMultiKeyring.Dummy__
var _ m_AwsKmsMrkDiscoveryKeyring.Dummy__
var _ m_MrkAwareDiscoveryMultiKeyring.Dummy__
var _ m_AwsKmsMrkKeyring.Dummy__
var _ m_MrkAwareStrictMultiKeyring.Dummy__
var _ m_LocalCMC.Dummy__
var _ m_SynchronizedLocalCMC.Dummy__
var _ m_StormTracker.Dummy__
var _ m_StormTrackingCMC.Dummy__
var _ m_CacheConstants.Dummy__
var _ m_AwsKmsHierarchicalKeyring.Dummy__
var _ m_AwsKmsRsaKeyring.Dummy__
var _ m_EcdhEdkWrapping.Dummy__
var _ m_RawECDHKeyring.Dummy__
var _ m_AwsKmsEcdhKeyring.Dummy__
var _ m_RawAESKeyring.Dummy__
var _ m_RawRSAKeyring.Dummy__
var _ m_CMM.Dummy__
var _ m_Defaults.Dummy__
var _ m_Commitment.Dummy__
var _ m_DefaultCMM.Dummy__
var _ m_DefaultClientSupplier.Dummy__
var _ m_Utils.Dummy__
var _ m_RequiredEncryptionContextCMM.Dummy__
var _ m_AwsCryptographyMaterialProvidersOperations.Dummy__
var _ m_MaterialProviders.Dummy__
var _ m_KeyStoreErrorMessages.Dummy__
var _ m_KmsArn.Dummy__
var _ m_Structure.Dummy__
var _ m_KMSKeystoreOperations.Dummy__
var _ m_DDBKeystoreOperations.Dummy__
var _ m_CreateKeys.Dummy__
var _ m_CreateKeyStoreTable.Dummy__
var _ m_GetKeys.Dummy__
var _ m_AwsCryptographyKeyStoreOperations.Dummy__
var _ m_KeyStore.Dummy__
var _ m__Unicode.Dummy__
var _ m_Functions.Dummy__
var _ m_Utf8EncodingForm.Dummy__
var _ m_Utf16EncodingForm.Dummy__
var _ m_UnicodeStrings.Dummy__
var _ m_FileIO.Dummy__
var _ m_GeneralInternals.Dummy__
var _ m_MulInternalsNonlinear.Dummy__
var _ m_MulInternals.Dummy__
var _ m_Mul.Dummy__
var _ m_ModInternalsNonlinear.Dummy__
var _ m_DivInternalsNonlinear.Dummy__
var _ m_ModInternals.Dummy__
var _ m_DivInternals.Dummy__
var _ m_DivMod.Dummy__
var _ m_Power.Dummy__
var _ m_Logarithm.Dummy__
var _ m_StandardLibraryInterop.Dummy__
var _ m_Streams.Dummy__
var _ m_Sorting.Dummy__
var _ m_HexStrings.Dummy__
var _ m_GetOpt.Dummy__
var _ m_FloatCompare.Dummy__
var _ m_Base64Lemmas.Dummy__
var _ m_AwsCryptographyEncryptionSdkTypes.Dummy__
var _ m_SerializableTypes.Dummy__
var _ m_SerializeFunctions.Dummy__

type Dummy__ struct{}

// Definition of class Default__
type Default__ struct {
	dummy byte
}

func New_Default___() *Default__ {
	_this := Default__{}

	return &_this
}

type CompanionStruct_Default___ struct {
}

var Companion_Default___ = CompanionStruct_Default___{}

func (_this *Default__) Equals(other *Default__) bool {
	return _this == other
}

func (_this *Default__) EqualsGeneric(x interface{}) bool {
	other, ok := x.(*Default__)
	return ok && _this.Equals(other)
}

func (*Default__) String() string {
	return "EncryptionContext.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) GetCanonicalEncryptionContext(encryptionContext _dafny.Map) _dafny.Sequence {
	return m_SerializableTypes.Companion_Default___.GetCanonicalLinearPairs(encryptionContext)
}
func (_static *CompanionStruct_Default___) GetEncryptionContext(canonicalEncryptionContext _dafny.Sequence) _dafny.Map {
	return func() _dafny.Map {
		var _coll0 = _dafny.NewMapBuilder()
		_ = _coll0
		for _iter0 := _dafny.Iterate(_dafny.IntegerRange(_dafny.Zero, _dafny.IntOfUint32((canonicalEncryptionContext).Cardinality()))); ; {
			_compr_0, _ok0 := _iter0()
			if !_ok0 {
				break
			}
			var _0_i _dafny.Int
			_0_i = interface{}(_compr_0).(_dafny.Int)
			if ((_0_i).Sign() != -1) && ((_0_i).Cmp(_dafny.IntOfUint32((canonicalEncryptionContext).Cardinality())) < 0) {
				_coll0.Add(((canonicalEncryptionContext).Select((_0_i).Uint32()).(m_SerializableTypes.Pair)).Dtor_key().(_dafny.Sequence), ((canonicalEncryptionContext).Select((_0_i).Uint32()).(m_SerializableTypes.Pair)).Dtor_value().(_dafny.Sequence))
			}
		}
		return _coll0.ToMap()
	}()
}
func (_static *CompanionStruct_Default___) WriteAADSection(ec _dafny.Sequence) _dafny.Sequence {
	if (_dafny.IntOfUint32((ec).Cardinality())).Sign() == 0 {
		return m_SerializeFunctions.Companion_Default___.WriteUint16(uint16(0))
	} else {
		var _0_aad _dafny.Sequence = Companion_Default___.WriteAAD(ec)
		_ = _0_aad
		return _dafny.Companion_Sequence_.Concatenate(m_SerializeFunctions.Companion_Default___.WriteUint16(uint16((_0_aad).Cardinality())), _0_aad)
	}
}
func (_static *CompanionStruct_Default___) WriteEmptyEcOrWriteAAD(ec _dafny.Sequence) _dafny.Sequence {
	if (_dafny.IntOfUint32((ec).Cardinality())).Sign() == 0 {
		return _dafny.SeqOf()
	} else {
		return Companion_Default___.WriteAAD(ec)
	}
}
func (_static *CompanionStruct_Default___) WriteAAD(ec _dafny.Sequence) _dafny.Sequence {
	return _dafny.Companion_Sequence_.Concatenate(m_SerializeFunctions.Companion_Default___.WriteUint16(uint16((ec).Cardinality())), Companion_Default___.WriteAADPairs(ec))
}
func (_static *CompanionStruct_Default___) WriteAADPairs(ec _dafny.Sequence) _dafny.Sequence {
	var _0___accumulator _dafny.Sequence = _dafny.SeqOf()
	_ = _0___accumulator
	goto TAIL_CALL_START
TAIL_CALL_START:
	if (_dafny.IntOfUint32((ec).Cardinality())).Sign() == 0 {
		return _dafny.Companion_Sequence_.Concatenate(_dafny.SeqOf(), _0___accumulator)
	} else {
		_0___accumulator = _dafny.Companion_Sequence_.Concatenate(Companion_Default___.WriteAADPair(m_Seq.Companion_Default___.Last(ec).(m_SerializableTypes.Pair)), _0___accumulator)
		var _in0 _dafny.Sequence = m_Seq.Companion_Default___.DropLast(ec)
		_ = _in0
		ec = _in0
		goto TAIL_CALL_START
	}
}
func (_static *CompanionStruct_Default___) WriteAADPair(pair m_SerializableTypes.Pair) _dafny.Sequence {
	return _dafny.Companion_Sequence_.Concatenate(m_SerializeFunctions.Companion_Default___.WriteShortLengthSeq((pair).Dtor_key().(_dafny.Sequence)), m_SerializeFunctions.Companion_Default___.WriteShortLengthSeq((pair).Dtor_value().(_dafny.Sequence)))
}
func (_static *CompanionStruct_Default___) ReadAADPair(buffer m_SerializeFunctions.ReadableBuffer) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.ReadShortLengthSeq(buffer)
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _let_tmp_rhs0 m_SerializeFunctions.SuccessfulRead = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
		_ = _let_tmp_rhs0
		var _1_key _dafny.Sequence = _let_tmp_rhs0.Get_().(m_SerializeFunctions.SuccessfulRead_SuccessfulRead).Data.(_dafny.Sequence)
		_ = _1_key
		var _2_keyEnd m_SerializeFunctions.ReadableBuffer = _let_tmp_rhs0.Get_().(m_SerializeFunctions.SuccessfulRead_SuccessfulRead).Tail
		_ = _2_keyEnd
		var _3_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(m_UTF8.Companion_Default___.ValidUTF8Seq(_1_key), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Invalid Encryption Context key")))
		_ = _3_valueOrError1
		if (_3_valueOrError1).IsFailure() {
			return (_3_valueOrError1).PropagateFailure()
		} else {
			var _4_valueOrError2 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.ReadShortLengthSeq(_2_keyEnd)
			_ = _4_valueOrError2
			if (_4_valueOrError2).IsFailure() {
				return (_4_valueOrError2).PropagateFailure()
			} else {
				var _let_tmp_rhs1 m_SerializeFunctions.SuccessfulRead = (_4_valueOrError2).Extract().(m_SerializeFunctions.SuccessfulRead)
				_ = _let_tmp_rhs1
				var _5_value _dafny.Sequence = _let_tmp_rhs1.Get_().(m_SerializeFunctions.SuccessfulRead_SuccessfulRead).Data.(_dafny.Sequence)
				_ = _5_value
				var _6_tail m_SerializeFunctions.ReadableBuffer = _let_tmp_rhs1.Get_().(m_SerializeFunctions.SuccessfulRead_SuccessfulRead).Tail
				_ = _6_tail
				var _7_valueOrError3 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(m_UTF8.Companion_Default___.ValidUTF8Seq(_5_value), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Invalid Encryption Context value")))
				_ = _7_valueOrError3
				if (_7_valueOrError3).IsFailure() {
					return (_7_valueOrError3).PropagateFailure()
				} else {
					var _8_pair m_SerializableTypes.Pair = m_SerializableTypes.Companion_Pair_.Create_Pair_(_1_key, _5_value)
					_ = _8_pair
					return m_Wrappers.Companion_Result_.Create_Success_(m_SerializeFunctions.Companion_SuccessfulRead_.Create_SuccessfulRead_(_8_pair, _6_tail))
				}
			}
		}
	}
}
func (_static *CompanionStruct_Default___) ReadAADPairs(buffer m_SerializeFunctions.ReadableBuffer, accumulator _dafny.Sequence, keys _dafny.Set, count uint16, nextPair m_SerializeFunctions.ReadableBuffer) m_Wrappers.Result {
	goto TAIL_CALL_START
TAIL_CALL_START:
	if (_dafny.IntOfUint16(count)).Cmp(_dafny.IntOfUint32((accumulator).Cardinality())) > 0 {
		var _0_valueOrError0 m_Wrappers.Result = Companion_Default___.ReadAADPair(nextPair)
		_ = _0_valueOrError0
		if (_0_valueOrError0).IsFailure() {
			return (_0_valueOrError0).PropagateFailure()
		} else {
			var _let_tmp_rhs0 m_SerializeFunctions.SuccessfulRead = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
			_ = _let_tmp_rhs0
			var _1_pair m_SerializableTypes.Pair = _let_tmp_rhs0.Get_().(m_SerializeFunctions.SuccessfulRead_SuccessfulRead).Data.(m_SerializableTypes.Pair)
			_ = _1_pair
			var _2_newPos m_SerializeFunctions.ReadableBuffer = _let_tmp_rhs0.Get_().(m_SerializeFunctions.SuccessfulRead_SuccessfulRead).Tail
			_ = _2_newPos
			var _3_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(!(keys).Contains((_1_pair).Dtor_key().(_dafny.Sequence)), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Duplicate Encryption Context key value.")))
			_ = _3_valueOrError1
			if (_3_valueOrError1).IsFailure() {
				return (_3_valueOrError1).PropagateFailure()
			} else {
				var _4_valueOrError2 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((((_2_newPos).Dtor_start()).Minus((buffer).Dtor_start())).Cmp(m_SerializableTypes.Companion_Default___.ESDK__CANONICAL__ENCRYPTION__CONTEXT__MAX__LENGTH()) < 0, m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Encryption Context exceeds maximum length.")))
				_ = _4_valueOrError2
				if (_4_valueOrError2).IsFailure() {
					return (_4_valueOrError2).PropagateFailure()
				} else {
					var _5_nextAcc _dafny.Sequence = _dafny.Companion_Sequence_.Concatenate(accumulator, _dafny.SeqOf(_1_pair))
					_ = _5_nextAcc
					var _6_nextKeys _dafny.Set = (keys).Union(Companion_Default___.KeysToSet(_dafny.SeqOf(_1_pair)))
					_ = _6_nextKeys
					var _in0 m_SerializeFunctions.ReadableBuffer = buffer
					_ = _in0
					var _in1 _dafny.Sequence = _5_nextAcc
					_ = _in1
					var _in2 _dafny.Set = _6_nextKeys
					_ = _in2
					var _in3 uint16 = count
					_ = _in3
					var _in4 m_SerializeFunctions.ReadableBuffer = _2_newPos
					_ = _in4
					buffer = _in0
					accumulator = _in1
					keys = _in2
					count = _in3
					nextPair = _in4
					goto TAIL_CALL_START
				}
			}
		}
	} else {
		return m_Wrappers.Companion_Result_.Create_Success_(m_SerializeFunctions.Companion_SuccessfulRead_.Create_SuccessfulRead_(accumulator, nextPair))
	}
}
func (_static *CompanionStruct_Default___) ReadAAD(buffer m_SerializeFunctions.ReadableBuffer) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.ReadUInt16(buffer)
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _let_tmp_rhs0 m_SerializeFunctions.SuccessfulRead = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
		_ = _let_tmp_rhs0
		var _1_count uint16 = _let_tmp_rhs0.Get_().(m_SerializeFunctions.SuccessfulRead_SuccessfulRead).Data.(uint16)
		_ = _1_count
		var _2_ecPos m_SerializeFunctions.ReadableBuffer = _let_tmp_rhs0.Get_().(m_SerializeFunctions.SuccessfulRead_SuccessfulRead).Tail
		_ = _2_ecPos
		if (_1_count) == (uint16(0)) {
			var _3_edks _dafny.Sequence = _dafny.SeqOf()
			_ = _3_edks
			return m_Wrappers.Companion_Result_.Create_Success_(m_SerializeFunctions.Companion_SuccessfulRead_.Create_SuccessfulRead_(_3_edks, _2_ecPos))
		} else {
			var _4_accumulator _dafny.Sequence = _dafny.SeqOf()
			_ = _4_accumulator
			var _5_keys _dafny.Set = Companion_Default___.KeysToSet(_4_accumulator)
			_ = _5_keys
			var _6_valueOrError1 m_Wrappers.Result = Companion_Default___.ReadAADPairs(_2_ecPos, _4_accumulator, _5_keys, _1_count, _2_ecPos)
			_ = _6_valueOrError1
			if (_6_valueOrError1).IsFailure() {
				return (_6_valueOrError1).PropagateFailure()
			} else {
				var _let_tmp_rhs1 m_SerializeFunctions.SuccessfulRead = (_6_valueOrError1).Extract().(m_SerializeFunctions.SuccessfulRead)
				_ = _let_tmp_rhs1
				var _7_pairs _dafny.Sequence = _let_tmp_rhs1.Get_().(m_SerializeFunctions.SuccessfulRead_SuccessfulRead).Data.(_dafny.Sequence)
				_ = _7_pairs
				var _8_tail m_SerializeFunctions.ReadableBuffer = _let_tmp_rhs1.Get_().(m_SerializeFunctions.SuccessfulRead_SuccessfulRead).Tail
				_ = _8_tail
				return m_Wrappers.Companion_Result_.Create_Success_(m_SerializeFunctions.Companion_SuccessfulRead_.Create_SuccessfulRead_(_7_pairs, _8_tail))
			}
		}
	}
}
func (_static *CompanionStruct_Default___) ReadAADSection(buffer m_SerializeFunctions.ReadableBuffer) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.ReadUInt16(buffer)
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _1_length m_SerializeFunctions.SuccessfulRead = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
		_ = _1_length
		if ((_1_length).Dtor_data().(uint16)) == (uint16(0)) {
			var _2_empty _dafny.Sequence = _dafny.SeqOf()
			_ = _2_empty
			return m_Wrappers.Companion_Result_.Create_Success_(m_SerializeFunctions.Companion_SuccessfulRead_.Create_SuccessfulRead_(_2_empty, (_1_length).Dtor_tail()))
		} else {
			var _3_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(((((_1_length).Dtor_tail()).Dtor_start()).Plus(_dafny.IntOfUint16((_1_length).Dtor_data().(uint16)))).Cmp(_dafny.IntOfUint32((((_1_length).Dtor_tail()).Dtor_bytes()).Cardinality())) <= 0, m_SerializeFunctions.Companion_ReadProblems_.Create_MoreNeeded_((((_1_length).Dtor_tail()).Dtor_start()).Plus(_dafny.IntOfUint16((_1_length).Dtor_data().(uint16)))))
			_ = _3_valueOrError1
			if (_3_valueOrError1).IsFailure() {
				return (_3_valueOrError1).PropagateFailure()
			} else {
				var _4_valueOrError2 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.ReadUInt16((_1_length).Dtor_tail())
				_ = _4_valueOrError2
				if (_4_valueOrError2).IsFailure() {
					return (_4_valueOrError2).PropagateFailure()
				} else {
					var _5_verifyCount m_SerializeFunctions.SuccessfulRead = (_4_valueOrError2).Extract().(m_SerializeFunctions.SuccessfulRead)
					_ = _5_verifyCount
					if ((_1_length).Dtor_data().(uint16)) == (uint16(2)) {
						var _6_valueOrError3 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(((_5_verifyCount).Dtor_data().(uint16)) == (uint16(0)), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Encryption Context pairs count can not exceed byte length")))
						_ = _6_valueOrError3
						if (_6_valueOrError3).IsFailure() {
							return (_6_valueOrError3).PropagateFailure()
						} else {
							var _7_empty _dafny.Sequence = _dafny.SeqOf()
							_ = _7_empty
							return m_Wrappers.Companion_Result_.Create_Success_(m_SerializeFunctions.Companion_SuccessfulRead_.Create_SuccessfulRead_(_7_empty, (_5_verifyCount).Dtor_tail()))
						}
					} else {
						var _8_valueOrError4 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((uint16(0)) < ((_5_verifyCount).Dtor_data().(uint16)), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Encryption Context byte length exceeds pairs count.")))
						_ = _8_valueOrError4
						if (_8_valueOrError4).IsFailure() {
							return (_8_valueOrError4).PropagateFailure()
						} else {
							var _9_valueOrError5 m_Wrappers.Result = Companion_Default___.ReadAAD((_1_length).Dtor_tail())
							_ = _9_valueOrError5
							if (_9_valueOrError5).IsFailure() {
								return (_9_valueOrError5).PropagateFailure()
							} else {
								var _10_aad m_SerializeFunctions.SuccessfulRead = (_9_valueOrError5).Extract().(m_SerializeFunctions.SuccessfulRead)
								_ = _10_aad
								var _11_valueOrError6 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(((((_10_aad).Dtor_tail()).Dtor_start()).Minus(((_1_length).Dtor_tail()).Dtor_start())).Cmp(_dafny.IntOfUint16((_1_length).Dtor_data().(uint16))) == 0, m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("AAD Length did not match stored length.")))
								_ = _11_valueOrError6
								if (_11_valueOrError6).IsFailure() {
									return (_11_valueOrError6).PropagateFailure()
								} else {
									return m_Wrappers.Companion_Result_.Create_Success_(_10_aad)
								}
							}
						}
					}
				}
			}
		}
	}
}
func (_static *CompanionStruct_Default___) KeysToSet(pairs _dafny.Sequence) _dafny.Set {
	return func() _dafny.Set {
		var _coll0 = _dafny.NewBuilder()
		_ = _coll0
		for _iter1 := _dafny.Iterate((pairs).Elements()); ; {
			_compr_0, _ok1 := _iter1()
			if !_ok1 {
				break
			}
			var _0_p m_SerializableTypes.Pair
			_0_p = interface{}(_compr_0).(m_SerializableTypes.Pair)
			if _dafny.Companion_Sequence_.Contains(pairs, _0_p) {
				_coll0.Add((_0_p).Dtor_key())
			}
		}
		return _coll0.ToSet()
	}()
}

// End of class Default__

// Definition of class ESDKEncryptionContextPair
type ESDKEncryptionContextPair struct {
}

func New_ESDKEncryptionContextPair_() *ESDKEncryptionContextPair {
	_this := ESDKEncryptionContextPair{}

	return &_this
}

type CompanionStruct_ESDKEncryptionContextPair_ struct {
}

var Companion_ESDKEncryptionContextPair_ = CompanionStruct_ESDKEncryptionContextPair_{}

func (*ESDKEncryptionContextPair) String() string {
	return "EncryptionContext.ESDKEncryptionContextPair"
}

// End of class ESDKEncryptionContextPair

func Type_ESDKEncryptionContextPair_() _dafny.TypeDescriptor {
	return type_ESDKEncryptionContextPair_{}
}

type type_ESDKEncryptionContextPair_ struct {
}

func (_this type_ESDKEncryptionContextPair_) Default() interface{} {
	return m_SerializableTypes.Companion_Pair_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness(), m_UTF8.Companion_ValidUTF8Bytes_.Witness())
}

func (_this type_ESDKEncryptionContextPair_) String() string {
	return "EncryptionContext.ESDKEncryptionContextPair"
}
func (_this *CompanionStruct_ESDKEncryptionContextPair_) Is_(__source m_SerializableTypes.Pair) bool {
	var _0_p m_SerializableTypes.Pair = (__source)
	_ = _0_p
	return (((m_StandardLibrary_UInt.Companion_Default___.HasUint16Len((_0_p).Dtor_key().(_dafny.Sequence))) && (m_UTF8.Companion_Default___.ValidUTF8Seq((_0_p).Dtor_key().(_dafny.Sequence)))) && (m_StandardLibrary_UInt.Companion_Default___.HasUint16Len((_0_p).Dtor_value().(_dafny.Sequence)))) && (m_UTF8.Companion_Default___.ValidUTF8Seq((_0_p).Dtor_value().(_dafny.Sequence)))
}

// Definition of class ESDKCanonicalEncryptionContext
type ESDKCanonicalEncryptionContext struct {
}

func New_ESDKCanonicalEncryptionContext_() *ESDKCanonicalEncryptionContext {
	_this := ESDKCanonicalEncryptionContext{}

	return &_this
}

type CompanionStruct_ESDKCanonicalEncryptionContext_ struct {
}

var Companion_ESDKCanonicalEncryptionContext_ = CompanionStruct_ESDKCanonicalEncryptionContext_{}

func (*ESDKCanonicalEncryptionContext) String() string {
	return "EncryptionContext.ESDKCanonicalEncryptionContext"
}

// End of class ESDKCanonicalEncryptionContext

func Type_ESDKCanonicalEncryptionContext_() _dafny.TypeDescriptor {
	return type_ESDKCanonicalEncryptionContext_{}
}

type type_ESDKCanonicalEncryptionContext_ struct {
}

func (_this type_ESDKCanonicalEncryptionContext_) Default() interface{} {
	return _dafny.EmptySeq
}

func (_this type_ESDKCanonicalEncryptionContext_) String() string {
	return "EncryptionContext.ESDKCanonicalEncryptionContext"
}
