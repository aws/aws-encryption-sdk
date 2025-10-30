// Package SerializableTypes
// Dafny module SerializableTypes compiled into Go

package SerializableTypes

import (
	os "os"

	m_ComAmazonawsDynamodbTypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/dynamodb/ComAmazonawsDynamodbTypes"
	m_Com_Amazonaws_Dynamodb "github.com/aws/aws-cryptographic-material-providers-library/releases/go/dynamodb/Com_Amazonaws_Dynamodb"
	m_ComAmazonawsKmsTypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/kms/ComAmazonawsKmsTypes"
	m_Com_Amazonaws_Kms "github.com/aws/aws-cryptographic-material-providers-library/releases/go/kms/Com_Amazonaws_Kms"
	m_AlgorithmSuites "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AlgorithmSuites"
	m_AwsArnParsing "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsArnParsing"
	m_AwsCryptographyKeyStoreOperations "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsCryptographyKeyStoreOperations"
	m_AwsCryptographyKeyStoreTypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsCryptographyKeyStoreTypes"
	m_AwsCryptographyMaterialProvidersOperations "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsCryptographyMaterialProvidersOperations"
	m_AwsCryptographyMaterialProvidersTypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsCryptographyMaterialProvidersTypes"
	m_AwsKmsDiscoveryKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsDiscoveryKeyring"
	m_AwsKmsEcdhKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsEcdhKeyring"
	m_AwsKmsHierarchicalKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsHierarchicalKeyring"
	m_AwsKmsKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsKeyring"
	m_AwsKmsMrkAreUnique "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsMrkAreUnique"
	m_AwsKmsMrkDiscoveryKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsMrkDiscoveryKeyring"
	m_AwsKmsMrkKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsMrkKeyring"
	m_AwsKmsMrkMatchForDecrypt "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsMrkMatchForDecrypt"
	m_AwsKmsRsaKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsRsaKeyring"
	m_AwsKmsUtils "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/AwsKmsUtils"
	m_CMM "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/CMM"
	m_CacheConstants "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/CacheConstants"
	m_CanonicalEncryptionContext "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/CanonicalEncryptionContext"
	m_Commitment "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/Commitment"
	m_Constants "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/Constants"
	m_CreateKeyStoreTable "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/CreateKeyStoreTable"
	m_CreateKeys "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/CreateKeys"
	m_DDBKeystoreOperations "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/DDBKeystoreOperations"
	m_DefaultCMM "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/DefaultCMM"
	m_DefaultClientSupplier "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/DefaultClientSupplier"
	m_Defaults "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/Defaults"
	m_DiscoveryMultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/DiscoveryMultiKeyring"
	m_EcdhEdkWrapping "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/EcdhEdkWrapping"
	m_EdkWrapping "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/EdkWrapping"
	m_ErrorMessages "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/ErrorMessages"
	m_GetKeys "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/GetKeys"
	m_IntermediateKeyWrapping "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/IntermediateKeyWrapping"
	m_KMSKeystoreOperations "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/KMSKeystoreOperations"
	m_KeyStore "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/KeyStore"
	m_KeyStoreErrorMessages "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/KeyStoreErrorMessages"
	m_Keyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/Keyring"
	m_KmsArn "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/KmsArn"
	m_LocalCMC "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/LocalCMC"
	m_MaterialProviders "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/MaterialProviders"
	m_MaterialWrapping "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/MaterialWrapping"
	m_Materials "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/Materials"
	m_MrkAwareDiscoveryMultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/MrkAwareDiscoveryMultiKeyring"
	m_MrkAwareStrictMultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/MrkAwareStrictMultiKeyring"
	m_MultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/MultiKeyring"
	m_RawAESKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/RawAESKeyring"
	m_RawECDHKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/RawECDHKeyring"
	m_RawRSAKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/RawRSAKeyring"
	m_RequiredEncryptionContextCMM "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/RequiredEncryptionContextCMM"
	m_StormTracker "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/StormTracker"
	m_StormTrackingCMC "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/StormTrackingCMC"
	m_StrictMultiKeyring "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/StrictMultiKeyring"
	m_Structure "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/Structure"
	m_SynchronizedLocalCMC "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/SynchronizedLocalCMC"
	m_Utils "github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl/Utils"
	m_AtomicPrimitives "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/AtomicPrimitives"
	m_AwsCryptographyPrimitivesOperations "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/AwsCryptographyPrimitivesOperations"
	m_AwsCryptographyPrimitivesTypes "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/AwsCryptographyPrimitivesTypes"
	m_Digest "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/Digest"
	m_HKDF "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/HKDF"
	m_KdfCtr "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/KdfCtr"
	m_Random "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/Random"
	m_WrappedHKDF "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/WrappedHKDF"
	m_WrappedHMAC "github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives/WrappedHMAC"
	m_Actions "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Actions"
	m_Base64 "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Base64"
	m_Base64Lemmas "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Base64Lemmas"
	m_BoundedInts "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/BoundedInts"
	m_DivInternals "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/DivInternals"
	m_DivInternalsNonlinear "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/DivInternalsNonlinear"
	m_DivMod "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/DivMod"
	m_FileIO "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/FileIO"
	m_FloatCompare "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/FloatCompare"
	m_Functions "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Functions"
	m_GeneralInternals "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/GeneralInternals"
	m_GetOpt "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/GetOpt"
	m_HexStrings "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/HexStrings"
	m_Logarithm "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Logarithm"
	m__Math "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Math_"
	m_ModInternals "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/ModInternals"
	m_ModInternalsNonlinear "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/ModInternalsNonlinear"
	m_Mul "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Mul"
	m_MulInternals "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/MulInternals"
	m_MulInternalsNonlinear "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/MulInternalsNonlinear"
	m_Power "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Power"
	m_Relations "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Relations"
	m_Seq "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Seq"
	m_Seq_MergeSort "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Seq_MergeSort"
	m_SortedSets "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/SortedSets"
	m_Sorting "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Sorting"
	m_StandardLibrary "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/StandardLibrary"
	m_StandardLibraryInterop "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/StandardLibraryInterop"
	m_StandardLibrary_MemoryMath "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/StandardLibrary_MemoryMath"
	m_StandardLibrary_Sequence "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/StandardLibrary_Sequence"
	m_StandardLibrary_String "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/StandardLibrary_String"
	m_StandardLibrary_UInt "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/StandardLibrary_UInt"
	m_Streams "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Streams"
	m_UTF8 "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/UTF8"
	m_UnicodeStrings "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/UnicodeStrings"
	m__Unicode "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Unicode_"
	m_Utf16EncodingForm "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Utf16EncodingForm"
	m_Utf8EncodingForm "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Utf8EncodingForm"
	m_Wrappers "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Wrappers"
	m_AwsCryptographyEncryptionSdkTypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/AwsCryptographyEncryptionSdkTypes"
	m__System "github.com/dafny-lang/DafnyRuntimeGo/v4/System_"
	_dafny "github.com/dafny-lang/DafnyRuntimeGo/v4/dafny"
)

