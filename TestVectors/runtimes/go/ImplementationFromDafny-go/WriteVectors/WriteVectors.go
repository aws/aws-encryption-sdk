// Package WriteVectors
// Dafny module WriteVectors compiled into Go

package WriteVectors

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
	m_AllAlgorithmSuites "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllAlgorithmSuites"
	m_AllDefaultCmm "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllDefaultCmm"
	m_AllHierarchy "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllHierarchy"
	m_AllKms "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllKms"
	m_AllKmsEcdh "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllKmsEcdh"
	m_AllKmsMrkAware "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllKmsMrkAware"
	m_AllKmsMrkAwareDiscovery "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllKmsMrkAwareDiscovery"
	m_AllKmsRsa "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllKmsRsa"
	m_AllMulti "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllMulti"
	m_AllRawAES "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllRawAES"
	m_AllRawECDH "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllRawECDH"
	m_AllRawRSA "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllRawRSA"
	m_AllRequiredEncryptionContextCmm "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AllRequiredEncryptionContextCmm"
	m_AwsCryptographyMaterialProvidersTestVectorKeysTypes "github.com/aws/aws-cryptographic-material-providers-library/testvectors/AwsCryptographyMaterialProvidersTestVectorKeysTypes"
	m_CmmFromKeyDescription "github.com/aws/aws-cryptographic-material-providers-library/testvectors/CmmFromKeyDescription"
	m_CompleteVectors "github.com/aws/aws-cryptographic-material-providers-library/testvectors/CompleteVectors"
	m_CreateStaticKeyStores "github.com/aws/aws-cryptographic-material-providers-library/testvectors/CreateStaticKeyStores"
	m_CreateStaticKeyrings "github.com/aws/aws-cryptographic-material-providers-library/testvectors/CreateStaticKeyrings"
	m_JSONHelpers "github.com/aws/aws-cryptographic-material-providers-library/testvectors/JSONHelpers"
	m_KeyDescription "github.com/aws/aws-cryptographic-material-providers-library/testvectors/KeyDescription"
	m_KeyMaterial "github.com/aws/aws-cryptographic-material-providers-library/testvectors/KeyMaterial"
	m_KeyVectors "github.com/aws/aws-cryptographic-material-providers-library/testvectors/KeyVectors"
	m_KeyringFromKeyDescription "github.com/aws/aws-cryptographic-material-providers-library/testvectors/KeyringFromKeyDescription"
	m_KeysVectorOperations "github.com/aws/aws-cryptographic-material-providers-library/testvectors/KeysVectorOperations"
	m_MplManifestOptions "github.com/aws/aws-cryptographic-material-providers-library/testvectors/MplManifestOptions"
	m_ParseJsonManifests "github.com/aws/aws-cryptographic-material-providers-library/testvectors/ParseJsonManifests"
	m_TestManifests "github.com/aws/aws-cryptographic-material-providers-library/testvectors/TestManifests"
	m_TestVectors "github.com/aws/aws-cryptographic-material-providers-library/testvectors/TestVectors"
	m_WrappedMaterialProviders "github.com/aws/aws-cryptographic-material-providers-library/testvectors/WrappedMaterialProviders"
	m_WrappedMaterialProvidersMain "github.com/aws/aws-cryptographic-material-providers-library/testvectors/WrappedMaterialProvidersMain"
	m_WriteJsonManifests "github.com/aws/aws-cryptographic-material-providers-library/testvectors/WriteJsonManifests"
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
	m_AllEsdkV4NoReqEc "github.com/aws/aws-encryption-sdk/testvectors/AllEsdkV4NoReqEc"
	m_AllEsdkV4WithReqEc "github.com/aws/aws-encryption-sdk/testvectors/AllEsdkV4WithReqEc"
	m_EsdkManifestOptions "github.com/aws/aws-encryption-sdk/testvectors/EsdkManifestOptions"
	m_EsdkTestVectors "github.com/aws/aws-encryption-sdk/testvectors/EsdkTestVectors"
	m_WrappedESDK "github.com/aws/aws-encryption-sdk/testvectors/WrappedESDK"
	m_WriteEsdkJsonManifests "github.com/aws/aws-encryption-sdk/testvectors/WriteEsdkJsonManifests"
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
	m_JSON_API "github.com/dafny-lang/DafnyStandardLibGo/JSON_API"
	m_JSON_ConcreteSyntax_Spec "github.com/dafny-lang/DafnyStandardLibGo/JSON_ConcreteSyntax_Spec"
	m_JSON_ConcreteSyntax_SpecProperties "github.com/dafny-lang/DafnyStandardLibGo/JSON_ConcreteSyntax_SpecProperties"
	m_JSON_Deserializer "github.com/dafny-lang/DafnyStandardLibGo/JSON_Deserializer"
	m_JSON_Deserializer_ByteStrConversion "github.com/dafny-lang/DafnyStandardLibGo/JSON_Deserializer_ByteStrConversion"
	m_JSON_Deserializer_Uint16StrConversion "github.com/dafny-lang/DafnyStandardLibGo/JSON_Deserializer_Uint16StrConversion"
	m_JSON_Errors "github.com/dafny-lang/DafnyStandardLibGo/JSON_Errors"
	m_JSON_Grammar "github.com/dafny-lang/DafnyStandardLibGo/JSON_Grammar"
	m_JSON_Serializer "github.com/dafny-lang/DafnyStandardLibGo/JSON_Serializer"
	m_JSON_Serializer_ByteStrConversion "github.com/dafny-lang/DafnyStandardLibGo/JSON_Serializer_ByteStrConversion"
	m_JSON_Spec "github.com/dafny-lang/DafnyStandardLibGo/JSON_Spec"
	m_JSON_Utils_Cursors "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Cursors"
	m_JSON_Utils_Lexers_Core "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Lexers_Core"
	m_JSON_Utils_Lexers_Strings "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Lexers_Strings"
	m_JSON_Utils_Parsers "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Parsers"
	m_JSON_Utils_Seq "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Seq"
	m_JSON_Utils_Str "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Str"
	m_JSON_Utils_Str_CharStrConversion "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Str_CharStrConversion"
	m_JSON_Utils_Str_CharStrEscaping "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Str_CharStrEscaping"
	m_JSON_Utils_Vectors "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Vectors"
	m_JSON_Utils_Views_Core "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Views_Core"
	m_JSON_Utils_Views_Writers "github.com/dafny-lang/DafnyStandardLibGo/JSON_Utils_Views_Writers"
	m_JSON_Values "github.com/dafny-lang/DafnyStandardLibGo/JSON_Values"
	m_JSON_ZeroCopy_API "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_API"
	m_JSON_ZeroCopy_Deserializer "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer"
	m_JSON_ZeroCopy_Deserializer_API "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_API"
	m_JSON_ZeroCopy_Deserializer_ArrayParams "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_ArrayParams"
	m_JSON_ZeroCopy_Deserializer_Arrays "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_Arrays"
	m_JSON_ZeroCopy_Deserializer_Constants "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_Constants"
	m_JSON_ZeroCopy_Deserializer_Core "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_Core"
	m_JSON_ZeroCopy_Deserializer_Numbers "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_Numbers"
	m_JSON_ZeroCopy_Deserializer_ObjectParams "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_ObjectParams"
	m_JSON_ZeroCopy_Deserializer_Objects "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_Objects"
	m_JSON_ZeroCopy_Deserializer_Strings "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_Strings"
	m_JSON_ZeroCopy_Deserializer_Values "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Deserializer_Values"
	m_JSON_ZeroCopy_Serializer "github.com/dafny-lang/DafnyStandardLibGo/JSON_ZeroCopy_Serializer"
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
	m_UUID "github.com/dafny-lang/DafnyStandardLibGo/UUID"
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
var _ m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Dummy__
var _ m_JSON_Utils_Views_Core.Dummy__
var _ m_JSON_Utils_Views_Writers.Dummy__
var _ m_JSON_Utils_Lexers_Core.Dummy__
var _ m_JSON_Utils_Lexers_Strings.Dummy__
var _ m_JSON_Utils_Cursors.Dummy__
var _ m_JSON_Utils_Parsers.Dummy__
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
var _ m_JSON_Utils_Str_CharStrConversion.Dummy__
var _ m_JSON_Utils_Str_CharStrEscaping.Dummy__
var _ m_JSON_Utils_Str.Dummy__
var _ m_JSON_Utils_Seq.Dummy__
var _ m_JSON_Utils_Vectors.Dummy__
var _ m_JSON_Errors.Dummy__
var _ m_JSON_Values.Dummy__
var _ m__Unicode.Dummy__
var _ m_Functions.Dummy__
var _ m_Utf8EncodingForm.Dummy__
var _ m_Utf16EncodingForm.Dummy__
var _ m_UnicodeStrings.Dummy__
var _ m_JSON_Spec.Dummy__
var _ m_JSON_Grammar.Dummy__
var _ m_JSON_Serializer_ByteStrConversion.Dummy__
var _ m_JSON_Serializer.Dummy__
var _ m_JSON_Deserializer_Uint16StrConversion.Dummy__
var _ m_JSON_Deserializer_ByteStrConversion.Dummy__
var _ m_JSON_Deserializer.Dummy__
var _ m_JSON_ConcreteSyntax_Spec.Dummy__
var _ m_JSON_ConcreteSyntax_SpecProperties.Dummy__
var _ m_JSON_ZeroCopy_Serializer.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_Core.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_Strings.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_Numbers.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_ObjectParams.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_Objects.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_ArrayParams.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_Arrays.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_Constants.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_Values.Dummy__
var _ m_JSON_ZeroCopy_Deserializer_API.Dummy__
var _ m_JSON_ZeroCopy_Deserializer.Dummy__
var _ m_JSON_ZeroCopy_API.Dummy__
var _ m_JSON_API.Dummy__
var _ m_JSONHelpers.Dummy__
var _ m_KeyDescription.Dummy__
var _ m_HexStrings.Dummy__
var _ m_KeyMaterial.Dummy__
var _ m_CreateStaticKeyrings.Dummy__
var _ m_CreateStaticKeyStores.Dummy__
var _ m_KeyringFromKeyDescription.Dummy__
var _ m_CmmFromKeyDescription.Dummy__
var _ m_WrappedMaterialProviders.Dummy__
var _ m_KeysVectorOperations.Dummy__
var _ m_FileIO.Dummy__
var _ m_KeyVectors.Dummy__
var _ m_AwsCryptographyEncryptionSdkTypes.Dummy__
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
var _ m_MplManifestOptions.Dummy__
var _ m_GetOpt.Dummy__
var _ m_AllAlgorithmSuites.Dummy__
var _ m_TestVectors.Dummy__
var _ m_AllHierarchy.Dummy__
var _ m_AllKms.Dummy__
var _ m_AllKmsMrkAware.Dummy__
var _ m_AllKmsMrkAwareDiscovery.Dummy__
var _ m_AllKmsRsa.Dummy__
var _ m_AllKmsEcdh.Dummy__
var _ m_AllRawAES.Dummy__
var _ m_AllRawRSA.Dummy__
var _ m_AllRawECDH.Dummy__
var _ m_AllDefaultCmm.Dummy__
var _ m_AllRequiredEncryptionContextCmm.Dummy__
var _ m_AllMulti.Dummy__
var _ m_WriteJsonManifests.Dummy__
var _ m_CompleteVectors.Dummy__
var _ m_ParseJsonManifests.Dummy__
var _ m_TestManifests.Dummy__
var _ m_WrappedMaterialProvidersMain.Dummy__
var _ m_StandardLibraryInterop.Dummy__
var _ m_Sorting.Dummy__
var _ m_FloatCompare.Dummy__
var _ m_Base64Lemmas.Dummy__
var _ m_WrappedESDK.Dummy__
var _ m_EsdkManifestOptions.Dummy__
var _ m_EsdkTestVectors.Dummy__
var _ m_AllEsdkV4NoReqEc.Dummy__
var _ m_AllEsdkV4WithReqEc.Dummy__
var _ m_WriteEsdkJsonManifests.Dummy__

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
	return "WriteVectors.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) GetCommitmentPolicyString(algorithmSuite m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo) _dafny.Sequence {
	var _source0 m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteId = (algorithmSuite).Dtor_id()
	_ = _source0
	{
		if _source0.Is_ESDK() {
			if ((algorithmSuite).Dtor_commitment()).Is_None() {
				return _dafny.SeqOfString("FORBID_ENCRYPT_ALLOW_DECRYPT")
			} else {
				return _dafny.SeqOfString("REQUIRE_ENCRYPT_REQUIRE_DECRYPT")
			}
		}
	}
	{
		return _dafny.SeqOfString("NOT SUPPORTED FOR UNSTRUCTURED ENCRYPTION")
	}
}
func (_static *CompanionStruct_Default___) GetCommitmentPolicyType(commitmentPolicy _dafny.Sequence) m_AwsCryptographyMaterialProvidersTypes.CommitmentPolicy {
	if _dafny.Companion_Sequence_.Equal(commitmentPolicy, _dafny.SeqOfString("FORBID_ENCRYPT_ALLOW_DECRYPT")) {
		return m_AwsCryptographyMaterialProvidersTypes.Companion_CommitmentPolicy_.Create_ESDK_(m_AwsCryptographyMaterialProvidersTypes.Companion_ESDKCommitmentPolicy_.Create_FORBID__ENCRYPT__ALLOW__DECRYPT_())
	} else {
		return m_AwsCryptographyMaterialProvidersTypes.Companion_CommitmentPolicy_.Create_ESDK_(m_AwsCryptographyMaterialProvidersTypes.Companion_ESDKCommitmentPolicy_.Create_REQUIRE__ENCRYPT__REQUIRE__DECRYPT_())
	}
}
func (_static *CompanionStruct_Default___) WriteTestVectors(op m_EsdkManifestOptions.ManifestOptions) m_Wrappers.Result {
	var output m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf())
	_ = output
	var _0_version _dafny.Int
	_ = _0_version
	_0_version = (op).Dtor_version()
	var _1_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySet)
	_ = _1_valueOrError0
	_1_valueOrError0 = Companion_Default___.GetVersionTests(_0_version)
	if (_1_valueOrError0).IsFailure() {
		output = (_1_valueOrError0).PropagateFailure()
		return output
	}
	var _2_allTests _dafny.Set
	_ = _2_allTests
	_2_allTests = (_1_valueOrError0).Extract().(_dafny.Set)
	var _3_tests _dafny.Sequence
	_ = _3_tests
	var _out0 _dafny.Sequence
	_ = _out0
	_out0 = m_SortedSets.SetToSequence(_2_allTests)
	_3_tests = _out0
	var _4_sortedTests _dafny.Sequence
	_ = _4_sortedTests
	_4_sortedTests = m_Seq_MergeSort.Companion_Default___.MergeSortBy(_3_tests, func(coer5 func(m_EsdkTestVectors.EsdkEncryptTestVector, m_EsdkTestVectors.EsdkEncryptTestVector) bool) func(interface{}, interface{}) bool {
		return func(arg6 interface{}, arg7 interface{}) bool {
			return coer5(arg6.(m_EsdkTestVectors.EsdkEncryptTestVector), arg7.(m_EsdkTestVectors.EsdkEncryptTestVector))
		}
	}(Companion_Default___.DescriptionLessThan))
	var _5_testsJSON _dafny.Sequence
	_ = _5_testsJSON
	_5_testsJSON = _dafny.SeqOf()
	var _hi0 _dafny.Int = _dafny.IntOfUint32((_4_sortedTests).Cardinality())
	_ = _hi0
	for _6_i := _dafny.Zero; _6_i.Cmp(_hi0) < 0; _6_i = _6_i.Plus(_dafny.One) {
		var _7_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
		_ = _7_valueOrError1
		_7_valueOrError1 = m_Wrappers.Companion_Default___.Need((true) && ((((_4_sortedTests).Select((_6_i).Uint32()).(m_EsdkTestVectors.EsdkEncryptTestVector)).Dtor_algorithmSuiteId()).Is_Some()), _dafny.SeqOfString("No algorithm suite defined in test"))
		if (_7_valueOrError1).IsFailure() {
			output = (_7_valueOrError1).PropagateFailure()
			return output
		}
		var _8_id _dafny.Sequence
		_ = _8_id
		_8_id = m_AllAlgorithmSuites.Companion_Default___.ToHex((((_4_sortedTests).Select((_6_i).Uint32()).(m_EsdkTestVectors.EsdkEncryptTestVector)).Dtor_algorithmSuiteId()).Dtor_value().(m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo))
		var _9_valueOrError2 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq.SetString())
		_ = _9_valueOrError2
		var _out1 m_Wrappers.Result
		_ = _out1
		_out1 = m_UUID.GenerateUUID()
		_9_valueOrError2 = _out1
		if !(!((_9_valueOrError2).IsFailure())) {
			panic("dafny/TestVectors/src/WriteVectors.dfy(82,18): " + (_9_valueOrError2).String())
		}
		var _10_uuid _dafny.Sequence
		_ = _10_uuid
		_10_uuid = (_9_valueOrError2).Extract().(_dafny.Sequence)
		var _11_valueOrError3 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_JSON_Values.Companion_JSON_.Default())
		_ = _11_valueOrError3
		_11_valueOrError3 = m_WriteEsdkJsonManifests.Companion_Default___.EncryptTestVectorToJson((_4_sortedTests).Select((_6_i).Uint32()).(m_EsdkTestVectors.EsdkEncryptTestVector))
		if (_11_valueOrError3).IsFailure() {
			output = (_11_valueOrError3).PropagateFailure()
			return output
		}
		var _12_test m_JSON_Values.JSON
		_ = _12_test
		_12_test = (_11_valueOrError3).Extract().(m_JSON_Values.JSON)
		_5_testsJSON = _dafny.Companion_Sequence_.Concatenate(_5_testsJSON, _dafny.SeqOf(_dafny.TupleOf(_10_uuid, _12_test)))
	}
	var _13_manifestJson m_JSON_Values.JSON
	_ = _13_manifestJson
	_13_manifestJson = m_JSON_Values.Companion_JSON_.Create_Object_(_dafny.SeqOf(_dafny.TupleOf(_dafny.SeqOfString("type"), m_JSON_Values.Companion_JSON_.Create_String_(_dafny.SeqOfString("awses-encrypt"))), _dafny.TupleOf(_dafny.SeqOfString("version"), m_JSON_Values.Companion_JSON_.Create_Number_(m_JSON_Values.Companion_Default___.Int(_dafny.IntOfInt64(5))))))
	var _14_plaintexts m_JSON_Values.JSON
	_ = _14_plaintexts
	_14_plaintexts = m_JSON_Values.Companion_JSON_.Create_Object_(_dafny.SeqOf(_dafny.TupleOf(_dafny.SeqOfString("small"), m_JSON_Values.Companion_JSON_.Create_Number_(m_JSON_Values.Companion_Default___.Int(_dafny.IntOfInt64(10240))))))
	var _15_esdkEncryptManifests m_JSON_Values.JSON
	_ = _15_esdkEncryptManifests
	_15_esdkEncryptManifests = m_JSON_Values.Companion_JSON_.Create_Object_(_dafny.SeqOf(_dafny.TupleOf(_dafny.SeqOfString("manifest"), _13_manifestJson), _dafny.TupleOf(_dafny.SeqOfString("keys"), m_JSON_Values.Companion_JSON_.Create_String_(_dafny.SeqOfString("file://keys.json"))), _dafny.TupleOf(_dafny.SeqOfString("plaintexts"), _14_plaintexts), _dafny.TupleOf(_dafny.SeqOfString("tests"), m_JSON_Values.Companion_JSON_.Create_Object_(_5_testsJSON))))
	var _16_valueOrError4 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _16_valueOrError4
	_16_valueOrError4 = m_JSON_API.Companion_Default___.Serialize(_15_esdkEncryptManifests)
	if !(!((_16_valueOrError4).IsFailure())) {
		panic("dafny/TestVectors/src/WriteVectors.dfy(102,36): " + (_16_valueOrError4).String())
	}
	var _17_esdkEncryptManifestBytes _dafny.Sequence
	_ = _17_esdkEncryptManifestBytes
	_17_esdkEncryptManifestBytes = (_16_valueOrError4).Extract().(_dafny.Sequence)
	var _18_esdkEncryptManifestBv _dafny.Sequence
	_ = _18_esdkEncryptManifestBv
	_18_esdkEncryptManifestBv = m_JSONHelpers.Companion_Default___.BytesBv(_17_esdkEncryptManifestBytes)
	var _19_valueOrError5 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf())
	_ = _19_valueOrError5
	var _out2 m_Wrappers.Result
	_ = _out2
	_out2 = m_FileIO.Companion_Default___.WriteBytesToFile(_dafny.Companion_Sequence_.Concatenate((op).Dtor_encryptManifestOutput(), _dafny.SeqOfString("encrypt-manifest.json")), _18_esdkEncryptManifestBv)
	_19_valueOrError5 = _out2
	if !(!((_19_valueOrError5).IsFailure())) {
		panic("dafny/TestVectors/src/WriteVectors.dfy(105,13): " + (_19_valueOrError5).String())
	}
	var _20___v2 _dafny.Tuple
	_ = _20___v2
	_20___v2 = (_19_valueOrError5).Extract().(_dafny.Tuple)
	output = m_Wrappers.Companion_Result_.Create_Success_(_dafny.TupleOf())
	return output
}
func (_static *CompanionStruct_Default___) WriteDecryptManifest(op m_EsdkManifestOptions.ManifestOptions, keys *m_KeyVectors.KeyVectorsClient, tests _dafny.Sequence) m_Wrappers.Result {
	var output m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf())
	_ = output
	var _0_testsJSON _dafny.Sequence
	_ = _0_testsJSON
	_0_testsJSON = _dafny.SeqOf()
	var _hi0 _dafny.Int = _dafny.IntOfUint32((tests).Cardinality())
	_ = _hi0
	for _1_i := _dafny.Zero; _1_i.Cmp(_hi0) < 0; _1_i = _1_i.Plus(_dafny.One) {
		var _2_name _dafny.Sequence
		_ = _2_name
		_2_name = ((tests).Select((_1_i).Uint32()).(m_EsdkTestVectors.EsdkDecryptTestVector)).Dtor_id()
		var _3_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_JSON_Values.Companion_JSON_.Default())
		_ = _3_valueOrError0
		_3_valueOrError0 = m_WriteEsdkJsonManifests.Companion_Default___.DecryptTestVectorToJson((tests).Select((_1_i).Uint32()).(m_EsdkTestVectors.EsdkDecryptTestVector))
		if (_3_valueOrError0).IsFailure() {
			output = (_3_valueOrError0).PropagateFailure()
			return output
		}
		var _4_test m_JSON_Values.JSON
		_ = _4_test
		_4_test = (_3_valueOrError0).Extract().(m_JSON_Values.JSON)
		_0_testsJSON = _dafny.Companion_Sequence_.Concatenate(_0_testsJSON, _dafny.SeqOf(_dafny.TupleOf(_2_name, _4_test)))
	}
	var _5_manifestJson m_JSON_Values.JSON
	_ = _5_manifestJson
	_5_manifestJson = m_JSON_Values.Companion_JSON_.Create_Object_(_dafny.SeqOf(_dafny.TupleOf(_dafny.SeqOfString("type"), m_JSON_Values.Companion_JSON_.Create_String_(_dafny.SeqOfString("awses-decrypt"))), _dafny.TupleOf(_dafny.SeqOfString("version"), m_JSON_Values.Companion_JSON_.Create_Number_(m_JSON_Values.Companion_Default___.Int(_dafny.IntOfInt64(5))))))
	var _6_clientJson m_JSON_Values.JSON
	_ = _6_clientJson
	_6_clientJson = m_JSON_Values.Companion_JSON_.Create_Object_(_dafny.SeqOf(_dafny.TupleOf(_dafny.SeqOfString("name"), m_JSON_Values.Companion_JSON_.Create_String_(_dafny.SeqOfString("aws-encryption-sdk-dafny"))), _dafny.TupleOf(_dafny.SeqOfString("version"), m_JSON_Values.Companion_JSON_.Create_String_(_dafny.SeqOfString("4.1.0")))))
	var _7_esdkDecryptManifest m_JSON_Values.JSON
	_ = _7_esdkDecryptManifest
	_7_esdkDecryptManifest = m_JSON_Values.Companion_JSON_.Create_Object_(_dafny.SeqOf(_dafny.TupleOf(_dafny.SeqOfString("manifest"), _5_manifestJson), _dafny.TupleOf(_dafny.SeqOfString("client"), _6_clientJson), _dafny.TupleOf(_dafny.SeqOfString("keys"), m_JSON_Values.Companion_JSON_.Create_String_(_dafny.SeqOfString("file://keys.json"))), _dafny.TupleOf(_dafny.SeqOfString("tests"), m_JSON_Values.Companion_JSON_.Create_Object_(_0_testsJSON))))
	var _8_valueOrError1 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _8_valueOrError1
	_8_valueOrError1 = m_JSON_API.Companion_Default___.Serialize(_7_esdkDecryptManifest)
	if !(!((_8_valueOrError1).IsFailure())) {
		panic("dafny/TestVectors/src/WriteVectors.dfy(149,36): " + (_8_valueOrError1).String())
	}
	var _9_esdkDecryptManifestBytes _dafny.Sequence
	_ = _9_esdkDecryptManifestBytes
	_9_esdkDecryptManifestBytes = (_8_valueOrError1).Extract().(_dafny.Sequence)
	var _10_esdkDecryptManifestBv _dafny.Sequence
	_ = _10_esdkDecryptManifestBv
	_10_esdkDecryptManifestBv = m_JSONHelpers.Companion_Default___.BytesBv(_9_esdkDecryptManifestBytes)
	var _11_valueOrError2 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf())
	_ = _11_valueOrError2
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_FileIO.Companion_Default___.WriteBytesToFile(_dafny.Companion_Sequence_.Concatenate((op).Dtor_decryptManifestOutput(), _dafny.SeqOfString("decrypt-manifest.json")), _10_esdkDecryptManifestBv)
	_11_valueOrError2 = _out0
	if !(!((_11_valueOrError2).IsFailure())) {
		panic("dafny/TestVectors/src/WriteVectors.dfy(152,13): " + (_11_valueOrError2).String())
	}
	var _12___v3 _dafny.Tuple
	_ = _12___v3
	_12___v3 = (_11_valueOrError2).Extract().(_dafny.Tuple)
	output = m_Wrappers.Companion_Result_.Create_Success_(_dafny.TupleOf())
	return output
}
func (_static *CompanionStruct_Default___) GetVersionTests(version _dafny.Int) m_Wrappers.Result {
	var _source0 _dafny.Int = version
	_ = _source0
	{
		if (_source0).Cmp(_dafny.IntOfInt64(5)) == 0 {
			return m_Wrappers.Companion_Result_.Create_Success_((m_AllEsdkV4NoReqEc.Companion_Default___.Tests()).Union(m_AllEsdkV4WithReqEc.Companion_Default___.Tests()))
		}
	}
	{
		return m_Wrappers.Companion_Result_.Create_Failure_(_dafny.SeqOfString("Only version 4 of generate manifest is supported\n"))
	}
}
func (_static *CompanionStruct_Default___) DescriptionLessThan(x m_EsdkTestVectors.EsdkEncryptTestVector, y m_EsdkTestVectors.EsdkEncryptTestVector) bool {
	return Companion_Default___.Below((x).Dtor_description(), (y).Dtor_description())
}
func (_static *CompanionStruct_Default___) Below(x _dafny.Sequence, y _dafny.Sequence) bool {
	return !((_dafny.IntOfUint32((x).Cardinality())).Sign() != 0) || ((((_dafny.IntOfUint32((y).Cardinality())).Sign() != 0) && (((x).Select(0).(_dafny.Char)) <= ((y).Select(0).(_dafny.Char)))) && (!(((x).Select(0).(_dafny.Char)) == ((y).Select(0).(_dafny.Char))) || (Companion_Default___.Below((x).Drop(1), (y).Drop(1)))))
}

// End of class Default__
