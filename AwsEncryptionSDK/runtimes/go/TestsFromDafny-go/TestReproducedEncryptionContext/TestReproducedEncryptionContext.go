// Package TestReproducedEncryptionContext
// Dafny module TestReproducedEncryptionContext compiled into Go

package TestReproducedEncryptionContext

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
	m_TestRequiredEncryptionContext "github.com/aws/aws-encryption-sdk/test/TestRequiredEncryptionContext"
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
var _ m_TestRequiredEncryptionContext.Dummy__

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
	return "TestReproducedEncryptionContext.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) TestEncryptionContextOnDecrypt() {
	var _0_kmsKey _dafny.Sequence
	_ = _0_kmsKey
	_0_kmsKey = m_Fixtures.Companion_Default___.KeyArn()
	var _1_asdf _dafny.Sequence
	_ = _1_asdf
	_1_asdf = _dafny.SeqOf(uint8(97), uint8(115), uint8(100), uint8(102))
	var _2_defaultConfig m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _2_defaultConfig
	_2_defaultConfig = m_EncryptionSdk.Companion_Default___.DefaultAwsEncryptionSdkConfig()
	var _3_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _3_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_EncryptionSdk.Companion_Default___.ESDK(_2_defaultConfig)
	_3_valueOrError0 = _out0
	if !(!((_3_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(24,20): " + (_3_valueOrError0).String())
	}
	var _4_esdk *m_EncryptionSdk.ESDKClient
	_ = _4_esdk
	_4_esdk = (_3_valueOrError0).Extract().(*m_EncryptionSdk.ESDKClient)
	var _5_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _5_valueOrError1
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
	_5_valueOrError1 = _out1
	if !(!((_5_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(25,19): " + (_5_valueOrError1).String())
	}
	var _6_mpl *m_MaterialProviders.MaterialProvidersClient
	_ = _6_mpl
	_6_mpl = (_5_valueOrError1).Extract().(*m_MaterialProviders.MaterialProvidersClient)
	var _7_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _7_valueOrError2
	var _out2 m_Wrappers.Result
	_ = _out2
	_out2 = (_6_mpl).CreateDefaultClientSupplier(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultClientSupplierInput_.Create_CreateDefaultClientSupplierInput_())
	_7_valueOrError2 = _out2
	if !(!((_7_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(26,30): " + (_7_valueOrError2).String())
	}
	var _8_clientSupplier m_AwsCryptographyMaterialProvidersTypes.IClientSupplier
	_ = _8_clientSupplier
	_8_clientSupplier = m_AwsCryptographyMaterialProvidersTypes.Companion_IClientSupplier_.CastTo_((_7_valueOrError2).Extract())
	var _9_valueOrError3 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _9_valueOrError3
	var _out3 m_Wrappers.Result
	_ = _out3
	_out3 = (_8_clientSupplier).GetClient(m_AwsCryptographyMaterialProvidersTypes.Companion_GetClientInput_.Create_GetClientInput_(_dafny.SeqOfString("us-west-2")))
	_9_valueOrError3 = _out3
	if !(!((_9_valueOrError3).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(27,25): " + (_9_valueOrError3).String())
	}
	var _10_kmsClient m_ComAmazonawsKmsTypes.IKMSClient
	_ = _10_kmsClient
	_10_kmsClient = m_ComAmazonawsKmsTypes.Companion_IKMSClient_.CastTo_((_9_valueOrError3).Extract())
	var _11_valueOrError4 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _11_valueOrError4
	var _out4 m_Wrappers.Result
	_ = _out4
	_out4 = (_6_mpl).CreateAwsKmsKeyring(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateAwsKmsKeyringInput_.Create_CreateAwsKmsKeyringInput_(_0_kmsKey, _10_kmsClient, m_Wrappers.Companion_Option_.Create_None_()))
	_11_valueOrError4 = _out4
	if !(!((_11_valueOrError4).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(29,26): " + (_11_valueOrError4).String())
	}
	var _12_kmsKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _12_kmsKeyring
	_12_kmsKeyring = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_11_valueOrError4).Extract())
	var _13_encryptionContext _dafny.Map
	_ = _13_encryptionContext
	var _out5 _dafny.Map
	_ = _out5
	_out5 = m_Fixtures.Companion_Default___.SmallEncryptionContext(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_AB_())
	_13_encryptionContext = _out5
	var _14_encryptOutput m_Wrappers.Result
	_ = _14_encryptOutput
	var _out6 m_Wrappers.Result
	_ = _out6
	_out6 = (_4_esdk).Encrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_EncryptInput_.Create_EncryptInput_(_1_asdf, m_Wrappers.Companion_Option_.Create_Some_(_13_encryptionContext), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_12_kmsKeyring), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_()))
	_14_encryptOutput = _out6
	if !((_14_encryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(48,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _15_esdkCiphertext _dafny.Sequence
	_ = _15_esdkCiphertext
	_15_esdkCiphertext = ((_14_encryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.EncryptOutput)).Dtor_ciphertext()
	var _16_decryptOutput m_Wrappers.Result
	_ = _16_decryptOutput
	var _out7 m_Wrappers.Result
	_ = _out7
	_out7 = (_4_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_15_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_12_kmsKeyring), m_Wrappers.Companion_Option_.Create_Some_(_13_encryptionContext)))
	_16_decryptOutput = _out7
	if !((_16_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(58,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _17_cycledPlaintext _dafny.Sequence
	_ = _17_cycledPlaintext
	_17_cycledPlaintext = ((_16_decryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.DecryptOutput)).Dtor_plaintext()
	if !(_dafny.Companion_Sequence_.Equal(_17_cycledPlaintext, _1_asdf)) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(61,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
}
func (_static *CompanionStruct_Default___) TestEncryptionContextOnDecryptFailure() {
	var _0_kmsKey _dafny.Sequence
	_ = _0_kmsKey
	_0_kmsKey = m_Fixtures.Companion_Default___.KeyArn()
	var _1_asdf _dafny.Sequence
	_ = _1_asdf
	_1_asdf = _dafny.SeqOf(uint8(97), uint8(115), uint8(100), uint8(102))
	var _2_defaultConfig m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _2_defaultConfig
	_2_defaultConfig = m_EncryptionSdk.Companion_Default___.DefaultAwsEncryptionSdkConfig()
	var _3_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _3_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_EncryptionSdk.Companion_Default___.ESDK(_2_defaultConfig)
	_3_valueOrError0 = _out0
	if !(!((_3_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(71,20): " + (_3_valueOrError0).String())
	}
	var _4_esdk *m_EncryptionSdk.ESDKClient
	_ = _4_esdk
	_4_esdk = (_3_valueOrError0).Extract().(*m_EncryptionSdk.ESDKClient)
	var _5_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _5_valueOrError1
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
	_5_valueOrError1 = _out1
	if !(!((_5_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(72,19): " + (_5_valueOrError1).String())
	}
	var _6_mpl *m_MaterialProviders.MaterialProvidersClient
	_ = _6_mpl
	_6_mpl = (_5_valueOrError1).Extract().(*m_MaterialProviders.MaterialProvidersClient)
	var _7_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _7_valueOrError2
	var _out2 m_Wrappers.Result
	_ = _out2
	_out2 = (_6_mpl).CreateDefaultClientSupplier(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateDefaultClientSupplierInput_.Create_CreateDefaultClientSupplierInput_())
	_7_valueOrError2 = _out2
	if !(!((_7_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(73,30): " + (_7_valueOrError2).String())
	}
	var _8_clientSupplier m_AwsCryptographyMaterialProvidersTypes.IClientSupplier
	_ = _8_clientSupplier
	_8_clientSupplier = m_AwsCryptographyMaterialProvidersTypes.Companion_IClientSupplier_.CastTo_((_7_valueOrError2).Extract())
	var _9_valueOrError3 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _9_valueOrError3
	var _out3 m_Wrappers.Result
	_ = _out3
	_out3 = (_8_clientSupplier).GetClient(m_AwsCryptographyMaterialProvidersTypes.Companion_GetClientInput_.Create_GetClientInput_(_dafny.SeqOfString("us-west-2")))
	_9_valueOrError3 = _out3
	if !(!((_9_valueOrError3).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(74,25): " + (_9_valueOrError3).String())
	}
	var _10_kmsClient m_ComAmazonawsKmsTypes.IKMSClient
	_ = _10_kmsClient
	_10_kmsClient = m_ComAmazonawsKmsTypes.Companion_IKMSClient_.CastTo_((_9_valueOrError3).Extract())
	var _11_valueOrError4 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _11_valueOrError4
	var _out4 m_Wrappers.Result
	_ = _out4
	_out4 = (_6_mpl).CreateAwsKmsKeyring(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateAwsKmsKeyringInput_.Create_CreateAwsKmsKeyringInput_(_0_kmsKey, _10_kmsClient, m_Wrappers.Companion_Option_.Create_None_()))
	_11_valueOrError4 = _out4
	if !(!((_11_valueOrError4).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(76,26): " + (_11_valueOrError4).String())
	}
	var _12_kmsKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _12_kmsKeyring
	_12_kmsKeyring = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_11_valueOrError4).Extract())
	var _13_encryptionContext _dafny.Map
	_ = _13_encryptionContext
	var _out5 _dafny.Map
	_ = _out5
	_out5 = m_Fixtures.Companion_Default___.SmallEncryptionContext(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_A_())
	_13_encryptionContext = _out5
	var _14_incorrectReproducedEncryptionContext _dafny.Map
	_ = _14_incorrectReproducedEncryptionContext
	var _out6 _dafny.Map
	_ = _out6
	_out6 = m_Fixtures.Companion_Default___.SmallEncryptionContext(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_AB_())
	_14_incorrectReproducedEncryptionContext = _out6
	var _15_encryptOutput m_Wrappers.Result
	_ = _15_encryptOutput
	var _out7 m_Wrappers.Result
	_ = _out7
	_out7 = (_4_esdk).Encrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_EncryptInput_.Create_EncryptInput_(_1_asdf, m_Wrappers.Companion_Option_.Create_Some_(_13_encryptionContext), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_12_kmsKeyring), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_()))
	_15_encryptOutput = _out7
	if !((_15_encryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(96,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _16_esdkCiphertext _dafny.Sequence
	_ = _16_esdkCiphertext
	_16_esdkCiphertext = ((_15_encryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.EncryptOutput)).Dtor_ciphertext()
	var _17_decryptOutput m_Wrappers.Result
	_ = _17_decryptOutput
	var _out8 m_Wrappers.Result
	_ = _out8
	_out8 = (_4_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_16_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_12_kmsKeyring), m_Wrappers.Companion_Option_.Create_Some_(_14_incorrectReproducedEncryptionContext)))
	_17_decryptOutput = _out8
	if !((_17_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(107,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
}
func (_static *CompanionStruct_Default___) TestMismatchedEncryptionContextOnDecrypt() {
	var _0_asdf _dafny.Sequence
	_ = _0_asdf
	_0_asdf = _dafny.SeqOf(uint8(97), uint8(115), uint8(100), uint8(102))
	var _1_namespace _dafny.Sequence
	_ = _1_namespace
	var _2_name _dafny.Sequence
	_ = _2_name
	var _out0 _dafny.Sequence
	_ = _out0
	var _out1 _dafny.Sequence
	_ = _out1
	_out0, _out1 = m_Fixtures.Companion_Default___.NamespaceAndName(_dafny.Zero)
	_1_namespace = _out0
	_2_name = _out1
	var _3_defaultConfig m_AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig
	_ = _3_defaultConfig
	_3_defaultConfig = m_EncryptionSdk.Companion_Default___.DefaultAwsEncryptionSdkConfig()
	var _4_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _4_valueOrError0
	var _out2 m_Wrappers.Result
	_ = _out2
	_out2 = m_EncryptionSdk.Companion_Default___.ESDK(_3_defaultConfig)
	_4_valueOrError0 = _out2
	if !(!((_4_valueOrError0).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(117,20): " + (_4_valueOrError0).String())
	}
	var _5_esdk *m_EncryptionSdk.ESDKClient
	_ = _5_esdk
	_5_esdk = (_4_valueOrError0).Extract().(*m_EncryptionSdk.ESDKClient)
	var _6_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _6_valueOrError1
	var _out3 m_Wrappers.Result
	_ = _out3
	_out3 = m_MaterialProviders.Companion_Default___.MaterialProviders(m_MaterialProviders.Companion_Default___.DefaultMaterialProvidersConfig())
	_6_valueOrError1 = _out3
	if !(!((_6_valueOrError1).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(118,19): " + (_6_valueOrError1).String())
	}
	var _7_mpl *m_MaterialProviders.MaterialProvidersClient
	_ = _7_mpl
	_7_mpl = (_6_valueOrError1).Extract().(*m_MaterialProviders.MaterialProvidersClient)
	var _8_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _8_valueOrError2
	var _out4 m_Wrappers.Result
	_ = _out4
	_out4 = (_7_mpl).CreateRawAesKeyring(m_AwsCryptographyMaterialProvidersTypes.Companion_CreateRawAesKeyringInput_.Create_CreateRawAesKeyringInput_(_1_namespace, _2_name, _dafny.SeqCreate(32, func(coer1 func(_dafny.Int) uint8) func(_dafny.Int) interface{} {
		return func(arg1 _dafny.Int) interface{} {
			return coer1(arg1)
		}
	}(func(_9_i _dafny.Int) uint8 {
		return uint8(0)
	})), m_AwsCryptographyMaterialProvidersTypes.Companion_AesWrappingAlg_.Create_ALG__AES256__GCM__IV12__TAG16_()))
	_8_valueOrError2 = _out4
	if !(!((_8_valueOrError2).IsFailure())) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(119,29): " + (_8_valueOrError2).String())
	}
	var _10_rawAESKeyring m_AwsCryptographyMaterialProvidersTypes.IKeyring
	_ = _10_rawAESKeyring
	_10_rawAESKeyring = m_AwsCryptographyMaterialProvidersTypes.Companion_IKeyring_.CastTo_((_8_valueOrError2).Extract())
	var _11_encryptionContext _dafny.Map
	_ = _11_encryptionContext
	var _out5 _dafny.Map
	_ = _out5
	_out5 = m_Fixtures.Companion_Default___.SmallEncryptionContext(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_A_())
	_11_encryptionContext = _out5
	var _12_mismatchedEncryptionContext _dafny.Map
	_ = _12_mismatchedEncryptionContext
	var _out6 _dafny.Map
	_ = _out6
	_out6 = m_Fixtures.Companion_Default___.SmallMismatchedEncryptionContex(m_Fixtures.Companion_SmallEncryptionContextVariation_.Create_A_())
	_12_mismatchedEncryptionContext = _out6
	var _13_encryptOutput m_Wrappers.Result
	_ = _13_encryptOutput
	var _out7 m_Wrappers.Result
	_ = _out7
	_out7 = (_5_esdk).Encrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_EncryptInput_.Create_EncryptInput_(_0_asdf, m_Wrappers.Companion_Option_.Create_Some_(_11_encryptionContext), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_10_rawAESKeyring), m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_None_()))
	_13_encryptOutput = _out7
	if !((_13_encryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(138,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _14_esdkCiphertext _dafny.Sequence
	_ = _14_esdkCiphertext
	_14_esdkCiphertext = ((_13_encryptOutput).Dtor_value().(m_AwsCryptographyEncryptionSdkTypes.EncryptOutput)).Dtor_ciphertext()
	var _15_decryptOutput m_Wrappers.Result
	_ = _15_decryptOutput
	var _out8 m_Wrappers.Result
	_ = _out8
	_out8 = (_5_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_14_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_10_rawAESKeyring), m_Wrappers.Companion_Option_.Create_Some_(_12_mismatchedEncryptionContext)))
	_15_decryptOutput = _out8
	if !((_15_decryptOutput).Is_Failure()) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(150,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out9 m_Wrappers.Result
	_ = _out9
	_out9 = (_5_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_14_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_10_rawAESKeyring), m_Wrappers.Companion_Option_.Create_Some_(_11_encryptionContext)))
	_15_decryptOutput = _out9
	if !((_15_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(160,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	var _out10 m_Wrappers.Result
	_ = _out10
	_out10 = (_5_esdk).Decrypt(m_AwsCryptographyEncryptionSdkTypes.Companion_DecryptInput_.Create_DecryptInput_(_14_esdkCiphertext, m_Wrappers.Companion_Option_.Create_None_(), m_Wrappers.Companion_Option_.Create_Some_(_10_rawAESKeyring), m_Wrappers.Companion_Option_.Create_None_()))
	_15_decryptOutput = _out10
	if !((_15_decryptOutput).Is_Success()) {
		panic("dafny/AwsEncryptionSdk/test/TestReproducedEncContext.dfy(171,8): " + (_dafny.SeqOfString("expectation violation")).String())
	}
}

// End of class Default__