var _ = os.Args
var _ _dafny.Dummy__
var _ m__System.Dummy__
var _ m_Wrappers.Dummy__
var _ m_BoundedInts.Dummy__
var _ m_StandardLibrary_UInt.Dummy__
var _ m_StandardLibrary_MemoryMath.Dummy__
var _ m_StandardLibrary_Sequence.Dummy__
var _ m_StandardLibrary_String.Dummy__
var _ m_StandardLibrary.Dummy__
var _ m_AwsCryptographyPrimitivesTypes.Dummy__
var _ m_Random.Dummy__
var _ m_Digest.Dummy__
var _ m_WrappedHMAC.Dummy__
var _ m_HKDF.Dummy__
var _ m_WrappedHKDF.Dummy__
var _ m_KdfCtr.Dummy__
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
var _ m_Actions.Dummy__
var _ m_CanonicalEncryptionContext.Dummy__
var _ m_MaterialWrapping.Dummy__
var _ m_IntermediateKeyWrapping.Dummy__
var _ m_EdkWrapping.Dummy__
var _ m_ErrorMessages.Dummy__
var _ m_RawAESKeyring.Dummy__
var _ m_AwsArnParsing.Dummy__
var _ m_Constants.Dummy__
var _ m_EcdhEdkWrapping.Dummy__
var _ m_RawECDHKeyring.Dummy__
var _ m_RawRSAKeyring.Dummy__
var _ m_AwsKmsMrkMatchForDecrypt.Dummy__
var _ m_AwsKmsUtils.Dummy__
var _ m_AwsKmsKeyring.Dummy__
var _ m_AwsKmsDiscoveryKeyring.Dummy__
var _ m_AwsKmsEcdhKeyring.Dummy__
var _ m_FileIO.Dummy__
var _ m_LocalCMC.Dummy__
var _ m_SynchronizedLocalCMC.Dummy__
var _ m_StormTracker.Dummy__
var _ m_StormTrackingCMC.Dummy__
var _ m_CacheConstants.Dummy__
var _ m_AwsKmsHierarchicalKeyring.Dummy__
var _ m_AwsKmsMrkDiscoveryKeyring.Dummy__
var _ m_AwsKmsMrkKeyring.Dummy__
var _ m_AwsKmsRsaKeyring.Dummy__
var _ m_MultiKeyring.Dummy__
var _ m_AwsKmsMrkAreUnique.Dummy__
var _ m_StrictMultiKeyring.Dummy__
var _ m_Com_Amazonaws_Kms.Dummy__
var _ m_Com_Amazonaws_Dynamodb.Dummy__
var _ m_DiscoveryMultiKeyring.Dummy__
var _ m_MrkAwareDiscoveryMultiKeyring.Dummy__
var _ m_MrkAwareStrictMultiKeyring.Dummy__
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
	return (((uint64((ec).CardinalityInt())) < ((m_StandardLibrary_UInt.Companion_Default___.UINT16__LIMIT()).Uint64())) && ((Companion_Default___.Length(ec)) < (Companion_Default___.ESDK__CANONICAL__ENCRYPTION__CONTEXT__MAX__LENGTH()))) && (_dafny.Quantifier(((_dafny.MultiSetFromSet((ec).Keys())).Union(_dafny.MultiSetFromSet((ec).Values()))).UniqueElements(), true, func(_forall_var_0 _dafny.Sequence) bool {
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
func (_static *CompanionStruct_Default___) GetIvLengthZeros(a m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo) _dafny.Sequence {
	var _0_len uint8 = Companion_Default___.GetIvLength(a)
	_ = _0_len
	if (_0_len) == (uint8(12)) {
		return _dafny.SeqOf(uint8(0), uint8(0), uint8(0), uint8(0), uint8(0), uint8(0), uint8(0), uint8(0), uint8(0), uint8(0), uint8(0), uint8(0))
	} else {
		return _dafny.SeqCreate(uint32(_0_len), func(coer0 func(_dafny.Int) uint8) func(_dafny.Int) interface{} {
			return func(arg0 _dafny.Int) interface{} {
				return coer0(arg0)
			}
		}(func(_1___v0 _dafny.Int) uint8 {
			return uint8(0)
		}))
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
func (_static *CompanionStruct_Default___) Length(encryptionContext _dafny.Map) uint64 {
	if (uint64((encryptionContext).CardinalityInt())) == (uint64(0)) {
		return uint64(0)
	} else {
		var _0_pairs _dafny.Sequence = Companion_Default___.GetCanonicalLinearPairs(encryptionContext)
		_ = _0_pairs
		return Companion_Default___.LinearLength(_0_pairs)
	}
}
func (_static *CompanionStruct_Default___) GetCanonicalLinearPairs(encryptionContext _dafny.Map) _dafny.Sequence {
	var _0_keys _dafny.Sequence = m_SortedSets.SetToOrderedSequence2((encryptionContext).Keys(), func(coer1 func(uint8, uint8) bool) func(interface{}, interface{}) bool {
		return func(arg1 interface{}, arg2 interface{}) bool {
			return coer1(arg1.(uint8), arg2.(uint8))
		}
	}(m_StandardLibrary_UInt.Companion_Default___.UInt8Less))
	_ = _0_keys
	return _dafny.SeqCreate((_dafny.IntOfUint32((_0_keys).Cardinality())).Uint32(), func(coer2 func(_dafny.Int) Pair) func(_dafny.Int) interface{} {
		return func(arg3 _dafny.Int) interface{} {
			return coer2(arg3)
		}
	}((func(_1_keys _dafny.Sequence, _2_encryptionContext _dafny.Map) func(_dafny.Int) Pair {
		return func(_3_i _dafny.Int) Pair {
			return Companion_Pair_.Create_Pair_((_1_keys).Select((_3_i).Uint32()).(_dafny.Sequence), (_2_encryptionContext).Get((_1_keys).Select((_3_i).Uint32()).(_dafny.Sequence)).(_dafny.Sequence))
		}
	})(_0_keys, encryptionContext)))
}
func (_static *CompanionStruct_Default___) LinearLength(pairs _dafny.Sequence) uint64 {
	var ret uint64 = uint64(0)
	_ = ret
	var _0_result uint64
	_ = _0_result
	_0_result = uint64(0)
	var _hi0 uint64 = uint64((pairs).Cardinality())
	_ = _hi0
	for _1_i := uint64(0); _1_i < _hi0; _1_i++ {
		_0_result = m_StandardLibrary_MemoryMath.Companion_Default___.Add(_0_result, Companion_Default___.PairLength((pairs).Select(uint32(_1_i)).(Pair)))
	}
	ret = _0_result
	return ret
	return ret
}
func (_static *CompanionStruct_Default___) PairLength(pair Pair) uint64 {
	return m_StandardLibrary_MemoryMath.Companion_Default___.Add4(uint64(2), uint64(((pair).Dtor_key().(_dafny.Sequence)).Cardinality()), uint64(2), uint64(((pair).Dtor_value().(_dafny.Sequence)).Cardinality()))
}
func (_static *CompanionStruct_Default___) ESDK__CANONICAL__ENCRYPTION__CONTEXT__MAX__LENGTH() uint64 {
	return ((m_StandardLibrary_UInt.Companion_Default___.UINT16__LIMIT()).Uint64()) - (func() uint64 { return (uint64(2)) })()
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
