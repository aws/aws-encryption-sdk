# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

CORES=2

include ../SharedMakefileV2.mk

DIR_STRUCTURE_V2=V2

PROJECT_SERVICES := \
	TestVectors \

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

transpile_implementation_java: _replace_main_method_name_java
transpile_implementation_net: _replace_main_method_name_net

_replace_main_method_name_java:
	$(MAKE) _sed_file SED_FILE_PATH=$(IMPLEMENTATION_FROM_DAFNY_TV_JAVA_FILE) SED_BEFORE_STRING=$(IMPLEMENTATION_FROM_DAFNY_TV_JAVA_MPL_MAIN) SED_AFTER_STRING=$(IMPLEMENTATION_FROM_DAFNY_TV_JAVA_ESDK_MAIN)

_replace_main_method_name_net:
	$(MAKE) _sed_file SED_FILE_PATH=$(IMPLEMENTATION_FROM_DAFNY_TV_NET_FILE) SED_BEFORE_STRING=$(IMPLEMENTATION_FROM_DAFNY_TV_NET_MPL_MAIN) SED_AFTER_STRING=$(IMPLEMENTATION_FROM_DAFNY_TV_NET_ESDK_MAIN)


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

test_encrypt_vectors_java:
	gradle -p runtimes/java run --args="encrypt --manifest-path . --decrypt-manifest-path ."

test_encrypt_vectors_net: FRAMEWORK=net6.0
test_encrypt_vectors_net:
	dotnet restore runtimes/net
	dotnet build runtimes/net
	dotnet run --project runtimes/net --framework $(FRAMEWORK) encrypt --manifest-path runtimes/net --decrypt-manifest-path runtimes/net

test_decrypt_encrypt_vectors_java:
	gradle -p runtimes/java run --args="decrypt --manifest-path ."

test_decrypt_encrypt_vectors_net: FRAMEWORK=net6.0
test_decrypt_encrypt_vectors_net:
	dotnet restore runtimes/net
	dotnet build runtimes/net
	dotnet run --project runtimes/net --framework $(FRAMEWORK) decrypt --manifest-path runtimes/net

_polymorph_dependencies:
	@echo "No polymorphing of dependency"

