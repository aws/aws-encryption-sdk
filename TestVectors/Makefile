# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

CORES=2
ENABLE_EXTERN_PROCESSING=1
TRANSPILE_TESTS_IN_RUST=1

include ../SharedMakefileV2.mk

DIR_STRUCTURE_V2=V2

PROJECT_SERVICES := \
	TestVectors \

MAIN_SERVICE_FOR_RUST := TestVectors

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
	runtimes/rust/src/uuid.rs \
	runtimes/rust/src/test_vec_dir.rs

SMITHY_MODEL_ROOT := $(PROJECT_ROOT)/AwsEncryptionSDK/dafny/AwsEncryptionSdk/Model
OUTPUT_LOCAL_SERVICE_TestVectors:= --local-service-test

SERVICE_NAMESPACE_TestVectors=aws.cryptography.encryptionSdk

MAX_RESOURCE_COUNT=10000000
# Order is important
# In java they MUST be built
# in the order they depend on each other
PROJECT_DEPENDENCIES := \
	mpl/AwsCryptographyPrimitives \
	mpl/ComAmazonawsKms \
	mpl/ComAmazonawsDynamodb \
	mpl/AwsCryptographicMaterialProviders \
	mpl/TestVectorsAwsCryptographicMaterialProviders \
	AwsEncryptionSDK \

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
	mpl/TestVectorsAwsCryptographicMaterialProviders/dafny/KeyVectors/src/Index.dfy \
	mpl/TestVectorsAwsCryptographicMaterialProviders/dafny/TestVectorsAwsCryptographicMaterialProviders/src/LibraryIndex.dfy \
	AwsEncryptionSDK/dafny/AwsEncryptionSdk/src/Index.dfy \

STD_LIBRARY=mpl/StandardLibrary
SMITHY_DEPS=mpl/model


# Dependencies for each local service
SERVICE_DEPS_TestVectors := \
	mpl/AwsCryptographyPrimitives \
	mpl/ComAmazonawsKms \
	mpl/ComAmazonawsDynamodb \
	mpl/AwsCryptographicMaterialProviders/dafny/AwsCryptographicMaterialProviders \
	mpl/AwsCryptographicMaterialProviders/dafny/AwsCryptographyKeyStore \
	AwsEncryptionSDK/dafny/AwsEncryptionSdk \
	mpl/TestVectorsAwsCryptographicMaterialProviders/dafny/TestVectorsAwsCryptographicMaterialProviders \


format_net:
	pushd runtimes/net && dotnet format && popd

IMPLEMENTATION_FROM_DAFNY_TV_JAVA_FILE=runtimes/java/src/main/dafny-generated/ImplementationFromDafny.java
IMPLEMENTATION_FROM_DAFNY_TV_JAVA_MPL_MAIN="dafny.Helpers.withHaltHandling(() -> { WrappedMaterialProvidersMain_Compile.__default.__Main(dafny.Helpers.FromMainArguments(args)); } );"
IMPLEMENTATION_FROM_DAFNY_TV_JAVA_ESDK_MAIN="dafny.Helpers.withHaltHandling(() -> { WrappedESDKMain_Compile.__default.Main2(dafny.Helpers.FromMainArguments(args)); } );"

IMPLEMENTATION_FROM_DAFNY_TV_NET_FILE=runtimes/net/ImplementationFromDafny.cs
IMPLEMENTATION_FROM_DAFNY_TV_NET_MPL_MAIN="Dafny.Helpers.WithHaltHandling(() => WrappedMaterialProvidersMain_Compile.__default._Main(Dafny.Sequence<Dafny.ISequence<char>>.FromMainArguments(args)));"
IMPLEMENTATION_FROM_DAFNY_TV_NET_ESDK_MAIN="Dafny.Helpers.WithHaltHandling(() => WrappedESDKMain_Compile.__default.Main2(Dafny.Sequence<Dafny.ISequence<char>>.FromMainArguments(args)));"

IMPLEMENTATION_FROM_DAFNY_TV_GO_FILE=runtimes/go/ImplementationFromDafny-go/ImplementationFromDafny.go
IMPLEMENTATION_FROM_DAFNY_TV_GO_MPL_MAIN="m_WrappedMaterialProvidersMain.Companion_Default___.Main(_dafny.FromMainArguments(os.Args))"
IMPLEMENTATION_FROM_DAFNY_TV_GO_ESDK_MAIN="m_WrappedESDKMain.Companion_Default___.Main2(_dafny.FromMainArguments(os.Args))"

