# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

CORES=2
ENABLE_EXTERN_PROCESSING=1

include ../SharedMakefileV2.mk

DIR_STRUCTURE_V2=V2

PROJECT_SERVICES := \
	ESDKRoundTrip \
	Keyrings \

SERVICE_NAMESPACE_ESDKRoundTrip=aws.cryptography.encryptionSdk.test.roundtrip
SERVICE_NAMESPACE_Keyrings=aws.cryptography.materialProviders.testvectors.keyrings.rawaes

MAX_RESOURCE_COUNT=500000000
LIBRARIES :=

STD_LIBRARY=mpl/StandardLibrary
SMITHY_DEPS=mpl/model

PROJECT_INDEX := \
	mpl/AwsCryptographyPrimitives/src/Index.dfy \
	mpl/ComAmazonawsKms/src/Index.dfy \
	mpl/ComAmazonawsDynamodb/src/Index.dfy \
	mpl/AwsCryptographicMaterialProviders/dafny/AwsCryptographicMaterialProviders/src/Index.dfy \
	mpl/AwsCryptographicMaterialProviders/dafny/AwsCryptographyKeyStore/src/Index.dfy \
	AwsEncryptionSDK/dafny/AwsEncryptionSdk/src/Index.dfy \

PROJECT_DEPENDENCIES := \
	mpl/AwsCryptographyPrimitives \
	mpl/ComAmazonawsKms \
	mpl/ComAmazonawsDynamodb \
	mpl/AwsCryptographicMaterialProviders \
	AwsEncryptionSDK \

SERVICE_DEPS_ESDKRoundTrip := \
	mpl/AwsCryptographyPrimitives \
	mpl/ComAmazonawsKms \
	mpl/ComAmazonawsDynamodb \
	mpl/AwsCryptographicMaterialProviders/dafny/AwsCryptographicMaterialProviders \
	mpl/AwsCryptographicMaterialProviders/dafny/AwsCryptographyKeyStore \
	AwsEncryptionSDK/dafny/AwsEncryptionSdk

SERVICE_DEPS_Keyrings := \
	mpl/AwsCryptographyPrimitives \
	mpl/ComAmazonawsKms \
	mpl/ComAmazonawsDynamodb \
	mpl/AwsCryptographicMaterialProviders/dafny/AwsCryptographicMaterialProviders \
	mpl/AwsCryptographicMaterialProviders/dafny/AwsCryptographyKeyStore \

# Python

PYTHON_MODULE_NAME=test_vectors_v2

TRANSLATION_RECORD_PYTHON := \
    --translation-record ../mpl/StandardLibrary/runtimes/python/src/smithy_dafny_standard_library/internaldafny/generated/dafny_src-py.dtr \
    --translation-record ../mpl/ComAmazonawsKms/runtimes/python/src/aws_cryptography_internal_kms/internaldafny/generated/dafny_src-py.dtr \
    --translation-record ../mpl/ComAmazonawsDynamodb/runtimes/python/src/aws_cryptography_internal_dynamodb/internaldafny/generated/dafny_src-py.dtr \
    --translation-record ../mpl/AwsCryptographyPrimitives/runtimes/python/src/aws_cryptography_primitives/internaldafny/generated/dafny_src-py.dtr \
	--translation-record ../mpl/AwsCryptographicMaterialProviders/runtimes/python/src/aws_cryptographic_material_providers/internaldafny/generated/dafny_src-py.dtr \
	--translation-record ../AwsEncryptionSDK/runtimes/python/src/aws_encryption_sdk_dafny/internaldafny/generated/dafny_src-py.dtr

PYTHON_DEPENDENCY_MODULE_NAMES := \
	--dependency-library-name=aws.cryptography.primitives=aws_cryptography_primitives \
	--dependency-library-name=com.amazonaws.kms=aws_cryptography_internal_kms \
	--dependency-library-name=com.amazonaws.dynamodb=aws_cryptography_internal_dynamodb \
	--dependency-library-name=aws.cryptography.materialProviders=aws_cryptographic_material_providers \
	--dependency-library-name=aws.cryptography.keyStore=aws_cryptographic_material_providers \
	--dependency-library-name=smithy.api=aws_cryptographic_material_providers \
	--dependency-library-name=aws.cryptography.encryptionSdk=aws_encryption_sdk_dafny

# Override extern processing to not attempt to do extern processing for this lib; only deps
_sed_types_file_remove_extern:
	@echo "Skipping sed for types file extern processing in TestVectorsV2"

_sed_index_file_remove_extern:
	@echo "Skipping sed for index file extern processing in TestVectorsV2"

_sed_types_file_add_extern:
	@echo "Skipping sed for types file extern processing in TestVectorsV2"

_sed_index_file_add_extern:
	@echo "Skipping sed for index file extern processing in TestVectorsV2"