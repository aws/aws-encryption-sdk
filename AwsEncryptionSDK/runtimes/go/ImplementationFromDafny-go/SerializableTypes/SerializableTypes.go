// Package SerializableTypes
// Dafny module SerializableTypes compiled into Go

package SerializableTypes

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
	m_SortedSets "github.com/dafny-lang/DafnyStandardLibGo/SortedSets"
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
	return "SerializableTypes.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) IsESDKEncryptedDataKey(edk m_AwsCryptographyMaterialProvidersTypes.EncryptedDataKey) bool {
	return (((m_StandardLibrary_UInt.Companion_Default___.HasUint16Len((edk).Dtor_keyProviderId())) && (m_UTF8.Companion_Default___.ValidUTF8Seq((edk).Dtor_keyProviderId()))) && (m_StandardLibrary_UInt.Companion_Default___.HasUint16Len((edk).Dtor_keyProviderInfo()))) && (m_StandardLibrary_UInt.Companion_Default___.HasUint16Len((edk).Dtor_ciphertext()))
}
func (_static *CompanionStruct_Default___) IsESDKEncryptionContext(ec _dafny.Map) bool {
	return ((((ec).Cardinality()).Cmp(m_StandardLibrary_UInt.Companion_Default___.UINT16__LIMIT()) < 0) && ((Companion_Default___.Length(ec)).Cmp(Companion_Default___.ESDK__CANONICAL__ENCRYPTION__CONTEXT__MAX__LENGTH()) < 0)) && (_dafny.Quantifier(((_dafny.MultiSetFromSet((ec).Keys())).Union(_dafny.MultiSetFromSet((ec).Values()))).UniqueElements(), true, func(_forall_var_0 _dafny.Sequence) bool {
		var _0_element _dafny.Sequence
		_0_element = interface{}(_forall_var_0).(_dafny.Sequence)
		if m_UTF8.Companion_ValidUTF8Bytes_.Is_(_0_element) {
			return !(((_dafny.MultiSetFromSet((ec).Keys())).Union(_dafny.MultiSetFromSet((ec).Values()))).Contains(_0_element)) || ((m_StandardLibrary_UInt.Companion_Default___.HasUint16Len(_0_element)) && (m_UTF8.Companion_Default___.ValidUTF8Seq(_0_element)))
		} else {
			return true
		}
	}))
}
func (_static *CompanionStruct_Default___) GetIvLength(a m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo) uint8 {
	var _source0 m_AwsCryptographyMaterialProvidersTypes.Encrypt = (a).Dtor_encrypt()
	_ = _source0
	{
		var _0_e m_AwsCryptographyPrimitivesTypes.AES__GCM = _source0.Get_().(m_AwsCryptographyMaterialProvidersTypes.Encrypt_AES__GCM).AES__GCM
		_ = _0_e
		return uint8((_0_e).Dtor_ivLength())
	}
}
func (_static *CompanionStruct_Default___) GetTagLength(a m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo) uint8 {
	var _source0 m_AwsCryptographyMaterialProvidersTypes.Encrypt = (a).Dtor_encrypt()
	_ = _source0
	{
		var _0_e m_AwsCryptographyPrimitivesTypes.AES__GCM = _source0.Get_().(m_AwsCryptographyMaterialProvidersTypes.Encrypt_AES__GCM).AES__GCM
		_ = _0_e
		return uint8((_0_e).Dtor_tagLength())
	}
}
func (_static *CompanionStruct_Default___) GetEncryptKeyLength(a m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo) int32 {
	var _source0 m_AwsCryptographyMaterialProvidersTypes.Encrypt = (a).Dtor_encrypt()
	_ = _source0
	{
		var _0_e m_AwsCryptographyPrimitivesTypes.AES__GCM = _source0.Get_().(m_AwsCryptographyMaterialProvidersTypes.Encrypt_AES__GCM).AES__GCM
		_ = _0_e
		return (_0_e).Dtor_keyLength()
	}
}
func (_static *CompanionStruct_Default___) Length(encryptionContext _dafny.Map) _dafny.Int {
	if ((encryptionContext).Cardinality()).Sign() == 0 {
		return _dafny.Zero
	} else {
		var _0_pairs _dafny.Sequence = Companion_Default___.GetCanonicalLinearPairs(encryptionContext)
		_ = _0_pairs
		return Companion_Default___.LinearLength(_0_pairs)
	}
}
func (_static *CompanionStruct_Default___) GetCanonicalLinearPairs(encryptionContext _dafny.Map) _dafny.Sequence {
	var _0_keys _dafny.Sequence = m_SortedSets.SetToOrderedSequence2((encryptionContext).Keys(), func(coer0 func(uint8, uint8) bool) func(interface{}, interface{}) bool {
		return func(arg0 interface{}, arg1 interface{}) bool {
			return coer0(arg0.(uint8), arg1.(uint8))
		}
	}(m_StandardLibrary_UInt.Companion_Default___.UInt8Less))
	_ = _0_keys
	return _dafny.SeqCreate((_dafny.IntOfUint32((_0_keys).Cardinality())).Uint32(), func(coer1 func(_dafny.Int) Pair) func(_dafny.Int) interface{} {
		return func(arg2 _dafny.Int) interface{} {
			return coer1(arg2)
		}
	}((func(_1_keys _dafny.Sequence, _2_encryptionContext _dafny.Map) func(_dafny.Int) Pair {
		return func(_3_i _dafny.Int) Pair {
			return Companion_Pair_.Create_Pair_((_1_keys).Select((_3_i).Uint32()).(_dafny.Sequence), (_2_encryptionContext).Get((_1_keys).Select((_3_i).Uint32()).(_dafny.Sequence)).(_dafny.Sequence))
		}
	})(_0_keys, encryptionContext)))
}
func (_static *CompanionStruct_Default___) LinearLength(pairs _dafny.Sequence) _dafny.Int {
	var _0___accumulator _dafny.Int = _dafny.Zero
	_ = _0___accumulator
	goto TAIL_CALL_START
TAIL_CALL_START:
	if (_dafny.IntOfUint32((pairs).Cardinality())).Sign() == 0 {
		return (_dafny.Zero).Plus(_0___accumulator)
	} else {
		_0___accumulator = (Companion_Default___.PairLength(m_Seq.Companion_Default___.Last(pairs).(Pair))).Plus(_0___accumulator)
		var _in0 _dafny.Sequence = m_Seq.Companion_Default___.DropLast(pairs)
		_ = _in0
		pairs = _in0
		goto TAIL_CALL_START
	}
}
func (_static *CompanionStruct_Default___) PairLength(pair Pair) _dafny.Int {
	return (((_dafny.IntOfInt64(2)).Plus(_dafny.IntOfUint32(((pair).Dtor_key().(_dafny.Sequence)).Cardinality()))).Plus(_dafny.IntOfInt64(2))).Plus(_dafny.IntOfUint32(((pair).Dtor_value().(_dafny.Sequence)).Cardinality()))
}
func (_static *CompanionStruct_Default___) ESDK__CANONICAL__ENCRYPTION__CONTEXT__MAX__LENGTH() _dafny.Int {
	return (m_StandardLibrary_UInt.Companion_Default___.UINT16__LIMIT()).Minus(_dafny.IntOfInt64(2))
}

