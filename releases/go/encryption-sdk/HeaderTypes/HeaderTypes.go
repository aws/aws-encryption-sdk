// Package HeaderTypes
// Dafny module HeaderTypes compiled into Go

package HeaderTypes

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
	m_UnicodeStrings "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/UnicodeStrings"
	m__Unicode "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Unicode_"
	m_Utf16EncodingForm "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Utf16EncodingForm"
	m_Utf8EncodingForm "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Utf8EncodingForm"
	m_Wrappers "github.com/aws/aws-cryptographic-material-providers-library/releases/go/smithy-dafny-standard-library/Wrappers"
	m_AwsCryptographyEncryptionSdkTypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/AwsCryptographyEncryptionSdkTypes"
	m_EncryptionContext "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/EncryptionContext"
	m_SerializableTypes "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/SerializableTypes"
	m_SerializeFunctions "github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk/SerializeFunctions"
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
	return "HeaderTypes.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) MESSAGE__ID__LEN__V1() uint64 {
	return uint64(16)
}
func (_static *CompanionStruct_Default___) MESSAGE__ID__LEN__V2() uint64 {
	return uint64(32)
}

// End of class Default__

// Definition of datatype MessageFormatVersion
type MessageFormatVersion struct {
	Data_MessageFormatVersion_
}

func (_this MessageFormatVersion) Get_() Data_MessageFormatVersion_ {
	return _this.Data_MessageFormatVersion_
}

type Data_MessageFormatVersion_ interface {
	isMessageFormatVersion()
}

type CompanionStruct_MessageFormatVersion_ struct {
}

var Companion_MessageFormatVersion_ = CompanionStruct_MessageFormatVersion_{}

type MessageFormatVersion_V1 struct {
}

func (MessageFormatVersion_V1) isMessageFormatVersion() {}

func (CompanionStruct_MessageFormatVersion_) Create_V1_() MessageFormatVersion {
	return MessageFormatVersion{MessageFormatVersion_V1{}}
}

func (_this MessageFormatVersion) Is_V1() bool {
	_, ok := _this.Get_().(MessageFormatVersion_V1)
	return ok
}

type MessageFormatVersion_V2 struct {
}

func (MessageFormatVersion_V2) isMessageFormatVersion() {}

func (CompanionStruct_MessageFormatVersion_) Create_V2_() MessageFormatVersion {
	return MessageFormatVersion{MessageFormatVersion_V2{}}
}

func (_this MessageFormatVersion) Is_V2() bool {
	_, ok := _this.Get_().(MessageFormatVersion_V2)
	return ok
}

func (CompanionStruct_MessageFormatVersion_) Default() MessageFormatVersion {
	return Companion_MessageFormatVersion_.Create_V1_()
}

func (_ CompanionStruct_MessageFormatVersion_) AllSingletonConstructors() _dafny.Iterator {
	i := -1
	return func() (interface{}, bool) {
		i++
		switch i {
		case 0:
			return Companion_MessageFormatVersion_.Create_V1_(), true
		case 1:
			return Companion_MessageFormatVersion_.Create_V2_(), true
		default:
			return MessageFormatVersion{}, false
		}
	}
}

