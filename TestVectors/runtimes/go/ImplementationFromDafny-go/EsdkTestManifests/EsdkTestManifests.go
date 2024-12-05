// Package EsdkTestManifests
// Dafny module EsdkTestManifests compiled into Go

package EsdkTestManifests

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
	m_ParseEsdkJsonManifest "github.com/aws/aws-encryption-sdk/testvectors/ParseEsdkJsonManifest"
	m_WrappedESDK "github.com/aws/aws-encryption-sdk/testvectors/WrappedESDK"
	m_WriteEsdkJsonManifests "github.com/aws/aws-encryption-sdk/testvectors/WriteEsdkJsonManifests"
	m_WriteVectors "github.com/aws/aws-encryption-sdk/testvectors/WriteVectors"
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
var _ m_WriteVectors.Dummy__
var _ m_ParseEsdkJsonManifest.Dummy__

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
	return "EsdkTestManifests.Default__"
}
func (_this *Default__) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = &Default__{}

func (_static *CompanionStruct_Default___) StartDecryptVectors(op m_EsdkManifestOptions.ManifestOptions) m_Wrappers.Result {
	var output m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = output
	var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _0_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = Companion_Default___.GetManifest((op).Dtor_manifestPath(), (op).Dtor_manifestFileName())
	_0_valueOrError0 = _out0
	if !(!((_0_valueOrError0).IsFailure())) {
		panic("dafny/TestVectors/src/EsdkTestManifests.dfy(40,27): " + (_0_valueOrError0).String())
	}
	var _1_decryptManifest ManifestData
	_ = _1_decryptManifest
	_1_decryptManifest = (_0_valueOrError0).Extract().(ManifestData)
	var _2_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _2_valueOrError1
	_2_valueOrError1 = m_Wrappers.Companion_Default___.Need((_1_decryptManifest).Is_DecryptManifest(), _dafny.SeqOfString("Not a decrypt manifest"))
	if (_2_valueOrError1).IsFailure() {
		output = (_2_valueOrError1).PropagateFailure()
		return output
	}
	var _3_valueOrError2 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _3_valueOrError2
	_3_valueOrError2 = m_ParseEsdkJsonManifest.Companion_Default___.BuildDecryptTestVector(op, (_1_decryptManifest).Dtor_version(), (_1_decryptManifest).Dtor_keys(), (_1_decryptManifest).Dtor_jsonTests())
	if (_3_valueOrError2).IsFailure() {
		output = (_3_valueOrError2).PropagateFailure()
		return output
	}
	var _4_decryptVectors _dafny.Sequence
	_ = _4_decryptVectors
	_4_decryptVectors = (_3_valueOrError2).Extract().(_dafny.Sequence)
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = Companion_Default___.TestDecrypts((_1_decryptManifest).Dtor_keys(), _4_decryptVectors)
	output = _out1
	return output
}
func (_static *CompanionStruct_Default___) TestDecryptVector_q(v m_EsdkTestVectors.EsdkDecryptTestVector) bool {
	return (true) && (((v).Dtor_decryptionMethod()).Is_OneShot())
}
func (_static *CompanionStruct_Default___) TestDecrypts(keys *m_KeyVectors.KeyVectorsClient, vectors _dafny.Sequence) m_Wrappers.Result {
	var manifest m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = manifest
	_dafny.Print((_dafny.SeqOfString("\n=================== Starting ")).SetString())
	_dafny.Print(_dafny.IntOfUint32((vectors).Cardinality()))
	_dafny.Print((_dafny.SeqOfString(" Decrypt Tests =================== \n\n")).SetString())
	var _0_hasFailure bool
	_ = _0_hasFailure
	_0_hasFailure = false
	var _1_skipped _dafny.Int
	_ = _1_skipped
	_1_skipped = _dafny.Zero
	var _hi0 _dafny.Int = _dafny.IntOfUint32((vectors).Cardinality())
	_ = _hi0
	for _2_i := _dafny.Zero; _2_i.Cmp(_hi0) < 0; _2_i = _2_i.Plus(_dafny.One) {
		var _3_vector m_EsdkTestVectors.EsdkDecryptTestVector
		_ = _3_vector
		_3_vector = (vectors).Select((_2_i).Uint32()).(m_EsdkTestVectors.EsdkDecryptTestVector)
		if Companion_Default___.TestDecryptVector_q(_3_vector) {
			var _4_pass bool
			_ = _4_pass
			var _out0 bool
			_ = _out0
			_out0 = m_EsdkTestVectors.Companion_Default___.TestDecrypt(keys, _3_vector)
			_4_pass = _out0
			if !(_4_pass) {
				_0_hasFailure = true
			}
		} else {
			_1_skipped = (_1_skipped).Plus(_dafny.One)
			_dafny.Print((_dafny.SeqOfString("\nSKIP===> ")).SetString())
			_dafny.Print(((_3_vector).Dtor_id()).SetString())
			_dafny.Print((_dafny.SeqOfString("\n")).SetString())
		}
	}
	_dafny.Print((_dafny.SeqOfString("\n=================== Completed ")).SetString())
	_dafny.Print(_dafny.IntOfUint32((vectors).Cardinality()))
	_dafny.Print((_dafny.SeqOfString(" Decrypt Tests =================== \n\n")).SetString())
	if (_1_skipped).Sign() == 1 {
		_dafny.Print((_dafny.SeqOfString("Skipped: ")).SetString())
		_dafny.Print(_1_skipped)
		_dafny.Print((_dafny.SeqOfString("\n")).SetString())
	}
	if !(!(_0_hasFailure)) {
		panic("dafny/TestVectors/src/EsdkTestManifests.dfy(92,4): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	manifest = m_Wrappers.Companion_Result_.Create_Success_(_dafny.SeqOf())
	return manifest
}
func (_static *CompanionStruct_Default___) StartEncryptVectors(op m_EsdkManifestOptions.ManifestOptions) m_Wrappers.Result {
	var output m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf())
	_ = output
	var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _0_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = Companion_Default___.GetManifest((op).Dtor_manifestPath(), (op).Dtor_manifest())
	_0_valueOrError0 = _out0
	if (_0_valueOrError0).IsFailure() {
		output = (_0_valueOrError0).PropagateFailure()
		return output
	}
	var _1_encryptManifest ManifestData
	_ = _1_encryptManifest
	_1_encryptManifest = (_0_valueOrError0).Extract().(ManifestData)
	var _2_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _2_valueOrError1
	_2_valueOrError1 = m_Wrappers.Companion_Default___.Need((_1_encryptManifest).Is_EncryptManifest(), _dafny.SeqOfString("Not a encrypt manifest"))
	if (_2_valueOrError1).IsFailure() {
		output = (_2_valueOrError1).PropagateFailure()
		return output
	}
	var _3_valueOrError2 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _3_valueOrError2
	_3_valueOrError2 = m_ParseEsdkJsonManifest.Companion_Default___.BuildEncryptTestVector(op, (_1_encryptManifest).Dtor_version(), (_1_encryptManifest).Dtor_keys(), (_1_encryptManifest).Dtor_jsonTests())
	if (_3_valueOrError2).IsFailure() {
		output = (_3_valueOrError2).PropagateFailure()
		return output
	}
	var _4_encryptVectors _dafny.Sequence
	_ = _4_encryptVectors
	_4_encryptVectors = (_3_valueOrError2).Extract().(_dafny.Sequence)
	var _5_valueOrError3 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _5_valueOrError3
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_AtomicPrimitives.Companion_Default___.AtomicPrimitives(m_AtomicPrimitives.Companion_Default___.DefaultCryptoConfig())
	_5_valueOrError3 = _out1
	if !(!((_5_valueOrError3).IsFailure())) {
		panic("dafny/TestVectors/src/EsdkTestManifests.dfy(114,13): " + (_5_valueOrError3).String())
	}
	var _6_p *m_AtomicPrimitives.AtomicPrimitivesClient
	_ = _6_p
	_6_p = (_5_valueOrError3).Extract().(*m_AtomicPrimitives.AtomicPrimitivesClient)
	var _7_plaintext _dafny.Map
	_ = _7_plaintext
	_7_plaintext = _dafny.NewMapBuilder().ToMap()
	var _hi0 _dafny.Int = _dafny.IntOfUint32(((_1_encryptManifest).Dtor_plaintext()).Cardinality())
	_ = _hi0
	for _8_i := _dafny.Zero; _8_i.Cmp(_hi0) < 0; _8_i = _8_i.Plus(_dafny.One) {
		var _let_tmp_rhs0 _dafny.Tuple = ((_1_encryptManifest).Dtor_plaintext()).Select((_8_i).Uint32()).(_dafny.Tuple)
		_ = _let_tmp_rhs0
		var _9_name _dafny.Sequence = (*(_let_tmp_rhs0).IndexInt(0)).(_dafny.Sequence)
		_ = _9_name
		var _10_length int32 = (*(_let_tmp_rhs0).IndexInt(1)).(int32)
		_ = _10_length
		var _11_valueOrError4 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
		_ = _11_valueOrError4
		var _out2 m_Wrappers.Result
		_ = _out2
		_out2 = (_6_p).GenerateRandomBytes(m_AwsCryptographyPrimitivesTypes.Companion_GenerateRandomBytesInput_.Create_GenerateRandomBytesInput_(_10_length))
		_11_valueOrError4 = _out2
		if !(!((_11_valueOrError4).IsFailure())) {
			panic("dafny/TestVectors/src/EsdkTestManifests.dfy(119,18): " + (_11_valueOrError4).String())
		}
		var _12_data _dafny.Sequence
		_ = _12_data
		_12_data = (_11_valueOrError4).Extract().(_dafny.Sequence)
		_dafny.Print((_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate((op).Dtor_decryptManifestOutput(), m_EsdkTestVectors.Companion_Default___.PlaintextPathRoot()), _9_name)).SetString())
		_dafny.Print((_dafny.SeqOfString("\n\n")).SetString())
		var _13_valueOrError5 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf())
		_ = _13_valueOrError5
		var _out3 m_Wrappers.Result
		_ = _out3
		_out3 = m_EsdkTestVectors.Companion_Default___.WriteVectorsFile(_dafny.Companion_Sequence_.Concatenate(_dafny.Companion_Sequence_.Concatenate((op).Dtor_decryptManifestOutput(), m_EsdkTestVectors.Companion_Default___.PlaintextPathRoot()), _9_name), _12_data)
		_13_valueOrError5 = _out3
		if (_13_valueOrError5).IsFailure() {
			output = (_13_valueOrError5).PropagateFailure()
			return output
		}
		var _14___v0 _dafny.Tuple
		_ = _14___v0
		_14___v0 = (_13_valueOrError5).Extract().(_dafny.Tuple)
		_7_plaintext = (_7_plaintext).Merge(_dafny.NewMapBuilder().ToMap().UpdateUnsafe(_9_name, _12_data))
	}
	var _15_encryptTests_q m_Wrappers.Result
	_ = _15_encryptTests_q
	var _out4 m_Wrappers.Result
	_ = _out4
	_out4 = Companion_Default___.ToEncryptTests((_1_encryptManifest).Dtor_keys(), _4_encryptVectors)
	_15_encryptTests_q = _out4
	var _16_valueOrError6 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _16_valueOrError6
	_16_valueOrError6 = (_15_encryptTests_q).MapFailure(func(coer8 func(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error) _dafny.Sequence) func(interface{}) interface{} {
		return func(arg10 interface{}) interface{} {
			return coer8(arg10.(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error))
		}
	}(func(_17_e m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Error) _dafny.Sequence {
		return func(_pat_let6_0 _dafny.Tuple) _dafny.Sequence {
			return func(_18___v1 _dafny.Tuple) _dafny.Sequence {
				return _dafny.SeqOfString("Cmm failure")
			}(_pat_let6_0)
		}(m_EsdkTestVectors.Companion_Default___.MplVectorPrintErr(_17_e))
	}))
	if (_16_valueOrError6).IsFailure() {
		output = (_16_valueOrError6).PropagateFailure()
		return output
	}
	var _19_encryptTests _dafny.Sequence
	_ = _19_encryptTests
	_19_encryptTests = (_16_valueOrError6).Extract().(_dafny.Sequence)
	var _20_valueOrError7 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _20_valueOrError7
	var _out5 m_Wrappers.Result
	_ = _out5
	_out5 = Companion_Default___.TestEncrypts(_7_plaintext, (_1_encryptManifest).Dtor_keys(), _19_encryptTests)
	_20_valueOrError7 = _out5
	if (_20_valueOrError7).IsFailure() {
		output = (_20_valueOrError7).PropagateFailure()
		return output
	}
	var _21_decryptVectors _dafny.Sequence
	_ = _21_decryptVectors
	_21_decryptVectors = (_20_valueOrError7).Extract().(_dafny.Sequence)
	var _22_valueOrError8 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.TupleOf())
	_ = _22_valueOrError8
	var _out6 m_Wrappers.Result
	_ = _out6
	_out6 = m_WriteVectors.Companion_Default___.WriteDecryptManifest(op, (_1_encryptManifest).Dtor_keys(), _21_decryptVectors)
	_22_valueOrError8 = _out6
	if (_22_valueOrError8).IsFailure() {
		output = (_22_valueOrError8).PropagateFailure()
		return output
	}
	var _23___v2 _dafny.Tuple
	_ = _23___v2
	_23___v2 = (_22_valueOrError8).Extract().(_dafny.Tuple)
	output = m_Wrappers.Companion_Result_.Create_Success_(_dafny.TupleOf())
	return output
}
func (_static *CompanionStruct_Default___) TestEncryptVector_q(vector m_EsdkTestVectors.EsdkEncryptTestVector) bool {
	return (true) && (!(((vector).Dtor_frameLength()).Is_Some()) || (m_AwsCryptographyEncryptionSdkTypes.Companion_Default___.IsValid__FrameLength(((vector).Dtor_frameLength()).Dtor_value().(int64))))
}
func (_static *CompanionStruct_Default___) ToEncryptTests(keys *m_KeyVectors.KeyVectorsClient, vectors _dafny.Sequence) m_Wrappers.Result {
	var output m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = output
	var _0_encryptTests _dafny.Sequence
	_ = _0_encryptTests
	_0_encryptTests = _dafny.SeqOf()
	var _hi0 _dafny.Int = _dafny.IntOfUint32((vectors).Cardinality())
	_ = _hi0
	for _1_i := _dafny.Zero; _1_i.Cmp(_hi0) < 0; _1_i = _1_i.Plus(_dafny.One) {
		var _2_valueOrError0 m_Wrappers.Result = m_Wrappers.Result{}
		_ = _2_valueOrError0
		var _out0 m_Wrappers.Result
		_ = _out0
		_out0 = m_EsdkTestVectors.Companion_Default___.EncryptVectorToEncryptTest(keys, (vectors).Select((_1_i).Uint32()).(m_EsdkTestVectors.EsdkEncryptTestVector))
		_2_valueOrError0 = _out0
		if (_2_valueOrError0).IsFailure() {
			output = (_2_valueOrError0).PropagateFailure()
			return output
		}
		var _3_test m_EsdkTestVectors.EncryptTest
		_ = _3_test
		_3_test = (_2_valueOrError0).Extract().(m_EsdkTestVectors.EncryptTest)
		_0_encryptTests = _dafny.Companion_Sequence_.Concatenate(_0_encryptTests, _dafny.SeqOf(_3_test))
	}
	output = m_Wrappers.Companion_Result_.Create_Success_(_0_encryptTests)
	return output
	return output
}
func (_static *CompanionStruct_Default___) TestEncrypts(plaintexts _dafny.Map, keys *m_KeyVectors.KeyVectorsClient, tests _dafny.Sequence) m_Wrappers.Result {
	var manifest m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = manifest
	_dafny.Print((_dafny.SeqOfString("\n=================== Starting ")).SetString())
	_dafny.Print(_dafny.IntOfUint32((tests).Cardinality()))
	_dafny.Print((_dafny.SeqOfString(" Encrypt Tests =================== \n\n")).SetString())
	var _0_hasFailure bool
	_ = _0_hasFailure
	_0_hasFailure = false
	var _1_decryptVectors _dafny.Sequence
	_ = _1_decryptVectors
	_1_decryptVectors = _dafny.SeqOf()
	var _2_skipped _dafny.Sequence
	_ = _2_skipped
	_2_skipped = _dafny.SeqOf()
	var _hi0 _dafny.Int = _dafny.IntOfUint32((tests).Cardinality())
	_ = _hi0
	for _3_i := _dafny.Zero; _3_i.Cmp(_hi0) < 0; _3_i = _3_i.Plus(_dafny.One) {
		var _4_test m_EsdkTestVectors.EncryptTest
		_ = _4_test
		_4_test = (tests).Select((_3_i).Uint32()).(m_EsdkTestVectors.EncryptTest)
		var _5_valueOrError0 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
		_ = _5_valueOrError0
		_5_valueOrError0 = m_Wrappers.Companion_Default___.Need((((_4_test).Dtor_vector()).Dtor_id()).Is_Some(), _dafny.SeqOfString("Vector is missing uuid"))
		if (_5_valueOrError0).IsFailure() {
			manifest = (_5_valueOrError0).PropagateFailure()
			return manifest
		}
		if Companion_Default___.TestEncryptVector_q((_4_test).Dtor_vector()) {
			var _6_valueOrError1 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
			_ = _6_valueOrError1
			_6_valueOrError1 = m_Wrappers.Companion_Default___.Need(((((_4_test).Dtor_vector()).Dtor_algorithmSuiteId()).Is_Some()) && ((((((_4_test).Dtor_vector()).Dtor_algorithmSuiteId()).Dtor_value().(m_AwsCryptographyMaterialProvidersTypes.AlgorithmSuiteInfo)).Dtor_id()).Is_ESDK()), _dafny.SeqOfString("Vector is using an algorithm suite other than ESDK"))
			if (_6_valueOrError1).IsFailure() {
				manifest = (_6_valueOrError1).PropagateFailure()
				return manifest
			}
			var _7_valueOrError2 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_EsdkTestVectors.Companion_EncryptTestOutput_.Default())
			_ = _7_valueOrError2
			var _out0 m_Wrappers.Result
			_ = _out0
			_out0 = m_EsdkTestVectors.Companion_Default___.TestEncrypt(plaintexts, keys, _4_test)
			_7_valueOrError2 = _out0
			if (_7_valueOrError2).IsFailure() {
				manifest = (_7_valueOrError2).PropagateFailure()
				return manifest
			}
			var _8_pass m_EsdkTestVectors.EncryptTestOutput
			_ = _8_pass
			_8_pass = (_7_valueOrError2).Extract().(m_EsdkTestVectors.EncryptTestOutput)
			if !((_8_pass).Dtor_output()) {
				_0_hasFailure = true
			} else if ((_8_pass).Dtor_vector()).Is_Some() {
				_1_decryptVectors = _dafny.Companion_Sequence_.Concatenate(_1_decryptVectors, _dafny.SeqOf(((_8_pass).Dtor_vector()).Dtor_value().(m_EsdkTestVectors.EsdkDecryptTestVector)))
			}
		} else {
			_2_skipped = _dafny.Companion_Sequence_.Concatenate(_2_skipped, _dafny.SeqOf(_dafny.Companion_Sequence_.Concatenate((((_4_test).Dtor_vector()).Dtor_id()).Dtor_value().(_dafny.Sequence), _dafny.SeqOfString("\n"))))
			_dafny.Print((_dafny.SeqOfString("\nSKIP===> ")).SetString())
			_dafny.Print(((((_4_test).Dtor_vector()).Dtor_id()).Dtor_value().(_dafny.Sequence)).SetString())
			_dafny.Print((_dafny.SeqOfString("\n")).SetString())
		}
	}
	_dafny.Print((_dafny.SeqOfString("\n=================== Completed ")).SetString())
	_dafny.Print(_dafny.IntOfUint32((tests).Cardinality()))
	_dafny.Print((_dafny.SeqOfString(" Encrypt Tests =================== \n\n")).SetString())
	if !(!(_0_hasFailure)) {
		panic("dafny/TestVectors/src/EsdkTestManifests.dfy(215,4): " + (_dafny.SeqOfString("expectation violation")).String())
	}
	manifest = m_Wrappers.Companion_Result_.Create_Success_(_1_decryptVectors)
	return manifest
}
func (_static *CompanionStruct_Default___) GetManifest(manifestPath _dafny.Sequence, manifestFileName _dafny.Sequence) m_Wrappers.Result {
	var manifestData m_Wrappers.Result = m_Wrappers.Result{}
	_ = manifestData
	var _0_valueOrError0 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _0_valueOrError0
	var _out0 m_Wrappers.Result
	_ = _out0
	_out0 = m_FileIO.Companion_Default___.ReadBytesFromFile(_dafny.Companion_Sequence_.Concatenate(manifestPath, manifestFileName))
	_0_valueOrError0 = _out0
	if (_0_valueOrError0).IsFailure() {
		manifestData = (_0_valueOrError0).PropagateFailure()
		return manifestData
	}
	var _1_decryptManifestBv _dafny.Sequence
	_ = _1_decryptManifestBv
	_1_decryptManifestBv = (_0_valueOrError0).Extract().(_dafny.Sequence)
	var _2_decryptManifestBytes _dafny.Sequence
	_ = _2_decryptManifestBytes
	_2_decryptManifestBytes = m_JSONHelpers.Companion_Default___.BvToBytes(_1_decryptManifestBv)
	var _3_valueOrError1 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_JSON_Values.Companion_JSON_.Default())
	_ = _3_valueOrError1
	_3_valueOrError1 = (m_JSON_API.Companion_Default___.Deserialize(_2_decryptManifestBytes)).MapFailure(func(coer9 func(m_JSON_Errors.DeserializationError) _dafny.Sequence) func(interface{}) interface{} {
		return func(arg11 interface{}) interface{} {
			return coer9(arg11.(m_JSON_Errors.DeserializationError))
		}
	}(func(_4_e m_JSON_Errors.DeserializationError) _dafny.Sequence {
		return (_4_e).ToString()
	}))
	if (_3_valueOrError1).IsFailure() {
		manifestData = (_3_valueOrError1).PropagateFailure()
		return manifestData
	}
	var _5_manifestJson m_JSON_Values.JSON
	_ = _5_manifestJson
	_5_manifestJson = (_3_valueOrError1).Extract().(m_JSON_Values.JSON)
	var _6_valueOrError2 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _6_valueOrError2
	_6_valueOrError2 = m_Wrappers.Companion_Default___.Need((_5_manifestJson).Is_Object(), _dafny.SeqOfString("Not a JSON object"))
	if (_6_valueOrError2).IsFailure() {
		manifestData = (_6_valueOrError2).PropagateFailure()
		return manifestData
	}
	var _7_valueOrError3 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _7_valueOrError3
	_7_valueOrError3 = m_JSONHelpers.Companion_Default___.GetObject(_dafny.SeqOfString("manifest"), (_5_manifestJson).Dtor_obj())
	if (_7_valueOrError3).IsFailure() {
		manifestData = (_7_valueOrError3).PropagateFailure()
		return manifestData
	}
	var _8_manifest _dafny.Sequence
	_ = _8_manifest
	_8_manifest = (_7_valueOrError3).Extract().(_dafny.Sequence)
	var _9_valueOrError4 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.Zero)
	_ = _9_valueOrError4
	_9_valueOrError4 = m_JSONHelpers.Companion_Default___.GetNat(_dafny.SeqOfString("version"), _8_manifest)
	if (_9_valueOrError4).IsFailure() {
		manifestData = (_9_valueOrError4).PropagateFailure()
		return manifestData
	}
	var _10_version _dafny.Int
	_ = _10_version
	_10_version = (_9_valueOrError4).Extract().(_dafny.Int)
	var _11_valueOrError5 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq.SetString())
	_ = _11_valueOrError5
	_11_valueOrError5 = m_JSONHelpers.Companion_Default___.GetString(_dafny.SeqOfString("type"), _8_manifest)
	if (_11_valueOrError5).IsFailure() {
		manifestData = (_11_valueOrError5).PropagateFailure()
		return manifestData
	}
	var _12_typ _dafny.Sequence
	_ = _12_typ
	_12_typ = (_11_valueOrError5).Extract().(_dafny.Sequence)
	var _13_valueOrError6 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq.SetString())
	_ = _13_valueOrError6
	_13_valueOrError6 = m_JSONHelpers.Companion_Default___.GetString(_dafny.SeqOfString("keys"), (_5_manifestJson).Dtor_obj())
	if (_13_valueOrError6).IsFailure() {
		manifestData = (_13_valueOrError6).PropagateFailure()
		return manifestData
	}
	var _14_keyManifestUri _dafny.Sequence
	_ = _14_keyManifestUri
	_14_keyManifestUri = (_13_valueOrError6).Extract().(_dafny.Sequence)
	var _15_valueOrError7 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
	_ = _15_valueOrError7
	_15_valueOrError7 = m_Wrappers.Companion_Default___.Need(_dafny.Companion_Sequence_.IsProperPrefixOf(_dafny.SeqOfString("file://"), _14_keyManifestUri), _dafny.SeqOfString("Unexpected URI prefix"))
	if (_15_valueOrError7).IsFailure() {
		manifestData = (_15_valueOrError7).PropagateFailure()
		return manifestData
	}
	var _16_keyManifestPath _dafny.Sequence
	_ = _16_keyManifestPath
	_16_keyManifestPath = _dafny.Companion_Sequence_.Concatenate(manifestPath, (_14_keyManifestUri).Drop(7))
	var _17_valueOrError8 m_Wrappers.Result = m_Wrappers.Result{}
	_ = _17_valueOrError8
	var _out1 m_Wrappers.Result
	_ = _out1
	_out1 = m_KeyVectors.Companion_Default___.KeyVectors(m_AwsCryptographyMaterialProvidersTestVectorKeysTypes.Companion_KeyVectorsConfig_.Create_KeyVectorsConfig_(_16_keyManifestPath))
	_17_valueOrError8 = _out1
	if !(!((_17_valueOrError8).IsFailure())) {
		panic("dafny/TestVectors/src/EsdkTestManifests.dfy(263,16): " + (_17_valueOrError8).String())
	}
	var _18_keys *m_KeyVectors.KeyVectorsClient
	_ = _18_keys
	_18_keys = (_17_valueOrError8).Extract().(*m_KeyVectors.KeyVectorsClient)
	var _19_valueOrError9 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
	_ = _19_valueOrError9
	_19_valueOrError9 = m_JSONHelpers.Companion_Default___.GetObject(_dafny.SeqOfString("tests"), (_5_manifestJson).Dtor_obj())
	if (_19_valueOrError9).IsFailure() {
		manifestData = (_19_valueOrError9).PropagateFailure()
		return manifestData
	}
	var _20_jsonTests _dafny.Sequence
	_ = _20_jsonTests
	_20_jsonTests = (_19_valueOrError9).Extract().(_dafny.Sequence)
	var _source0 _dafny.Sequence = _12_typ
	_ = _source0
	{
		{
			if (_source0).Equals(_dafny.SeqOfString("awses-decrypt")) {
				var _21_valueOrError10 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
				_ = _21_valueOrError10
				_21_valueOrError10 = m_Wrappers.Companion_Default___.Need(m_EsdkTestVectors.Companion_Default___.SupportedDecryptVersion_q(_10_version), _dafny.SeqOfString("Unsupported manifest version"))
				if (_21_valueOrError10).IsFailure() {
					manifestData = (_21_valueOrError10).PropagateFailure()
					return manifestData
				}
				var _22_valueOrError11 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(m_JSON_Values.Companion_JSON_.Default())
				_ = _22_valueOrError11
				_22_valueOrError11 = m_JSONHelpers.Companion_Default___.Get(_dafny.SeqOfString("client"), (_5_manifestJson).Dtor_obj())
				if (_22_valueOrError11).IsFailure() {
					manifestData = (_22_valueOrError11).PropagateFailure()
					return manifestData
				}
				var _23_client m_JSON_Values.JSON
				_ = _23_client
				_23_client = (_22_valueOrError11).Extract().(m_JSON_Values.JSON)
				manifestData = m_Wrappers.Companion_Result_.Create_Success_(Companion_ManifestData_.Create_DecryptManifest_(_10_version, _18_keys, _23_client, _20_jsonTests))
				goto Lmatch0
			}
		}
		{
			if (_source0).Equals(_dafny.SeqOfString("awses-encrypt")) {
				var _24_valueOrError12 m_Wrappers.Outcome = m_Wrappers.Companion_Outcome_.Default()
				_ = _24_valueOrError12
				_24_valueOrError12 = m_Wrappers.Companion_Default___.Need(m_EsdkTestVectors.Companion_Default___.SupportedEncryptVersion_q(_10_version), _dafny.SeqOfString("Unsupported manifest version"))
				if (_24_valueOrError12).IsFailure() {
					manifestData = (_24_valueOrError12).PropagateFailure()
					return manifestData
				}
				var _25_valueOrError13 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
				_ = _25_valueOrError13
				_25_valueOrError13 = m_JSONHelpers.Companion_Default___.GetObject(_dafny.SeqOfString("plaintexts"), (_5_manifestJson).Dtor_obj())
				if (_25_valueOrError13).IsFailure() {
					manifestData = (_25_valueOrError13).PropagateFailure()
					return manifestData
				}
				var _26_plaintextsJson _dafny.Sequence
				_ = _26_plaintextsJson
				_26_plaintextsJson = (_25_valueOrError13).Extract().(_dafny.Sequence)
				var _27_valueOrError15 m_Wrappers.Result = m_Wrappers.Companion_Result_.Default(_dafny.EmptySeq)
				_ = _27_valueOrError15
				_27_valueOrError15 = m_Seq.Companion_Default___.MapWithResult(func(coer10 func(_dafny.Tuple) m_Wrappers.Result) func(interface{}) m_Wrappers.Result {
					return func(arg12 interface{}) m_Wrappers.Result {
						return coer10(arg12.(_dafny.Tuple))
					}
				}(func(_28_obj _dafny.Tuple) m_Wrappers.Result {
					return func(_pat_let7_0 m_Wrappers.Outcome) m_Wrappers.Result {
						return func(_29_valueOrError14 m_Wrappers.Outcome) m_Wrappers.Result {
							return (func() m_Wrappers.Result {
								if (_29_valueOrError14).IsFailure() {
									return (_29_valueOrError14).PropagateFailure()
								}
								return m_Wrappers.Companion_Result_.Create_Success_(_dafny.TupleOf((*(_28_obj).IndexInt(0)).(_dafny.Sequence), ((((*(_28_obj).IndexInt(1)).(m_JSON_Values.JSON)).Dtor_num()).Dtor_n()).Int32()))
							})()
						}(_pat_let7_0)
					}(m_Wrappers.Companion_Default___.Need((((*(_28_obj).IndexInt(1)).(m_JSON_Values.JSON)).Is_Number()) && ((((((*(_28_obj).IndexInt(1)).(m_JSON_Values.JSON)).Dtor_num()).Dtor_n()).Sign() == 1) && (((((*(_28_obj).IndexInt(1)).(m_JSON_Values.JSON)).Dtor_num()).Dtor_n()).Cmp(_dafny.IntOfInt32(m_BoundedInts.Companion_Default___.INT32__MAX())) <= 0)), _dafny.SeqOfString("Size is not a natural number.")))
				}), _26_plaintextsJson)
				if (_27_valueOrError15).IsFailure() {
					manifestData = (_27_valueOrError15).PropagateFailure()
					return manifestData
				}
				var _30_plaintextsLength _dafny.Sequence
				_ = _30_plaintextsLength
				_30_plaintextsLength = (_27_valueOrError15).Extract().(_dafny.Sequence)
				manifestData = m_Wrappers.Companion_Result_.Create_Success_(Companion_ManifestData_.Create_EncryptManifest_(_10_version, _18_keys, _30_plaintextsLength, _20_jsonTests))
				goto Lmatch0
			}
		}
		{
			manifestData = m_Wrappers.Companion_Result_.Create_Failure_(_dafny.Companion_Sequence_.Concatenate(_dafny.SeqOfString("Unsupported manifest type:"), _12_typ))
		}
		goto Lmatch0
	}
