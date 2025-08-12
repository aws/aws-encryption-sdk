// Package MessageBody
// Dafny module MessageBody compiled into Go

package MessageBody

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
	m_EncryptedDataKeys "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/EncryptedDataKeys"
	m_EncryptionContext "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/EncryptionContext"
	m_Frames "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/Frames"
	m_Header "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/Header"
	m_HeaderAuth "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/HeaderAuth"
	m_HeaderTypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/HeaderTypes"
	m_SerializableTypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/SerializableTypes"
	m_SerializeFunctions "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/SerializeFunctions"
	m_SharedHeaderFunctions "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/SharedHeaderFunctions"
	m_V1HeaderBody "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/V1HeaderBody"
	m_V2HeaderBody "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/V2HeaderBody"
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
	return "MessageBody.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) BodyAADContentTypeString(bc BodyAADContent) _dafny.Sequence {
	var _source0 BodyAADContent = bc
	_ = _source0
	{
		if _source0.Is_AADRegularFrame() {
			return Companion_Default___.BODY__AAD__CONTENT__REGULAR__FRAME()
		}
	}
	{
		if _source0.Is_AADFinalFrame() {
			return Companion_Default___.BODY__AAD__CONTENT__FINAL__FRAME()
		}
	}
	{
		return Companion_Default___.BODY__AAD__CONTENT__SINGLE__BLOCK()
	}
}
func (_static *CompanionStruct_Default___) IVSeq(suite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo, sequenceNumber uint32) _dafny.Sequence {
	var _0_len uint8 = m_SerializableTypes.Companion_Default___.GetIvLength(suite)
	_ = _0_len
	var _1_num _dafny.Sequence = m_StandardLibrary_UInt.Companion_Default___.UInt32ToSeq(sequenceNumber)
	_ = _1_num
	if (_0_len) == (uint8(12)) {
		return _dafny.Companion_Sequence_.Concatenate(_dafny.SeqOf(uint8(0), uint8(0), uint8(0), uint8(0), uint8(0), uint8(0), uint8(0), uint8(0)), _1_num)
	} else {
		return _dafny.Companion_Sequence_.Concatenate(_dafny.SeqCreate(((_dafny.IntOfUint8(_0_len)).Minus(_dafny.IntOfInt64(4))).Uint32(), func(coer7 func(_dafny.Int) uint8) func(_dafny.Int) interface{} {
			return func(arg8 _dafny.Int) interface{} {
				return coer7(arg8)
			}
		}(func(_2___v0 _dafny.Int) uint8 {
			return uint8(0)
		})), _1_num)
	}
}
func (_static *CompanionStruct_Default___) EncryptMessageBody(plaintext _dafny.Sequence, header m_Header.HeaderInfo, key _dafny.Sequence, crypto *m_AtomicPrimitives.AtomicPrimitivesClient) m_Wrappers.Result {
	var result m_Wrappers.Result = m_Wrappers.Result{}
	_ = result
	var _0_n uint64
	_ = _0_n
	var _1_sequenceNumber uint32
	_ = _1_sequenceNumber
	var _rhs0 uint64 = uint64(0)
	_ = _rhs0
	var _rhs1 uint32 = Companion_Default___.START__SEQUENCE__NUMBER()
	_ = _rhs1
	_0_n = _rhs0
	_1_sequenceNumber = _rhs1
	var _2_regularFrames _dafny.Sequence
	_ = _2_regularFrames
	_2_regularFrames = _dafny.SeqOf()
	for (m_StandardLibrary_MemoryMath.Companion_Default___.Add(_0_n, uint64(((header).Dtor_body()).Dtor_frameLength()))) < (uint64((plaintext).Cardinality())) {
		var _3_valueOrError0 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
		_ = _3_valueOrError0
		_3_valueOrError0 = m_Wrappers.Companion_Default___.Need((_1_sequenceNumber) < (Companion_Default___.ENDFRAME__SEQUENCE__NUMBER()), m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.SeqOfString("too many frames")))
		if (_3_valueOrError0).IsFailure() {
			result = (_3_valueOrError0).PropagateFailure()
			return result
		}
		var _4_plaintextFrame _dafny.Sequence
		_ = _4_plaintextFrame
		_4_plaintextFrame = (plaintext).Subsequence(uint32(_0_n), uint32(m_StandardLibrary_MemoryMath.Companion_Default___.Add(_0_n, uint64(((header).Dtor_body()).Dtor_frameLength()))))
		var _5_valueOrError1 m_Wrappers.Result = m_Wrappers.Result{}
		_ = _5_valueOrError1
		var _out0 m_Wrappers.Result
		_ = _out0
		_out0 = Companion_Default___.EncryptRegularFrame(key, header, _4_plaintextFrame, _1_sequenceNumber, crypto)
		_5_valueOrError1 = _out0
		if (_5_valueOrError1).IsFailure() {
			result = (_5_valueOrError1).PropagateFailure()
			return result
		}
		var _6_regularFrame m_Frames.Frame
		_ = _6_regularFrame
		_6_regularFrame = (_5_valueOrError1).Extract().(m_Frames.Frame)
		_2_regularFrames = _dafny.Companion_Sequence_.Concatenate(_2_regularFrames, _dafny.SeqOf(_6_regularFrame))
		_0_n = m_StandardLibrary_MemoryMath.Companion_Default___.Add(_0_n, uint64(((header).Dtor_body()).Dtor_frameLength()))
		_1_sequenceNumber = (_1_sequenceNumber) + (uint32(1))
	}
	var _7_valueOrError2 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _7_valueOrError2
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = Companion_Default___.EncryptFinalFrame(key, header, (plaintext).Drop(uint32(_0_n)), _1_sequenceNumber, crypto)
	_7_valueOrError2 = _out1
	if (_7_valueOrError2).IsFailure() {
		result = (_7_valueOrError2).PropagateFailure()
		return result
	}
	var _8_finalFrame m_Frames.Frame
	_ = _8_finalFrame
	_8_finalFrame = (_7_valueOrError2).Extract().(m_Frames.Frame)
	result = m_Wrappers.Companion_Result_.Create_Success_(Companion_FramedMessageBody_.Create_FramedMessageBody_(_2_regularFrames, _8_finalFrame))
	return result
}
func (_static *CompanionStruct_Default___) EncryptRegularFrame(key _dafny.Sequence, header m_Header.HeaderInfo, plaintext _dafny.Sequence, sequenceNumber uint32, crypto *m_AtomicPrimitives.AtomicPrimitivesClient) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Result{}
	_ = res
	var _0_iv _dafny.Sequence
	_ = _0_iv
	_0_iv = Companion_Default___.IVSeq((header).Dtor_suite(), sequenceNumber)
	var _1_aad _dafny.Sequence
	_ = _1_aad
	_1_aad = Companion_Default___.BodyAAD(((header).Dtor_body()).Dtor_messageId(), Companion_BodyAADContent_.Create_AADRegularFrame_(), sequenceNumber, uint64((plaintext).Cardinality()))
	var _2_aesEncryptInput m_AwsCryptographyPrimitivesTypes.AESEncryptInput
	_ = _2_aesEncryptInput
	_2_aesEncryptInput = m_AwsCryptographyPrimitivesTypes.Companion_AESEncryptInput_.Create_AESEncryptInput_((((header).Dtor_suite()).Dtor_encrypt()).Dtor_AES__GCM(), _0_iv, key, plaintext, _1_aad)
	var _3_maybeEncryptionOutput m_Wrappers.Result
	_ = _3_maybeEncryptionOutput
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = (crypto).AESEncrypt(_2_aesEncryptInput)
	_3_maybeEncryptionOutput = _out0
	var _4_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_AwsCryptographyPrimitivesTypes.Companion_AESEncryptOutput_.Default())
	_ = _4_valueOrError0
	_4_valueOrError0 = (_3_maybeEncryptionOutput).MapFailure(func(coer8 func(m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg9 interface{}) interface{} {
			return coer8(arg9.(m_AwsCryptographyPrimitivesTypes.Error))
		}
	}(func(_5_e m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyPrimitives_(_5_e)
	}))
	if (_4_valueOrError0).IsFailure() {
		res = (_4_valueOrError0).PropagateFailure()
		return res
	}
	var _6_encryptionOutput m_AwsCryptographyPrimitivesTypes.AESEncryptOutput
	_ = _6_encryptionOutput
	_6_encryptionOutput = (_4_valueOrError0).Extract().(m_AwsCryptographyPrimitivesTypes.AESEncryptOutput)
	var _7_frame m_Frames.Frame
	_ = _7_frame
	_7_frame = m_Frames.Companion_Frame_.Create_RegularFrame_(header, sequenceNumber, _0_iv, (_6_encryptionOutput).Dtor_cipherText(), (_6_encryptionOutput).Dtor_authTag())
	res = m_Wrappers.Companion_Result_.Create_Success_(_7_frame)
	return res
	return res
}
func (_static *CompanionStruct_Default___) EncryptFinalFrame(key _dafny.Sequence, header m_Header.HeaderInfo, plaintext _dafny.Sequence, sequenceNumber uint32, crypto *m_AtomicPrimitives.AtomicPrimitivesClient) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Result{}
	_ = res
	var _0_iv _dafny.Sequence
	_ = _0_iv
	_0_iv = Companion_Default___.IVSeq((header).Dtor_suite(), sequenceNumber)
	var _1_aad _dafny.Sequence
	_ = _1_aad
	_1_aad = Companion_Default___.BodyAAD(((header).Dtor_body()).Dtor_messageId(), Companion_BodyAADContent_.Create_AADFinalFrame_(), sequenceNumber, uint64((plaintext).Cardinality()))
	var _2_aesEncryptInput m_AwsCryptographyPrimitivesTypes.AESEncryptInput
	_ = _2_aesEncryptInput
	_2_aesEncryptInput = m_AwsCryptographyPrimitivesTypes.Companion_AESEncryptInput_.Create_AESEncryptInput_((((header).Dtor_suite()).Dtor_encrypt()).Dtor_AES__GCM(), _0_iv, key, plaintext, _1_aad)
	var _3_maybeEncryptionOutput m_Wrappers.Result
	_ = _3_maybeEncryptionOutput
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = (crypto).AESEncrypt(_2_aesEncryptInput)
	_3_maybeEncryptionOutput = _out0
	var _4_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_AwsCryptographyPrimitivesTypes.Companion_AESEncryptOutput_.Default())
	_ = _4_valueOrError0
	_4_valueOrError0 = (_3_maybeEncryptionOutput).MapFailure(func(coer9 func(m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg10 interface{}) interface{} {
			return coer9(arg10.(m_AwsCryptographyPrimitivesTypes.Error))
		}
	}(func(_5_e m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyPrimitives_(_5_e)
	}))
	if (_4_valueOrError0).IsFailure() {
		res = (_4_valueOrError0).PropagateFailure()
		return res
	}
	var _6_encryptionOutput m_AwsCryptographyPrimitivesTypes.AESEncryptOutput
	_ = _6_encryptionOutput
	_6_encryptionOutput = (_4_valueOrError0).Extract().(m_AwsCryptographyPrimitivesTypes.AESEncryptOutput)
	var _7_finalFrame m_Frames.Frame
	_ = _7_finalFrame
	_7_finalFrame = m_Frames.Companion_Frame_.Create_FinalFrame_(header, sequenceNumber, _0_iv, (_6_encryptionOutput).Dtor_cipherText(), (_6_encryptionOutput).Dtor_authTag())
	res = m_Wrappers.Companion_Result_.Create_Success_(_7_finalFrame)
	return res
	return res
}
func (_static *CompanionStruct_Default___) DecryptFramedMessageBody(body FramedMessageBody, key _dafny.Sequence, crypto *m_AtomicPrimitives.AtomicPrimitivesClient) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = res
	var _0_plaintext _dafny.Sequence
	_ = _0_plaintext
	_0_plaintext = _dafny.SeqOf()
	var _hi0 uint64 = uint64(((body).Dtor_regularFrames()).Cardinality())
	_ = _hi0
	for _1_i := uint64(0); _1_i < _hi0; _1_i++ {
		var _2_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
		_ = _2_valueOrError0
		var _out0 m_Wrappers.Result
		_ = _out0
		_out0 = Companion_Default___.DecryptFrame(((body).Dtor_regularFrames()).Select(uint32(_1_i)).(m_Frames.Frame), key, crypto)
		_2_valueOrError0 = _out0
		if (_2_valueOrError0).IsFailure() {
			res = (_2_valueOrError0).PropagateFailure()
			return res
		}
		var _3_plaintextSegment _dafny.Sequence
		_ = _3_plaintextSegment
		_3_plaintextSegment = (_2_valueOrError0).Extract().(_dafny.Sequence)
		_0_plaintext = _dafny.Companion_Sequence_.Concatenate(_0_plaintext, _3_plaintextSegment)
	}
	var _4_valueOrError1 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _4_valueOrError1
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = Companion_Default___.DecryptFrame((body).Dtor_finalFrame(), key, crypto)
	_4_valueOrError1 = _out1
	if (_4_valueOrError1).IsFailure() {
		res = (_4_valueOrError1).PropagateFailure()
		return res
	}
	var _5_finalPlaintextSegment _dafny.Sequence
	_ = _5_finalPlaintextSegment
	_5_finalPlaintextSegment = (_4_valueOrError1).Extract().(_dafny.Sequence)
	_0_plaintext = _dafny.Companion_Sequence_.Concatenate(_0_plaintext, _5_finalPlaintextSegment)
	res = m_Wrappers.Companion_Result_.Create_Success_(_0_plaintext)
	return res
}
func (_static *CompanionStruct_Default___) DecryptFrame(frame m_Frames.Frame, key _dafny.Sequence, crypto *m_AtomicPrimitives.AtomicPrimitivesClient) m_Wrappers.Result {
	var res m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = res
	var _0_aad _dafny.Sequence
	_ = _0_aad
	_0_aad = Companion_Default___.BodyAADByFrameType(frame)
	var _1_maybePlaintextSegment m_Wrappers.Result
	_ = _1_maybePlaintextSegment
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = (crypto).AESDecrypt(m_AwsCryptographyPrimitivesTypes.Companion_AESDecryptInput_.Create_AESDecryptInput_(((((frame).Dtor_header()).Dtor_suite()).Dtor_encrypt()).Dtor_AES__GCM(), key, (frame).Dtor_encContent(), (frame).Dtor_authTag(), (frame).Dtor_iv(), _0_aad))
	_1_maybePlaintextSegment = _out0
	var _2_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _2_valueOrError0
	_2_valueOrError0 = (_1_maybePlaintextSegment).MapFailure(func(coer10 func(m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error) func(interface{}) interface{} {
		return func(arg11 interface{}) interface{} {
			return coer10(arg11.(m_AwsCryptographyPrimitivesTypes.Error))
		}
	}(func(_3_e m_AwsCryptographyPrimitivesTypes.Error) m_AwsCryptographyEncryptionSdkTypes.Error {
		return m_AwsCryptographyEncryptionSdkTypes.Companion_Error_.Create_AwsCryptographyPrimitives_(_3_e)
	}))
	if (_2_valueOrError0).IsFailure() {
		res = (_2_valueOrError0).PropagateFailure()
		return res
	}
	var _4_plaintextSegment _dafny.Sequence
	_ = _4_plaintextSegment
	_4_plaintextSegment = (_2_valueOrError0).Extract().(_dafny.Sequence)
	res = m_Wrappers.Companion_Result_.Create_Success_(_4_plaintextSegment)
	return res
	return res
}
func (_static *CompanionStruct_Default___) BodyAADByFrameType(frame m_Frames.Frame) _dafny.Sequence {
	var _let_tmp_rhs0 _dafny.Tuple = func() _dafny.Tuple {
		var _source0 m_Frames.Frame = frame
		_ = _source0
		{
			if _source0.Is_RegularFrame() {
				var _0_header m_Header.HeaderInfo = _source0.Get_().(m_Frames.Frame_RegularFrame).Header
				_ = _0_header
				var _1_seqNum uint32 = _source0.Get_().(m_Frames.Frame_RegularFrame).SeqNum
				_ = _1_seqNum
				return _dafny.TupleOf(_1_seqNum, Companion_BodyAADContent_.Create_AADRegularFrame_(), uint64(((_0_header).Dtor_body()).Dtor_frameLength()))
			}
		}
		{
			if _source0.Is_FinalFrame() {
				var _2_seqNum uint32 = _source0.Get_().(m_Frames.Frame_FinalFrame).SeqNum
				_ = _2_seqNum
				var _3_encContent _dafny.Sequence = _source0.Get_().(m_Frames.Frame_FinalFrame).EncContent
				_ = _3_encContent
				return _dafny.TupleOf(_2_seqNum, Companion_BodyAADContent_.Create_AADFinalFrame_(), uint64((_3_encContent).Cardinality()))
			}
		}
		{
			var _4_encContent _dafny.Sequence = _source0.Get_().(m_Frames.Frame_NonFramed).EncContent
			_ = _4_encContent
			return _dafny.TupleOf(Companion_Default___.NONFRAMED__SEQUENCE__NUMBER(), Companion_BodyAADContent_.Create_AADSingleBlock_(), uint64((_4_encContent).Cardinality()))
		}
	}()
	_ = _let_tmp_rhs0
	var _5_sequenceNumber uint32 = (*(_let_tmp_rhs0).IndexInt(0)).(uint32)
	_ = _5_sequenceNumber
	var _6_bc BodyAADContent = (*(_let_tmp_rhs0).IndexInt(1)).(BodyAADContent)
	_ = _6_bc
	var _7_length uint64 = (*(_let_tmp_rhs0).IndexInt(2)).(uint64)
	_ = _7_length
	return Companion_Default___.BodyAAD((((frame).Dtor_header()).Dtor_body()).Dtor_messageId(), _6_bc, _5_sequenceNumber, _7_length)
}
func (_static *CompanionStruct_Default___) BodyAAD(messageID _dafny.Sequence, bc BodyAADContent, sequenceNumber uint32, length uint64) _dafny.Sequence {
	var _0_contentAAD m_Wrappers.Result = m_UTF8.Encode(Companion_Default___.BodyAADContentTypeString(bc))
	_ = _0_contentAAD
	return _dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate(messageID, (_0_contentAAD).Dtor_value().(_dafny.Sequence)), m_StandardLibrary_UInt.Companion_Default___.UInt32ToSeq(sequenceNumber)), m_StandardLibrary_UInt.Companion_Default___.UInt64ToSeq(length))
}
func (_static *CompanionStruct_Default___) WriteFramedMessageBody(body FramedMessageBody) _dafny.Sequence {
	return _dafny.Companion_Sequence_.Concatenate(Companion_Default___.WriteMessageRegularFrames((body).Dtor_regularFrames()), m_Frames.Companion_Default___.WriteFinalFrame((body).Dtor_finalFrame()))
}
func (_static *CompanionStruct_Default___) WriteMessageRegularFrames(frames _dafny.Sequence) _dafny.Sequence {
	var ret _dafny.Sequence = _dafny.EmptySeq
	_ = ret
	var _0_result _dafny.Sequence
	_ = _0_result
	_0_result = _dafny.SeqOf()
	var _hi0 uint64 = uint64((frames).Cardinality())
	_ = _hi0
	for _1_i := uint64(0); _1_i < _hi0; _1_i++ {
		_0_result = _dafny.Companion_Sequence_.Concatenate(_0_result, m_Frames.Companion_Default___.WriteRegularFrame((frames).Select(uint32(_1_i)).(m_Frames.Frame)))
	}
	ret = _0_result
	return ret
	return ret
}
func (_static *CompanionStruct_Default___) ReadFramedMessageBody(buffer m_SerializeFunctions.ReadableBuffer, header m_Header.HeaderInfo, regularFrames _dafny.Sequence, continuation m_SerializeFunctions.ReadableBuffer) m_Wrappers.Result {
	goto TAIL_CALL_START
TAIL_CALL_START:
	var _0_valueOrError0 m_Wrappers.Result = m_SerializeFunctions.Companion_Default___.ReadUInt32(continuation)
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _1_sequenceNumber m_SerializeFunctions.SuccessfulRead = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
		_ = _1_sequenceNumber
		if ((_1_sequenceNumber).Dtor_data().(uint32)) != (Companion_Default___.ENDFRAME__SEQUENCE__NUMBER()) /* dircomp */ {
			var _2_valueOrError1 m_Wrappers.Result = m_Frames.Companion_Default___.ReadRegularFrame(continuation, header)
			_ = _2_valueOrError1
			if (_2_valueOrError1).IsFailure() {
				return (_2_valueOrError1).PropagateFailure()
			} else {
				var _3_regularFrame m_SerializeFunctions.SuccessfulRead = (_2_valueOrError1).Extract().(m_SerializeFunctions.SuccessfulRead)
				_ = _3_regularFrame
				var _4_valueOrError2 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((uint64(((_3_regularFrame).Dtor_data().(m_Frames.Frame)).Dtor_seqNum())) == ((uint64((regularFrames).Cardinality()))+(uint64(1))), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Sequence number out of order.")))
				_ = _4_valueOrError2
				if (_4_valueOrError2).IsFailure() {
					return (_4_valueOrError2).PropagateFailure()
				} else {
					var _5_nextRegularFrames _dafny.Sequence = _dafny.Companion_Sequence_.Concatenate(regularFrames, _dafny.SeqOf((_3_regularFrame).Dtor_data().(m_Frames.Frame)))
					_ = _5_nextRegularFrames
					var _in0 m_SerializeFunctions.ReadableBuffer = buffer
					_ = _in0
					var _in1 m_Header.HeaderInfo = header
					_ = _in1
					var _in2 _dafny.Sequence = _5_nextRegularFrames
					_ = _in2
					var _in3 m_SerializeFunctions.ReadableBuffer = (_3_regularFrame).Dtor_tail()
					_ = _in3
					buffer = _in0
					header = _in1
					regularFrames = _in2
					continuation = _in3
					goto TAIL_CALL_START
				}
			}
		} else {
			var _6_valueOrError3 m_Wrappers.Result = m_Frames.Companion_Default___.ReadFinalFrame(continuation, header)
			_ = _6_valueOrError3
			if (_6_valueOrError3).IsFailure() {
				return (_6_valueOrError3).PropagateFailure()
			} else {
				var _7_finalFrame m_SerializeFunctions.SuccessfulRead = (_6_valueOrError3).Extract().(m_SerializeFunctions.SuccessfulRead)
				_ = _7_finalFrame
				var _8_valueOrError4 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((uint64(((_7_finalFrame).Dtor_data().(m_Frames.Frame)).Dtor_seqNum())) == ((uint64((regularFrames).Cardinality()))+(uint64(1))), m_SerializeFunctions.Companion_ReadProblems_.Create_Error_(_dafny.SeqOfString("Sequence number out of order.")))
				_ = _8_valueOrError4
				if (_8_valueOrError4).IsFailure() {
					return (_8_valueOrError4).PropagateFailure()
				} else {
					var _9_body FramedMessageBody = Companion_FramedMessageBody_.Create_FramedMessageBody_(regularFrames, (_7_finalFrame).Dtor_data().(m_Frames.Frame))
					_ = _9_body
					return m_Wrappers.Companion_Result_.Create_Success_(m_SerializeFunctions.Companion_SuccessfulRead_.Create_SuccessfulRead_(_9_body, (_7_finalFrame).Dtor_tail()))
				}
			}
		}
	}
}
func (_static *CompanionStruct_Default___) ReadNonFramedMessageBody(buffer m_SerializeFunctions.ReadableBuffer, header m_Header.HeaderInfo) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Result = m_Frames.Companion_Default___.ReadNonFrame(buffer, header)
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		var _1_block m_SerializeFunctions.SuccessfulRead = (_0_valueOrError0).Extract().(m_SerializeFunctions.SuccessfulRead)
		_ = _1_block
		return m_Wrappers.Companion_Result_.Create_Success_(m_SerializeFunctions.Companion_SuccessfulRead_.Create_SuccessfulRead_((_1_block).Dtor_data().(m_Frames.Frame), (_1_block).Dtor_tail()))
	}
}
func (_static *CompanionStruct_Default___) BODY__AAD__CONTENT__REGULAR__FRAME() _dafny.Sequence {
	return _dafny.SeqOfString("AWSKMSEncryptionClient Frame")
}
func (_static *CompanionStruct_Default___) BODY__AAD__CONTENT__FINAL__FRAME() _dafny.Sequence {
	return _dafny.SeqOfString("AWSKMSEncryptionClient Final Frame")
}
func (_static *CompanionStruct_Default___) BODY__AAD__CONTENT__SINGLE__BLOCK() _dafny.Sequence {
	return _dafny.SeqOfString("AWSKMSEncryptionClient Single Block")
}
func (_static *CompanionStruct_Default___) ENDFRAME__SEQUENCE__NUMBER() uint32 {
	return m_Frames.Companion_Default___.ENDFRAME__SEQUENCE__NUMBER()
}
func (_static *CompanionStruct_Default___) START__SEQUENCE__NUMBER() uint32 {
	return m_Frames.Companion_Default___.START__SEQUENCE__NUMBER()
}
func (_static *CompanionStruct_Default___) NONFRAMED__SEQUENCE__NUMBER() uint32 {
	return m_Frames.Companion_Default___.NONFRAMED__SEQUENCE__NUMBER()
}