# TODO: Remove after wrapped client issue is fixed in Rust
REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_PRIMITIVES=runtimes/rust/src/deps/aws_cryptography_primitives.rs
REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_KEYSTORE=runtimes/rust/src/deps/aws_cryptography_keyStore.rs
REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_FROM_1 = "\#\[cfg(feature = \"wrapped-client\")\]"
REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_FROM_2 := 'pub mod wrapped;'
REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_TO_1 := '\/\/ removed wrapped-client feature using sed;'
REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_TO_2 := '\/\/ removed wrapped module using sed;'

# Go

GO_MODULE_NAME="github.com/aws/aws-encryption-sdk/testvectors"

GO_DEPENDENCY_MODULE_NAMES := \
	--dependency-library-name=aws.cryptography.encryptionSdk=github.com/aws/aws-encryption-sdk/releases/go/encryption-sdk \
	--dependency-library-name=com.amazonaws.dynamodb=github.com/aws/aws-cryptographic-material-providers-library/releases/go/dynamodb \
	--dependency-library-name=com.amazonaws.kms=github.com/aws/aws-cryptographic-material-providers-library/releases/go/kms \
	--dependency-library-name=aws.cryptography.keyStore=github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl \
	--dependency-library-name=aws.cryptography.primitives=github.com/aws/aws-cryptographic-material-providers-library/releases/go/primitives \
	--dependency-library-name=aws.cryptography.materialProviders=github.com/aws/aws-cryptographic-material-providers-library/releases/go/mpl \
	--dependency-library-name=sdk.com.amazonaws.dynamodb=github.com/aws/aws-sdk-go-v2/service/dynamodb \
	--dependency-library-name=sdk.com.amazonaws.kms=github.com/aws/aws-sdk-go-v2/service/kms

TRANSLATION_RECORD_GO := \
	AwsEncryptionSDK/runtimes/go/ImplementationFromDafny-go/ImplementationFromDafny-go.dtr \
	mpl/StandardLibrary/runtimes/go/ImplementationFromDafny-go/ImplementationFromDafny-go.dtr \
	mpl/ComAmazonawsKms/runtimes/go/ImplementationFromDafny-go/ImplementationFromDafny-go.dtr \
	mpl/ComAmazonawsDynamodb/runtimes/go/ImplementationFromDafny-go/ImplementationFromDafny-go.dtr \
	mpl/AwsCryptographyPrimitives/runtimes/go/ImplementationFromDafny-go/ImplementationFromDafny-go.dtr \
	mpl/AwsCryptographicMaterialProviders/runtimes/go/ImplementationFromDafny-go/ImplementationFromDafny-go.dtr \
	mpl/TestVectorsAwsCryptographicMaterialProviders/runtimes/go/ImplementationFromDafny-go/ImplementationFromDafny-go.dtr

# Constants for languages that drop extern names (Go)

WRAPPED_INDEX_FILE_PATH=dafny/TestVectors/src/LibraryIndex.dfy
WRAPPED_INDEX_FILE_WITH_EXTERN_STRING="module {:extern \"software.amazon.cryptography.encryptionsdk.internaldafny.wrapped\" } WrappedESDK refines WrappedAbstractAwsCryptographyEncryptionSdkService"
WRAPPED_INDEX_FILE_WITHOUT_EXTERN_STRING="module WrappedESDK refines WrappedAbstractAwsCryptographyEncryptionSdkService"

transpile_implementation_java: _replace_main_method_name_java
transpile_implementation_net: _replace_main_method_name_net
transpile_implementation_go: _replace_main_method_name_go

_polymorph_go: purge_polymorph_code

# Smithy-dafny generated shim needs a long term fix. 
# TODO: Remove this commands once smithy-dafny is fixed
# This commands does not work on windows
# https://taskei.amazon.dev/tasks/CrypTool-5283
purge_polymorph_code:
	find .. -name "shim.go" | xargs sed -i $(SED_PARAMETER) 's/(_static \*CompanionStruct_Default___)//g'
	rm -rf runtimes/go/ImplementationFromDafny-go/awscryptographyencryptionsdksmithygenerated \
		runtimes/go/ImplementationFromDafny-go/awscryptographyencryptionsdksmithygeneratedtypes \
		runtimes/go/ImplementationFromDafny-go/WrappedAwsCryptographyEncryptionSdkService \
		runtimes/go/TestsFromDafny-go/awscryptographyencryptionsdksmithygenerated \
		runtimes/go/TestsFromDafny-go/awscryptographyencryptionsdksmithygeneratedtypes \
		runtimes/go/TestsFromDafny-go/WrappedAwsCryptographyEncryptionSdkService