Lmatch0:
	return manifestData
}

// End of class Default__

// Definition of datatype ManifestData
type ManifestData struct {
	Data_ManifestData_
}

func (_this ManifestData) Get_() Data_ManifestData_ {
	return _this.Data_ManifestData_
}

type Data_ManifestData_ interface {
	isManifestData()
}

type CompanionStruct_ManifestData_ struct {
}

var Companion_ManifestData_ = CompanionStruct_ManifestData_{}

type ManifestData_DecryptManifest struct {
	Version   _dafny.Int
	Keys      *m_KeyVectors.KeyVectorsClient
	Client    m_JSON_Values.JSON
	JsonTests _dafny.Sequence
}

func (ManifestData_DecryptManifest) isManifestData() {}

func (CompanionStruct_ManifestData_) Create_DecryptManifest_(Version _dafny.Int, Keys *m_KeyVectors.KeyVectorsClient, Client m_JSON_Values.JSON, JsonTests _dafny.Sequence) ManifestData {
	return ManifestData{ManifestData_DecryptManifest{Version, Keys, Client, JsonTests}}
}

func (_this ManifestData) Is_DecryptManifest() bool {
	_, ok := _this.Get_().(ManifestData_DecryptManifest)
	return ok
}

type ManifestData_EncryptManifest struct {
	Version   _dafny.Int
	Keys      *m_KeyVectors.KeyVectorsClient
	Plaintext _dafny.Sequence
	JsonTests _dafny.Sequence
}