// End of class Default__

// Definition of datatype BodyAADContent
type BodyAADContent struct {
	Data_BodyAADContent_
}

func (_this BodyAADContent) Get_() Data_BodyAADContent_ {
	return _this.Data_BodyAADContent_
}

type Data_BodyAADContent_ interface {
	isBodyAADContent()
}

type CompanionStruct_BodyAADContent_ struct {
}

var Companion_BodyAADContent_ = CompanionStruct_BodyAADContent_{}

type BodyAADContent_AADRegularFrame struct {
}

func (BodyAADContent_AADRegularFrame) isBodyAADContent() {}

func (CompanionStruct_BodyAADContent_) Create_AADRegularFrame_() BodyAADContent {
	return BodyAADContent{BodyAADContent_AADRegularFrame{}}
}

func (_this BodyAADContent) Is_AADRegularFrame() bool {
	_, ok := _this.Get_().(BodyAADContent_AADRegularFrame)
	return ok
}

type BodyAADContent_AADFinalFrame struct {
}

func (BodyAADContent_AADFinalFrame) isBodyAADContent() {}

func (CompanionStruct_BodyAADContent_) Create_AADFinalFrame_() BodyAADContent {
	return BodyAADContent{BodyAADContent_AADFinalFrame{}}
}

func (_this BodyAADContent) Is_AADFinalFrame() bool {
	_, ok := _this.Get_().(BodyAADContent_AADFinalFrame)
	return ok
}