// End of class Default__

// Definition of class ShortUTF8Seq
type ShortUTF8Seq struct {
}

func New_ShortUTF8Seq_() *ShortUTF8Seq {
	_this := ShortUTF8Seq{}

	return &_this
}

type CompanionStruct_ShortUTF8Seq_ struct {
}

var Companion_ShortUTF8Seq_ = CompanionStruct_ShortUTF8Seq_{}

func (*ShortUTF8Seq) String() string {
	return "SerializableTypes.ShortUTF8Seq"
}

// End of class ShortUTF8Seq

func Type_ShortUTF8Seq_() _dafny.TypeDescriptor {
	return type_ShortUTF8Seq_{}
}

type type_ShortUTF8Seq_ struct {
}

func (_this type_ShortUTF8Seq_) Default() interface{} {
	return m_UTF8.Companion_ValidUTF8Bytes_.Witness()
}

func (_this type_ShortUTF8Seq_) String() string {
	return "SerializableTypes.ShortUTF8Seq"
}
func (_this *CompanionStruct_ShortUTF8Seq_) Is_(__source _dafny.Sequence) bool {
	var _0_s _dafny.Sequence = (__source)
	_ = _0_s
	if m_UTF8.Companion_ValidUTF8Bytes_.Is_(_0_s) {
		return m_StandardLibrary_UInt.Companion_Default___.HasUint16Len(_0_s)
	}
	return false
}

// Definition of datatype Pair
type Pair struct {
	Data_Pair_
}

func (_this Pair) Get_() Data_Pair_ {
	return _this.Data_Pair_
}

type Data_Pair_ interface {
	isPair()
}

type CompanionStruct_Pair_ struct {
}

var Companion_Pair_ = CompanionStruct_Pair_{}

type Pair_Pair struct {
	Key   interface{}
	Value interface{}
}

func (Pair_Pair) isPair() {}

func (CompanionStruct_Pair_) Create_Pair_(Key interface{}, Value interface{}) Pair {
	return Pair{Pair_Pair{Key, Value}}
}