func (_this MessageFormatVersion) String() string {
	switch _this.Get_().(type) {
	case nil:
		return "null"
	case MessageFormatVersion_V1:
		{
			return "HeaderTypes.MessageFormatVersion.V1"
		}
	case MessageFormatVersion_V2:
		{
			return "HeaderTypes.MessageFormatVersion.V2"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this MessageFormatVersion) Equals(other MessageFormatVersion) bool {
	switch _this.Get_().(type) {
	case MessageFormatVersion_V1:
		{
			_, ok := other.Get_().(MessageFormatVersion_V1)
			return ok
		}
	case MessageFormatVersion_V2:
		{
			_, ok := other.Get_().(MessageFormatVersion_V2)
			return ok
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this MessageFormatVersion) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(MessageFormatVersion)
	return ok && _this.Equals(typed)
}

func Type_MessageFormatVersion_() _dafny.TypeDescriptor {
	return type_MessageFormatVersion_{}
}

type type_MessageFormatVersion_ struct {
}

func (_this type_MessageFormatVersion_) Default() interface{} {
	return Companion_MessageFormatVersion_.Default()
}

func (_this type_MessageFormatVersion_) String() string {
	return "HeaderTypes.MessageFormatVersion"
}
func (_this MessageFormatVersion) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = MessageFormatVersion{}

func (_this MessageFormatVersion) Serialize() _dafny.Sequence {
	{
		var _source0 MessageFormatVersion = _this
		_ = _source0
		{
			if _source0.Is_V1() {
				return _dafny.SeqOf(uint8(1))
			}
		}
		{
			return _dafny.SeqOf(uint8(2))
		}
	}
}
func (_static CompanionStruct_MessageFormatVersion_) Get(x _dafny.Sequence) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((_dafny.Companion_Sequence_.Equal(x, _dafny.SeqOf(uint8(1)))) || (_dafny.Companion_Sequence_.Equal(x, _dafny.SeqOf(uint8(2)))), _dafny.SeqOfString("Unsupported Version value."))
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		return m_Wrappers.Companion_Result_.Create_Success_(func() MessageFormatVersion {
			var _source0 uint8 = (x).Select(uint32(uint32(0))).(uint8)
			_ = _source0
			{
				if (_source0) == (uint8(1)) {
					return Companion_MessageFormatVersion_.Create_V1_()
				}
			}
			{
				return Companion_MessageFormatVersion_.Create_V2_()
			}
		}())
	}
}

// End of datatype MessageFormatVersion

// Definition of class ESDKAlgorithmSuite
type ESDKAlgorithmSuite struct {
}

func New_ESDKAlgorithmSuite_() *ESDKAlgorithmSuite {
	_this := ESDKAlgorithmSuite{}

	return &_this
}

type CompanionStruct_ESDKAlgorithmSuite_ struct {
}

var Companion_ESDKAlgorithmSuite_ = CompanionStruct_ESDKAlgorithmSuite_{}

func (*ESDKAlgorithmSuite) String() string {
	return "HeaderTypes.ESDKAlgorithmSuite"
}

// End of class ESDKAlgorithmSuite

func Type_ESDKAlgorithmSuite_() _dafny.TypeDescriptor {
	return type_ESDKAlgorithmSuite_{}
}

type type_ESDKAlgorithmSuite_ struct {
}

func (_this type_ESDKAlgorithmSuite_) Default() interface{} {
	return m_AwsCryptographyMaterialProvidersTypes.Companion_AlgorithmSuiteInfo_.Default()
}

func (_this type_ESDKAlgorithmSuite_) String() string {
	return "HeaderTypes.ESDKAlgorithmSuite"
}

// Definition of datatype HeaderBody
type HeaderBody struct {
	Data_HeaderBody_
}

func (_this HeaderBody) Get_() Data_HeaderBody_ {
	return _this.Data_HeaderBody_
}

type Data_HeaderBody_ interface {
	isHeaderBody()
}

type CompanionStruct_HeaderBody_ struct {
}

var Companion_HeaderBody_ = CompanionStruct_HeaderBody_{}

type HeaderBody_V1HeaderBody struct {
	MessageType       MessageType
	AlgorithmSuite    m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo
	MessageId         _dafny.Sequence
	EncryptionContext _dafny.Sequence
	EncryptedDataKeys _dafny.Sequence
	ContentType       ContentType
	HeaderIvLength    uint64
	FrameLength       uint32
}

func (HeaderBody_V1HeaderBody) isHeaderBody() {}

func (CompanionStruct_HeaderBody_) Create_V1HeaderBody_(MessageType MessageType, AlgorithmSuite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo, MessageId _dafny.Sequence, EncryptionContext _dafny.Sequence, EncryptedDataKeys _dafny.Sequence, ContentType ContentType, HeaderIvLength uint64, FrameLength uint32) HeaderBody {
	return HeaderBody{HeaderBody_V1HeaderBody{MessageType, AlgorithmSuite, MessageId, EncryptionContext, EncryptedDataKeys, ContentType, HeaderIvLength, FrameLength}}
}

func (_this HeaderBody) Is_V1HeaderBody() bool {
	_, ok := _this.Get_().(HeaderBody_V1HeaderBody)
	return ok
}

type HeaderBody_V2HeaderBody struct {
	AlgorithmSuite    m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo
	MessageId         _dafny.Sequence
	EncryptionContext _dafny.Sequence
	EncryptedDataKeys _dafny.Sequence
	ContentType       ContentType
	FrameLength       uint32
	SuiteData         _dafny.Sequence
}

func (HeaderBody_V2HeaderBody) isHeaderBody() {}

func (CompanionStruct_HeaderBody_) Create_V2HeaderBody_(AlgorithmSuite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo, MessageId _dafny.Sequence, EncryptionContext _dafny.Sequence, EncryptedDataKeys _dafny.Sequence, ContentType ContentType, FrameLength uint32, SuiteData _dafny.Sequence) HeaderBody {
	return HeaderBody{HeaderBody_V2HeaderBody{AlgorithmSuite, MessageId, EncryptionContext, EncryptedDataKeys, ContentType, FrameLength, SuiteData}}
}

func (_this HeaderBody) Is_V2HeaderBody() bool {
	_, ok := _this.Get_().(HeaderBody_V2HeaderBody)
	return ok
}

func (CompanionStruct_HeaderBody_) Default() HeaderBody {
	return Companion_HeaderBody_.Create_V1HeaderBody_(Companion_MessageType_.Default(), m_AwsCryptographyMaterialProvidersTypes.Companion_AlgorithmSuiteInfo_.Default(), _dafny.EmptySeq, _dafny.EmptySeq, _dafny.EmptySeq, Companion_ContentType_.Default(), uint64(0), uint32(0))
}

func (_this HeaderBody) Dtor_messageType() MessageType {
	return _this.Get_().(HeaderBody_V1HeaderBody).MessageType
}

func (_this HeaderBody) Dtor_algorithmSuite() m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo {
	switch data := _this.Get_().(type) {
	case HeaderBody_V1HeaderBody:
		return data.AlgorithmSuite
	default:
		return data.(HeaderBody_V2HeaderBody).AlgorithmSuite
	}
}

func (_this HeaderBody) Dtor_messageId() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case HeaderBody_V1HeaderBody:
		return data.MessageId
	default:
		return data.(HeaderBody_V2HeaderBody).MessageId
	}
}