type BodyAADContent_AADSingleBlock struct {
}

func (BodyAADContent_AADSingleBlock) isBodyAADContent() {}

func (CompanionStruct_BodyAADContent_) Create_AADSingleBlock_() BodyAADContent {
	return BodyAADContent{BodyAADContent_AADSingleBlock{}}
}

func (_this BodyAADContent) Is_AADSingleBlock() bool {
	_, ok := _this.Get_().(BodyAADContent_AADSingleBlock)
	return ok
}

func (CompanionStruct_BodyAADContent_) Default() BodyAADContent {
	return Companion_BodyAADContent_.Create_AADRegularFrame_()
}

func (_ CompanionStruct_BodyAADContent_) AllSingletonConstructors() _dafny.Iterator {
	i := -1
	return func() (interface{}, bool) {
		i++
		switch i {
		case 0:
			return Companion_BodyAADContent_.Create_AADRegularFrame_(), true
		case 1:
			return Companion_BodyAADContent_.Create_AADFinalFrame_(), true
		case 2:
			return Companion_BodyAADContent_.Create_AADSingleBlock_(), true
		default:
			return BodyAADContent{}, false
		}
	}
}

func (_this BodyAADContent) String() string {
	switch _this.Get_().(type) {
	case nil:
		return "null"
	case BodyAADContent_AADRegularFrame:
		{
			return "MessageBody.BodyAADContent.AADRegularFrame"
		}
	case BodyAADContent_AADFinalFrame:
		{
			return "MessageBody.BodyAADContent.AADFinalFrame"
		}
	case BodyAADContent_AADSingleBlock:
		{
			return "MessageBody.BodyAADContent.AADSingleBlock"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this BodyAADContent) Equals(other BodyAADContent) bool {
	switch _this.Get_().(type) {
	case BodyAADContent_AADRegularFrame:
		{
			_, ok := other.Get_().(BodyAADContent_AADRegularFrame)
			return ok
		}
	case BodyAADContent_AADFinalFrame:
		{
			_, ok := other.Get_().(BodyAADContent_AADFinalFrame)
			return ok
		}
	case BodyAADContent_AADSingleBlock:
		{
			_, ok := other.Get_().(BodyAADContent_AADSingleBlock)
			return ok
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this BodyAADContent) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(BodyAADContent)
	return ok && _this.Equals(typed)
}

func Type_BodyAADContent_() _dafny.TypeDescriptor {
	return type_BodyAADContent_{}
}

type type_BodyAADContent_ struct {
}

func (_this type_BodyAADContent_) Default() interface{} {
	return Companion_BodyAADContent_.Default()
}

func (_this type_BodyAADContent_) String() string {
	return "MessageBody.BodyAADContent"
}
func (_this BodyAADContent) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = BodyAADContent{}

// End of datatype BodyAADContent

// Definition of class MessageRegularFrames
type MessageRegularFrames struct {
}

func New_MessageRegularFrames_() *MessageRegularFrames {
	_this := MessageRegularFrames{}

	return &_this
}

type CompanionStruct_MessageRegularFrames_ struct {
}

var Companion_MessageRegularFrames_ = CompanionStruct_MessageRegularFrames_{}

func (*MessageRegularFrames) String() string {
	return "MessageBody.MessageRegularFrames"
}

// End of class MessageRegularFrames

func Type_MessageRegularFrames_() _dafny.TypeDescriptor {
	return type_MessageRegularFrames_{}
}

type type_MessageRegularFrames_ struct {
}

func (_this type_MessageRegularFrames_) Default() interface{} {
	return _dafny.EmptySeq
}

func (_this type_MessageRegularFrames_) String() string {
	return "MessageBody.MessageRegularFrames"
}

// Definition of datatype FramedMessageBody
type FramedMessageBody struct {
	Data_FramedMessageBody_
}

func (_this FramedMessageBody) Get_() Data_FramedMessageBody_ {
	return _this.Data_FramedMessageBody_
}

type Data_FramedMessageBody_ interface {
	isFramedMessageBody()
}

type CompanionStruct_FramedMessageBody_ struct {
}

var Companion_FramedMessageBody_ = CompanionStruct_FramedMessageBody_{}

type FramedMessageBody_FramedMessageBody struct {
	RegularFrames _dafny.Sequence
	FinalFrame    m_Frames.Frame
}

func (FramedMessageBody_FramedMessageBody) isFramedMessageBody() {}

func (CompanionStruct_FramedMessageBody_) Create_FramedMessageBody_(RegularFrames _dafny.Sequence, FinalFrame m_Frames.Frame) FramedMessageBody {
	return FramedMessageBody{FramedMessageBody_FramedMessageBody{RegularFrames, FinalFrame}}
}

func (_this FramedMessageBody) Is_FramedMessageBody() bool {
	_, ok := _this.Get_().(FramedMessageBody_FramedMessageBody)
	return ok
}

func (CompanionStruct_FramedMessageBody_) Default() FramedMessageBody {
	return Companion_FramedMessageBody_.Create_FramedMessageBody_(_dafny.EmptySeq, m_Frames.Companion_Frame_.Default())
}

func (_this FramedMessageBody) Dtor_regularFrames() _dafny.Sequence {
	return _this.Get_().(FramedMessageBody_FramedMessageBody).RegularFrames
}

func (_this FramedMessageBody) Dtor_finalFrame() m_Frames.Frame {
	return _this.Get_().(FramedMessageBody_FramedMessageBody).FinalFrame
}

func (_this FramedMessageBody) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case FramedMessageBody_FramedMessageBody:
		{
			return "MessageBody.FramedMessageBody.FramedMessageBody" + "(" + _dafny.String(data.RegularFrames) + ", " + _dafny.String(data.FinalFrame) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this FramedMessageBody) Equals(other FramedMessageBody) bool {
	switch data1 := _this.Get_().(type) {
	case FramedMessageBody_FramedMessageBody:
		{
			data2, ok := other.Get_().(FramedMessageBody_FramedMessageBody)
			return ok && data1.RegularFrames.Equals(data2.RegularFrames) && data1.FinalFrame.Equals(data2.FinalFrame)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this FramedMessageBody) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(FramedMessageBody)
	return ok && _this.Equals(typed)
}

func Type_FramedMessageBody_() _dafny.TypeDescriptor {
	return type_FramedMessageBody_{}
}

type type_FramedMessageBody_ struct {
}

func (_this type_FramedMessageBody_) Default() interface{} {
	return Companion_FramedMessageBody_.Default()
}

func (_this type_FramedMessageBody_) String() string {
	return "MessageBody.FramedMessageBody"
}
func (_this FramedMessageBody) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = FramedMessageBody{}

// End of datatype FramedMessageBody

// Definition of class FramedMessage
type FramedMessage struct {
}

func New_FramedMessage_() *FramedMessage {
	_this := FramedMessage{}

	return &_this
}

type CompanionStruct_FramedMessage_ struct {
}

var Companion_FramedMessage_ = CompanionStruct_FramedMessage_{}

func (*FramedMessage) String() string {
	return "MessageBody.FramedMessage"
}

// End of class FramedMessage

func Type_FramedMessage_() _dafny.TypeDescriptor {
	return type_FramedMessage_{}
}

type type_FramedMessage_ struct {
}

func (_this type_FramedMessage_) Default() interface{} {
	return Companion_FramedMessageBody_.Default()
}

func (_this type_FramedMessage_) String() string {
	return "MessageBody.FramedMessage"
}

// Definition of class MessageFrame
type MessageFrame struct {
}

func New_MessageFrame_() *MessageFrame {
	_this := MessageFrame{}

	return &_this
}

type CompanionStruct_MessageFrame_ struct {
}

var Companion_MessageFrame_ = CompanionStruct_MessageFrame_{}

func (*MessageFrame) String() string {
	return "MessageBody.MessageFrame"
}

// End of class MessageFrame

func Type_MessageFrame_() _dafny.TypeDescriptor {
	return type_MessageFrame_{}
}

type type_MessageFrame_ struct {
}

func (_this type_MessageFrame_) Default() interface{} {
	return m_Frames.Companion_Frame_.Default()
}

func (_this type_MessageFrame_) String() string {
	return "MessageBody.MessageFrame"
}

// Definition of class Frame
type Frame struct {
}

func New_Frame_() *Frame {
	_this := Frame{}

	return &_this
}

type CompanionStruct_Frame_ struct {
}

var Companion_Frame_ = CompanionStruct_Frame_{}

func (*Frame) String() string {
	return "MessageBody.Frame"
}

// End of class Frame

func Type_Frame_() _dafny.TypeDescriptor {
	return type_Frame_{}
}

type type_Frame_ struct {
}

func (_this type_Frame_) Default() interface{} {
	return m_Frames.Companion_Frame_.Default()
}

func (_this type_Frame_) String() string {
	return "MessageBody.Frame"
}
