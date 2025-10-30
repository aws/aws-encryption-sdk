// Package AwsCryptographyEncryptionSdkTypes
// Dafny module AwsCryptographyEncryptionSdkTypes compiled into Go

package AwsCryptographyEncryptionSdkTypes

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
	return "AwsCryptographyEncryptionSdkTypes.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) IsValid__CountingNumbers(x int64) bool {
	return (int64(1)) <= (x)
}
func (_static *CompanionStruct_Default___) IsValid__FrameLength(x int64) bool {
	return ((int64(1)) <= (x)) && ((x) <= (int64(4294967296)))
}
func (_static *CompanionStruct_Default___) IsDummySubsetType(x _dafny.Int) bool {
	return (x).Sign() == 1
}

// End of class Default__

// Definition of datatype DafnyCallEvent
type DafnyCallEvent struct {
	Data_DafnyCallEvent_
}

func (_this DafnyCallEvent) Get_() Data_DafnyCallEvent_ {
	return _this.Data_DafnyCallEvent_
}

type Data_DafnyCallEvent_ interface {
	isDafnyCallEvent()
}

type CompanionStruct_DafnyCallEvent_ struct {
}

var Companion_DafnyCallEvent_ = CompanionStruct_DafnyCallEvent_{}

type DafnyCallEvent_DafnyCallEvent struct {
	Input  interface{}
	Output interface{}
}

func (DafnyCallEvent_DafnyCallEvent) isDafnyCallEvent() {}

func (CompanionStruct_DafnyCallEvent_) Create_DafnyCallEvent_(Input interface{}, Output interface{}) DafnyCallEvent {
	return DafnyCallEvent{DafnyCallEvent_DafnyCallEvent{Input, Output}}
}

func (_this DafnyCallEvent) Is_DafnyCallEvent() bool {
	_, ok := _this.Get_().(DafnyCallEvent_DafnyCallEvent)
	return ok
}

func (CompanionStruct_DafnyCallEvent_) Default(_default_I interface{}, _default_O interface{}) DafnyCallEvent {
	return Companion_DafnyCallEvent_.Create_DafnyCallEvent_(_default_I, _default_O)
}

func (_this DafnyCallEvent) Dtor_input() interface{} {
	return _this.Get_().(DafnyCallEvent_DafnyCallEvent).Input
}

func (_this DafnyCallEvent) Dtor_output() interface{} {
	return _this.Get_().(DafnyCallEvent_DafnyCallEvent).Output
}

