# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
CORES=2

ENABLE_EXTERN_PROCESSING=1

include ../SharedMakefileV2.mk

DIR_STRUCTURE_V2=V2

PROJECT_SERVICES := \
	AwsEncryptionSdk \

SERVICE_NAMESPACE_AwsEncryptionSdk=aws.cryptography.encryptionSdk
MAX_RESOURCE_COUNT=90000000
# Order is important
# In java they MUST be built
# in the order they depend on each other
PROJECT_DEPENDENCIES := \
	mpl/AwsCryptographyPrimitives \
	mpl/ComAmazonawsKms \
	mpl/ComAmazonawsDynamodb \
	mpl/AwsCryptographicMaterialProviders
STD_LIBRARY=mpl/StandardLibrary
SMITHY_DEPS=mpl/model

# Since we are packaging projects differently, we cannot make assumptions
# about where the files are located. 
# This is here to get unblocked but should be removed once we have migrated packages
# to the new packaging format.
PROJECT_INDEX := \
	mpl/AwsCryptographyPrimitives/src/Index.dfy \
	mpl/ComAmazonawsKms/src/Index.dfy \
	mpl/ComAmazonawsDynamodb/src/Index.dfy \
	mpl/AwsCryptographicMaterialProviders/dafny/AwsCryptographicMaterialProviders/src/Index.dfy \
	mpl/AwsCryptographicMaterialProviders/dafny/AwsCryptographyKeyStore/src/Index.dfy \

SERVICE_DEPS_AwsEncryptionSdk := \
	mpl/AwsCryptographyPrimitives \
	mpl/ComAmazonawsKms \
	mpl/ComAmazonawsDynamodb \
	mpl/AwsCryptographicMaterialProviders/dafny/AwsCryptographicMaterialProviders \
	mpl/AwsCryptographicMaterialProviders/dafny/AwsCryptographyKeyStore

# Constants for languages that drop extern names (Python, Go)
TYPES_FILE_PATH=dafny/AwsEncryptionSdk/Model/AwsCryptographyEncryptionSdkTypes.dfy
TYPES_FILE_WITH_EXTERN_STRING="module {:extern \"software.amazon.cryptography.encryptionsdk.internaldafny.types\" } AwsCryptographyEncryptionSdkTypes"
TYPES_FILE_WITHOUT_EXTERN_STRING="module AwsCryptographyEncryptionSdkTypes"

INDEX_FILE_PATH=dafny/AwsEncryptionSdk/src/Index.dfy
INDEX_FILE_WITH_EXTERN_STRING="module {:extern \"software.amazon.cryptography.encryptionsdk.internaldafny\" } EncryptionSdk refines AbstractAwsCryptographyEncryptionSdkService {"
INDEX_FILE_WITHOUT_EXTERN_STRING="module EncryptionSdk refines AbstractAwsCryptographyEncryptionSdkService {"

PYTHON_MODULE_NAME=aws_encryption_sdk

TRANSLATION_RECORD_PYTHON := \
    --translation-record ../mpl/StandardLibrary/runtimes/python/src/smithy_dafny_standard_library/internaldafny/generated/dafny_src-py.dtr \
    --translation-record ../mpl/ComAmazonawsKms/runtimes/python/src/aws_cryptography_internal_kms/internaldafny/generated/dafny_src-py.dtr \
    --translation-record ../mpl/ComAmazonawsDynamodb/runtimes/python/src/aws_cryptography_internal_dynamodb/internaldafny/generated/dafny_src-py.dtr \
    --translation-record ../mpl/AwsCryptographyPrimitives/runtimes/python/src/aws_cryptography_primitives/internaldafny/generated/dafny_src-py.dtr \
		--translation-record ../mpl/AwsCryptographicMaterialProviders/runtimes/python/src/aws_cryptographic_material_providers/internaldafny/generated/dafny_src-py.dtr


PYTHON_DEPENDENCY_MODULE_NAMES := \
	--dependency-library-name=aws.cryptography.primitives=aws_cryptography_primitives \
	--dependency-library-name=com.amazonaws.kms=aws_cryptography_internal_kms \
	--dependency-library-name=com.amazonaws.dynamodb=aws_cryptography_internal_dynamodb \
	--dependency-library-name=aws.cryptography.materialProviders=aws_cryptographic_material_providers \
	--dependency-library-name=aws.cryptography.keyStore=aws_cryptographic_material_providers \
	--dependency-library-name=smithy.api=aws_cryptographic_material_providers
