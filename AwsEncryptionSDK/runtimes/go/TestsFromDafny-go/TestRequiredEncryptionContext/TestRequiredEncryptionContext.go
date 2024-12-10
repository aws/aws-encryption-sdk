// Package TestRequiredEncryptionContext
// Dafny module TestRequiredEncryptionContext compiled into Go

package TestRequiredEncryptionContext

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
	m_Fixtures "github.com/aws/aws-encryption-sdk/test/Fixtures"
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
var _ m_Fixtures.Dummy__

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
	return "TestRequiredEncryptionContext.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) TestReprEncryptionContextWithSameECHappyCase() {
	var _0_asdf _dafny.Sequence
	_ = _0_asdf
	_0_asdf = _dafny.SeqOf(uint8(97), uint8(115), uint8(100), uint8(102))
	var _1_defaultConfig m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _1_defaultConfig
	_1_defaultConfig = m_EncryptionSdk.Companion_Default___.DefaultAwsEncryptionSdkConfig()
	var _2_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _2_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_EncryptionSdk.Companion_Default___.ESDK(_1_defaultConfig)
	_2_valueOrError0 = _out0
	if !(!((_2_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(39,20): " + (_2_valueOrError0).String())
	}
	var _3_esdk *m_EncryptionSdk.ESDKClient
	_ = _3_esdk
	_3_esdk = (_2_valueOrError0).Extract().(*m_EncryptionSdk.ESDKClient)
	var _4_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _4_valueOrError1
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
	_4_valueOrError1 = _out1
	if !(!((_4_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(40,19): " + (_4_valueOrError1).String())
	}
	var _5_mpl *m_MaterialProviders.MaterialProvidersClient
	_ = _5_mpl
	_5_mpl = (_4_valueOrError1).Extract().(*m_MaterialProviders.MaterialProvidersClient)
	var _6_rsaKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _6_rsaKeyring
	var _out2 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out2
	_out2 = Companion_Default___.GetRsaKeyring()
	_6_rsaKeyring = _out2
	var _7_kmsKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _7_kmsKeyring
	var _out3 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out3
	_out3 = Companion_Default___.GetKmsKeyring()
	_7_kmsKeyring = _out3
	var _8_aesKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _8_aesKeyring
	var _out4 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out4
	_out4 = Companion_Default___.GetAesKeyring()
	_8_aesKeyring = _out4
	var _9_hKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _9_hKeyring
	var _out5 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out5
	_out5 = Companion_Default___.GetHierarchicalKeyring()
	_9_hKeyring = _out5
	var _10_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _10_valueOrError2
	var _out6 m_Wrappers.Result
	_ = _out6
	_out6 = (_5_mpl).CreateMultiKeyring(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateMultiKeyringInput_.Create_CreateMultiKeyringInput_(m_Wrappers.Companion_Option_.Create_Some_(_8_aesKeyring), _dafny.SeqOf(_6_rsaKeyring, _7_kmsKeyring, _9_hKeyring)))
	_10_valueOrError2 = _out6
	if !(!((_10_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(48,28): " + (_10_valueOrError2).String())
	}
	var _11_multiKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _11_multiKeyring
	_11_multiKeyring = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_10_valueOrError2).Extract())
	var _12_encryptionContext _dafny.Map
	_ = _12_encryptionContext
	var _out7 _dafny.Map
	_ = _out7
	_out7 = m_Fixtures.Companion_Default___.SmallEncryptionContext(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_AB_())
	_12_encryptionContext = _out7
	var _13_encryptOutput m_Wrappers.Result
	_ = _13_encryptOutput
	var _out8 m_Wrappers.Result
	_ = _out8
	_out8 = (_3_esdk).Encrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_EncryptInput_.Create_EncryptInput_(_0_asdf, m_Wrappers.Companion_Option_.Create_Some_(_12_encryptionContext), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_11_multiKeyring), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_()))
	_13_encryptOutput = _out8
	if !((_13_encryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(66,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _14_esdkCiphertext _dafny.Sequence
	_ = _14_esdkCiphertext
	_14_esdkCiphertext = ((_13_encryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.EncryptOutput)).Dtor_ciphertext()
	var _15_decryptOutput m_Wrappers.Result
	_ = _15_decryptOutput
	var _out9 m_Wrappers.Result
	_ = _out9
	_out9 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_14_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_6_rsaKeyring), m_Wrappers.Companion_Option_.Create_Some_(_12_encryptionContext)))
	_15_decryptOutput = _out9
	if !((_15_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(77,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _16_cycledPlaintext _dafny.Sequence
	_ = _16_cycledPlaintext
	_16_cycledPlaintext = ((_15_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext()
	if !(_dafny.Companion_Sequence_.Equal(_16_cycledPlaintext, _0_asdf)) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(79,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out10 m_Wrappers.Result
	_ = _out10
	_out10 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_14_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_7_kmsKeyring), m_Wrappers.Companion_Option_.Create_Some_(_12_encryptionContext)))
	_15_decryptOutput = _out10
	if !((_15_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(89,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	_16_cycledPlaintext = ((_15_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext()
	if !(_dafny.Companion_Sequence_.Equal(_16_cycledPlaintext, _0_asdf)) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(91,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out11 m_Wrappers.Result
	_ = _out11
	_out11 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_14_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_8_aesKeyring), m_Wrappers.Companion_Option_.Create_Some_(_12_encryptionContext)))
	_15_decryptOutput = _out11
	if !((_15_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(101,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	_16_cycledPlaintext = ((_15_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext()
	if !(_dafny.Companion_Sequence_.Equal(_16_cycledPlaintext, _0_asdf)) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(103,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out12 m_Wrappers.Result
	_ = _out12
	_out12 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_14_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_9_hKeyring), m_Wrappers.Companion_Option_.Create_Some_(_12_encryptionContext)))
	_15_decryptOutput = _out12
	if !((_15_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(113,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	_16_cycledPlaintext = ((_15_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext()
	if !(_dafny.Companion_Sequence_.Equal(_16_cycledPlaintext, _0_asdf)) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(115,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
}
func (_static *CompanionStruct_Default___) TestRemoveOnEncryptAndSupplyOnDecryptHappyCase() {
	var _0_asdf _dafny.Sequence
	_ = _0_asdf
	_0_asdf = _dafny.SeqOf(uint8(97), uint8(115), uint8(100), uint8(102))
	var _1_defaultConfig m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _1_defaultConfig
	_1_defaultConfig = m_EncryptionSdk.Companion_Default___.DefaultAwsEncryptionSdkConfig()
	var _2_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _2_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_EncryptionSdk.Companion_Default___.ESDK(_1_defaultConfig)
	_2_valueOrError0 = _out0
	if !(!((_2_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(124,20): " + (_2_valueOrError0).String())
	}
	var _3_esdk *m_EncryptionSdk.ESDKClient
	_ = _3_esdk
	_3_esdk = (_2_valueOrError0).Extract().(*m_EncryptionSdk.ESDKClient)
	var _4_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _4_valueOrError1
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
	_4_valueOrError1 = _out1
	if !(!((_4_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(125,19): " + (_4_valueOrError1).String())
	}
	var _5_mpl *m_MaterialProviders.MaterialProvidersClient
	_ = _5_mpl
	_5_mpl = (_4_valueOrError1).Extract().(*m_MaterialProviders.MaterialProvidersClient)
	var _6_rsaKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _6_rsaKeyring
	var _out2 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out2
	_out2 = Companion_Default___.GetRsaKeyring()
	_6_rsaKeyring = _out2
	var _7_kmsKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _7_kmsKeyring
	var _out3 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out3
	_out3 = Companion_Default___.GetKmsKeyring()
	_7_kmsKeyring = _out3
	var _8_aesKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _8_aesKeyring
	var _out4 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out4
	_out4 = Companion_Default___.GetAesKeyring()
	_8_aesKeyring = _out4
	var _9_hKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _9_hKeyring
	var _out5 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out5
	_out5 = Companion_Default___.GetHierarchicalKeyring()
	_9_hKeyring = _out5
	var _10_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _10_valueOrError2
	var _out6 m_Wrappers.Result
	_ = _out6
	_out6 = (_5_mpl).CreateMultiKeyring(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateMultiKeyringInput_.Create_CreateMultiKeyringInput_(m_Wrappers.Companion_Option_.Create_Some_(_8_aesKeyring), _dafny.SeqOf(_6_rsaKeyring, _7_kmsKeyring, _9_hKeyring)))
	_10_valueOrError2 = _out6
	if !(!((_10_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(133,28): " + (_10_valueOrError2).String())
	}
	var _11_multiKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _11_multiKeyring
	_11_multiKeyring = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_10_valueOrError2).Extract())
	var _12_encryptionContext _dafny.Map
	_ = _12_encryptionContext
	var _out7 _dafny.Map
	_ = _out7
	_out7 = m_Fixtures.Companion_Default___.SmallEncryptionContext(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_AB_())
	_12_encryptionContext = _out7
	var _13_reproducedEncryptionContext _dafny.Map
	_ = _13_reproducedEncryptionContext
	var _out8 _dafny.Map
	_ = _out8
	_out8 = m_Fixtures.Companion_Default___.SmallEncryptionContext(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_A_())
	_13_reproducedEncryptionContext = _out8
	var _14_requiredEncryptionContextKeys _dafny.Sequence
	_ = _14_requiredEncryptionContextKeys
	var _out9 _dafny.Sequence
	_ = _out9
	_out9 = m_Fixtures.Companion_Default___.SmallEncryptionContextKeys(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_A_())
	_14_requiredEncryptionContextKeys = _out9
	var _15_valueOrError3 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _15_valueOrError3
	var _out10 m_Wrappers.Result
	_ = _out10
	_out10 = (_5_mpl).CreateDefaultCryptographicMaterialsManager(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultCryptographicMaterialsManagerInput_.Create_CreateDefaultCryptographicMaterialsManagerInput_(_11_multiKeyring))
	_15_valueOrError3 = _out10
	if !(!((_15_valueOrError3).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(148,26): " + (_15_valueOrError3).String())
	}
	var _16_defaultCMM m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _16_defaultCMM
	_16_defaultCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_15_valueOrError3).Extract())
	var _17_valueOrError4 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _17_valueOrError4
	var _out11 m_Wrappers.Result
	_ = _out11
	_out11 = (_5_mpl).CreateRequiredEncryptionContextCMM(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRequiredEncryptionContextCMMInput_.Create_CreateRequiredEncryptionContextCMMInput_(m_Wrappers.Companion_Option_.Create_Some_(_16_defaultCMM), m_Wrappers.Companion_Option_.Create_None_(), _14_requiredEncryptionContextKeys))
	_17_valueOrError4 = _out11
	if !(!((_17_valueOrError4).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(155,22): " + (_17_valueOrError4).String())
	}
	var _18_reqCMM m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _18_reqCMM
	_18_reqCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_17_valueOrError4).Extract())
	var _19_encryptOutput m_Wrappers.Result
	_ = _19_encryptOutput
	var _out12 m_Wrappers.Result
	_ = _out12
	_out12 = (_3_esdk).Encrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_EncryptInput_.Create_EncryptInput_(_0_asdf, m_Wrappers.Companion_Option_.Create_Some_(_12_encryptionContext), m_Wrappers.Companion_Option_.Create_Some_(_18_reqCMM), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_()))
	_19_encryptOutput = _out12
	if !((_19_encryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(174,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _20_esdkCiphertext _dafny.Sequence
	_ = _20_esdkCiphertext
	_20_esdkCiphertext = ((_19_encryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.EncryptOutput)).Dtor_ciphertext()
	var _21_decryptOutput m_Wrappers.Result
	_ = _21_decryptOutput
	var _out13 m_Wrappers.Result
	_ = _out13
	_out13 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_20_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_6_rsaKeyring), m_Wrappers.Companion_Option_.Create_Some_(_13_reproducedEncryptionContext)))
	_21_decryptOutput = _out13
	if !((_21_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(185,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _22_cycledPlaintext _dafny.Sequence
	_ = _22_cycledPlaintext
	_22_cycledPlaintext = ((_21_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext()
	if !(_dafny.Companion_Sequence_.Equal(_22_cycledPlaintext, _0_asdf)) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(187,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out14 m_Wrappers.Result
	_ = _out14
	_out14 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_20_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_7_kmsKeyring), m_Wrappers.Companion_Option_.Create_Some_(_13_reproducedEncryptionContext)))
	_21_decryptOutput = _out14
	if !((_21_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(197,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	_22_cycledPlaintext = ((_21_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext()
	if !(_dafny.Companion_Sequence_.Equal(_22_cycledPlaintext, _0_asdf)) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(199,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out15 m_Wrappers.Result
	_ = _out15
	_out15 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_20_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_8_aesKeyring), m_Wrappers.Companion_Option_.Create_Some_(_13_reproducedEncryptionContext)))
	_21_decryptOutput = _out15
	if !((_21_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(209,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	_22_cycledPlaintext = ((_21_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext()
	if !(_dafny.Companion_Sequence_.Equal(_22_cycledPlaintext, _0_asdf)) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(211,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out16 m_Wrappers.Result
	_ = _out16
	_out16 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_20_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_9_hKeyring), m_Wrappers.Companion_Option_.Create_Some_(_13_reproducedEncryptionContext)))
	_21_decryptOutput = _out16
	if !((_21_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(221,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	_22_cycledPlaintext = ((_21_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext()
	if !(_dafny.Companion_Sequence_.Equal(_22_cycledPlaintext, _0_asdf)) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(223,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
}
func (_static *CompanionStruct_Default___) TestRemoveOnEncryptRemoveAndSupplyOnDecryptHappyCase() {
	var _0_asdf _dafny.Sequence
	_ = _0_asdf
	_0_asdf = _dafny.SeqOf(uint8(97), uint8(115), uint8(100), uint8(102))
	var _1_defaultConfig m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _1_defaultConfig
	_1_defaultConfig = m_EncryptionSdk.Companion_Default___.DefaultAwsEncryptionSdkConfig()
	var _2_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _2_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_EncryptionSdk.Companion_Default___.ESDK(_1_defaultConfig)
	_2_valueOrError0 = _out0
	if !(!((_2_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(233,20): " + (_2_valueOrError0).String())
	}
	var _3_esdk *m_EncryptionSdk.ESDKClient
	_ = _3_esdk
	_3_esdk = (_2_valueOrError0).Extract().(*m_EncryptionSdk.ESDKClient)
	var _4_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _4_valueOrError1
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
	_4_valueOrError1 = _out1
	if !(!((_4_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(234,19): " + (_4_valueOrError1).String())
	}
	var _5_mpl *m_MaterialProviders.MaterialProvidersClient
	_ = _5_mpl
	_5_mpl = (_4_valueOrError1).Extract().(*m_MaterialProviders.MaterialProvidersClient)
	var _6_rsaKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _6_rsaKeyring
	var _out2 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out2
	_out2 = Companion_Default___.GetRsaKeyring()
	_6_rsaKeyring = _out2
	var _7_kmsKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _7_kmsKeyring
	var _out3 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out3
	_out3 = Companion_Default___.GetKmsKeyring()
	_7_kmsKeyring = _out3
	var _8_aesKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _8_aesKeyring
	var _out4 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out4
	_out4 = Companion_Default___.GetAesKeyring()
	_8_aesKeyring = _out4
	var _9_hKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _9_hKeyring
	var _out5 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out5
	_out5 = Companion_Default___.GetHierarchicalKeyring()
	_9_hKeyring = _out5
	var _10_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _10_valueOrError2
	var _out6 m_Wrappers.Result
	_ = _out6
	_out6 = (_5_mpl).CreateMultiKeyring(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateMultiKeyringInput_.Create_CreateMultiKeyringInput_(m_Wrappers.Companion_Option_.Create_Some_(_8_aesKeyring), _dafny.SeqOf(_6_rsaKeyring, _7_kmsKeyring, _9_hKeyring)))
	_10_valueOrError2 = _out6
	if !(!((_10_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(242,28): " + (_10_valueOrError2).String())
	}
	var _11_multiKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _11_multiKeyring
	_11_multiKeyring = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_10_valueOrError2).Extract())
	var _12_encryptionContext _dafny.Map
	_ = _12_encryptionContext
	var _out7 _dafny.Map
	_ = _out7
	_out7 = m_Fixtures.Companion_Default___.SmallEncryptionContext(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_AB_())
	_12_encryptionContext = _out7
	var _13_reproducedEncryptionContext _dafny.Map
	_ = _13_reproducedEncryptionContext
	var _out8 _dafny.Map
	_ = _out8
	_out8 = m_Fixtures.Companion_Default___.SmallEncryptionContext(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_A_())
	_13_reproducedEncryptionContext = _out8
	var _14_requiredEncryptionContextKeys _dafny.Sequence
	_ = _14_requiredEncryptionContextKeys
	var _out9 _dafny.Sequence
	_ = _out9
	_out9 = m_Fixtures.Companion_Default___.SmallEncryptionContextKeys(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_A_())
	_14_requiredEncryptionContextKeys = _out9
	var _15_valueOrError3 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _15_valueOrError3
	var _out10 m_Wrappers.Result
	_ = _out10
	_out10 = (_5_mpl).CreateDefaultCryptographicMaterialsManager(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultCryptographicMaterialsManagerInput_.Create_CreateDefaultCryptographicMaterialsManagerInput_(_11_multiKeyring))
	_15_valueOrError3 = _out10
	if !(!((_15_valueOrError3).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(257,26): " + (_15_valueOrError3).String())
	}
	var _16_defaultCMM m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _16_defaultCMM
	_16_defaultCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_15_valueOrError3).Extract())
	var _17_valueOrError4 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _17_valueOrError4
	var _out11 m_Wrappers.Result
	_ = _out11
	_out11 = (_5_mpl).CreateRequiredEncryptionContextCMM(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRequiredEncryptionContextCMMInput_.Create_CreateRequiredEncryptionContextCMMInput_(m_Wrappers.Companion_Option_.Create_Some_(_16_defaultCMM), m_Wrappers.Companion_Option_.Create_None_(), _14_requiredEncryptionContextKeys))
	_17_valueOrError4 = _out11
	if !(!((_17_valueOrError4).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(264,22): " + (_17_valueOrError4).String())
	}
	var _18_reqCMM m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _18_reqCMM
	_18_reqCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_17_valueOrError4).Extract())
	var _19_encryptOutput m_Wrappers.Result
	_ = _19_encryptOutput
	var _out12 m_Wrappers.Result
	_ = _out12
	_out12 = (_3_esdk).Encrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_EncryptInput_.Create_EncryptInput_(_0_asdf, m_Wrappers.Companion_Option_.Create_Some_(_12_encryptionContext), m_Wrappers.Companion_Option_.Create_Some_(_18_reqCMM), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_()))
	_19_encryptOutput = _out12
	if !((_19_encryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(283,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _20_esdkCiphertext _dafny.Sequence
	_ = _20_esdkCiphertext
	_20_esdkCiphertext = ((_19_encryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.EncryptOutput)).Dtor_ciphertext()
	var _21_valueOrError5 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _21_valueOrError5
	var _out13 m_Wrappers.Result
	_ = _out13
	_out13 = (_5_mpl).CreateDefaultCryptographicMaterialsManager(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultCryptographicMaterialsManagerInput_.Create_CreateDefaultCryptographicMaterialsManagerInput_(_6_rsaKeyring))
	_21_valueOrError5 = _out13
	if !(!((_21_valueOrError5).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(287,22): " + (_21_valueOrError5).String())
	}
	_16_defaultCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_21_valueOrError5).Extract())
	var _22_valueOrError6 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _22_valueOrError6
	var _out14 m_Wrappers.Result
	_ = _out14
	_out14 = (_5_mpl).CreateRequiredEncryptionContextCMM(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRequiredEncryptionContextCMMInput_.Create_CreateRequiredEncryptionContextCMMInput_(m_Wrappers.Companion_Option_.Create_Some_(_16_defaultCMM), m_Wrappers.Companion_Option_.Create_None_(), _14_requiredEncryptionContextKeys))
	_22_valueOrError6 = _out14
	if !(!((_22_valueOrError6).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(294,18): " + (_22_valueOrError6).String())
	}
	_18_reqCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_22_valueOrError6).Extract())
	var _23_decryptOutput m_Wrappers.Result
	_ = _23_decryptOutput
	var _out15 m_Wrappers.Result
	_ = _out15
	_out15 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_20_esdkCiphertext, m_Wrappers.Companion_Option_.Create_Some_(_18_reqCMM), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_13_reproducedEncryptionContext)))
	_23_decryptOutput = _out15
	if !((_23_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(312,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _24_cycledPlaintext _dafny.Sequence
	_ = _24_cycledPlaintext
	_24_cycledPlaintext = ((_23_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext()
	if !(_dafny.Companion_Sequence_.Equal(_24_cycledPlaintext, _0_asdf)) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(314,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _25_valueOrError7 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _25_valueOrError7
	var _out16 m_Wrappers.Result
	_ = _out16
	_out16 = (_5_mpl).CreateDefaultCryptographicMaterialsManager(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultCryptographicMaterialsManagerInput_.Create_CreateDefaultCryptographicMaterialsManagerInput_(_7_kmsKeyring))
	_25_valueOrError7 = _out16
	if !(!((_25_valueOrError7).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(318,22): " + (_25_valueOrError7).String())
	}
	_16_defaultCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_25_valueOrError7).Extract())
	var _26_valueOrError8 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _26_valueOrError8
	var _out17 m_Wrappers.Result
	_ = _out17
	_out17 = (_5_mpl).CreateRequiredEncryptionContextCMM(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRequiredEncryptionContextCMMInput_.Create_CreateRequiredEncryptionContextCMMInput_(m_Wrappers.Companion_Option_.Create_Some_(_16_defaultCMM), m_Wrappers.Companion_Option_.Create_None_(), _14_requiredEncryptionContextKeys))
	_26_valueOrError8 = _out17
	if !(!((_26_valueOrError8).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(325,18): " + (_26_valueOrError8).String())
	}
	_18_reqCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_26_valueOrError8).Extract())
	var _out18 m_Wrappers.Result
	_ = _out18
	_out18 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_20_esdkCiphertext, m_Wrappers.Companion_Option_.Create_Some_(_18_reqCMM), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_13_reproducedEncryptionContext)))
	_23_decryptOutput = _out18
	if !((_23_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(343,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	_24_cycledPlaintext = ((_23_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext()
	if !(_dafny.Companion_Sequence_.Equal(_24_cycledPlaintext, _0_asdf)) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(345,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _27_valueOrError9 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _27_valueOrError9
	var _out19 m_Wrappers.Result
	_ = _out19
	_out19 = (_5_mpl).CreateDefaultCryptographicMaterialsManager(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultCryptographicMaterialsManagerInput_.Create_CreateDefaultCryptographicMaterialsManagerInput_(_8_aesKeyring))
	_27_valueOrError9 = _out19
	if !(!((_27_valueOrError9).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(349,22): " + (_27_valueOrError9).String())
	}
	_16_defaultCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_27_valueOrError9).Extract())
	var _28_valueOrError10 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _28_valueOrError10
	var _out20 m_Wrappers.Result
	_ = _out20
	_out20 = (_5_mpl).CreateRequiredEncryptionContextCMM(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRequiredEncryptionContextCMMInput_.Create_CreateRequiredEncryptionContextCMMInput_(m_Wrappers.Companion_Option_.Create_Some_(_16_defaultCMM), m_Wrappers.Companion_Option_.Create_None_(), _14_requiredEncryptionContextKeys))
	_28_valueOrError10 = _out20
	if !(!((_28_valueOrError10).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(356,18): " + (_28_valueOrError10).String())
	}
	_18_reqCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_28_valueOrError10).Extract())
	var _out21 m_Wrappers.Result
	_ = _out21
	_out21 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_20_esdkCiphertext, m_Wrappers.Companion_Option_.Create_Some_(_18_reqCMM), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_13_reproducedEncryptionContext)))
	_23_decryptOutput = _out21
	if !((_23_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(374,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	_24_cycledPlaintext = ((_23_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext()
	if !(_dafny.Companion_Sequence_.Equal(_24_cycledPlaintext, _0_asdf)) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(376,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _29_valueOrError11 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _29_valueOrError11
	var _out22 m_Wrappers.Result
	_ = _out22
	_out22 = (_5_mpl).CreateDefaultCryptographicMaterialsManager(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultCryptographicMaterialsManagerInput_.Create_CreateDefaultCryptographicMaterialsManagerInput_(_9_hKeyring))
	_29_valueOrError11 = _out22
	if !(!((_29_valueOrError11).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(379,22): " + (_29_valueOrError11).String())
	}
	_16_defaultCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_29_valueOrError11).Extract())
	var _30_valueOrError12 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _30_valueOrError12
	var _out23 m_Wrappers.Result
	_ = _out23
	_out23 = (_5_mpl).CreateRequiredEncryptionContextCMM(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRequiredEncryptionContextCMMInput_.Create_CreateRequiredEncryptionContextCMMInput_(m_Wrappers.Companion_Option_.Create_Some_(_16_defaultCMM), m_Wrappers.Companion_Option_.Create_None_(), _14_requiredEncryptionContextKeys))
	_30_valueOrError12 = _out23
	if !(!((_30_valueOrError12).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(386,18): " + (_30_valueOrError12).String())
	}
	_18_reqCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_30_valueOrError12).Extract())
	var _out24 m_Wrappers.Result
	_ = _out24
	_out24 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_20_esdkCiphertext, m_Wrappers.Companion_Option_.Create_Some_(_18_reqCMM), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_13_reproducedEncryptionContext)))
	_23_decryptOutput = _out24
	if !((_23_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(404,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	_24_cycledPlaintext = ((_23_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext()
	if !(_dafny.Companion_Sequence_.Equal(_24_cycledPlaintext, _0_asdf)) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(406,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
}
func (_static *CompanionStruct_Default___) TestRemoveOnDecryptIsBackwardsCompatibleHappyCase() {
	var _0_asdf _dafny.Sequence
	_ = _0_asdf
	_0_asdf = _dafny.SeqOf(uint8(97), uint8(115), uint8(100), uint8(102))
	var _1_defaultConfig m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _1_defaultConfig
	_1_defaultConfig = m_EncryptionSdk.Companion_Default___.DefaultAwsEncryptionSdkConfig()
	var _2_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _2_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_EncryptionSdk.Companion_Default___.ESDK(_1_defaultConfig)
	_2_valueOrError0 = _out0
	if !(!((_2_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(415,20): " + (_2_valueOrError0).String())
	}
	var _3_esdk *m_EncryptionSdk.ESDKClient
	_ = _3_esdk
	_3_esdk = (_2_valueOrError0).Extract().(*m_EncryptionSdk.ESDKClient)
	var _4_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _4_valueOrError1
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
	_4_valueOrError1 = _out1
	if !(!((_4_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(416,19): " + (_4_valueOrError1).String())
	}
	var _5_mpl *m_MaterialProviders.MaterialProvidersClient
	_ = _5_mpl
	_5_mpl = (_4_valueOrError1).Extract().(*m_MaterialProviders.MaterialProvidersClient)
	var _6_rsaKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _6_rsaKeyring
	var _out2 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out2
	_out2 = Companion_Default___.GetRsaKeyring()
	_6_rsaKeyring = _out2
	var _7_kmsKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _7_kmsKeyring
	var _out3 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out3
	_out3 = Companion_Default___.GetKmsKeyring()
	_7_kmsKeyring = _out3
	var _8_aesKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _8_aesKeyring
	var _out4 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out4
	_out4 = Companion_Default___.GetAesKeyring()
	_8_aesKeyring = _out4
	var _9_hKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _9_hKeyring
	var _out5 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out5
	_out5 = Companion_Default___.GetHierarchicalKeyring()
	_9_hKeyring = _out5
	var _10_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _10_valueOrError2
	var _out6 m_Wrappers.Result
	_ = _out6
	_out6 = (_5_mpl).CreateMultiKeyring(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateMultiKeyringInput_.Create_CreateMultiKeyringInput_(m_Wrappers.Companion_Option_.Create_Some_(_8_aesKeyring), _dafny.SeqOf(_6_rsaKeyring, _7_kmsKeyring, _9_hKeyring)))
	_10_valueOrError2 = _out6
	if !(!((_10_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(424,28): " + (_10_valueOrError2).String())
	}
	var _11_multiKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _11_multiKeyring
	_11_multiKeyring = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_10_valueOrError2).Extract())
	var _12_encryptionContext _dafny.Map
	_ = _12_encryptionContext
	var _out7 _dafny.Map
	_ = _out7
	_out7 = m_Fixtures.Companion_Default___.SmallEncryptionContext(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_AB_())
	_12_encryptionContext = _out7
	var _13_reproducedEncryptionContext _dafny.Map
	_ = _13_reproducedEncryptionContext
	var _out8 _dafny.Map
	_ = _out8
	_out8 = m_Fixtures.Companion_Default___.SmallEncryptionContext(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_A_())
	_13_reproducedEncryptionContext = _out8
	var _14_requiredEncryptionContextKeys _dafny.Sequence
	_ = _14_requiredEncryptionContextKeys
	var _out9 _dafny.Sequence
	_ = _out9
	_out9 = m_Fixtures.Companion_Default___.SmallEncryptionContextKeys(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_A_())
	_14_requiredEncryptionContextKeys = _out9
	var _15_valueOrError3 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _15_valueOrError3
	var _out10 m_Wrappers.Result
	_ = _out10
	_out10 = (_5_mpl).CreateDefaultCryptographicMaterialsManager(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultCryptographicMaterialsManagerInput_.Create_CreateDefaultCryptographicMaterialsManagerInput_(_11_multiKeyring))
	_15_valueOrError3 = _out10
	if !(!((_15_valueOrError3).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(439,26): " + (_15_valueOrError3).String())
	}
	var _16_defaultCMM m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _16_defaultCMM
	_16_defaultCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_15_valueOrError3).Extract())
	var _17_encryptOutput m_Wrappers.Result
	_ = _17_encryptOutput
	var _out11 m_Wrappers.Result
	_ = _out11
	_out11 = (_3_esdk).Encrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_EncryptInput_.Create_EncryptInput_(_0_asdf, m_Wrappers.Companion_Option_.Create_Some_(_12_encryptionContext), m_Wrappers.Companion_Option_.Create_Some_(_16_defaultCMM), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_()))
	_17_encryptOutput = _out11
	if !((_17_encryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(455,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _18_esdkCiphertext _dafny.Sequence
	_ = _18_esdkCiphertext
	_18_esdkCiphertext = ((_17_encryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.EncryptOutput)).Dtor_ciphertext()
	var _19_valueOrError4 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _19_valueOrError4
	var _out12 m_Wrappers.Result
	_ = _out12
	_out12 = (_5_mpl).CreateDefaultCryptographicMaterialsManager(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultCryptographicMaterialsManagerInput_.Create_CreateDefaultCryptographicMaterialsManagerInput_(_6_rsaKeyring))
	_19_valueOrError4 = _out12
	if !(!((_19_valueOrError4).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(459,22): " + (_19_valueOrError4).String())
	}
	_16_defaultCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_19_valueOrError4).Extract())
	var _20_valueOrError5 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _20_valueOrError5
	var _out13 m_Wrappers.Result
	_ = _out13
	_out13 = (_5_mpl).CreateRequiredEncryptionContextCMM(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRequiredEncryptionContextCMMInput_.Create_CreateRequiredEncryptionContextCMMInput_(m_Wrappers.Companion_Option_.Create_Some_(_16_defaultCMM), m_Wrappers.Companion_Option_.Create_None_(), _14_requiredEncryptionContextKeys))
	_20_valueOrError5 = _out13
	if !(!((_20_valueOrError5).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(466,22): " + (_20_valueOrError5).String())
	}
	var _21_reqCMM m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _21_reqCMM
	_21_reqCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_20_valueOrError5).Extract())
	var _22_decryptOutput m_Wrappers.Result
	_ = _22_decryptOutput
	var _out14 m_Wrappers.Result
	_ = _out14
	_out14 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_18_esdkCiphertext, m_Wrappers.Companion_Option_.Create_Some_(_21_reqCMM), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_13_reproducedEncryptionContext)))
	_22_decryptOutput = _out14
	if !((_22_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(484,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _23_cycledPlaintext _dafny.Sequence
	_ = _23_cycledPlaintext
	_23_cycledPlaintext = ((_22_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext()
	if !(_dafny.Companion_Sequence_.Equal(_23_cycledPlaintext, _0_asdf)) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(486,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _24_valueOrError6 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _24_valueOrError6
	var _out15 m_Wrappers.Result
	_ = _out15
	_out15 = (_5_mpl).CreateDefaultCryptographicMaterialsManager(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultCryptographicMaterialsManagerInput_.Create_CreateDefaultCryptographicMaterialsManagerInput_(_7_kmsKeyring))
	_24_valueOrError6 = _out15
	if !(!((_24_valueOrError6).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(490,22): " + (_24_valueOrError6).String())
	}
	_16_defaultCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_24_valueOrError6).Extract())
	var _25_valueOrError7 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _25_valueOrError7
	var _out16 m_Wrappers.Result
	_ = _out16
	_out16 = (_5_mpl).CreateRequiredEncryptionContextCMM(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRequiredEncryptionContextCMMInput_.Create_CreateRequiredEncryptionContextCMMInput_(m_Wrappers.Companion_Option_.Create_Some_(_16_defaultCMM), m_Wrappers.Companion_Option_.Create_None_(), _14_requiredEncryptionContextKeys))
	_25_valueOrError7 = _out16
	if !(!((_25_valueOrError7).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(497,18): " + (_25_valueOrError7).String())
	}
	_21_reqCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_25_valueOrError7).Extract())
	var _out17 m_Wrappers.Result
	_ = _out17
	_out17 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_18_esdkCiphertext, m_Wrappers.Companion_Option_.Create_Some_(_21_reqCMM), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_13_reproducedEncryptionContext)))
	_22_decryptOutput = _out17
	if !((_22_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(515,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	_23_cycledPlaintext = ((_22_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext()
	if !(_dafny.Companion_Sequence_.Equal(_23_cycledPlaintext, _0_asdf)) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(517,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _26_valueOrError8 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _26_valueOrError8
	var _out18 m_Wrappers.Result
	_ = _out18
	_out18 = (_5_mpl).CreateDefaultCryptographicMaterialsManager(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultCryptographicMaterialsManagerInput_.Create_CreateDefaultCryptographicMaterialsManagerInput_(_8_aesKeyring))
	_26_valueOrError8 = _out18
	if !(!((_26_valueOrError8).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(521,22): " + (_26_valueOrError8).String())
	}
	_16_defaultCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_26_valueOrError8).Extract())
	var _27_valueOrError9 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _27_valueOrError9
	var _out19 m_Wrappers.Result
	_ = _out19
	_out19 = (_5_mpl).CreateRequiredEncryptionContextCMM(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRequiredEncryptionContextCMMInput_.Create_CreateRequiredEncryptionContextCMMInput_(m_Wrappers.Companion_Option_.Create_Some_(_16_defaultCMM), m_Wrappers.Companion_Option_.Create_None_(), _14_requiredEncryptionContextKeys))
	_27_valueOrError9 = _out19
	if !(!((_27_valueOrError9).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(528,18): " + (_27_valueOrError9).String())
	}
	_21_reqCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_27_valueOrError9).Extract())
	var _out20 m_Wrappers.Result
	_ = _out20
	_out20 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_18_esdkCiphertext, m_Wrappers.Companion_Option_.Create_Some_(_21_reqCMM), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_13_reproducedEncryptionContext)))
	_22_decryptOutput = _out20
	if !((_22_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(546,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	_23_cycledPlaintext = ((_22_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext()
	if !(_dafny.Companion_Sequence_.Equal(_23_cycledPlaintext, _0_asdf)) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(548,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _28_valueOrError10 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _28_valueOrError10
	var _out21 m_Wrappers.Result
	_ = _out21
	_out21 = (_5_mpl).CreateDefaultCryptographicMaterialsManager(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultCryptographicMaterialsManagerInput_.Create_CreateDefaultCryptographicMaterialsManagerInput_(_9_hKeyring))
	_28_valueOrError10 = _out21
	if !(!((_28_valueOrError10).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(551,22): " + (_28_valueOrError10).String())
	}
	_16_defaultCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_28_valueOrError10).Extract())
	var _29_valueOrError11 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _29_valueOrError11
	var _out22 m_Wrappers.Result
	_ = _out22
	_out22 = (_5_mpl).CreateRequiredEncryptionContextCMM(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRequiredEncryptionContextCMMInput_.Create_CreateRequiredEncryptionContextCMMInput_(m_Wrappers.Companion_Option_.Create_Some_(_16_defaultCMM), m_Wrappers.Companion_Option_.Create_None_(), _14_requiredEncryptionContextKeys))
	_29_valueOrError11 = _out22
	if !(!((_29_valueOrError11).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(558,18): " + (_29_valueOrError11).String())
	}
	_21_reqCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_29_valueOrError11).Extract())
	var _out23 m_Wrappers.Result
	_ = _out23
	_out23 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_18_esdkCiphertext, m_Wrappers.Companion_Option_.Create_Some_(_21_reqCMM), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_13_reproducedEncryptionContext)))
	_22_decryptOutput = _out23
	if !((_22_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(576,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	_23_cycledPlaintext = ((_22_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext()
	if !(_dafny.Companion_Sequence_.Equal(_23_cycledPlaintext, _0_asdf)) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(578,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
}
func (_static *CompanionStruct_Default___) TestDifferentECOnDecryptFailure() {
	var _0_asdf _dafny.Sequence
	_ = _0_asdf
	_0_asdf = _dafny.SeqOf(uint8(97), uint8(115), uint8(100), uint8(102))
	var _1_defaultConfig m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _1_defaultConfig
	_1_defaultConfig = m_EncryptionSdk.Companion_Default___.DefaultAwsEncryptionSdkConfig()
	var _2_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _2_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_EncryptionSdk.Companion_Default___.ESDK(_1_defaultConfig)
	_2_valueOrError0 = _out0
	if !(!((_2_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(590,20): " + (_2_valueOrError0).String())
	}
	var _3_esdk *m_EncryptionSdk.ESDKClient
	_ = _3_esdk
	_3_esdk = (_2_valueOrError0).Extract().(*m_EncryptionSdk.ESDKClient)
	var _4_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _4_valueOrError1
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
	_4_valueOrError1 = _out1
	if !(!((_4_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(591,19): " + (_4_valueOrError1).String())
	}
	var _5_mpl *m_MaterialProviders.MaterialProvidersClient
	_ = _5_mpl
	_5_mpl = (_4_valueOrError1).Extract().(*m_MaterialProviders.MaterialProvidersClient)
	var _6_rsaKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _6_rsaKeyring
	var _out2 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out2
	_out2 = Companion_Default___.GetRsaKeyring()
	_6_rsaKeyring = _out2
	var _7_kmsKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _7_kmsKeyring
	var _out3 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out3
	_out3 = Companion_Default___.GetKmsKeyring()
	_7_kmsKeyring = _out3
	var _8_aesKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _8_aesKeyring
	var _out4 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out4
	_out4 = Companion_Default___.GetAesKeyring()
	_8_aesKeyring = _out4
	var _9_hKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _9_hKeyring
	var _out5 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out5
	_out5 = Companion_Default___.GetHierarchicalKeyring()
	_9_hKeyring = _out5
	var _10_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _10_valueOrError2
	var _out6 m_Wrappers.Result
	_ = _out6
	_out6 = (_5_mpl).CreateMultiKeyring(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateMultiKeyringInput_.Create_CreateMultiKeyringInput_(m_Wrappers.Companion_Option_.Create_Some_(_8_aesKeyring), _dafny.SeqOf(_6_rsaKeyring, _7_kmsKeyring, _9_hKeyring)))
	_10_valueOrError2 = _out6
	if !(!((_10_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(599,28): " + (_10_valueOrError2).String())
	}
	var _11_multiKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _11_multiKeyring
	_11_multiKeyring = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_10_valueOrError2).Extract())
	var _12_encryptionContext _dafny.Map
	_ = _12_encryptionContext
	var _out7 _dafny.Map
	_ = _out7
	_out7 = m_Fixtures.Companion_Default___.SmallEncryptionContext(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_AB_())
	_12_encryptionContext = _out7
	var _13_reproducedAdditionalEncryptionContext _dafny.Map
	_ = _13_reproducedAdditionalEncryptionContext
	var _out8 _dafny.Map
	_ = _out8
	_out8 = m_Fixtures.Companion_Default___.SmallEncryptionContext(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_C_())
	_13_reproducedAdditionalEncryptionContext = _out8
	var _14_reproducedMismatchedEncryptionContext _dafny.Map
	_ = _14_reproducedMismatchedEncryptionContext
	var _out9 _dafny.Map
	_ = _out9
	_out9 = m_Fixtures.Companion_Default___.SmallMismatchedEncryptionContex(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_AB_())
	_14_reproducedMismatchedEncryptionContext = _out9
	var _15_encryptOutput m_Wrappers.Result
	_ = _15_encryptOutput
	var _out10 m_Wrappers.Result
	_ = _out10
	_out10 = (_3_esdk).Encrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_EncryptInput_.Create_EncryptInput_(_0_asdf, m_Wrappers.Companion_Option_.Create_Some_(_12_encryptionContext), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_11_multiKeyring), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_()))
	_15_encryptOutput = _out10
	if !((_15_encryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(623,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _16_esdkCiphertext _dafny.Sequence
	_ = _16_esdkCiphertext
	_16_esdkCiphertext = ((_15_encryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.EncryptOutput)).Dtor_ciphertext()
	var _17_decryptOutput m_Wrappers.Result
	_ = _17_decryptOutput
	var _out11 m_Wrappers.Result
	_ = _out11
	_out11 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_16_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_6_rsaKeyring), m_Wrappers.Companion_Option_.Create_Some_(_13_reproducedAdditionalEncryptionContext)))
	_17_decryptOutput = _out11
	if !((_17_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(634,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out12 m_Wrappers.Result
	_ = _out12
	_out12 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_16_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_6_rsaKeyring), m_Wrappers.Companion_Option_.Create_Some_(_14_reproducedMismatchedEncryptionContext)))
	_17_decryptOutput = _out12
	if !((_17_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(643,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out13 m_Wrappers.Result
	_ = _out13
	_out13 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_16_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_7_kmsKeyring), m_Wrappers.Companion_Option_.Create_Some_(_13_reproducedAdditionalEncryptionContext)))
	_17_decryptOutput = _out13
	if !((_17_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(653,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out14 m_Wrappers.Result
	_ = _out14
	_out14 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_16_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_7_kmsKeyring), m_Wrappers.Companion_Option_.Create_Some_(_14_reproducedMismatchedEncryptionContext)))
	_17_decryptOutput = _out14
	if !((_17_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(662,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out15 m_Wrappers.Result
	_ = _out15
	_out15 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_16_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_8_aesKeyring), m_Wrappers.Companion_Option_.Create_Some_(_13_reproducedAdditionalEncryptionContext)))
	_17_decryptOutput = _out15
	if !((_17_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(672,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out16 m_Wrappers.Result
	_ = _out16
	_out16 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_16_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_8_aesKeyring), m_Wrappers.Companion_Option_.Create_Some_(_14_reproducedMismatchedEncryptionContext)))
	_17_decryptOutput = _out16
	if !((_17_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(681,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out17 m_Wrappers.Result
	_ = _out17
	_out17 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_16_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_9_hKeyring), m_Wrappers.Companion_Option_.Create_Some_(_13_reproducedAdditionalEncryptionContext)))
	_17_decryptOutput = _out17
	if !((_17_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(691,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out18 m_Wrappers.Result
	_ = _out18
	_out18 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_16_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_9_hKeyring), m_Wrappers.Companion_Option_.Create_Some_(_14_reproducedMismatchedEncryptionContext)))
	_17_decryptOutput = _out18
	if !((_17_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(700,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
}
func (_static *CompanionStruct_Default___) TestRemoveECAndNotSupplyOnDecryptFailure() {
	var _0_asdf _dafny.Sequence
	_ = _0_asdf
	_0_asdf = _dafny.SeqOf(uint8(97), uint8(115), uint8(100), uint8(102))
	var _1_defaultConfig m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _1_defaultConfig
	_1_defaultConfig = m_EncryptionSdk.Companion_Default___.DefaultAwsEncryptionSdkConfig()
	var _2_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _2_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_EncryptionSdk.Companion_Default___.ESDK(_1_defaultConfig)
	_2_valueOrError0 = _out0
	if !(!((_2_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(715,20): " + (_2_valueOrError0).String())
	}
	var _3_esdk *m_EncryptionSdk.ESDKClient
	_ = _3_esdk
	_3_esdk = (_2_valueOrError0).Extract().(*m_EncryptionSdk.ESDKClient)
	var _4_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _4_valueOrError1
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
	_4_valueOrError1 = _out1
	if !(!((_4_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(716,19): " + (_4_valueOrError1).String())
	}
	var _5_mpl *m_MaterialProviders.MaterialProvidersClient
	_ = _5_mpl
	_5_mpl = (_4_valueOrError1).Extract().(*m_MaterialProviders.MaterialProvidersClient)
	var _6_rsaKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _6_rsaKeyring
	var _out2 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out2
	_out2 = Companion_Default___.GetRsaKeyring()
	_6_rsaKeyring = _out2
	var _7_kmsKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _7_kmsKeyring
	var _out3 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out3
	_out3 = Companion_Default___.GetKmsKeyring()
	_7_kmsKeyring = _out3
	var _8_aesKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _8_aesKeyring
	var _out4 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out4
	_out4 = Companion_Default___.GetAesKeyring()
	_8_aesKeyring = _out4
	var _9_hKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _9_hKeyring
	var _out5 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out5
	_out5 = Companion_Default___.GetHierarchicalKeyring()
	_9_hKeyring = _out5
	var _10_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _10_valueOrError2
	var _out6 m_Wrappers.Result
	_ = _out6
	_out6 = (_5_mpl).CreateMultiKeyring(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateMultiKeyringInput_.Create_CreateMultiKeyringInput_(m_Wrappers.Companion_Option_.Create_Some_(_8_aesKeyring), _dafny.SeqOf(_6_rsaKeyring, _7_kmsKeyring, _9_hKeyring)))
	_10_valueOrError2 = _out6
	if !(!((_10_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(724,28): " + (_10_valueOrError2).String())
	}
	var _11_multiKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _11_multiKeyring
	_11_multiKeyring = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_10_valueOrError2).Extract())
	var _12_encryptionContext _dafny.Map
	_ = _12_encryptionContext
	var _out7 _dafny.Map
	_ = _out7
	_out7 = m_Fixtures.Companion_Default___.SmallEncryptionContext(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_AB_())
	_12_encryptionContext = _out7
	var _13_requiredECKeys _dafny.Sequence
	_ = _13_requiredECKeys
	var _out8 _dafny.Sequence
	_ = _out8
	_out8 = m_Fixtures.Companion_Default___.SmallEncryptionContextKeys(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_A_())
	_13_requiredECKeys = _out8
	var _14_valueOrError3 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _14_valueOrError3
	var _out9 m_Wrappers.Result
	_ = _out9
	_out9 = (_5_mpl).CreateDefaultCryptographicMaterialsManager(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultCryptographicMaterialsManagerInput_.Create_CreateDefaultCryptographicMaterialsManagerInput_(_11_multiKeyring))
	_14_valueOrError3 = _out9
	if !(!((_14_valueOrError3).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(736,26): " + (_14_valueOrError3).String())
	}
	var _15_defaultCMM m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _15_defaultCMM
	_15_defaultCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_14_valueOrError3).Extract())
	var _16_valueOrError4 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _16_valueOrError4
	var _out10 m_Wrappers.Result
	_ = _out10
	_out10 = (_5_mpl).CreateRequiredEncryptionContextCMM(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRequiredEncryptionContextCMMInput_.Create_CreateRequiredEncryptionContextCMMInput_(m_Wrappers.Companion_Option_.Create_Some_(_15_defaultCMM), m_Wrappers.Companion_Option_.Create_None_(), _13_requiredECKeys))
	_16_valueOrError4 = _out10
	if !(!((_16_valueOrError4).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(743,22): " + (_16_valueOrError4).String())
	}
	var _17_reqCMM m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _17_reqCMM
	_17_reqCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_16_valueOrError4).Extract())
	var _18_encryptOutput m_Wrappers.Result
	_ = _18_encryptOutput
	var _out11 m_Wrappers.Result
	_ = _out11
	_out11 = (_3_esdk).Encrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_EncryptInput_.Create_EncryptInput_(_0_asdf, m_Wrappers.Companion_Option_.Create_Some_(_12_encryptionContext), m_Wrappers.Companion_Option_.Create_Some_(_17_reqCMM), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_()))
	_18_encryptOutput = _out11
	if !((_18_encryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(762,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _19_esdkCiphertext _dafny.Sequence
	_ = _19_esdkCiphertext
	_19_esdkCiphertext = ((_18_encryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.EncryptOutput)).Dtor_ciphertext()
	var _20_decryptOutput m_Wrappers.Result
	_ = _20_decryptOutput
	var _out12 m_Wrappers.Result
	_ = _out12
	_out12 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_19_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_6_rsaKeyring), m_Wrappers.Companion_Option_.Create_None_()))
	_20_decryptOutput = _out12
	if !((_20_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(773,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out13 m_Wrappers.Result
	_ = _out13
	_out13 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_19_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_7_kmsKeyring), m_Wrappers.Companion_Option_.Create_None_()))
	_20_decryptOutput = _out13
	if !((_20_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(783,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out14 m_Wrappers.Result
	_ = _out14
	_out14 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_19_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_8_aesKeyring), m_Wrappers.Companion_Option_.Create_None_()))
	_20_decryptOutput = _out14
	if !((_20_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(793,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out15 m_Wrappers.Result
	_ = _out15
	_out15 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_19_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_9_hKeyring), m_Wrappers.Companion_Option_.Create_None_()))
	_20_decryptOutput = _out15
	if !((_20_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(803,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
}
func (_static *CompanionStruct_Default___) TestRemoveECAndSupplyMismatchedReprECFailure() {
	var _0_asdf _dafny.Sequence
	_ = _0_asdf
	_0_asdf = _dafny.SeqOf(uint8(97), uint8(115), uint8(100), uint8(102))
	var _1_defaultConfig m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _1_defaultConfig
	_1_defaultConfig = m_EncryptionSdk.Companion_Default___.DefaultAwsEncryptionSdkConfig()
	var _2_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _2_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_EncryptionSdk.Companion_Default___.ESDK(_1_defaultConfig)
	_2_valueOrError0 = _out0
	if !(!((_2_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(818,20): " + (_2_valueOrError0).String())
	}
	var _3_esdk *m_EncryptionSdk.ESDKClient
	_ = _3_esdk
	_3_esdk = (_2_valueOrError0).Extract().(*m_EncryptionSdk.ESDKClient)
	var _4_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _4_valueOrError1
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
	_4_valueOrError1 = _out1
	if !(!((_4_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(819,19): " + (_4_valueOrError1).String())
	}
	var _5_mpl *m_MaterialProviders.MaterialProvidersClient
	_ = _5_mpl
	_5_mpl = (_4_valueOrError1).Extract().(*m_MaterialProviders.MaterialProvidersClient)
	var _6_rsaKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _6_rsaKeyring
	var _out2 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out2
	_out2 = Companion_Default___.GetRsaKeyring()
	_6_rsaKeyring = _out2
	var _7_kmsKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _7_kmsKeyring
	var _out3 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out3
	_out3 = Companion_Default___.GetKmsKeyring()
	_7_kmsKeyring = _out3
	var _8_aesKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _8_aesKeyring
	var _out4 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out4
	_out4 = Companion_Default___.GetAesKeyring()
	_8_aesKeyring = _out4
	var _9_hKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _9_hKeyring
	var _out5 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out5
	_out5 = Companion_Default___.GetHierarchicalKeyring()
	_9_hKeyring = _out5
	var _10_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _10_valueOrError2
	var _out6 m_Wrappers.Result
	_ = _out6
	_out6 = (_5_mpl).CreateMultiKeyring(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateMultiKeyringInput_.Create_CreateMultiKeyringInput_(m_Wrappers.Companion_Option_.Create_Some_(_8_aesKeyring), _dafny.SeqOf(_6_rsaKeyring, _7_kmsKeyring, _9_hKeyring)))
	_10_valueOrError2 = _out6
	if !(!((_10_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(827,28): " + (_10_valueOrError2).String())
	}
	var _11_multiKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _11_multiKeyring
	_11_multiKeyring = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_10_valueOrError2).Extract())
	var _12_encryptionContext _dafny.Map
	_ = _12_encryptionContext
	var _out7 _dafny.Map
	_ = _out7
	_out7 = m_Fixtures.Companion_Default___.SmallEncryptionContext(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_AB_())
	_12_encryptionContext = _out7
	var _13_requiredECKeys _dafny.Sequence
	_ = _13_requiredECKeys
	var _out8 _dafny.Sequence
	_ = _out8
	_out8 = m_Fixtures.Companion_Default___.SmallEncryptionContextKeys(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_A_())
	_13_requiredECKeys = _out8
	var _14_mismatchedReproducedEncryptionContext _dafny.Map
	_ = _14_mismatchedReproducedEncryptionContext
	var _out9 _dafny.Map
	_ = _out9
	_out9 = m_Fixtures.Companion_Default___.SmallMismatchedEncryptionContex(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_A_())
	_14_mismatchedReproducedEncryptionContext = _out9
	var _15_valueOrError3 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _15_valueOrError3
	var _out10 m_Wrappers.Result
	_ = _out10
	_out10 = (_5_mpl).CreateDefaultCryptographicMaterialsManager(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultCryptographicMaterialsManagerInput_.Create_CreateDefaultCryptographicMaterialsManagerInput_(_11_multiKeyring))
	_15_valueOrError3 = _out10
	if !(!((_15_valueOrError3).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(841,26): " + (_15_valueOrError3).String())
	}
	var _16_defaultCMM m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _16_defaultCMM
	_16_defaultCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_15_valueOrError3).Extract())
	var _17_valueOrError4 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _17_valueOrError4
	var _out11 m_Wrappers.Result
	_ = _out11
	_out11 = (_5_mpl).CreateRequiredEncryptionContextCMM(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRequiredEncryptionContextCMMInput_.Create_CreateRequiredEncryptionContextCMMInput_(m_Wrappers.Companion_Option_.Create_Some_(_16_defaultCMM), m_Wrappers.Companion_Option_.Create_None_(), _13_requiredECKeys))
	_17_valueOrError4 = _out11
	if !(!((_17_valueOrError4).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(848,22): " + (_17_valueOrError4).String())
	}
	var _18_reqCMM m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _18_reqCMM
	_18_reqCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_17_valueOrError4).Extract())
	var _19_encryptOutput m_Wrappers.Result
	_ = _19_encryptOutput
	var _out12 m_Wrappers.Result
	_ = _out12
	_out12 = (_3_esdk).Encrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_EncryptInput_.Create_EncryptInput_(_0_asdf, m_Wrappers.Companion_Option_.Create_Some_(_12_encryptionContext), m_Wrappers.Companion_Option_.Create_Some_(_18_reqCMM), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_()))
	_19_encryptOutput = _out12
	if !((_19_encryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(867,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _20_esdkCiphertext _dafny.Sequence
	_ = _20_esdkCiphertext
	_20_esdkCiphertext = ((_19_encryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.EncryptOutput)).Dtor_ciphertext()
	var _21_decryptOutput m_Wrappers.Result
	_ = _21_decryptOutput
	var _out13 m_Wrappers.Result
	_ = _out13
	_out13 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_20_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_6_rsaKeyring), m_Wrappers.Companion_Option_.Create_Some_(_14_mismatchedReproducedEncryptionContext)))
	_21_decryptOutput = _out13
	if !((_21_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(878,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out14 m_Wrappers.Result
	_ = _out14
	_out14 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_20_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_7_kmsKeyring), m_Wrappers.Companion_Option_.Create_Some_(_14_mismatchedReproducedEncryptionContext)))
	_21_decryptOutput = _out14
	if !((_21_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(888,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out15 m_Wrappers.Result
	_ = _out15
	_out15 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_20_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_8_aesKeyring), m_Wrappers.Companion_Option_.Create_Some_(_14_mismatchedReproducedEncryptionContext)))
	_21_decryptOutput = _out15
	if !((_21_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(898,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out16 m_Wrappers.Result
	_ = _out16
	_out16 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_20_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_9_hKeyring), m_Wrappers.Companion_Option_.Create_Some_(_14_mismatchedReproducedEncryptionContext)))
	_21_decryptOutput = _out16
	if !((_21_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(908,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
}
func (_static *CompanionStruct_Default___) TestRemoveECAndSupplyWithMissingRequiredValueDecryptFailure() {
	var _0_asdf _dafny.Sequence
	_ = _0_asdf
	_0_asdf = _dafny.SeqOf(uint8(97), uint8(115), uint8(100), uint8(102))
	var _1_defaultConfig m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _1_defaultConfig
	_1_defaultConfig = m_EncryptionSdk.Companion_Default___.DefaultAwsEncryptionSdkConfig()
	var _2_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _2_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_EncryptionSdk.Companion_Default___.ESDK(_1_defaultConfig)
	_2_valueOrError0 = _out0
	if !(!((_2_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(922,20): " + (_2_valueOrError0).String())
	}
	var _3_esdk *m_EncryptionSdk.ESDKClient
	_ = _3_esdk
	_3_esdk = (_2_valueOrError0).Extract().(*m_EncryptionSdk.ESDKClient)
	var _4_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _4_valueOrError1
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
	_4_valueOrError1 = _out1
	if !(!((_4_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(923,19): " + (_4_valueOrError1).String())
	}
	var _5_mpl *m_MaterialProviders.MaterialProvidersClient
	_ = _5_mpl
	_5_mpl = (_4_valueOrError1).Extract().(*m_MaterialProviders.MaterialProvidersClient)
	var _6_rsaKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _6_rsaKeyring
	var _out2 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out2
	_out2 = Companion_Default___.GetRsaKeyring()
	_6_rsaKeyring = _out2
	var _7_kmsKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _7_kmsKeyring
	var _out3 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out3
	_out3 = Companion_Default___.GetKmsKeyring()
	_7_kmsKeyring = _out3
	var _8_aesKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _8_aesKeyring
	var _out4 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out4
	_out4 = Companion_Default___.GetAesKeyring()
	_8_aesKeyring = _out4
	var _9_hKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _9_hKeyring
	var _out5 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out5
	_out5 = Companion_Default___.GetHierarchicalKeyring()
	_9_hKeyring = _out5
	var _10_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _10_valueOrError2
	var _out6 m_Wrappers.Result
	_ = _out6
	_out6 = (_5_mpl).CreateMultiKeyring(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateMultiKeyringInput_.Create_CreateMultiKeyringInput_(m_Wrappers.Companion_Option_.Create_Some_(_8_aesKeyring), _dafny.SeqOf(_6_rsaKeyring, _7_kmsKeyring, _9_hKeyring)))
	_10_valueOrError2 = _out6
	if !(!((_10_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(931,28): " + (_10_valueOrError2).String())
	}
	var _11_multiKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _11_multiKeyring
	_11_multiKeyring = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_10_valueOrError2).Extract())
	var _12_encryptionContext _dafny.Map
	_ = _12_encryptionContext
	var _out7 _dafny.Map
	_ = _out7
	_out7 = m_Fixtures.Companion_Default___.SmallEncryptionContext(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_AB_())
	_12_encryptionContext = _out7
	var _13_requiredECKeys _dafny.Sequence
	_ = _13_requiredECKeys
	var _out8 _dafny.Sequence
	_ = _out8
	_out8 = m_Fixtures.Companion_Default___.SmallEncryptionContextKeys(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_A_())
	_13_requiredECKeys = _out8
	var _14_droppedRequiredKeyEncryptionContext _dafny.Map
	_ = _14_droppedRequiredKeyEncryptionContext
	var _out9 _dafny.Map
	_ = _out9
	_out9 = m_Fixtures.Companion_Default___.SmallEncryptionContext(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_B_())
	_14_droppedRequiredKeyEncryptionContext = _out9
	var _15_valueOrError3 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _15_valueOrError3
	var _out10 m_Wrappers.Result
	_ = _out10
	_out10 = (_5_mpl).CreateDefaultCryptographicMaterialsManager(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultCryptographicMaterialsManagerInput_.Create_CreateDefaultCryptographicMaterialsManagerInput_(_11_multiKeyring))
	_15_valueOrError3 = _out10
	if !(!((_15_valueOrError3).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(945,26): " + (_15_valueOrError3).String())
	}
	var _16_defaultCMM m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _16_defaultCMM
	_16_defaultCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_15_valueOrError3).Extract())
	var _17_valueOrError4 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _17_valueOrError4
	var _out11 m_Wrappers.Result
	_ = _out11
	_out11 = (_5_mpl).CreateRequiredEncryptionContextCMM(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRequiredEncryptionContextCMMInput_.Create_CreateRequiredEncryptionContextCMMInput_(m_Wrappers.Companion_Option_.Create_Some_(_16_defaultCMM), m_Wrappers.Companion_Option_.Create_None_(), _13_requiredECKeys))
	_17_valueOrError4 = _out11
	if !(!((_17_valueOrError4).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(952,22): " + (_17_valueOrError4).String())
	}
	var _18_reqCMM m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _18_reqCMM
	_18_reqCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_17_valueOrError4).Extract())
	var _19_encryptOutput m_Wrappers.Result
	_ = _19_encryptOutput
	var _out12 m_Wrappers.Result
	_ = _out12
	_out12 = (_3_esdk).Encrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_EncryptInput_.Create_EncryptInput_(_0_asdf, m_Wrappers.Companion_Option_.Create_Some_(_12_encryptionContext), m_Wrappers.Companion_Option_.Create_Some_(_18_reqCMM), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_()))
	_19_encryptOutput = _out12
	if !((_19_encryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(971,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _20_esdkCiphertext _dafny.Sequence
	_ = _20_esdkCiphertext
	_20_esdkCiphertext = ((_19_encryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.EncryptOutput)).Dtor_ciphertext()
	var _21_decryptOutput m_Wrappers.Result
	_ = _21_decryptOutput
	var _out13 m_Wrappers.Result
	_ = _out13
	_out13 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_20_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_6_rsaKeyring), m_Wrappers.Companion_Option_.Create_Some_(_14_droppedRequiredKeyEncryptionContext)))
	_21_decryptOutput = _out13
	if !((_21_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(982,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out14 m_Wrappers.Result
	_ = _out14
	_out14 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_20_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_7_kmsKeyring), m_Wrappers.Companion_Option_.Create_Some_(_14_droppedRequiredKeyEncryptionContext)))
	_21_decryptOutput = _out14
	if !((_21_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(992,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out15 m_Wrappers.Result
	_ = _out15
	_out15 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_20_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_8_aesKeyring), m_Wrappers.Companion_Option_.Create_Some_(_14_droppedRequiredKeyEncryptionContext)))
	_21_decryptOutput = _out15
	if !((_21_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1002,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out16 m_Wrappers.Result
	_ = _out16
	_out16 = (_3_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_20_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_9_hKeyring), m_Wrappers.Companion_Option_.Create_Some_(_14_droppedRequiredKeyEncryptionContext)))
	_21_decryptOutput = _out16
	if !((_21_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1012,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
}
func (_static *CompanionStruct_Default___) TestReservedEncryptionContextKeyFailure() {
	var _0_asdf _dafny.Sequence
	_ = _0_asdf
	_0_asdf = _dafny.SeqOf(uint8(97), uint8(115), uint8(100), uint8(102))
	var _1_defaultConfig m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _1_defaultConfig
	_1_defaultConfig = m_EncryptionSdk.Companion_Default___.DefaultAwsEncryptionSdkConfig()
	var _2_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _2_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_EncryptionSdk.Companion_Default___.ESDK(_1_defaultConfig)
	_2_valueOrError0 = _out0
	if !(!((_2_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1022,20): " + (_2_valueOrError0).String())
	}
	var _3_esdk *m_EncryptionSdk.ESDKClient
	_ = _3_esdk
	_3_esdk = (_2_valueOrError0).Extract().(*m_EncryptionSdk.ESDKClient)
	var _4_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _4_valueOrError1
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
	_4_valueOrError1 = _out1
	if !(!((_4_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1023,19): " + (_4_valueOrError1).String())
	}
	var _5_mpl *m_MaterialProviders.MaterialProvidersClient
	_ = _5_mpl
	_5_mpl = (_4_valueOrError1).Extract().(*m_MaterialProviders.MaterialProvidersClient)
	var _6_rsaKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _6_rsaKeyring
	var _out2 m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _out2
	_out2 = Companion_Default___.GetRsaKeyring()
	_6_rsaKeyring = _out2
	var _7_encryptionContext _dafny.Map
	_ = _7_encryptionContext
	var _out3 _dafny.Map
	_ = _out3
	_out3 = m_Fixtures.Companion_Default___.GetResrvedECMap()
	_7_encryptionContext = _out3
	var _8_requiredECKeys _dafny.Sequence
	_ = _8_requiredECKeys
	_8_requiredECKeys = _dafny.SeqOf(m_Fixtures.Companion_Default___.RESERVED__ENCRYPTION__CONTEXT())
	var _9_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _9_valueOrError2
	var _out4 m_Wrappers.Result
	_ = _out4
	_out4 = (_5_mpl).CreateDefaultCryptographicMaterialsManager(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultCryptographicMaterialsManagerInput_.Create_CreateDefaultCryptographicMaterialsManagerInput_(_6_rsaKeyring))
	_9_valueOrError2 = _out4
	if !(!((_9_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1031,26): " + (_9_valueOrError2).String())
	}
	var _10_defaultCMM m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _10_defaultCMM
	_10_defaultCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_9_valueOrError2).Extract())
	var _11_valueOrError3 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _11_valueOrError3
	var _out5 m_Wrappers.Result
	_ = _out5
	_out5 = (_5_mpl).CreateRequiredEncryptionContextCMM(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRequiredEncryptionContextCMMInput_.Create_CreateRequiredEncryptionContextCMMInput_(m_Wrappers.Companion_Option_.Create_Some_(_10_defaultCMM), m_Wrappers.Companion_Option_.Create_None_(), _8_requiredECKeys))
	_11_valueOrError3 = _out5
	if !(!((_11_valueOrError3).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1041,22): " + (_11_valueOrError3).String())
	}
	var _12_reqCMM m_AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager
	_ = _12_reqCMM
	_12_reqCMM = m_AwsCryptographyMaterialProvidersTypes.Companion_ICryptographicMaterialsManager_.CastTo_((_11_valueOrError3).Extract())
	var _13_encryptOutput m_Wrappers.Result
	_ = _13_encryptOutput
	var _out6 m_Wrappers.Result
	_ = _out6
	_out6 = (_3_esdk).Encrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_EncryptInput_.Create_EncryptInput_(_0_asdf, m_Wrappers.Companion_Option_.Create_Some_(_7_encryptionContext), m_Wrappers.Companion_Option_.Create_Some_(_12_reqCMM), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_()))
	_13_encryptOutput = _out6
	if !((_13_encryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1060,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
}
func (_static *CompanionStruct_Default___) GetHierarchicalKeyring() m_AwsCryptographyMaterialProvidersTypes.IKeyring {
	var output m_AwsCryptographyMaterialProvidersTypes.IKeyring = (m_AwsCryptographyMaterialProvidersTypes.IKeyring)(nil)
	_ = output
	var _0_branchKeyId _dafny.Sequence
	_ = _0_branchKeyId
	_0_branchKeyId = Companion_Default___.BRANCH__KEY__ID()
	var _1_ttl int64
	_ = _1_ttl
	_1_ttl = ((int64(1)) * (int64(60000))) * (int64(10))
	var _2_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _2_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
	_2_valueOrError0 = _out0
	if !(!((_2_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1070,19): " + (_2_valueOrError0).String())
	}
	var _3_mpl *m_MaterialProviders.MaterialProvidersClient
	_ = _3_mpl
	_3_mpl = (_2_valueOrError0).Extract().(*m_MaterialProviders.MaterialProvidersClient)
	var _4_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _4_valueOrError1
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_Com_Amazonaws_Kms.Companion_Default___.KMSClient()
	_4_valueOrError1 = _out1
	if !(!((_4_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1072,25): " + (_4_valueOrError1).String())
	}
	var _5_kmsClient m_ComAmazonawsKmsTypes.IKMSClient
	_ = _5_kmsClient
	_5_kmsClient = m_ComAmazonawsKmsTypes.Companion_IKMSClient_.CastTo_((_4_valueOrError1).Extract())
	var _6_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _6_valueOrError2
	var _out2 m_Wrappers.Result
	_ = _out2
	_out2 = m_Com_Amazonaws_Dynamodb.Companion_Default___.DynamoDBClient()
	_6_valueOrError2 = _out2
	if !(!((_6_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1073,25): " + (_6_valueOrError2).String())
	}
	var _7_ddbClient m_ComAmazonawsDynamodbTypes.IDynamoDBClient
	_ = _7_ddbClient
	_7_ddbClient = m_ComAmazonawsDynamodbTypes.Companion_IDynamoDBClient_.CastTo_((_6_valueOrError2).Extract())
	var _8_kmsConfig m_AwsCryptographyKeyStoreTypes.KMSConfiguration
	_ = _8_kmsConfig
	_8_kmsConfig = m_AwsCryptographyKeyStoreTypes.Companion_KMSConfiguration_.Create_kmsKeyArn_(Companion_Default___.HierarchyKeyArn())
	var _9_keyStoreConfig m_AwsCryptographyKeyStoreTypes.KeyStoreConfig
	_ = _9_keyStoreConfig
	_9_keyStoreConfig = m_AwsCryptographyKeyStoreTypes.Companion_KeyStoreConfig_.Create_KeyStoreConfig_(Companion_Default___.BranchKeyStoreName(), _8_kmsConfig, Companion_Default___.LogicalKeyStoreName(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_7_ddbClient), m_Wrappers.Companion_Option_.Create_Some_(_5_kmsClient))
	var _10_valueOrError3 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _10_valueOrError3
	var _out3 m_Wrappers.Result
	_ = _out3
	_out3 = m_KeyStore.Companion_Default___.KeyStore(_9_keyStoreConfig)
	_10_valueOrError3 = _out3
	if !(!((_10_valueOrError3).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1086,24): " + (_10_valueOrError3).String())
	}
	var _11_keyStore *m_KeyStore.KeyStoreClient
	_ = _11_keyStore
	_11_keyStore = (_10_valueOrError3).Extract().(*m_KeyStore.KeyStoreClient)
	var _12_valueOrError4 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _12_valueOrError4
	var _out4 m_Wrappers.Result
	_ = _out4
	_out4 = (_3_mpl).CreateAwsKmsHierarchicalKeyring(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateAwsKmsHierarchicalKeyringInput_.Create_CreateAwsKmsHierarchicalKeyringInput_(m_Wrappers.Companion_Option_.Create_Some_(_0_branchKeyId), m_Wrappers.Companion_Option_.Create_None_(), _11_keyStore, _1_ttl, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_()))
	_12_valueOrError4 = _out4
	if !(!((_12_valueOrError4).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1088,18): " + (_12_valueOrError4).String())
	}
	output = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_12_valueOrError4).Extract())
	return output
}
func (_static *CompanionStruct_Default___) GetRsaKeyring() m_AwsCryptographyMaterialProvidersTypes.IKeyring {
	var output m_AwsCryptographyMaterialProvidersTypes.IKeyring = (m_AwsCryptographyMaterialProvidersTypes.IKeyring)(nil)
	_ = output
	var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _0_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
	_0_valueOrError0 = _out0
	if !(!((_0_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1102,19): " + (_0_valueOrError0).String())
	}
	var _1_mpl *m_MaterialProviders.MaterialProvidersClient
	_ = _1_mpl
	_1_mpl = (_0_valueOrError0).Extract().(*m_MaterialProviders.MaterialProvidersClient)
	var _2_namespace _dafny.Sequence
	_ = _2_namespace
	var _3_name _dafny.Sequence
	_ = _3_name
	var _out1 _dafny.Sequence
	_ = _out1
	var _out2 _dafny.Sequence
	_ = _out2
	_out1, _out2 = m_Fixtures.Companion_Default___.NamespaceAndName(_dafny.Zero)
	_2_namespace = _out1
	_3_name = _out2
	var _4_keys m_AwsCryptographyPrimitivesTypes.GenerateRSAKeyPairOutput
	_ = _4_keys
	var _out3 m_AwsCryptographyPrimitivesTypes.GenerateRSAKeyPairOutput
	_ = _out3
	_out3 = m_Fixtures.Companion_Default___.GenerateKeyPair(int32(2048))
	_4_keys = _out3
	var _5_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _5_valueOrError1
	var _out4 m_Wrappers.Result
	_ = _out4
	_out4 = (_1_mpl).CreateRawRsaKeyring(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRawRsaKeyringInput_.Create_CreateRawRsaKeyringInput_(_2_namespace, _3_name, m_AwsCryptographyMaterialProvidersTypes.Companion_PaddingScheme_.Create_OAEP__SHA1__MGF1_(), m_Wrappers.Companion_Option_.Create_Some_(((_4_keys).Dtor_publicKey()).Dtor_pem()), m_Wrappers.Companion_Option_.Create_Some_(((_4_keys).Dtor_privateKey()).Dtor_pem())))
	_5_valueOrError1 = _out4
	if !(!((_5_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1106,18): " + (_5_valueOrError1).String())
	}
	output = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_5_valueOrError1).Extract())
	return output
}
func (_static *CompanionStruct_Default___) GetAesKeyring() m_AwsCryptographyMaterialProvidersTypes.IKeyring {
	var output m_AwsCryptographyMaterialProvidersTypes.IKeyring = (m_AwsCryptographyMaterialProvidersTypes.IKeyring)(nil)
	_ = output
	var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _0_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
	_0_valueOrError0 = _out0
	if !(!((_0_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1120,19): " + (_0_valueOrError0).String())
	}
	var _1_mpl *m_MaterialProviders.MaterialProvidersClient
	_ = _1_mpl
	_1_mpl = (_0_valueOrError0).Extract().(*m_MaterialProviders.MaterialProvidersClient)
	var _2_namespace _dafny.Sequence
	_ = _2_namespace
	var _3_name _dafny.Sequence
	_ = _3_name
	var _out1 _dafny.Sequence
	_ = _out1
	var _out2 _dafny.Sequence
	_ = _out2
	_out1, _out2 = m_Fixtures.Companion_Default___.NamespaceAndName(_dafny.Zero)
	_2_namespace = _out1
	_3_name = _out2
	var _4_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _4_valueOrError1
	var _out3 m_Wrappers.Result
	_ = _out3
	_out3 = (_1_mpl).CreateRawAesKeyring(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRawAesKeyringInput_.Create_CreateRawAesKeyringInput_(_2_namespace, _3_name, _dafny.SeqCreate(32, func(coer0 func(_dafny.Int) uint8) func(_dafny.Int) interface{} {
		return func(arg0 _dafny.Int) interface{} {
			return coer0(arg0)
		}
	}(func(_5_i _dafny.Int) uint8 {
		return uint8(0)
	})), m_AwsCryptographyMaterialProvidersTypes.Companion_AesWrappingAlg_.Create_ALG__AES256__GCM__IV12__TAG16_()))
	_4_valueOrError1 = _out3
	if !(!((_4_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1123,18): " + (_4_valueOrError1).String())
	}
	output = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_4_valueOrError1).Extract())
	return output
}
func (_static *CompanionStruct_Default___) GetKmsKeyring() m_AwsCryptographyMaterialProvidersTypes.IKeyring {
	var output m_AwsCryptographyMaterialProvidersTypes.IKeyring = (m_AwsCryptographyMaterialProvidersTypes.IKeyring)(nil)
	_ = output
	var _0_kmsKey _dafny.Sequence
	_ = _0_kmsKey
	_0_kmsKey = m_Fixtures.Companion_Default___.KeyArn()
	var _1_defaultConfig m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _1_defaultConfig
	_1_defaultConfig = m_EncryptionSdk.Companion_Default___.DefaultAwsEncryptionSdkConfig()
	var _2_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _2_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_EncryptionSdk.Companion_Default___.ESDK(_1_defaultConfig)
	_2_valueOrError0 = _out0
	if !(!((_2_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1137,20): " + (_2_valueOrError0).String())
	}
	var _3_esdk *m_EncryptionSdk.ESDKClient
	_ = _3_esdk
	_3_esdk = (_2_valueOrError0).Extract().(*m_EncryptionSdk.ESDKClient)
	var _4_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _4_valueOrError1
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
	_4_valueOrError1 = _out1
	if !(!((_4_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1138,19): " + (_4_valueOrError1).String())
	}
	var _5_mpl *m_MaterialProviders.MaterialProvidersClient
	_ = _5_mpl
	_5_mpl = (_4_valueOrError1).Extract().(*m_MaterialProviders.MaterialProvidersClient)
	var _6_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _6_valueOrError2
	var _out2 m_Wrappers.Result
	_ = _out2
	_out2 = (_5_mpl).CreateDefaultClientSupplier(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultClientSupplierInput_.Create_CreateDefaultClientSupplierInput_())
	_6_valueOrError2 = _out2
	if !(!((_6_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1139,30): " + (_6_valueOrError2).String())
	}
	var _7_clientSupplier m_AwsCryptographyMaterialProvidersTypes.IClientSupplier
	_ = _7_clientSupplier
	_7_clientSupplier = m_AwsCryptographyMaterialProvidersTypes.Companion_IClientSupplier_.CastTo_((_6_valueOrError2).Extract())
	var _8_valueOrError3 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _8_valueOrError3
	var _out3 m_Wrappers.Result
	_ = _out3
	_out3 = (_7_clientSupplier).GetClient(m_AwsCryptographyMaterialProvidersTypes.Companion_GetClientInput_.Create_GetClientInput_(_dafny.SeqOfString("us-west-2")))
	_8_valueOrError3 = _out3
	if !(!((_8_valueOrError3).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1140,25): " + (_8_valueOrError3).String())
	}
	var _9_kmsClient m_ComAmazonawsKmsTypes.IKMSClient
	_ = _9_kmsClient
	_9_kmsClient = m_ComAmazonawsKmsTypes.Companion_IKMSClient_.CastTo_((_8_valueOrError3).Extract())
	var _10_valueOrError4 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _10_valueOrError4
	var _out4 m_Wrappers.Result
	_ = _out4
	_out4 = (_5_mpl).CreateAwsKmsKeyring(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateAwsKmsKeyringInput_.Create_CreateAwsKmsKeyringInput_(_0_kmsKey, _9_kmsClient, m_Wrappers.Companion_Option_.Create_None_()))
	_10_valueOrError4 = _out4
	if !(!((_10_valueOrError4).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestRequiredEncryptionContext.dfy(1142,18): " + (_10_valueOrError4).String())
	}
	output = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_10_valueOrError4).Extract())
	return output
}
func (_static *CompanionStruct_Default___) BranchKeyStoreName() _dafny.Sequence {
	return m_Fixtures.Companion_Default___.BranchKeyStoreName()
}
func (_static *CompanionStruct_Default___) LogicalKeyStoreName() _dafny.Sequence {
	return Companion_Default___.BranchKeyStoreName()
}
func (_static *CompanionStruct_Default___) BRANCH__KEY__ID() _dafny.Sequence {
	return m_Fixtures.Companion_Default___.BranchKeyId()
}
func (_static *CompanionStruct_Default___) HierarchyKeyArn() _dafny.Sequence {
	return m_Fixtures.Companion_Default___.HierarchyKeyArn()
}
func (_static *CompanionStruct_Default___) KeyArn() _dafny.Sequence {
	return m_Fixtures.Companion_Default___.KeyArn()
}

// End of class Default__