func (_this HeaderBody) Dtor_encryptionContext() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case HeaderBody_V1HeaderBody:
		return data.EncryptionContext
	default:
		return data.(HeaderBody_V2HeaderBody).EncryptionContext
	}
}

func (_this HeaderBody) Dtor_encryptedDataKeys() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case HeaderBody_V1HeaderBody:
		return data.EncryptedDataKeys
	default:
		return data.(HeaderBody_V2HeaderBody).EncryptedDataKeys
	}
}

func (_this HeaderBody) Dtor_contentType() ContentType {
	switch data := _this.Get_().(type) {
	case HeaderBody_V1HeaderBody:
		return data.ContentType
	default:
		return data.(HeaderBody_V2HeaderBody).ContentType
	}
}

func (_this HeaderBody) Dtor_headerIvLength() uint64 {
	return _this.Get_().(HeaderBody_V1HeaderBody).HeaderIvLength
}

func (_this HeaderBody) Dtor_frameLength() uint32 {
	switch data := _this.Get_().(type) {
	case HeaderBody_V1HeaderBody:
		return data.FrameLength
	default:
		return data.(HeaderBody_V2HeaderBody).FrameLength
	}
}

func (_this HeaderBody) Dtor_suiteData() _dafny.Sequence {
	return _this.Get_().(HeaderBody_V2HeaderBody).SuiteData
}