# TODO: Remove after wrapped client issue is fixed in Rust
_polymorph_rust: _remove_wrapped_client_rust

_replace_main_method_name_java:
	$(MAKE) _sed_file SED_FILE_PATH=$(IMPLEMENTATION_FROM_DAFNY_TV_JAVA_FILE) SED_BEFORE_STRING=$(IMPLEMENTATION_FROM_DAFNY_TV_JAVA_MPL_MAIN) SED_AFTER_STRING=$(IMPLEMENTATION_FROM_DAFNY_TV_JAVA_ESDK_MAIN)

_replace_main_method_name_net:
	$(MAKE) _sed_file SED_FILE_PATH=$(IMPLEMENTATION_FROM_DAFNY_TV_NET_FILE) SED_BEFORE_STRING=$(IMPLEMENTATION_FROM_DAFNY_TV_NET_MPL_MAIN) SED_AFTER_STRING=$(IMPLEMENTATION_FROM_DAFNY_TV_NET_ESDK_MAIN)

_replace_main_method_name_go:
	$(MAKE) _sed_file SED_FILE_PATH=$(IMPLEMENTATION_FROM_DAFNY_TV_GO_FILE) SED_BEFORE_STRING=$(IMPLEMENTATION_FROM_DAFNY_TV_GO_MPL_MAIN) SED_AFTER_STRING=$(IMPLEMENTATION_FROM_DAFNY_TV_GO_ESDK_MAIN)

# TODO: Remove after wrapped client issue is fixed in Rust
_remove_wrapped_client_rust:
	$(MAKE) _sed_file SED_FILE_PATH=$(REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_PRIMITIVES) SED_BEFORE_STRING=$(REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_FROM_1) SED_AFTER_STRING=$(REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_TO_1)
	$(MAKE) _sed_file SED_FILE_PATH=$(REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_PRIMITIVES) SED_BEFORE_STRING=$(REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_FROM_2) SED_AFTER_STRING=$(REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_TO_2)
	$(MAKE) _sed_file SED_FILE_PATH=$(REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_KEYSTORE) SED_BEFORE_STRING=$(REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_FROM_1) SED_AFTER_STRING=$(REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_TO_1)
	$(MAKE) _sed_file SED_FILE_PATH=$(REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_KEYSTORE) SED_BEFORE_STRING=$(REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_FROM_2) SED_AFTER_STRING=$(REMOVE_WRAPPED_CLIENT_AFTER_POLYMORPH_RUST_TO_2)

# Commands to generate and test test vectors
test_generate_vectors_java:
	gradle -p runtimes/java run --args="encrypt-manifest --encrypt-manifest-output ."
	gradle -p runtimes/java copyKeysJSONCurr

test_generate_vectors_net: FRAMEWORK=net6.0
test_generate_vectors_net:
	dotnet restore runtimes/net
	dotnet build runtimes/net
	dotnet run --project runtimes/net --framework $(FRAMEWORK) encrypt-manifest --encrypt-manifest-output runtimes/net
	cp dafny/TestVectors/test/keys.json runtimes/net

test_generate_vectors_rust:
	cd runtimes/rust && \
	cargo run --bin test-vectors --features="wrapped-client" --release -- encrypt-manifest --encrypt-manifest-output . && \
	cd ../../
	cp dafny/TestVectors/test/keys.json runtimes/rust/

test_generate_vectors_go:
	go -C runtimes/go/ImplementationFromDafny-go run ImplementationFromDafny.go encrypt-manifest --encrypt-manifest-output ../
	cp dafny/TestVectors/test/keys.json runtimes/go

test_generate_vectors_python:
	rm -rf runtimes/python/.tox \
	python3 -m tox -c runtimes/python --verbose -e cli -- encrypt-manifest --encrypt-manifest-output runtimes/python \
	cp dafny/TestVectors/test/keys.json runtimes/python

test_encrypt_vectors_java:
	gradle -p runtimes/java run --args="encrypt --manifest-path . --decrypt-manifest-path ."

test_encrypt_vectors_net: FRAMEWORK=net6.0
test_encrypt_vectors_net:
	dotnet restore runtimes/net
	dotnet build runtimes/net
	dotnet run --project runtimes/net --framework $(FRAMEWORK) encrypt --manifest-path runtimes/net --decrypt-manifest-path runtimes/net

test_encrypt_vectors_rust:
	cd runtimes/rust && \
	cargo run --bin test-vectors --features="wrapped-client" --release -- encrypt --manifest-path . --decrypt-manifest-path . && \
	cd ../../