func (ManifestData_EncryptManifest) isManifestData() {}

func (CompanionStruct_ManifestData_) Create_EncryptManifest_(Version _dafny.Int, Keys *m_KeyVectors.KeyVectorsClient, Plaintext _dafny.Sequence, JsonTests _dafny.Sequence) ManifestData {
	return ManifestData{ManifestData_EncryptManifest{Version, Keys, Plaintext, JsonTests}}
}

func (_this ManifestData) Is_EncryptManifest() bool {
	_, ok := _this.Get_().(ManifestData_EncryptManifest)
	return ok
}

func (CompanionStruct_ManifestData_) Default() ManifestData {
	return Companion_ManifestData_.Create_DecryptManifest_(_dafny.Zero, (*m_KeyVectors.KeyVectorsClient)(nil), m_JSON_Values.Companion_JSON_.Default(), _dafny.EmptySeq)
}

func (_this ManifestData) Dtor_version() _dafny.Int {
	switch data := _this.Get_().(type) {
	case ManifestData_DecryptManifest:
		return data.Version
	default:
		return data.(ManifestData_EncryptManifest).Version
	}
}

func (_this ManifestData) Dtor_keys() *m_KeyVectors.KeyVectorsClient {
	switch data := _this.Get_().(type) {
	case ManifestData_DecryptManifest:
		return data.Keys
	default:
		return data.(ManifestData_EncryptManifest).Keys
	}
}