func (_this Pair) Is_Pair() bool {
	_, ok := _this.Get_().(Pair_Pair)
	return ok
}

func (CompanionStruct_Pair_) Default(_default_K interface{}, _default_V interface{}) Pair {
	return Companion_Pair_.Create_Pair_(_default_K, _default_V)
}

func (_this Pair) Dtor_key() interface{} {
	return _this.Get_().(Pair_Pair).Key
}

func (_this Pair) Dtor_value() interface{} {
	return _this.Get_().(Pair_Pair).Value
}

func (_this Pair) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case Pair_Pair:
		{
			return "SerializableTypes.Pair.Pair" + "(" + _dafny.String(data.Key) + ", " + _dafny.String(data.Value) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this Pair) Equals(other Pair) bool {
	switch data1 := _this.Get_().(type) {
	case Pair_Pair:
		{
			data2, ok := other.Get_().(Pair_Pair)
			return ok && _dafny.AreEqual(data1.Key, data2.Key) && _dafny.AreEqual(data1.Value, data2.Value)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this Pair) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(Pair)
	return ok && _this.Equals(typed)
}

func Type_Pair_(Type_K_ _dafny.TypeDescriptor, Type_V_ _dafny.TypeDescriptor) _dafny.TypeDescriptor {
	return type_Pair_{Type_K_, Type_V_}
}

type type_Pair_ struct {
	Type_K_ _dafny.TypeDescriptor
	Type_V_ _dafny.TypeDescriptor
}

func (_this type_Pair_) Default() interface{} {
	Type_K_ := _this.Type_K_
	_ = Type_K_
	Type_V_ := _this.Type_V_
	_ = Type_V_
	return Companion_Pair_.Default(Type_K_.Default(), Type_V_.Default())
}

func (_this type_Pair_) String() string {
	return "SerializableTypes.Pair"
}
func (_this Pair) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = Pair{}

// End of datatype Pair

// Definition of class ESDKEncryptedDataKey
type ESDKEncryptedDataKey struct {
}

func New_ESDKEncryptedDataKey_() *ESDKEncryptedDataKey {
	_this := ESDKEncryptedDataKey{}

	return &_this
}

type CompanionStruct_ESDKEncryptedDataKey_ struct {
}

var Companion_ESDKEncryptedDataKey_ = CompanionStruct_ESDKEncryptedDataKey_{}

func (*ESDKEncryptedDataKey) String() string {
	return "SerializableTypes.ESDKEncryptedDataKey"
}

// End of class ESDKEncryptedDataKey

func Type_ESDKEncryptedDataKey_() _dafny.TypeDescriptor {
	return type_ESDKEncryptedDataKey_{}
}

type type_ESDKEncryptedDataKey_ struct {
}

func (_this type_ESDKEncryptedDataKey_) Default() interface{} {
	return m_AwsCryptographyMaterialProvidersTypes.Companion_EncryptedDataKey_.Default()
}

func (_this type_ESDKEncryptedDataKey_) String() string {
	return "SerializableTypes.ESDKEncryptedDataKey"
}
func (_this *CompanionStruct_ESDKEncryptedDataKey_) Is_(__source m_AwsCryptographyMaterialProvidersTypes.EncryptedDataKey) bool {
	var _1_e m_AwsCryptographyMaterialProvidersTypes.EncryptedDataKey = (__source)
	_ = _1_e
	return Companion_Default___.IsESDKEncryptedDataKey(_1_e)
}

// Definition of class ESDKEncryptionContext
type ESDKEncryptionContext struct {
}

func New_ESDKEncryptionContext_() *ESDKEncryptionContext {
	_this := ESDKEncryptionContext{}

	return &_this
}

type CompanionStruct_ESDKEncryptionContext_ struct {
}

var Companion_ESDKEncryptionContext_ = CompanionStruct_ESDKEncryptionContext_{}

func (*ESDKEncryptionContext) String() string {
	return "SerializableTypes.ESDKEncryptionContext"
}

// End of class ESDKEncryptionContext

func Type_ESDKEncryptionContext_() _dafny.TypeDescriptor {
	return type_ESDKEncryptionContext_{}
}

type type_ESDKEncryptionContext_ struct {
}

func (_this type_ESDKEncryptionContext_) Default() interface{} {
	return _dafny.EmptyMap
}

func (_this type_ESDKEncryptionContext_) String() string {
	return "SerializableTypes.ESDKEncryptionContext"
}
func (_this *CompanionStruct_ESDKEncryptionContext_) Is_(__source _dafny.Map) bool {
	var _2_ec _dafny.Map = (__source)
	_ = _2_ec
	return Companion_Default___.IsESDKEncryptionContext(_2_ec)
}