func (_this DafnyCallEvent) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case DafnyCallEvent_DafnyCallEvent:
		{
			return "AwsCryptographyEncryptionSdkTypes.DafnyCallEvent.DafnyCallEvent" + "(" + _dafny.String(data.Input) + ", " + _dafny.String(data.Output) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this DafnyCallEvent) Equals(other DafnyCallEvent) bool {
	switch data1 := _this.Get_().(type) {
	case DafnyCallEvent_DafnyCallEvent:
		{
			data2, ok := other.Get_().(DafnyCallEvent_DafnyCallEvent)
			return ok && _dafny.AreEqual(data1.Input, data2.Input) && _dafny.AreEqual(data1.Output, data2.Output)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this DafnyCallEvent) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(DafnyCallEvent)
	return ok && _this.Equals(typed)
}

func Type_DafnyCallEvent_(Type_I_ _dafny.TypeDescriptor, Type_O_ _dafny.TypeDescriptor) _dafny.TypeDescriptor {
	return type_DafnyCallEvent_{Type_I_, Type_O_}
}

type type_DafnyCallEvent_ struct {
	Type_I_ _dafny.TypeDescriptor
	Type_O_ _dafny.TypeDescriptor
}

func (_this type_DafnyCallEvent_) Default() interface{} {
	Type_I_ := _this.Type_I_
	_ = Type_I_
	Type_O_ := _this.Type_O_
	_ = Type_O_
	return Companion_DafnyCallEvent_.Default(Type_I_.Default(), Type_O_.Default())
}

func (_this type_DafnyCallEvent_) String() string {
	return "AwsCryptographyEncryptionSdkTypes.DafnyCallEvent"
}
func (_this DafnyCallEvent) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = DafnyCallEvent{}

// End of datatype DafnyCallEvent

// Definition of class IAwsEncryptionSdkClientCallHistory
type IAwsEncryptionSdkClientCallHistory struct {
	dummy byte
}

func New_IAwsEncryptionSdkClientCallHistory_() *IAwsEncryptionSdkClientCallHistory {
	_this := IAwsEncryptionSdkClientCallHistory{}

	return &_this
}

type CompanionStruct_IAwsEncryptionSdkClientCallHistory_ struct {
}

var Companion_IAwsEncryptionSdkClientCallHistory_ = CompanionStruct_IAwsEncryptionSdkClientCallHistory_{}

func (_this *IAwsEncryptionSdkClientCallHistory) Equals(other *IAwsEncryptionSdkClientCallHistory) bool {
	return _this == other
}

func (_this *IAwsEncryptionSdkClientCallHistory) EqualsGeneric(x interface{}) bool {
	other, ok := x.(*IAwsEncryptionSdkClientCallHistory)
	return ok && _this.Equals(other)
}

func (*IAwsEncryptionSdkClientCallHistory) String() string {
	return "AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClientCallHistory"
}

func Type_IAwsEncryptionSdkClientCallHistory_() _dafny.TypeDescriptor {
	return type_IAwsEncryptionSdkClientCallHistory_{}
}

type type_IAwsEncryptionSdkClientCallHistory_ struct {
}

func (_this type_IAwsEncryptionSdkClientCallHistory_) Default() interface{} {
	return (*IAwsEncryptionSdkClientCallHistory)(nil)
}

func (_this type_IAwsEncryptionSdkClientCallHistory_) String() string {
	return "AwsCryptographyEncryptionSdkTypes.IAwsEncryptionSdkClientCallHistory"
}
func (_this *IAwsEncryptionSdkClientCallHistory) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &IAwsEncryptionSdkClientCallHistory{}

// End of class IAwsEncryptionSdkClientCallHistory

// Definition of trait IAwsEncryptionSdkClient
type IAwsEncryptionSdkClient interface {
	String() string
	Encrypt(input EncryptInput) m_Wrappers.Result
	Decrypt(input DecryptInput) m_Wrappers.Result
}
type CompanionStruct_IAwsEncryptionSdkClient_ struct {
	TraitID_ *_dafny.TraitID
}

var Companion_IAwsEncryptionSdkClient_ = CompanionStruct_IAwsEncryptionSdkClient_{
	TraitID_: &_dafny.TraitID{},
}

func (CompanionStruct_IAwsEncryptionSdkClient_) CastTo_(x interface{}) IAwsEncryptionSdkClient {
	var t IAwsEncryptionSdkClient
	t, _ = x.(IAwsEncryptionSdkClient)
	return t
}

// End of trait IAwsEncryptionSdkClient

// Definition of datatype AwsEncryptionSdkConfig
type AwsEncryptionSdkConfig struct {
	Data_AwsEncryptionSdkConfig_
}

func (_this AwsEncryptionSdkConfig) Get_() Data_AwsEncryptionSdkConfig_ {
	return _this.Data_AwsEncryptionSdkConfig_
}

type Data_AwsEncryptionSdkConfig_ interface {
	isAwsEncryptionSdkConfig()
}

type CompanionStruct_AwsEncryptionSdkConfig_ struct {
}

var Companion_AwsEncryptionSdkConfig_ = CompanionStruct_AwsEncryptionSdkConfig_{}

type AwsEncryptionSdkConfig_AwsEncryptionSdkConfig struct {
	CommitmentPolicy         m_Wrappers.Option
	MaxEncryptedDataKeys     m_Wrappers.Option
	NetV4__0__0__RetryPolicy m_Wrappers.Option
}

func (AwsEncryptionSdkConfig_AwsEncryptionSdkConfig) isAwsEncryptionSdkConfig() {}

func (CompanionStruct_AwsEncryptionSdkConfig_) Create_AwsEncryptionSdkConfig_(CommitmentPolicy m_Wrappers.Option, MaxEncryptedDataKeys m_Wrappers.Option, NetV4__0__0__RetryPolicy m_Wrappers.Option) AwsEncryptionSdkConfig {
	return AwsEncryptionSdkConfig{AwsEncryptionSdkConfig_AwsEncryptionSdkConfig{CommitmentPolicy, MaxEncryptedDataKeys, NetV4__0__0__RetryPolicy}}
}

func (_this AwsEncryptionSdkConfig) Is_AwsEncryptionSdkConfig() bool {
	_, ok := _this.Get_().(AwsEncryptionSdkConfig_AwsEncryptionSdkConfig)
	return ok
}

func (CompanionStruct_AwsEncryptionSdkConfig_) Default() AwsEncryptionSdkConfig {
	return Companion_AwsEncryptionSdkConfig_.Create_AwsEncryptionSdkConfig_(m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default())
}

func (_this AwsEncryptionSdkConfig) Dtor_commitmentPolicy() m_Wrappers.Option {
	return _this.Get_().(AwsEncryptionSdkConfig_AwsEncryptionSdkConfig).CommitmentPolicy
}

func (_this AwsEncryptionSdkConfig) Dtor_maxEncryptedDataKeys() m_Wrappers.Option {
	return _this.Get_().(AwsEncryptionSdkConfig_AwsEncryptionSdkConfig).MaxEncryptedDataKeys
}

func (_this AwsEncryptionSdkConfig) Dtor_netV4__0__0__RetryPolicy() m_Wrappers.Option {
	return _this.Get_().(AwsEncryptionSdkConfig_AwsEncryptionSdkConfig).NetV4__0__0__RetryPolicy
}

func (_this AwsEncryptionSdkConfig) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case AwsEncryptionSdkConfig_AwsEncryptionSdkConfig:
		{
			return "AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig.AwsEncryptionSdkConfig" + "(" + _dafny.String(data.CommitmentPolicy) + ", " + _dafny.String(data.MaxEncryptedDataKeys) + ", " + _dafny.String(data.NetV4__0__0__RetryPolicy) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this AwsEncryptionSdkConfig) Equals(other AwsEncryptionSdkConfig) bool {
	switch data1 := _this.Get_().(type) {
	case AwsEncryptionSdkConfig_AwsEncryptionSdkConfig:
		{
			data2, ok := other.Get_().(AwsEncryptionSdkConfig_AwsEncryptionSdkConfig)
			return ok && data1.CommitmentPolicy.Equals(data2.CommitmentPolicy) && data1.MaxEncryptedDataKeys.Equals(data2.MaxEncryptedDataKeys) && data1.NetV4__0__0__RetryPolicy.Equals(data2.NetV4__0__0__RetryPolicy)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this AwsEncryptionSdkConfig) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(AwsEncryptionSdkConfig)
	return ok && _this.Equals(typed)
}

func Type_AwsEncryptionSdkConfig_() _dafny.TypeDescriptor {
	return type_AwsEncryptionSdkConfig_{}
}

type type_AwsEncryptionSdkConfig_ struct {
}

func (_this type_AwsEncryptionSdkConfig_) Default() interface{} {
	return Companion_AwsEncryptionSdkConfig_.Default()
}

func (_this type_AwsEncryptionSdkConfig_) String() string {
	return "AwsCryptographyEncryptionSdkTypes.AwsEncryptionSdkConfig"
}
func (_this AwsEncryptionSdkConfig) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = AwsEncryptionSdkConfig{}

// End of datatype AwsEncryptionSdkConfig

// Definition of class CountingNumbers
type CountingNumbers struct {
}

func New_CountingNumbers_() *CountingNumbers {
	_this := CountingNumbers{}

	return &_this
}

type CompanionStruct_CountingNumbers_ struct {
}

var Companion_CountingNumbers_ = CompanionStruct_CountingNumbers_{}

func (*CountingNumbers) String() string {
	return "AwsCryptographyEncryptionSdkTypes.CountingNumbers"
}

// End of class CountingNumbers

func Type_CountingNumbers_() _dafny.TypeDescriptor {
	return type_CountingNumbers_{}
}

type type_CountingNumbers_ struct {
}

func (_this type_CountingNumbers_) Default() interface{} {
	return int64(0)
}

func (_this type_CountingNumbers_) String() string {
	return "AwsCryptographyEncryptionSdkTypes.CountingNumbers"
}
func (_this *CompanionStruct_CountingNumbers_) Is_(__source int64) bool {
	var _0_x int64 = (__source)
	_ = _0_x
	if true {
		return Companion_Default___.IsValid__CountingNumbers(_0_x)
	}
	return false
}

// Definition of datatype DecryptInput
type DecryptInput struct {
	Data_DecryptInput_
}

func (_this DecryptInput) Get_() Data_DecryptInput_ {
	return _this.Data_DecryptInput_
}

type Data_DecryptInput_ interface {
	isDecryptInput()
}

type CompanionStruct_DecryptInput_ struct {
}

var Companion_DecryptInput_ = CompanionStruct_DecryptInput_{}

type DecryptInput_DecryptInput struct {
	Ciphertext        _dafny.Sequence
	MaterialsManager  m_Wrappers.Option
	Keyring           m_Wrappers.Option
	EncryptionContext m_Wrappers.Option
}

func (DecryptInput_DecryptInput) isDecryptInput() {}

func (CompanionStruct_DecryptInput_) Create_DecryptInput_(Ciphertext _dafny.Sequence, MaterialsManager m_Wrappers.Option, Keyring m_Wrappers.Option, EncryptionContext m_Wrappers.Option) DecryptInput {
	return DecryptInput{DecryptInput_DecryptInput{Ciphertext, MaterialsManager, Keyring, EncryptionContext}}
}

func (_this DecryptInput) Is_DecryptInput() bool {
	_, ok := _this.Get_().(DecryptInput_DecryptInput)
	return ok
}

func (CompanionStruct_DecryptInput_) Default() DecryptInput {
	return Companion_DecryptInput_.Create_DecryptInput_(_dafny.EmptySeq, m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default())
}

func (_this DecryptInput) Dtor_ciphertext() _dafny.Sequence {
	return _this.Get_().(DecryptInput_DecryptInput).Ciphertext
}

func (_this DecryptInput) Dtor_materialsManager() m_Wrappers.Option {
	return _this.Get_().(DecryptInput_DecryptInput).MaterialsManager
}

func (_this DecryptInput) Dtor_keyring() m_Wrappers.Option {
	return _this.Get_().(DecryptInput_DecryptInput).Keyring
}

func (_this DecryptInput) Dtor_encryptionContext() m_Wrappers.Option {
	return _this.Get_().(DecryptInput_DecryptInput).EncryptionContext
}

func (_this DecryptInput) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case DecryptInput_DecryptInput:
		{
			return "AwsCryptographyEncryptionSdkTypes.DecryptInput.DecryptInput" + "(" + _dafny.String(data.Ciphertext) + ", " + _dafny.String(data.MaterialsManager) + ", " + _dafny.String(data.Keyring) + ", " + _dafny.String(data.EncryptionContext) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this DecryptInput) Equals(other DecryptInput) bool {
	switch data1 := _this.Get_().(type) {
	case DecryptInput_DecryptInput:
		{
			data2, ok := other.Get_().(DecryptInput_DecryptInput)
			return ok && data1.Ciphertext.Equals(data2.Ciphertext) && data1.MaterialsManager.Equals(data2.MaterialsManager) && data1.Keyring.Equals(data2.Keyring) && data1.EncryptionContext.Equals(data2.EncryptionContext)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this DecryptInput) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(DecryptInput)
	return ok && _this.Equals(typed)
}

func Type_DecryptInput_() _dafny.TypeDescriptor {
	return type_DecryptInput_{}
}

type type_DecryptInput_ struct {
}

func (_this type_DecryptInput_) Default() interface{} {
	return Companion_DecryptInput_.Default()
}

func (_this type_DecryptInput_) String() string {
	return "AwsCryptographyEncryptionSdkTypes.DecryptInput"
}
func (_this DecryptInput) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = DecryptInput{}

// End of datatype DecryptInput

// Definition of datatype DecryptOutput
type DecryptOutput struct {
	Data_DecryptOutput_
}

func (_this DecryptOutput) Get_() Data_DecryptOutput_ {
	return _this.Data_DecryptOutput_
}

type Data_DecryptOutput_ interface {
	isDecryptOutput()
}

type CompanionStruct_DecryptOutput_ struct {
}

var Companion_DecryptOutput_ = CompanionStruct_DecryptOutput_{}

type DecryptOutput_DecryptOutput struct {
	Plaintext         _dafny.Sequence
	EncryptionContext _dafny.Map
	AlgorithmSuiteId  m_AwsCryptographyMaterialProvidersTypes.ESDKAlgorithmSuiteId
}

func (DecryptOutput_DecryptOutput) isDecryptOutput() {}

func (CompanionStruct_DecryptOutput_) Create_DecryptOutput_(Plaintext _dafny.Sequence, EncryptionContext _dafny.Map, AlgorithmSuiteId m_AwsCryptographyMaterialProvidersTypes.ESDKAlgorithmSuiteId) DecryptOutput {
	return DecryptOutput{DecryptOutput_DecryptOutput{Plaintext, EncryptionContext, AlgorithmSuiteId}}
}

func (_this DecryptOutput) Is_DecryptOutput() bool {
	_, ok := _this.Get_().(DecryptOutput_DecryptOutput)
	return ok
}

func (CompanionStruct_DecryptOutput_) Default() DecryptOutput {
	return Companion_DecryptOutput_.Create_DecryptOutput_(_dafny.EmptySeq, _dafny.EmptyMap, m_AwsCryptographyMaterialProvidersTypes.Companion_ESDKAlgorithmSuiteId_.Default())
}

func (_this DecryptOutput) Dtor_plaintext() _dafny.Sequence {
	return _this.Get_().(DecryptOutput_DecryptOutput).Plaintext
}

func (_this DecryptOutput) Dtor_encryptionContext() _dafny.Map {
	return _this.Get_().(DecryptOutput_DecryptOutput).EncryptionContext
}

func (_this DecryptOutput) Dtor_algorithmSuiteId() m_AwsCryptographyMaterialProvidersTypes.ESDKAlgorithmSuiteId {
	return _this.Get_().(DecryptOutput_DecryptOutput).AlgorithmSuiteId
}

func (_this DecryptOutput) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case DecryptOutput_DecryptOutput:
		{
			return "AwsCryptographyEncryptionSdkTypes.DecryptOutput.DecryptOutput" + "(" + _dafny.String(data.Plaintext) + ", " + _dafny.String(data.EncryptionContext) + ", " + _dafny.String(data.AlgorithmSuiteId) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this DecryptOutput) Equals(other DecryptOutput) bool {
	switch data1 := _this.Get_().(type) {
	case DecryptOutput_DecryptOutput:
		{
			data2, ok := other.Get_().(DecryptOutput_DecryptOutput)
			return ok && data1.Plaintext.Equals(data2.Plaintext) && data1.EncryptionContext.Equals(data2.EncryptionContext) && data1.AlgorithmSuiteId.Equals(data2.AlgorithmSuiteId)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this DecryptOutput) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(DecryptOutput)
	return ok && _this.Equals(typed)
}

func Type_DecryptOutput_() _dafny.TypeDescriptor {
	return type_DecryptOutput_{}
}

type type_DecryptOutput_ struct {
}

func (_this type_DecryptOutput_) Default() interface{} {
	return Companion_DecryptOutput_.Default()
}

func (_this type_DecryptOutput_) String() string {
	return "AwsCryptographyEncryptionSdkTypes.DecryptOutput"
}
func (_this DecryptOutput) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = DecryptOutput{}

// End of datatype DecryptOutput

// Definition of datatype EncryptInput
type EncryptInput struct {
	Data_EncryptInput_
}

func (_this EncryptInput) Get_() Data_EncryptInput_ {
	return _this.Data_EncryptInput_
}

type Data_EncryptInput_ interface {
	isEncryptInput()
}

type CompanionStruct_EncryptInput_ struct {
}

var Companion_EncryptInput_ = CompanionStruct_EncryptInput_{}

type EncryptInput_EncryptInput struct {
	Plaintext         _dafny.Sequence
	EncryptionContext m_Wrappers.Option
	MaterialsManager  m_Wrappers.Option
	Keyring           m_Wrappers.Option
	AlgorithmSuiteId  m_Wrappers.Option
	FrameLength       m_Wrappers.Option
}

func (EncryptInput_EncryptInput) isEncryptInput() {}

func (CompanionStruct_EncryptInput_) Create_EncryptInput_(Plaintext _dafny.Sequence, EncryptionContext m_Wrappers.Option, MaterialsManager m_Wrappers.Option, Keyring m_Wrappers.Option, AlgorithmSuiteId m_Wrappers.Option, FrameLength m_Wrappers.Option) EncryptInput {
	return EncryptInput{EncryptInput_EncryptInput{Plaintext, EncryptionContext, MaterialsManager, Keyring, AlgorithmSuiteId, FrameLength}}
}

func (_this EncryptInput) Is_EncryptInput() bool {
	_, ok := _this.Get_().(EncryptInput_EncryptInput)
	return ok
}

func (CompanionStruct_EncryptInput_) Default() EncryptInput {
	return Companion_EncryptInput_.Create_EncryptInput_(_dafny.EmptySeq, m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default(), m_Wrappers.Companion_Option_.Default())
}

func (_this EncryptInput) Dtor_plaintext() _dafny.Sequence {
	return _this.Get_().(EncryptInput_EncryptInput).Plaintext
}

func (_this EncryptInput) Dtor_encryptionContext() m_Wrappers.Option {
	return _this.Get_().(EncryptInput_EncryptInput).EncryptionContext
}

func (_this EncryptInput) Dtor_materialsManager() m_Wrappers.Option {
	return _this.Get_().(EncryptInput_EncryptInput).MaterialsManager
}

func (_this EncryptInput) Dtor_keyring() m_Wrappers.Option {
	return _this.Get_().(EncryptInput_EncryptInput).Keyring
}

func (_this EncryptInput) Dtor_algorithmSuiteId() m_Wrappers.Option {
	return _this.Get_().(EncryptInput_EncryptInput).AlgorithmSuiteId
}

func (_this EncryptInput) Dtor_frameLength() m_Wrappers.Option {
	return _this.Get_().(EncryptInput_EncryptInput).FrameLength
}

func (_this EncryptInput) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case EncryptInput_EncryptInput:
		{
			return "AwsCryptographyEncryptionSdkTypes.EncryptInput.EncryptInput" + "(" + _dafny.String(data.Plaintext) + ", " + _dafny.String(data.EncryptionContext) + ", " + _dafny.String(data.MaterialsManager) + ", " + _dafny.String(data.Keyring) + ", " + _dafny.String(data.AlgorithmSuiteId) + ", " + _dafny.String(data.FrameLength) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this EncryptInput) Equals(other EncryptInput) bool {
	switch data1 := _this.Get_().(type) {
	case EncryptInput_EncryptInput:
		{
			data2, ok := other.Get_().(EncryptInput_EncryptInput)
			return ok && data1.Plaintext.Equals(data2.Plaintext) && data1.EncryptionContext.Equals(data2.EncryptionContext) && data1.MaterialsManager.Equals(data2.MaterialsManager) && data1.Keyring.Equals(data2.Keyring) && data1.AlgorithmSuiteId.Equals(data2.AlgorithmSuiteId) && data1.FrameLength.Equals(data2.FrameLength)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this EncryptInput) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(EncryptInput)
	return ok && _this.Equals(typed)
}

func Type_EncryptInput_() _dafny.TypeDescriptor {
	return type_EncryptInput_{}
}

type type_EncryptInput_ struct {
}

func (_this type_EncryptInput_) Default() interface{} {
	return Companion_EncryptInput_.Default()
}

func (_this type_EncryptInput_) String() string {
	return "AwsCryptographyEncryptionSdkTypes.EncryptInput"
}
func (_this EncryptInput) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = EncryptInput{}

// End of datatype EncryptInput

// Definition of datatype EncryptOutput
type EncryptOutput struct {
	Data_EncryptOutput_
}

func (_this EncryptOutput) Get_() Data_EncryptOutput_ {
	return _this.Data_EncryptOutput_
}

type Data_EncryptOutput_ interface {
	isEncryptOutput()
}

type CompanionStruct_EncryptOutput_ struct {
}

var Companion_EncryptOutput_ = CompanionStruct_EncryptOutput_{}

type EncryptOutput_EncryptOutput struct {
	Ciphertext        _dafny.Sequence
	EncryptionContext _dafny.Map
	AlgorithmSuiteId  m_AwsCryptographyMaterialProvidersTypes.ESDKAlgorithmSuiteId
}

func (EncryptOutput_EncryptOutput) isEncryptOutput() {}

func (CompanionStruct_EncryptOutput_) Create_EncryptOutput_(Ciphertext _dafny.Sequence, EncryptionContext _dafny.Map, AlgorithmSuiteId m_AwsCryptographyMaterialProvidersTypes.ESDKAlgorithmSuiteId) EncryptOutput {
	return EncryptOutput{EncryptOutput_EncryptOutput{Ciphertext, EncryptionContext, AlgorithmSuiteId}}
}

func (_this EncryptOutput) Is_EncryptOutput() bool {
	_, ok := _this.Get_().(EncryptOutput_EncryptOutput)
	return ok
}

func (CompanionStruct_EncryptOutput_) Default() EncryptOutput {
	return Companion_EncryptOutput_.Create_EncryptOutput_(_dafny.EmptySeq, _dafny.EmptyMap, m_AwsCryptographyMaterialProvidersTypes.Companion_ESDKAlgorithmSuiteId_.Default())
}

func (_this EncryptOutput) Dtor_ciphertext() _dafny.Sequence {
	return _this.Get_().(EncryptOutput_EncryptOutput).Ciphertext
}

func (_this EncryptOutput) Dtor_encryptionContext() _dafny.Map {
	return _this.Get_().(EncryptOutput_EncryptOutput).EncryptionContext
}

func (_this EncryptOutput) Dtor_algorithmSuiteId() m_AwsCryptographyMaterialProvidersTypes.ESDKAlgorithmSuiteId {
	return _this.Get_().(EncryptOutput_EncryptOutput).AlgorithmSuiteId
}

func (_this EncryptOutput) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case EncryptOutput_EncryptOutput:
		{
			return "AwsCryptographyEncryptionSdkTypes.EncryptOutput.EncryptOutput" + "(" + _dafny.String(data.Ciphertext) + ", " + _dafny.String(data.EncryptionContext) + ", " + _dafny.String(data.AlgorithmSuiteId) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this EncryptOutput) Equals(other EncryptOutput) bool {
	switch data1 := _this.Get_().(type) {
	case EncryptOutput_EncryptOutput:
		{
			data2, ok := other.Get_().(EncryptOutput_EncryptOutput)
			return ok && data1.Ciphertext.Equals(data2.Ciphertext) && data1.EncryptionContext.Equals(data2.EncryptionContext) && data1.AlgorithmSuiteId.Equals(data2.AlgorithmSuiteId)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this EncryptOutput) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(EncryptOutput)
	return ok && _this.Equals(typed)
}

func Type_EncryptOutput_() _dafny.TypeDescriptor {
	return type_EncryptOutput_{}
}

type type_EncryptOutput_ struct {
}

func (_this type_EncryptOutput_) Default() interface{} {
	return Companion_EncryptOutput_.Default()
}

func (_this type_EncryptOutput_) String() string {
	return "AwsCryptographyEncryptionSdkTypes.EncryptOutput"
}
func (_this EncryptOutput) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = EncryptOutput{}

// End of datatype EncryptOutput

// Definition of class FrameLength
type FrameLength struct {
}

func New_FrameLength_() *FrameLength {
	_this := FrameLength{}

	return &_this
}

type CompanionStruct_FrameLength_ struct {
}

var Companion_FrameLength_ = CompanionStruct_FrameLength_{}

func (*FrameLength) String() string {
	return "AwsCryptographyEncryptionSdkTypes.FrameLength"
}

// End of class FrameLength

func Type_FrameLength_() _dafny.TypeDescriptor {
	return type_FrameLength_{}
}

type type_FrameLength_ struct {
}

func (_this type_FrameLength_) Default() interface{} {
	return int64(0)
}

func (_this type_FrameLength_) String() string {
	return "AwsCryptographyEncryptionSdkTypes.FrameLength"
}
func (_this *CompanionStruct_FrameLength_) Is_(__source int64) bool {
	var _1_x int64 = (__source)
	_ = _1_x
	if true {
		return Companion_Default___.IsValid__FrameLength(_1_x)
	}
	return false
}

// Definition of datatype NetV4__0__0__RetryPolicy
type NetV4__0__0__RetryPolicy struct {
	Data_NetV4__0__0__RetryPolicy_
}

func (_this NetV4__0__0__RetryPolicy) Get_() Data_NetV4__0__0__RetryPolicy_ {
	return _this.Data_NetV4__0__0__RetryPolicy_
}

type Data_NetV4__0__0__RetryPolicy_ interface {
	isNetV4__0__0__RetryPolicy()
}

type CompanionStruct_NetV4__0__0__RetryPolicy_ struct {
}

var Companion_NetV4__0__0__RetryPolicy_ = CompanionStruct_NetV4__0__0__RetryPolicy_{}

type NetV4__0__0__RetryPolicy_FORBID__RETRY struct {
}

func (NetV4__0__0__RetryPolicy_FORBID__RETRY) isNetV4__0__0__RetryPolicy() {}

func (CompanionStruct_NetV4__0__0__RetryPolicy_) Create_FORBID__RETRY_() NetV4__0__0__RetryPolicy {
	return NetV4__0__0__RetryPolicy{NetV4__0__0__RetryPolicy_FORBID__RETRY{}}
}

func (_this NetV4__0__0__RetryPolicy) Is_FORBID__RETRY() bool {
	_, ok := _this.Get_().(NetV4__0__0__RetryPolicy_FORBID__RETRY)
	return ok
}

type NetV4__0__0__RetryPolicy_ALLOW__RETRY struct {
}

func (NetV4__0__0__RetryPolicy_ALLOW__RETRY) isNetV4__0__0__RetryPolicy() {}

func (CompanionStruct_NetV4__0__0__RetryPolicy_) Create_ALLOW__RETRY_() NetV4__0__0__RetryPolicy {
	return NetV4__0__0__RetryPolicy{NetV4__0__0__RetryPolicy_ALLOW__RETRY{}}
}

func (_this NetV4__0__0__RetryPolicy) Is_ALLOW__RETRY() bool {
	_, ok := _this.Get_().(NetV4__0__0__RetryPolicy_ALLOW__RETRY)
	return ok
}

func (CompanionStruct_NetV4__0__0__RetryPolicy_) Default() NetV4__0__0__RetryPolicy {
	return Companion_NetV4__0__0__RetryPolicy_.Create_FORBID__RETRY_()
}

func (_ CompanionStruct_NetV4__0__0__RetryPolicy_) AllSingletonConstructors() _dafny.Iterator {
	i := -1
	return func() (interface{}, bool) {
		i++
		switch i {
		case 0:
			return Companion_NetV4__0__0__RetryPolicy_.Create_FORBID__RETRY_(), true
		case 1:
			return Companion_NetV4__0__0__RetryPolicy_.Create_ALLOW__RETRY_(), true
		default:
			return NetV4__0__0__RetryPolicy{}, false
		}
	}
}

func (_this NetV4__0__0__RetryPolicy) String() string {
	switch _this.Get_().(type) {
	case nil:
		return "null"
	case NetV4__0__0__RetryPolicy_FORBID__RETRY:
		{
			return "AwsCryptographyEncryptionSdkTypes.NetV4_0_0_RetryPolicy.FORBID_RETRY"
		}
	case NetV4__0__0__RetryPolicy_ALLOW__RETRY:
		{
			return "AwsCryptographyEncryptionSdkTypes.NetV4_0_0_RetryPolicy.ALLOW_RETRY"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this NetV4__0__0__RetryPolicy) Equals(other NetV4__0__0__RetryPolicy) bool {
	switch _this.Get_().(type) {
	case NetV4__0__0__RetryPolicy_FORBID__RETRY:
		{
			_, ok := other.Get_().(NetV4__0__0__RetryPolicy_FORBID__RETRY)
			return ok
		}
	case NetV4__0__0__RetryPolicy_ALLOW__RETRY:
		{
			_, ok := other.Get_().(NetV4__0__0__RetryPolicy_ALLOW__RETRY)
			return ok
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this NetV4__0__0__RetryPolicy) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(NetV4__0__0__RetryPolicy)
	return ok && _this.Equals(typed)
}

func Type_NetV4__0__0__RetryPolicy_() _dafny.TypeDescriptor {
	return type_NetV4__0__0__RetryPolicy_{}
}

type type_NetV4__0__0__RetryPolicy_ struct {
}

func (_this type_NetV4__0__0__RetryPolicy_) Default() interface{} {
	return Companion_NetV4__0__0__RetryPolicy_.Default()
}

func (_this type_NetV4__0__0__RetryPolicy_) String() string {
	return "AwsCryptographyEncryptionSdkTypes.NetV4__0__0__RetryPolicy"
}
func (_this NetV4__0__0__RetryPolicy) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = NetV4__0__0__RetryPolicy{}

// End of datatype NetV4__0__0__RetryPolicy

// Definition of datatype Error
type Error struct {
	Data_Error_
}

func (_this Error) Get_() Data_Error_ {
	return _this.Data_Error_
}

type Data_Error_ interface {
	isError()
}

type CompanionStruct_Error_ struct {
}

var Companion_Error_ = CompanionStruct_Error_{}

type Error_AwsEncryptionSdkException struct {
	Message _dafny.Sequence
}

func (Error_AwsEncryptionSdkException) isError() {}

func (CompanionStruct_Error_) Create_AwsEncryptionSdkException_(Message _dafny.Sequence) Error {
	return Error{Error_AwsEncryptionSdkException{Message}}
}

func (_this Error) Is_AwsEncryptionSdkException() bool {
	_, ok := _this.Get_().(Error_AwsEncryptionSdkException)
	return ok
}

type Error_AwsCryptographyMaterialProviders struct {
	AwsCryptographyMaterialProviders m_AwsCryptographyMaterialProvidersTypes.Error
}

func (Error_AwsCryptographyMaterialProviders) isError() {}

func (CompanionStruct_Error_) Create_AwsCryptographyMaterialProviders_(AwsCryptographyMaterialProviders m_AwsCryptographyMaterialProvidersTypes.Error) Error {
	return Error{Error_AwsCryptographyMaterialProviders{AwsCryptographyMaterialProviders}}
}

func (_this Error) Is_AwsCryptographyMaterialProviders() bool {
	_, ok := _this.Get_().(Error_AwsCryptographyMaterialProviders)
	return ok
}

type Error_AwsCryptographyPrimitives struct {
	AwsCryptographyPrimitives m_AwsCryptographyPrimitivesTypes.Error
}

func (Error_AwsCryptographyPrimitives) isError() {}

func (CompanionStruct_Error_) Create_AwsCryptographyPrimitives_(AwsCryptographyPrimitives m_AwsCryptographyPrimitivesTypes.Error) Error {
	return Error{Error_AwsCryptographyPrimitives{AwsCryptographyPrimitives}}
}

func (_this Error) Is_AwsCryptographyPrimitives() bool {
	_, ok := _this.Get_().(Error_AwsCryptographyPrimitives)
	return ok
}

type Error_CollectionOfErrors struct {
	List    _dafny.Sequence
	Message _dafny.Sequence
}

func (Error_CollectionOfErrors) isError() {}

func (CompanionStruct_Error_) Create_CollectionOfErrors_(List _dafny.Sequence, Message _dafny.Sequence) Error {
	return Error{Error_CollectionOfErrors{List, Message}}
}

func (_this Error) Is_CollectionOfErrors() bool {
	_, ok := _this.Get_().(Error_CollectionOfErrors)
	return ok
}

type Error_Opaque struct {
	Obj interface{}
}

func (Error_Opaque) isError() {}

func (CompanionStruct_Error_) Create_Opaque_(Obj interface{}) Error {
	return Error{Error_Opaque{Obj}}
}

func (_this Error) Is_Opaque() bool {
	_, ok := _this.Get_().(Error_Opaque)
	return ok
}

type Error_OpaqueWithText struct {
	Obj        interface{}
	ObjMessage _dafny.Sequence
}

func (Error_OpaqueWithText) isError() {}

func (CompanionStruct_Error_) Create_OpaqueWithText_(Obj interface{}, ObjMessage _dafny.Sequence) Error {
	return Error{Error_OpaqueWithText{Obj, ObjMessage}}
}

func (_this Error) Is_OpaqueWithText() bool {
	_, ok := _this.Get_().(Error_OpaqueWithText)
	return ok
}

func (CompanionStruct_Error_) Default() Error {
	return Companion_Error_.Create_AwsEncryptionSdkException_(_dafny.EmptySeq.SetString())
}

func (_this Error) Dtor_message() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case Error_AwsEncryptionSdkException:
		return data.Message
	default:
		return data.(Error_CollectionOfErrors).Message
	}
}

func (_this Error) Dtor_AwsCryptographyMaterialProviders() m_AwsCryptographyMaterialProvidersTypes.Error {
	return _this.Get_().(Error_AwsCryptographyMaterialProviders).AwsCryptographyMaterialProviders
}

func (_this Error) Dtor_AwsCryptographyPrimitives() m_AwsCryptographyPrimitivesTypes.Error {
	return _this.Get_().(Error_AwsCryptographyPrimitives).AwsCryptographyPrimitives
}

func (_this Error) Dtor_list() _dafny.Sequence {
	return _this.Get_().(Error_CollectionOfErrors).List
}

func (_this Error) Dtor_obj() interface{} {
	switch data := _this.Get_().(type) {
	case Error_Opaque:
		return data.Obj
	default:
		return data.(Error_OpaqueWithText).Obj
	}
}

func (_this Error) Dtor_objMessage() _dafny.Sequence {
	return _this.Get_().(Error_OpaqueWithText).ObjMessage
}

func (_this Error) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case Error_AwsEncryptionSdkException:
		{
			return "AwsCryptographyEncryptionSdkTypes.Error.AwsEncryptionSdkException" + "(" + _dafny.String(data.Message) + ")"
		}
	case Error_AwsCryptographyMaterialProviders:
		{
			return "AwsCryptographyEncryptionSdkTypes.Error.AwsCryptographyMaterialProviders" + "(" + _dafny.String(data.AwsCryptographyMaterialProviders) + ")"
		}
	case Error_AwsCryptographyPrimitives:
		{
			return "AwsCryptographyEncryptionSdkTypes.Error.AwsCryptographyPrimitives" + "(" + _dafny.String(data.AwsCryptographyPrimitives) + ")"
		}
	case Error_CollectionOfErrors:
		{
			return "AwsCryptographyEncryptionSdkTypes.Error.CollectionOfErrors" + "(" + _dafny.String(data.List) + ", " + _dafny.String(data.Message) + ")"
		}
	case Error_Opaque:
		{
			return "AwsCryptographyEncryptionSdkTypes.Error.Opaque" + "(" + _dafny.String(data.Obj) + ")"
		}
	case Error_OpaqueWithText:
		{
			return "AwsCryptographyEncryptionSdkTypes.Error.OpaqueWithText" + "(" + _dafny.String(data.Obj) + ", " + _dafny.String(data.ObjMessage) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this Error) Equals(other Error) bool {
	switch data1 := _this.Get_().(type) {
	case Error_AwsEncryptionSdkException:
		{
			data2, ok := other.Get_().(Error_AwsEncryptionSdkException)
			return ok && data1.Message.Equals(data2.Message)
		}
	case Error_AwsCryptographyMaterialProviders:
		{
			data2, ok := other.Get_().(Error_AwsCryptographyMaterialProviders)
			return ok && data1.AwsCryptographyMaterialProviders.Equals(data2.AwsCryptographyMaterialProviders)
		}
	case Error_AwsCryptographyPrimitives:
		{
			data2, ok := other.Get_().(Error_AwsCryptographyPrimitives)
			return ok && data1.AwsCryptographyPrimitives.Equals(data2.AwsCryptographyPrimitives)
		}
	case Error_CollectionOfErrors:
		{
			data2, ok := other.Get_().(Error_CollectionOfErrors)
			return ok && data1.List.Equals(data2.List) && data1.Message.Equals(data2.Message)
		}
	case Error_Opaque:
		{
			data2, ok := other.Get_().(Error_Opaque)
			return ok && _dafny.AreEqual(data1.Obj, data2.Obj)
		}
	case Error_OpaqueWithText:
		{
			data2, ok := other.Get_().(Error_OpaqueWithText)
			return ok && _dafny.AreEqual(data1.Obj, data2.Obj) && data1.ObjMessage.Equals(data2.ObjMessage)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this Error) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(Error)
	return ok && _this.Equals(typed)
}

func Type_Error_() _dafny.TypeDescriptor {
	return type_Error_{}
}

type type_Error_ struct {
}

func (_this type_Error_) Default() interface{} {
	return Companion_Error_.Default()
}

func (_this type_Error_) String() string {
	return "AwsCryptographyEncryptionSdkTypes.Error"
}
func (_this Error) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = Error{}

// End of datatype Error

// Definition of class OpaqueError
type OpaqueError struct {
}

func New_OpaqueError_() *OpaqueError {
	_this := OpaqueError{}

	return &_this
}

type CompanionStruct_OpaqueError_ struct {
}

var Companion_OpaqueError_ = CompanionStruct_OpaqueError_{}

func (*OpaqueError) String() string {
	return "AwsCryptographyEncryptionSdkTypes.OpaqueError"
}

// End of class OpaqueError

func Type_OpaqueError_() _dafny.TypeDescriptor {
	return type_OpaqueError_{}
}

type type_OpaqueError_ struct {
}

func (_this type_OpaqueError_) Default() interface{} {
	return Companion_Error_.Default()
}

func (_this type_OpaqueError_) String() string {
	return "AwsCryptographyEncryptionSdkTypes.OpaqueError"
}
func (_this *CompanionStruct_OpaqueError_) Is_(__source Error) bool {
	var _2_e Error = (__source)
	_ = _2_e
	return ((_2_e).Is_Opaque()) || ((_2_e).Is_OpaqueWithText())
}

// Definition of class DummySubsetType
type DummySubsetType struct {
}

func New_DummySubsetType_() *DummySubsetType {
	_this := DummySubsetType{}

	return &_this
}

type CompanionStruct_DummySubsetType_ struct {
}

var Companion_DummySubsetType_ = CompanionStruct_DummySubsetType_{}

func (*DummySubsetType) String() string {
	return "AwsCryptographyEncryptionSdkTypes.DummySubsetType"
}
func (_this *CompanionStruct_DummySubsetType_) Witness() _dafny.Int {
	return _dafny.One
}

// End of class DummySubsetType

func Type_DummySubsetType_() _dafny.TypeDescriptor {
	return type_DummySubsetType_{}
}

type type_DummySubsetType_ struct {
}

func (_this type_DummySubsetType_) Default() interface{} {
	return Companion_DummySubsetType_.Witness()
}

func (_this type_DummySubsetType_) String() string {
	return "AwsCryptographyEncryptionSdkTypes.DummySubsetType"
}
func (_this *CompanionStruct_DummySubsetType_) Is_(__source _dafny.Int) bool {
	var _3_x _dafny.Int = (__source)
	_ = _3_x
	return Companion_Default___.IsDummySubsetType(_3_x)
}