func (_this ManifestData) Dtor_client() m_JSON_Values.JSON {
	return _this.Get_().(ManifestData_DecryptManifest).Client
}

func (_this ManifestData) Dtor_jsonTests() _dafny.Sequence {
	switch data := _this.Get_().(type) {
	case ManifestData_DecryptManifest:
		return data.JsonTests
	default:
		return data.(ManifestData_EncryptManifest).JsonTests
	}
}

func (_this ManifestData) Dtor_plaintext() _dafny.Sequence {
	return _this.Get_().(ManifestData_EncryptManifest).Plaintext
}

func (_this ManifestData) String() string {
	switch data := _this.Get_().(type) {
	case nil:
		return "null"
	case ManifestData_DecryptManifest:
		{
			return "EsdkTestManifests.ManifestData.DecryptManifest" + "(" + _dafny.String(data.Version) + ", " + _dafny.String(data.Keys) + ", " + _dafny.String(data.Client) + ", " + _dafny.String(data.JsonTests) + ")"
		}
	case ManifestData_EncryptManifest:
		{
			return "EsdkTestManifests.ManifestData.EncryptManifest" + "(" + _dafny.String(data.Version) + ", " + _dafny.String(data.Keys) + ", " + _dafny.String(data.Plaintext) + ", " + _dafny.String(data.JsonTests) + ")"
		}
	default:
		{
			return "<unexpected>"
		}
	}
}

