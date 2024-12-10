// Package Fixtures
// Dafny module Fixtures compiled into Go

package Fixtures

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
	m_AwsEncryptionSdkOperations "github.com/aws/aws-encryption-sdk/AwsEncryptionSdkOperations"
	m_EncryptDecryptHelpers "github.com/aws/aws-encryption-sdk/EncryptDecryptHelpers"
	m_EncryptedDataKeys "github.com/aws/aws-encryption-sdk/EncryptedDataKeys"
	m_EncryptionContext "github.com/aws/aws-encryption-sdk/EncryptionContext"
	m_EncryptionSdk "github.com/aws/aws-encryption-sdk/EncryptionSdk"
	m_Frames "github.com/aws/aws-encryption-sdk/Frames"
	m_Header "github.com/aws/aws-encryption-sdk/Header"
	m_HeaderAuth "github.com/aws/aws-encryption-sdk/HeaderAuth"
	m_HeaderTypes "github.com/aws/aws-encryption-sdk/HeaderTypes"
	m_KeyDerivation "github.com/aws/aws-encryption-sdk/KeyDerivation"
	m_MessageBody "github.com/aws/aws-encryption-sdk/MessageBody"
	m_SerializableTypes "github.com/aws/aws-encryption-sdk/SerializableTypes"
	m_SerializeFunctions "github.com/aws/aws-encryption-sdk/SerializeFunctions"
	m_SharedHeaderFunctions "github.com/aws/aws-encryption-sdk/SharedHeaderFunctions"
	m_V1HeaderBody "github.com/aws/aws-encryption-sdk/V1HeaderBody"
	m_V2HeaderBody "github.com/aws/aws-encryption-sdk/V2HeaderBody"
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
var _ m_ComAmazonawsDynamodbTypes.Dummy__
var _ m_ComAmazonawsKmsTypes.Dummy__
var _ m_AwsCryptographyKeyStoreTypes.Dummy__
var _ m_AwsCryptographyPrimitivesTypes.Dummy__
var _ m_AwsCryptographyMaterialProvidersTypes.Dummy__
var _ m_AwsCryptographyEncryptionSdkTypes.Dummy__
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
var _ m_Streams.Dummy__
var _ m_SerializableTypes.Dummy__
var _ m_SerializeFunctions.Dummy__
var _ m_EncryptionContext.Dummy__
var _ m_HeaderTypes.Dummy__
var _ m_SharedHeaderFunctions.Dummy__
var _ m_EncryptedDataKeys.Dummy__
var _ m_V1HeaderBody.Dummy__
var _ m_V2HeaderBody.Dummy__
var _ m_HeaderAuth.Dummy__
var _ m_Header.Dummy__
var _ m_Frames.Dummy__
var _ m_MessageBody.Dummy__
var _ m_KeyDerivation.Dummy__
var _ m_EncryptDecryptHelpers.Dummy__
var _ m_AwsEncryptionSdkOperations.Dummy__
var _ m_EncryptionSdk.Dummy__
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
var _ m_Sorting.Dummy__
var _ m_HexStrings.Dummy__
var _ m_GetOpt.Dummy__
var _ m_FloatCompare.Dummy__
var _ m_Base64Lemmas.Dummy__

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
	return "Fixtures.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) SmallEncryptionContext(v SmallEncryptionContextVariation) _dafny.Map {
	var encryptionContext _dafny.Map = _dafny.EmptyMap
	_ = encryptionContext
	var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _0_valueOrError0
	_0_valueOrError0 = m_UTF8.Encode(_dafny.SeqOfString("keyA"))
	if !(!((_0_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(36,16): " + (_0_valueOrError0).String())
	}
	var _1_keyA _dafny.Sequence
	_ = _1_keyA
	_1_keyA = (_0_valueOrError0).Extract().(_dafny.Sequence)
	var _2_valueOrError1 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _2_valueOrError1
	_2_valueOrError1 = m_UTF8.Encode(_dafny.SeqOfString("valA"))
	if !(!((_2_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(37,16): " + (_2_valueOrError1).String())
	}
	var _3_valA _dafny.Sequence
	_ = _3_valA
	_3_valA = (_2_valueOrError1).Extract().(_dafny.Sequence)
	var _4_valueOrError2 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _4_valueOrError2
	_4_valueOrError2 = m_UTF8.Encode(_dafny.SeqOfString("keyB"))
	if !(!((_4_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(38,16): " + (_4_valueOrError2).String())
	}
	var _5_keyB _dafny.Sequence
	_ = _5_keyB
	_5_keyB = (_4_valueOrError2).Extract().(_dafny.Sequence)
	var _6_valueOrError3 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _6_valueOrError3
	_6_valueOrError3 = m_UTF8.Encode(_dafny.SeqOfString("valB"))
	if !(!((_6_valueOrError3).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(39,16): " + (_6_valueOrError3).String())
	}
	var _7_valB _dafny.Sequence
	_ = _7_valB
	_7_valB = (_6_valueOrError3).Extract().(_dafny.Sequence)
	var _8_valueOrError4 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _8_valueOrError4
	_8_valueOrError4 = m_UTF8.Encode(_dafny.SeqOfString("keyC"))
	if !(!((_8_valueOrError4).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(40,16): " + (_8_valueOrError4).String())
	}
	var _9_keyC _dafny.Sequence
	_ = _9_keyC
	_9_keyC = (_8_valueOrError4).Extract().(_dafny.Sequence)
	var _10_valueOrError5 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _10_valueOrError5
	_10_valueOrError5 = m_UTF8.Encode(_dafny.SeqOfString("valC"))
	if !(!((_10_valueOrError5).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(41,16): " + (_10_valueOrError5).String())
	}
	var _11_valC _dafny.Sequence
	_ = _11_valC
	_11_valC = (_10_valueOrError5).Extract().(_dafny.Sequence)
	var _12_valueOrError6 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _12_valueOrError6
	_12_valueOrError6 = m_UTF8.Encode(_dafny.SeqOfString("keyD"))
	if !(!((_12_valueOrError6).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(42,16): " + (_12_valueOrError6).String())
	}
	var _13_keyD _dafny.Sequence
	_ = _13_keyD
	_13_keyD = (_12_valueOrError6).Extract().(_dafny.Sequence)
	var _14_valueOrError7 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _14_valueOrError7
	_14_valueOrError7 = m_UTF8.Encode(_dafny.SeqOfString("valD"))
	if !(!((_14_valueOrError7).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(43,16): " + (_14_valueOrError7).String())
	}
	var _15_valD _dafny.Sequence
	_ = _15_valD
	_15_valD = (_14_valueOrError7).Extract().(_dafny.Sequence)
	var _source0 SmallEncryptionContextVariation = v
	_ = _source0
	{
		{
			if _source0.Is_Empty() {
				encryptionContext = _dafny.NewMapBuilder().ToMap()
				goto Lmatch0
			}
		}
		{
			if _source0.Is_A() {
				encryptionContext = _dafny.NewMapBuilder().ToMap().UpdateUnsafe(_1_keyA, _3_valA)
				goto Lmatch0
			}
		}
		{
			if _source0.Is_B() {
				encryptionContext = _dafny.NewMapBuilder().ToMap().UpdateUnsafe(_5_keyB, _7_valB)
				goto Lmatch0
			}
		}
		{
			if _source0.Is_AB() {
				encryptionContext = _dafny.NewMapBuilder().ToMap().UpdateUnsafe(_1_keyA, _3_valA).UpdateUnsafe(_5_keyB, _7_valB)
				goto Lmatch0
			}
		}
		{
			if _source0.Is_BA() {
				encryptionContext = _dafny.NewMapBuilder().ToMap().UpdateUnsafe(_5_keyB, _7_valB).UpdateUnsafe(_1_keyA, _3_valA)
				goto Lmatch0
			}
		}
		{
			if _source0.Is_C() {
				encryptionContext = _dafny.NewMapBuilder().ToMap().UpdateUnsafe(_9_keyC, _11_valC)
				goto Lmatch0
			}
		}
		{
			var _16_CE SmallEncryptionContextVariation = _source0
			_ = _16_CE
			encryptionContext = _dafny.NewMapBuilder().ToMap().UpdateUnsafe(_9_keyC, _11_valC).UpdateUnsafe(_13_keyD, _15_valD)
		}
		goto Lmatch0
	}
Lmatch0:
	return encryptionContext
}
func (_static *CompanionStruct_Default___) GetResrvedECMap() _dafny.Map {
	var encryptionContext _dafny.Map = _dafny.EmptyMap
	_ = encryptionContext
	var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _0_valueOrError0
	_0_valueOrError0 = m_UTF8.Encode(_dafny.SeqOfString("aws-crypto-public-key"))
	if !(!((_0_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(67,23): " + (_0_valueOrError0).String())
	}
	var _1_reservedKey _dafny.Sequence
	_ = _1_reservedKey
	_1_reservedKey = (_0_valueOrError0).Extract().(_dafny.Sequence)
	var _2_valueOrError1 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _2_valueOrError1
	_2_valueOrError1 = m_UTF8.Encode(_dafny.SeqOfString("not a real public key"))
	if !(!((_2_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(68,15): " + (_2_valueOrError1).String())
	}
	var _3_val _dafny.Sequence
	_ = _3_val
	_3_val = (_2_valueOrError1).Extract().(_dafny.Sequence)
	encryptionContext = _dafny.NewMapBuilder().ToMap().UpdateUnsafe(_1_reservedKey, _3_val)
	return encryptionContext
}
func (_static *CompanionStruct_Default___) SmallEncryptionContextKeys(v SmallEncryptionContextVariation) _dafny.Sequence {
	var encryptionContextKeys _dafny.Sequence = _dafny.EmptySeq
	_ = encryptionContextKeys
	var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _0_valueOrError0
	_0_valueOrError0 = m_UTF8.Encode(_dafny.SeqOfString("keyA"))
	if !(!((_0_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(76,16): " + (_0_valueOrError0).String())
	}
	var _1_keyA _dafny.Sequence
	_ = _1_keyA
	_1_keyA = (_0_valueOrError0).Extract().(_dafny.Sequence)
	var _2_valueOrError1 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _2_valueOrError1
	_2_valueOrError1 = m_UTF8.Encode(_dafny.SeqOfString("keyB"))
	if !(!((_2_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(77,16): " + (_2_valueOrError1).String())
	}
	var _3_keyB _dafny.Sequence
	_ = _3_keyB
	_3_keyB = (_2_valueOrError1).Extract().(_dafny.Sequence)
	var _4_valueOrError2 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _4_valueOrError2
	_4_valueOrError2 = m_UTF8.Encode(_dafny.SeqOfString("keyC"))
	if !(!((_4_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(78,16): " + (_4_valueOrError2).String())
	}
	var _5_keyC _dafny.Sequence
	_ = _5_keyC
	_5_keyC = (_4_valueOrError2).Extract().(_dafny.Sequence)
	var _6_valueOrError3 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _6_valueOrError3
	_6_valueOrError3 = m_UTF8.Encode(_dafny.SeqOfString("keyD"))
	if !(!((_6_valueOrError3).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(79,16): " + (_6_valueOrError3).String())
	}
	var _7_keyD _dafny.Sequence
	_ = _7_keyD
	_7_keyD = (_6_valueOrError3).Extract().(_dafny.Sequence)
	var _source0 SmallEncryptionContextVariation = v
	_ = _source0
	{
		{
			if _source0.Is_Empty() {
				encryptionContextKeys = _dafny.SeqOf()
				goto Lmatch0
			}
		}
		{
			if _source0.Is_A() {
				encryptionContextKeys = _dafny.SeqOf(_1_keyA)
				goto Lmatch0
			}
		}
		{
			if _source0.Is_B() {
				encryptionContextKeys = _dafny.SeqOf(_3_keyB)
				goto Lmatch0
			}
		}
		{
			if _source0.Is_AB() {
				encryptionContextKeys = _dafny.SeqOf(_1_keyA, _3_keyB)
				goto Lmatch0
			}
		}
		{
			if _source0.Is_BA() {
				encryptionContextKeys = _dafny.SeqOf(_3_keyB, _1_keyA)
				goto Lmatch0
			}
		}
		{
			if _source0.Is_C() {
				encryptionContextKeys = _dafny.SeqOf(_5_keyC)
				goto Lmatch0
			}
		}
		{
			var _8_CE SmallEncryptionContextVariation = _source0
			_ = _8_CE
			encryptionContextKeys = _dafny.SeqOf(_5_keyC, _7_keyD)
		}
		goto Lmatch0
	}
Lmatch0:
	return encryptionContextKeys
}
func (_static *CompanionStruct_Default___) SmallMismatchedEncryptionContex(v SmallEncryptionContextVariation) _dafny.Map {
	var encryptionContext _dafny.Map = _dafny.EmptyMap
	_ = encryptionContext
	var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _0_valueOrError0
	_0_valueOrError0 = m_UTF8.Encode(_dafny.SeqOfString("keyA"))
	if !(!((_0_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(102,16): " + (_0_valueOrError0).String())
	}
	var _1_keyA _dafny.Sequence
	_ = _1_keyA
	_1_keyA = (_0_valueOrError0).Extract().(_dafny.Sequence)
	var _2_valueOrError1 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _2_valueOrError1
	_2_valueOrError1 = m_UTF8.Encode(_dafny.SeqOfString("valA"))
	if !(!((_2_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(103,16): " + (_2_valueOrError1).String())
	}
	var _3_valA _dafny.Sequence
	_ = _3_valA
	_3_valA = (_2_valueOrError1).Extract().(_dafny.Sequence)
	var _4_valueOrError2 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _4_valueOrError2
	_4_valueOrError2 = m_UTF8.Encode(_dafny.SeqOfString("keyB"))
	if !(!((_4_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(104,16): " + (_4_valueOrError2).String())
	}
	var _5_keyB _dafny.Sequence
	_ = _5_keyB
	_5_keyB = (_4_valueOrError2).Extract().(_dafny.Sequence)
	var _6_valueOrError3 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _6_valueOrError3
	_6_valueOrError3 = m_UTF8.Encode(_dafny.SeqOfString("valB"))
	if !(!((_6_valueOrError3).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(105,16): " + (_6_valueOrError3).String())
	}
	var _7_valB _dafny.Sequence
	_ = _7_valB
	_7_valB = (_6_valueOrError3).Extract().(_dafny.Sequence)
	var _8_valueOrError4 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _8_valueOrError4
	_8_valueOrError4 = m_UTF8.Encode(_dafny.SeqOfString("keyC"))
	if !(!((_8_valueOrError4).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(106,16): " + (_8_valueOrError4).String())
	}
	var _9_keyC _dafny.Sequence
	_ = _9_keyC
	_9_keyC = (_8_valueOrError4).Extract().(_dafny.Sequence)
	var _10_valueOrError5 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _10_valueOrError5
	_10_valueOrError5 = m_UTF8.Encode(_dafny.SeqOfString("valC"))
	if !(!((_10_valueOrError5).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(107,16): " + (_10_valueOrError5).String())
	}
	var _11_valC _dafny.Sequence
	_ = _11_valC
	_11_valC = (_10_valueOrError5).Extract().(_dafny.Sequence)
	var _12_valueOrError6 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _12_valueOrError6
	_12_valueOrError6 = m_UTF8.Encode(_dafny.SeqOfString("keyD"))
	if !(!((_12_valueOrError6).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(108,16): " + (_12_valueOrError6).String())
	}
	var _13_keyD _dafny.Sequence
	_ = _13_keyD
	_13_keyD = (_12_valueOrError6).Extract().(_dafny.Sequence)
	var _14_valueOrError7 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_UTF8.Companion_ValidUTF8Bytes_.Witness())
	_ = _14_valueOrError7
	_14_valueOrError7 = m_UTF8.Encode(_dafny.SeqOfString("valD"))
	if !(!((_14_valueOrError7).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(109,16): " + (_14_valueOrError7).String())
	}
	var _15_valD _dafny.Sequence
	_ = _15_valD
	_15_valD = (_14_valueOrError7).Extract().(_dafny.Sequence)
	var _source0 SmallEncryptionContextVariation = v
	_ = _source0
	{
		{
			if _source0.Is_Empty() {
				encryptionContext = _dafny.NewMapBuilder().ToMap()
				goto Lmatch0
			}
		}
		{
			if _source0.Is_A() {
				encryptionContext = _dafny.NewMapBuilder().ToMap().UpdateUnsafe(_1_keyA, _7_valB)
				goto Lmatch0
			}
		}
		{
			if _source0.Is_B() {
				encryptionContext = _dafny.NewMapBuilder().ToMap().UpdateUnsafe(_5_keyB, _3_valA)
				goto Lmatch0
			}
		}
		{
			if _source0.Is_AB() {
				encryptionContext = _dafny.NewMapBuilder().ToMap().UpdateUnsafe(_1_keyA, _11_valC).UpdateUnsafe(_5_keyB, _15_valD)
				goto Lmatch0
			}
		}
		{
			if _source0.Is_BA() {
				encryptionContext = _dafny.NewMapBuilder().ToMap().UpdateUnsafe(_5_keyB, _3_valA).UpdateUnsafe(_1_keyA, _7_valB)
				goto Lmatch0
			}
		}
		{
			if _source0.Is_C() {
				encryptionContext = _dafny.NewMapBuilder().ToMap().UpdateUnsafe(_9_keyC, _3_valA)
				goto Lmatch0
			}
		}
		{
			var _16_CE SmallEncryptionContextVariation = _source0
			_ = _16_CE
			encryptionContext = _dafny.NewMapBuilder().ToMap().UpdateUnsafe(_9_keyC, _3_valA).UpdateUnsafe(_13_keyD, _7_valB)
		}
		goto Lmatch0
	}
Lmatch0:
	return encryptionContext
}
func (_static *CompanionStruct_Default___) NamespaceAndName(n _dafny.Int) (_dafny.Sequence, _dafny.Sequence) {
	var namespace _dafny.Sequence = _dafny.EmptySeq.SetString()
	_ = namespace
	var name _dafny.Sequence = _dafny.EmptySeq.SetString()
	_ = name
	var _0_s _dafny.Sequence
	_ = _0_s
	_0_s = _dafny.Companion_Sequence_.Concatenate(_dafny.SeqOfString("child"), _dafny.SeqOfChars((_dafny.Char((n).Int32()))+(_dafny.Char('0'))))
	namespace = _dafny.Companion_Sequence_.Concatenate(_0_s, _dafny.SeqOfString(" Namespace"))
	name = _dafny.Companion_Sequence_.Concatenate(_0_s, _dafny.SeqOfString(" Name"))
	return namespace, name
}
func (_static *CompanionStruct_Default___) GenerateKeyPair(keyModulusLength int32) m_AwsCryptographyPrimitivesTypes.GenerateRSAKeyPairOutput {
	var keys m_AwsCryptographyPrimitivesTypes.GenerateRSAKeyPairOutput = m_AwsCryptographyPrimitivesTypes.GenerateRSAKeyPairOutput{}
	_ = keys
	var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _0_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_AtomicPrimitives.Companion_Default___.AtomicPrimitives(m_AtomicPrimitives.Companion_Default___.DefaultCryptoConfig())
	_0_valueOrError0 = _out0
	if !(!((_0_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(143,72): " + (_0_valueOrError0).String())
	}
	var _1_cryptoX m_AwsCryptographyPrimitivesTypes.IAwsCryptographicPrimitivesClient
	_ = _1_cryptoX
	_1_cryptoX = (_0_valueOrError0).Extract().(*m_AtomicPrimitives.AtomicPrimitivesClient)
	var _2_crypto *m_AtomicPrimitives.AtomicPrimitivesClient
	_ = _2_crypto
	_2_crypto = _1_cryptoX.(*m_AtomicPrimitives.AtomicPrimitivesClient)
	var _3_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _3_valueOrError1
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = (_2_crypto).GenerateRSAKeyPair(m_AwsCryptographyPrimitivesTypes.Companion_GenerateRSAKeyPairInput_.Create_GenerateRSAKeyPairInput_(keyModulusLength))
	_3_valueOrError1 = _out1
	if !(!((_3_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/Fixtures.dfy(147,14): " + (_3_valueOrError1).String())
	}
	keys = (_3_valueOrError1).Extract().(m_AwsCryptographyPrimitivesTypes.GenerateRSAKeyPairOutput)
	return keys
}
func (_static *CompanionStruct_Default___) BranchKeyStoreName() _dafny.Sequence {
	return _dafny.SeqOfString("KeyStoreDdbTable")
}
func (_static *CompanionStruct_Default___) LogicalKeyStoreName() _dafny.Sequence {
	return Companion_Default___.BranchKeyStoreName()
}
func (_static *CompanionStruct_Default___) KeyArn() _dafny.Sequence {
	return _dafny.SeqOfString("arn:aws:kms:us-west-2:658956600833:key/b3537ef1-d8dc-4780-9f5a-55776cbb2f7f")
}
func (_static *CompanionStruct_Default___) HierarchyKeyArn() _dafny.Sequence {
	return _dafny.SeqOfString("arn:aws:kms:us-west-2:370957321024:key/9d989aa2-2f9c-438c-a745-cc57d3ad0126")
}
func (_static *CompanionStruct_Default___) MkrKeyArn() _dafny.Sequence {
	return _dafny.SeqOfString("arn:aws:kms:us-west-2:370957321024:key/mrk-63d386cb70614ea59b32ad65c9315297")
}
func (_static *CompanionStruct_Default___) BranchKeyId() _dafny.Sequence {
	return _dafny.SeqOfString("75789115-1deb-4fe3-a2ec-be9e885d1945")
}
func (_static *CompanionStruct_Default___) RESERVED__ENCRYPTION__CONTEXT() _dafny.Sequence {
	var _0_s _dafny.Sequence = _dafny.SeqOf(uint8(97), uint8(119), uint8(115), uint8(45), uint8(99), uint8(114), uint8(121), uint8(112), uint8(116), uint8(111), uint8(45))
	_ = _0_s
	return _0_s
}

// End of class Default__

// Definition of datatype SmallEncryptionContextVariation
type SmallEncryptionContextVariation struct {
	Data_SmallEncryptionContextVariation_
}

func (_this SmallEncryptionContextVariation) Get_() Data_SmallEncryptionContextVariation_ {
	return _this.Data_SmallEncryptionContextVariation_
}

type Data_SmallEncryptionContextVariation_ interface {
	isSmallEncryptionContextVariation()
}

type CompanionStruct_SmallEncryptionContextVariation_ struct {
}

var Companion_SmallEncryptionContextVariation_ = CompanionStruct_SmallEncryptionContextVariation_{}

type SmallEncryptionContextVariation_Empty struct {
}

func (SmallEncryptionContextVariation_Empty) isSmallEncryptionContextVariation() {}

func (CompanionStruct_SmallEncryptionContextVariation_) Create_Empty_() SmallEncryptionContextVariation {
	return SmallEncryptionContextVariation{SmallEncryptionContextVariation_Empty{}}
}

func (_this SmallEncryptionContextVariation) Is_Empty() bool {
	_, ok := _this.Get_().(SmallEncryptionContextVariation_Empty)
	return ok
}

type SmallEncryptionContextVariation_A struct {
}

func (SmallEncryptionContextVariation_A) isSmallEncryptionContextVariation() {}

func (CompanionStruct_SmallEncryptionContextVariation_) Create_A_() SmallEncryptionContextVariation {
	return SmallEncryptionContextVariation{SmallEncryptionContextVariation_A{}}
}

func (_this SmallEncryptionContextVariation) Is_A() bool {
	_, ok := _this.Get_().(SmallEncryptionContextVariation_A)
	return ok
}

type SmallEncryptionContextVariation_B struct {
}

func (SmallEncryptionContextVariation_B) isSmallEncryptionContextVariation() {}

func (CompanionStruct_SmallEncryptionContextVariation_) Create_B_() SmallEncryptionContextVariation {
	return SmallEncryptionContextVariation{SmallEncryptionContextVariation_B{}}
}

func (_this SmallEncryptionContextVariation) Is_B() bool {
	_, ok := _this.Get_().(SmallEncryptionContextVariation_B)
	return ok
}

type SmallEncryptionContextVariation_AB struct {
}

func (SmallEncryptionContextVariation_AB) isSmallEncryptionContextVariation() {}

func (CompanionStruct_SmallEncryptionContextVariation_) Create_AB_() SmallEncryptionContextVariation {
	return SmallEncryptionContextVariation{SmallEncryptionContextVariation_AB{}}
}

func (_this SmallEncryptionContextVariation) Is_AB() bool {
	_, ok := _this.Get_().(SmallEncryptionContextVariation_AB)
	return ok
}

type SmallEncryptionContextVariation_BA struct {
}

func (SmallEncryptionContextVariation_BA) isSmallEncryptionContextVariation() {}

func (CompanionStruct_SmallEncryptionContextVariation_) Create_BA_() SmallEncryptionContextVariation {
	return SmallEncryptionContextVariation{SmallEncryptionContextVariation_BA{}}
}

func (_this SmallEncryptionContextVariation) Is_BA() bool {
	_, ok := _this.Get_().(SmallEncryptionContextVariation_BA)
	return ok
}

type SmallEncryptionContextVariation_C struct {
}

func (SmallEncryptionContextVariation_C) isSmallEncryptionContextVariation() {}

func (CompanionStruct_SmallEncryptionContextVariation_) Create_C_() SmallEncryptionContextVariation {
	return SmallEncryptionContextVariation{SmallEncryptionContextVariation_C{}}
}

func (_this SmallEncryptionContextVariation) Is_C() bool {
	_, ok := _this.Get_().(SmallEncryptionContextVariation_C)
	return ok
}

type SmallEncryptionContextVariation_CD struct {
}

func (SmallEncryptionContextVariation_CD) isSmallEncryptionContextVariation() {}

func (CompanionStruct_SmallEncryptionContextVariation_) Create_CD_() SmallEncryptionContextVariation {
	return SmallEncryptionContextVariation{SmallEncryptionContextVariation_CD{}}
}

func (_this SmallEncryptionContextVariation) Is_CD() bool {
	_, ok := _this.Get_().(SmallEncryptionContextVariation_CD)
	return ok
}

func (CompanionStruct_SmallEncryptionContextVariation_) Default() SmallEncryptionContextVariation {
	return Companion_SmallEncryptionContextVariation_.Create_Empty_()
}

func (_ CompanionStruct_SmallEncryptionContextVariation_) AllSingletonConstructors() _dafny.Iterator {
	i := -1
	return func() (interface{}, bool) {
		i++
		switch i {
		case 0:
			return Companion_SmallEncryptionContextVariation_.Create_Empty_(), true
		case 1:
			return Companion_SmallEncryptionContextVariation_.Create_A_(), true
		case 2:
			return Companion_SmallEncryptionContextVariation_.Create_B_(), true
		case 3:
			return Companion_SmallEncryptionContextVariation_.Create_AB_(), true
		case 4:
			return Companion_SmallEncryptionContextVariation_.Create_BA_(), true
		case 5:
			return Companion_SmallEncryptionContextVariation_.Create_C_(), true
		case 6:
			return Companion_SmallEncryptionContextVariation_.Create_CD_(), true
		default:
			return SmallEncryptionContextVariation{}, false
		}
	}
}

func (_this SmallEncryptionContextVariation) String() string {
	switch _this.Get_().(type) {
	case nil:
		return "null"
	case SmallEncryptionContextVariation_Empty:
		{
			return "Fixtures.SmallEncryptionContextVariation.Empty"
		}
	case SmallEncryptionContextVariation_A:
		{
			return "Fixtures.SmallEncryptionContextVariation.A"
		}
	case SmallEncryptionContextVariation_B:
		{
			return "Fixtures.SmallEncryptionContextVariation.B"
		}
	case SmallEncryptionContextVariation_AB:
		{
			return "Fixtures.SmallEncryptionContextVariation.AB"
		}
	case SmallEncryptionContextVariation_BA:
		{
			return "Fixtures.SmallEncryptionContextVariation.BA"
		}
	case SmallEncryptionContextVariation_C:
		{
			return "Fixtures.SmallEncryptionContextVariation.C"
		}
	case SmallEncryptionContextVariation_CD:
		{
			return "Fixtures.SmallEncryptionContextVariation.CD"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this SmallEncryptionContextVariation) Equals(other SmallEncryptionContextVariation) bool {
	switch _this.Get_().(type) {
	case SmallEncryptionContextVariation_Empty:
		{
			_, ok := other.Get_().(SmallEncryptionContextVariation_Empty)
			return ok
		}
	case SmallEncryptionContextVariation_A:
		{
			_, ok := other.Get_().(SmallEncryptionContextVariation_A)
			return ok
		}
	case SmallEncryptionContextVariation_B:
		{
			_, ok := other.Get_().(SmallEncryptionContextVariation_B)
			return ok
		}
	case SmallEncryptionContextVariation_AB:
		{
			_, ok := other.Get_().(SmallEncryptionContextVariation_AB)
			return ok
		}
	case SmallEncryptionContextVariation_BA:
		{
			_, ok := other.Get_().(SmallEncryptionContextVariation_BA)
			return ok
		}
	case SmallEncryptionContextVariation_C:
		{
			_, ok := other.Get_().(SmallEncryptionContextVariation_C)
			return ok
		}
	case SmallEncryptionContextVariation_CD:
		{
			_, ok := other.Get_().(SmallEncryptionContextVariation_CD)
			return ok
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this SmallEncryptionContextVariation) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(SmallEncryptionContextVariation)
	return ok && _this.Equals(typed)
}

func Type_SmallEncryptionContextVariation_() _dafny.TypeDescriptor {
	return type_SmallEncryptionContextVariation_{}
}

type type_SmallEncryptionContextVariation_ struct {
}

func (_this type_SmallEncryptionContextVariation_) Default() interface{} {
	return Companion_SmallEncryptionContextVariation_.Default()
}

func (_this type_SmallEncryptionContextVariation_) String() string {
	return "Fixtures.SmallEncryptionContextVariation"
}
func (_this SmallEncryptionContextVariation) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = SmallEncryptionContextVariation{}

// End of datatype SmallEncryptionContextVariation