func (_this HeaderBody) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case HeaderBody_V1HeaderBody:
		{
			return "HeaderTypes.HeaderBody.V1HeaderBody" + "(" + _dafny.String(data.MessageType) + ", " + _dafny.String(data.AlgorithmSuite) + ", " + _dafny.String(data.MessageId) + ", " + _dafny.String(data.EncryptionContext) + ", " + _dafny.String(data.EncryptedDataKeys) + ", " + _dafny.String(data.ContentType) + ", " + _dafny.String(data.HeaderIvLength) + ", " + _dafny.String(data.FrameLength) + ")"
		}
	case HeaderBody_V2HeaderBody:
		{
			return "HeaderTypes.HeaderBody.V2HeaderBody" + "(" + _dafny.String(data.AlgorithmSuite) + ", " + _dafny.String(data.MessageId) + ", " + _dafny.String(data.EncryptionContext) + ", " + _dafny.String(data.EncryptedDataKeys) + ", " + _dafny.String(data.ContentType) + ", " + _dafny.String(data.FrameLength) + ", " + _dafny.String(data.SuiteData) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this HeaderBody) Equals(other HeaderBody) bool {
	switch data1 := _this.Get_().(type) {
	case HeaderBody_V1HeaderBody:
		{
			data2, ok := other.Get_().(HeaderBody_V1HeaderBody)
			return ok && data1.MessageType.Equals(data2.MessageType) && data1.AlgorithmSuite.Equals(data2.AlgorithmSuite) && data1.MessageId.Equals(data2.MessageId) && data1.EncryptionContext.Equals(data2.EncryptionContext) && data1.EncryptedDataKeys.Equals(data2.EncryptedDataKeys) && data1.ContentType.Equals(data2.ContentType) && data1.HeaderIvLength == data2.HeaderIvLength && data1.FrameLength == data2.FrameLength
		}
	case HeaderBody_V2HeaderBody:
		{
			data2, ok := other.Get_().(HeaderBody_V2HeaderBody)
			return ok && data1.AlgorithmSuite.Equals(data2.AlgorithmSuite) && data1.MessageId.Equals(data2.MessageId) && data1.EncryptionContext.Equals(data2.EncryptionContext) && data1.EncryptedDataKeys.Equals(data2.EncryptedDataKeys) && data1.ContentType.Equals(data2.ContentType) && data1.FrameLength == data2.FrameLength && data1.SuiteData.Equals(data2.SuiteData)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this HeaderBody) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(HeaderBody)
	return ok && _this.Equals(typed)
}

func Type_HeaderBody_() _dafny.TypeDescriptor {
	return type_HeaderBody_{}
}

type type_HeaderBody_ struct {
}

func (_this type_HeaderBody_) Default() interface{} {
	return Companion_HeaderBody_.Default()
}

func (_this type_HeaderBody_) String() string {
	return "HeaderTypes.HeaderBody"
}
func (_this HeaderBody) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = HeaderBody{}

// End of datatype HeaderBody

// Definition of datatype HeaderAuth
type HeaderAuth struct {
	Data_HeaderAuth_
}

func (_this HeaderAuth) Get_() Data_HeaderAuth_ {
	return _this.Data_HeaderAuth_
}

type Data_HeaderAuth_ interface {
	isHeaderAuth()
}

type CompanionStruct_HeaderAuth_ struct {
}

var Companion_HeaderAuth_ = CompanionStruct_HeaderAuth_{}

type HeaderAuth_AESMac struct {
	HeaderIv      _dafny.Sequence
	HeaderAuthTag _dafny.Sequence
}

func (HeaderAuth_AESMac) isHeaderAuth() {}

func (CompanionStruct_HeaderAuth_) Create_AESMac_(HeaderIv _dafny.Sequence, HeaderAuthTag _dafny.Sequence) HeaderAuth {
	return HeaderAuth{HeaderAuth_AESMac{HeaderIv, HeaderAuthTag}}
}

func (_this HeaderAuth) Is_AESMac() bool {
	_, ok := _this.Get_().(HeaderAuth_AESMac)
	return ok
}

func (CompanionStruct_HeaderAuth_) Default() HeaderAuth {
	return Companion_HeaderAuth_.Create_AESMac_(_dafny.EmptySeq, _dafny.EmptySeq)
}

func (_this HeaderAuth) Dtor_headerIv() _dafny.Sequence {
	return _this.Get_().(HeaderAuth_AESMac).HeaderIv
}

func (_this HeaderAuth) Dtor_headerAuthTag() _dafny.Sequence {
	return _this.Get_().(HeaderAuth_AESMac).HeaderAuthTag
}

func (_this HeaderAuth) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case HeaderAuth_AESMac:
		{
			return "HeaderTypes.HeaderAuth.AESMac" + "(" + _dafny.String(data.HeaderIv) + ", " + _dafny.String(data.HeaderAuthTag) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this HeaderAuth) Equals(other HeaderAuth) bool {
	switch data1 := _this.Get_().(type) {
	case HeaderAuth_AESMac:
		{
			data2, ok := other.Get_().(HeaderAuth_AESMac)
			return ok && data1.HeaderIv.Equals(data2.HeaderIv) && data1.HeaderAuthTag.Equals(data2.HeaderAuthTag)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this HeaderAuth) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(HeaderAuth)
	return ok && _this.Equals(typed)
}

func Type_HeaderAuth_() _dafny.TypeDescriptor {
	return type_HeaderAuth_{}
}

type type_HeaderAuth_ struct {
}

func (_this type_HeaderAuth_) Default() interface{} {
	return Companion_HeaderAuth_.Default()
}

func (_this type_HeaderAuth_) String() string {
	return "HeaderTypes.HeaderAuth"
}
func (_this HeaderAuth) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = HeaderAuth{}

// End of datatype HeaderAuth

// Definition of datatype MessageType
type MessageType struct {
	Data_MessageType_
}

func (_this MessageType) Get_() Data_MessageType_ {
	return _this.Data_MessageType_
}

type Data_MessageType_ interface {
	isMessageType()
}

type CompanionStruct_MessageType_ struct {
}

var Companion_MessageType_ = CompanionStruct_MessageType_{}

type MessageType_TYPE__CUSTOMER__AED struct {
}

func (MessageType_TYPE__CUSTOMER__AED) isMessageType() {}

func (CompanionStruct_MessageType_) Create_TYPE__CUSTOMER__AED_() MessageType {
	return MessageType{MessageType_TYPE__CUSTOMER__AED{}}
}

func (_this MessageType) Is_TYPE__CUSTOMER__AED() bool {
	_, ok := _this.Get_().(MessageType_TYPE__CUSTOMER__AED)
	return ok
}

func (CompanionStruct_MessageType_) Default() MessageType {
	return Companion_MessageType_.Create_TYPE__CUSTOMER__AED_()
}

func (_ CompanionStruct_MessageType_) AllSingletonConstructors() _dafny.Iterator {
	i := -1
	return func() (interface{}, bool) {
		i++
		switch i {
		case 0:
			return Companion_MessageType_.Create_TYPE__CUSTOMER__AED_(), true
		default:
			return MessageType{}, false
		}
	}
}

func (_this MessageType) String() string {
	switch _this.Get_().(type) {
	case nil:
		return "null"
	case MessageType_TYPE__CUSTOMER__AED:
		{
			return "HeaderTypes.MessageType.TYPE_CUSTOMER_AED"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this MessageType) Equals(other MessageType) bool {
	switch _this.Get_().(type) {
	case MessageType_TYPE__CUSTOMER__AED:
		{
			_, ok := other.Get_().(MessageType_TYPE__CUSTOMER__AED)
			return ok
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this MessageType) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(MessageType)
	return ok && _this.Equals(typed)
}

func Type_MessageType_() _dafny.TypeDescriptor {
	return type_MessageType_{}
}

type type_MessageType_ struct {
}

func (_this type_MessageType_) Default() interface{} {
	return Companion_MessageType_.Default()
}

func (_this type_MessageType_) String() string {
	return "HeaderTypes.MessageType"
}
func (_this MessageType) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = MessageType{}

func (_this MessageType) Serialize() uint8 {
	{
		var _source0 MessageType = _this
		_ = _source0
		{
			return uint8(128)
		}
	}
}
func (_static CompanionStruct_MessageType_) Get(x uint8) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need((x) == (uint8(128)), _dafny.SeqOfString("Unsupported ContentType value."))
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		return m_Wrappers.Companion_Result_.Create_Success_(func() MessageType {
			var _source0 uint8 = x
			_ = _source0
			{
				return Companion_MessageType_.Create_TYPE__CUSTOMER__AED_()
			}
		}())
	}
}

// End of datatype MessageType

// Definition of datatype ContentType
type ContentType struct {
	Data_ContentType_
}

func (_this ContentType) Get_() Data_ContentType_ {
	return _this.Data_ContentType_
}

type Data_ContentType_ interface {
	isContentType()
}

type CompanionStruct_ContentType_ struct {
}

var Companion_ContentType_ = CompanionStruct_ContentType_{}

type ContentType_NonFramed struct {
}

func (ContentType_NonFramed) isContentType() {}

func (CompanionStruct_ContentType_) Create_NonFramed_() ContentType {
	return ContentType{ContentType_NonFramed{}}
}

func (_this ContentType) Is_NonFramed() bool {
	_, ok := _this.Get_().(ContentType_NonFramed)
	return ok
}

type ContentType_Framed struct {
}

func (ContentType_Framed) isContentType() {}

func (CompanionStruct_ContentType_) Create_Framed_() ContentType {
	return ContentType{ContentType_Framed{}}
}

func (_this ContentType) Is_Framed() bool {
	_, ok := _this.Get_().(ContentType_Framed)
	return ok
}

func (CompanionStruct_ContentType_) Default() ContentType {
	return Companion_ContentType_.Create_NonFramed_()
}

func (_ CompanionStruct_ContentType_) AllSingletonConstructors() _dafny.Iterator {
	i := -1
	return func() (interface{}, bool) {
		i++
		switch i {
		case 0:
			return Companion_ContentType_.Create_NonFramed_(), true
		case 1:
			return Companion_ContentType_.Create_Framed_(), true
		default:
			return ContentType{}, false
		}
	}
}

func (_this ContentType) String() string {
	switch _this.Get_().(type) {
	case nil:
		return "null"
	case ContentType_NonFramed:
		{
			return "HeaderTypes.ContentType.NonFramed"
		}
	case ContentType_Framed:
		{
			return "HeaderTypes.ContentType.Framed"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this ContentType) Equals(other ContentType) bool {
	switch _this.Get_().(type) {
	case ContentType_NonFramed:
		{
			_, ok := other.Get_().(ContentType_NonFramed)
			return ok
		}
	case ContentType_Framed:
		{
			_, ok := other.Get_().(ContentType_Framed)
			return ok
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this ContentType) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(ContentType)
	return ok && _this.Equals(typed)
}

func Type_ContentType_() _dafny.TypeDescriptor {
	return type_ContentType_{}
}

type type_ContentType_ struct {
}

func (_this type_ContentType_) Default() interface{} {
	return Companion_ContentType_.Default()
}

func (_this type_ContentType_) String() string {
	return "HeaderTypes.ContentType"
}
func (_this ContentType) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = ContentType{}

func (_this ContentType) Serialize() uint8 {
	{
		var _source0 ContentType = _this
		_ = _source0
		{
			if _source0.Is_NonFramed() {
				return uint8(1)
			}
		}
		{
			return uint8(2)
		}
	}
}
func (_static CompanionStruct_ContentType_) Get(x uint8) m_Wrappers.Result {
	var _0_valueOrError0 m_Wrappers.Outcome = m_Wrappers.Companion_Default___.Need(((x) == (uint8(1))) || ((x) == (uint8(2))), _dafny.SeqOfString("Unsupported ContentType value."))
	_ = _0_valueOrError0
	if (_0_valueOrError0).IsFailure() {
		return (_0_valueOrError0).PropagateFailure()
	} else {
		return m_Wrappers.Companion_Result_.Create_Success_(func() ContentType {
			var _source0 uint8 = x
			_ = _source0
			{
				if (_source0) == (uint8(1)) {
					return Companion_ContentType_.Create_NonFramed_()
				}
			}
			{
				return Companion_ContentType_.Create_Framed_()
			}
		}())
	}
}

// End of datatype ContentType

// Definition of class MessageId
type MessageId struct {
}

func New_MessageId_() *MessageId {
	_this := MessageId{}

	return &_this
}

type CompanionStruct_MessageId_ struct {
}

var Companion_MessageId_ = CompanionStruct_MessageId_{}

func (*MessageId) String() string {
	return "HeaderTypes.MessageId"
}

// End of class MessageId

func Type_MessageId_() _dafny.TypeDescriptor {
	return type_MessageId_{}
}

type type_MessageId_ struct {
}

func (_this type_MessageId_) Default() interface{} {
	return _dafny.EmptySeq
}

func (_this type_MessageId_) String() string {
	return "HeaderTypes.MessageId"
}
func (_this *CompanionStruct_MessageId_) Is_(__source _dafny.Sequence) bool {
	var _1_x _dafny.Sequence = (__source)
	_ = _1_x
	return ((_dafny.IntOfUint32((_1_x).Cardinality())).Cmp(_dafny.IntOfUint64(Companion_Default___.MESSAGE__ID__LEN__V1())) == 0) || ((_dafny.IntOfUint32((_1_x).Cardinality())).Cmp(_dafny.IntOfUint64(Companion_Default___.MESSAGE__ID__LEN__V2())) == 0)
}