func (_this ManifestData) Equals(other ManifestData) bool {
	switch data1 := _this.Get_().(type) {
	case ManifestData_DecryptManifest:
		{
			data2, ok := other.Get_().(ManifestData_DecryptManifest)
			return ok && data1.Version.Cmp(data2.Version) == 0 && data1.Keys == data2.Keys && data1.Client.Equals(data2.Client) && data1.JsonTests.Equals(data2.JsonTests)
		}
	case ManifestData_EncryptManifest:
		{
			data2, ok := other.Get_().(ManifestData_EncryptManifest)
			return ok && data1.Version.Cmp(data2.Version) == 0 && data1.Keys == data2.Keys && data1.Plaintext.Equals(data2.Plaintext) && data1.JsonTests.Equals(data2.JsonTests)
		}
	default:
		{
			return false // unexpected
		}
	}
}

func (_this ManifestData) EqualsGeneric(other interface{}) bool {
	typed, ok := other.(ManifestData)
	return ok && _this.Equals(typed)
}

func Type_ManifestData_() _dafny.TypeDescriptor {
	return type_ManifestData_{}
}

type type_ManifestData_ struct {
}

func (_this type_ManifestData_) Default() interface{} {
	return Companion_ManifestData_.Default()
}

func (_this type_ManifestData_) String() string {
	return "EsdkTestManifests.ManifestData"
}
func (_this ManifestData) ParentTraits_() []*_dafny.TraitID {
	return [](*_dafny.TraitID){}
}

var _ _dafny.TraitOffspring = ManifestData{}

// End of datatype ManifestData