test_encrypt_vectors_go:
	go -C runtimes/go/ImplementationFromDafny-go run ImplementationFromDafny.go encrypt --manifest-path=.. --decrypt-manifest-path=..

test_encrypt_vectors_python:
	rm -rf runtimes/python/.tox \
  python3 -m tox -c runtimes/python --verbose -e cli -- encrypt --manifest-path runtimes/python --decrypt-manifest-path runtimes/python

test_decrypt_encrypt_vectors_java:
	gradle -p runtimes/java run --args="decrypt --manifest-path . --manifest-name decrypt-manifest.json"

test_decrypt_encrypt_vectors_net: FRAMEWORK=net6.0
test_decrypt_encrypt_vectors_net:
	dotnet restore runtimes/net
	dotnet build runtimes/net
	dotnet run --project runtimes/net --framework $(FRAMEWORK) decrypt --manifest-path runtimes/net --manifest-name decrypt-manifest.json

test_decrypt_encrypt_vectors_net_legacy: FRAMEWORK=net6.0
test_decrypt_encrypt_vectors_net_legacy:
	dotnet restore runtimes/net
	dotnet build runtimes/net
	dotnet run --project runtimes/net --framework $(FRAMEWORK) decrypt --manifest-path $(MANIFEST_PATH) --manifest-name $(MANIFEST_NAME)

test_decrypt_encrypt_vectors_rust:
	cd runtimes/rust && \
	cargo run --bin test-vectors --features="wrapped-client" --release -- decrypt --manifest-path . --manifest-name decrypt-manifest.json && \
	cd ../../

test_decrypt_encrypt_vectors_go:
	go -C runtimes/go/ImplementationFromDafny-go run ImplementationFromDafny.go decrypt --manifest-path=.. --manifest-name=decrypt-manifest.json

test_decrypt_encrypt_vectors_python:
	rm -rf runtimes/python/.tox \
  python3 -m tox -c runtimes/python --verbose -e cli -- decrypt --manifest-path runtimes/python --manifest-name decrypt-manifest.json

_polymorph_dependencies:
	@echo "No polymorphing of dependency"

_sed_types_file_remove_extern:
	@echo "No extern to process for ESDK TestVectors"

_sed_index_file_remove_extern:
	@echo "No extern to process for ESDK TestVectors"

_sed_types_file_add_extern:
	@echo "No extern to process for ESDK TestVectors"

_sed_index_file_add_extern:
	@echo "No extern to process for ESDK TestVectors"

PYTHON_MODULE_NAME=aws_encryption_sdk_test_vectors

TRANSLATION_RECORD_PYTHON := \
		--translation-record ../mpl/StandardLibrary/runtimes/python/src/smithy_dafny_standard_library/internaldafny/generated/dafny_src-py.dtr \
    --translation-record ../mpl/ComAmazonawsKms/runtimes/python/src/aws_cryptography_internal_kms/internaldafny/generated/dafny_src-py.dtr \
    --translation-record ../mpl/ComAmazonawsDynamodb/runtimes/python/src/aws_cryptography_internal_dynamodb/internaldafny/generated/dafny_src-py.dtr \
    --translation-record ../mpl/AwsCryptographyPrimitives/runtimes/python/src/aws_cryptography_primitives/internaldafny/generated/dafny_src-py.dtr \
        --translation-record ../mpl/AwsCryptographicMaterialProviders/runtimes/python/src/aws_cryptographic_material_providers/internaldafny/generated/dafny_src-py.dtr \
        --translation-record ../mpl/TestVectorsAwsCryptographicMaterialProviders/runtimes/python/src/aws_cryptography_materialproviders_test_vectors/internaldafny/generated/dafny_src-py.dtr \
        --translation-record ../AwsEncryptionSDK/runtimes/python/src/aws_encryption_sdk/internaldafny/generated/dafny_src-py.dtr

PYTHON_DEPENDENCY_MODULE_NAMES := \
        --dependency-library-name=aws.cryptography.primitives=aws_cryptography_primitives \
        --dependency-library-name=com.amazonaws.kms=aws_cryptography_internal_kms \
        --dependency-library-name=com.amazonaws.dynamodb=aws_cryptography_internal_dynamodb \
        --dependency-library-name=aws.cryptography.materialProviders=aws_cryptographic_material_providers \
        --dependency-library-name=aws.cryptography.keyStore=aws_cryptographic_material_providers \
        --dependency-library-name=smithy.api=aws_cryptographic_material_providers \
        --dependency-library-name=aws.cryptography.materialProvidersTestVectorKeys=aws_cryptographic_material_providers_test_vectors \
        --dependency-library-name=aws.cryptography.encryptionSdk=aws_encryption_sdk
