# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

# This evaluates to the local path _of this file_.
# This means that these are the project roots
# that are shared by all libraries in this repo.
PROJECT_ROOT := $(abspath $(dir $(abspath $(lastword $(MAKEFILE_LIST)))))

SMITHY_DAFNY_ROOT := $(PROJECT_ROOT)/mpl/smithy-dafny
GRADLEW := ./runtimes/java/gradlew

VERIFY_TIMEOUT := 150
PYTHON_MODULE_NAME=aws_encryption_sdk_python_dafny

TRANSLATION_RECORD_PYTHON := \
    --translation-record mpl/StandardLibrary/runtimes/python/src/smithy_dafny_standard_library/internaldafny/generated/dafny_src-py.dtr

include $(SMITHY_DAFNY_ROOT)/SmithyDafnyMakefile.mk

verify:DAFNY_OPTIONS=--allow-warnings --allow-external-contracts
verify_single:DAFNY_OPTIONS=--allow-warnings --allow-external-contracts
verify_service:DAFNY_OPTIONS=--allow-warnings --allow-external-contracts

transpile_implementation_net: DAFNY_OPTIONS=--allow-warnings --compile-suffix --legacy-module-names --allow-external-contracts
transpile_test_net: DAFNY_OPTIONS=--allow-warnings --include-test-runner --compile-suffix --legacy-module-names --allow-external-contracts

transpile_implementation_java: DAFNY_OPTIONS=--allow-warnings --compile-suffix --legacy-data-constructors --legacy-module-names --allow-external-contracts
transpile_test_java: DAFNY_OPTIONS=--allow-warnings --include-test-runner --compile-suffix --legacy-data-constructors --legacy-module-names --allow-external-contracts
