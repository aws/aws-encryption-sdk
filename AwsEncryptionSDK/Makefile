# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
CORES=2
ENABLE_EXTERN_PROCESSING=1

TRANSPILE_TESTS_IN_RUST := 1

include ../SharedMakefileV2.mk

DIR_STRUCTURE_V2=V2

PROJECT_SERVICES := \
	AwsEncryptionSdk \

MAIN_SERVICE_FOR_RUST := AwsEncryptionSdk

RUST_OTHER_FILES := \
	runtimes/rust/src/aes_gcm.rs \
	runtimes/rust/src/aes_kdf_ctr.rs \
	runtimes/rust/src/ddb.rs \
	runtimes/rust/src/concurrent_call.rs \
	runtimes/rust/src/dafny_libraries.rs \
	runtimes/rust/src/digest.rs \
	runtimes/rust/src/ecdh.rs \
	runtimes/rust/src/ecdsa.rs \
	runtimes/rust/src/hmac.rs \
	runtimes/rust/src/kms.rs \
	runtimes/rust/src/local_cmc.rs \
	runtimes/rust/src/oslang.rs \
	runtimes/rust/src/random.rs \
	runtimes/rust/src/rsa.rs \
	runtimes/rust/src/sets.rs \
	runtimes/rust/src/software_externs.rs \
	runtimes/rust/src/storm_tracker.rs \
	runtimes/rust/src/time.rs \
	runtimes/rust/src/uuid.rs

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

GO_MODULE_NAME="github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk"

GO_DEPENDENCY_MODULE_NAMES := \
	--dependency-library-name=com.amazonaws.dynamodb=github.com/aws/aws-cryptographic-material-providers-library/releases/go/dynamodb \
	--dependency-library-name=com.amazonaws.kms=github.com/aws/aws-cryptographic-material-providers-library/releases/go/kms \
	--dependency-library-name=aws.cryptography.keyStore=github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl \
	--dependency-library-name=aws.cryptography.primitives=github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives \
	--dependency-library-name=aws.cryptography.materialProviders=github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl \
    	--dependency-library-name=sdk.com.amazonaws.dynamodb=github.com/aws/aws-sdk-go-v2/service/dynamodb \
    	--dependency-library-name=sdk.com.amazonaws.kms=github.com/aws/aws-sdk-go-v2/service/kms


TRANSLATION_RECORD_GO := \
    mpl/StandardLibrary/runtimes/go/ImplementationFromDafny-go/ImplementationFromDafny-go.dtr \
    mpl/ComAmazonawsKms/runtimes/go/ImplementationFromDafny-go/ImplementationFromDafny-go.dtr \
    mpl/ComAmazonawsDynamodb/runtimes/go/ImplementationFromDafny-go/ImplementationFromDafny-go.dtr \
    mpl/AwsCryptographyPrimitives/runtimes/go/ImplementationFromDafny-go/ImplementationFromDafny-go.dtr \
    mpl/AwsCryptographicMaterialProviders/runtimes/go/ImplementationFromDafny-go/ImplementationFromDafny-go.dtr

TYPES_FILE_PATH=dafny/AwsEncryptionSdk/Model/AwsCryptographyEncryptionSdkTypes.dfy
TYPES_FILE_WITH_EXTERN_STRING="module {:extern \"software.amazon.cryptography.encryptionsdk.internaldafny.types\" } AwsCryptographyEncryptionSdkTypes"
TYPES_FILE_WITHOUT_EXTERN_STRING="module AwsCryptographyEncryptionSdkTypes"

INDEX_FILE_PATH=dafny/AwsEncryptionSdk/src/Index.dfy
INDEX_FILE_WITH_EXTERN_STRING="module {:extern \"software.amazon.cryptography.encryptionsdk.internaldafny\" } ESDK refines AbstractAwsCryptographyEncryptionSdkService {"
INDEX_FILE_WITHOUT_EXTERN_STRING="module ESDK refines AbstractAwsCryptographyEncryptionSdkService {"

# Target to restore all directories in a list
# TODO: remove this once we don't copy all of the directories into implementation and test. This is done by make file target _mv_polymorph_go in smithy-dafny.
RESTORE_DIRS := examples
_polymorph_go: restore_directories
restore_directories:
	@for dir in $(RESTORE_DIRS); do \
		if [ -d "runtimes/go/ImplementationFromDafny-go/$$dir" ]; then \
			cp -Rf runtimes/go/ImplementationFromDafny-go/$$dir runtimes/go/; \
			rm -rf runtimes/go/ImplementationFromDafny-go/$$dir; \
			rm -rf runtimes/go/TestsFromDafny-go/$$dir; \
		else \
			echo "Directory $$dir not found"; \
		fi \
	done

# Go codegen (smithy-dafny) does not yet support handlePatching unlike
# Java, .NET, and Rust. Apply Go patches manually after codegen runs.
# TODO: Remove this once handlePatching is added to generateGo() in
# smithy-dafny CodegenEngine.java.
_polymorph_go: apply_go_patches
apply_go_patches:
	@if [ -d "codegen-patches/AwsEncryptionSdk/go" ]; then \
		for patch in codegen-patches/AwsEncryptionSdk/go/*.patch; do \
			if [ -f "$$patch" ]; then \
				echo "Applying Go patch: $$patch"; \
				git apply -v "$$patch"; \
			fi \
		done \
	fi

PYTHON_MODULE_NAME=aws_encryption_sdk_dafny

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
