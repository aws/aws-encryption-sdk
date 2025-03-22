# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

# ESDK TestVectors have 2 main methods; one in ESDK TestVectors, another in MPL TestVectors.
# This isn't really supported, and results in running the MPL TestVectors' main method.
# Other languages use sed on Dafny-generated code to replace the generated call to the main method.
# It's (arguably) less hacky to override the function that is called in Python.
import aws_encryption_sdk_test_vectors.internaldafny.generated.module_ as module_
import aws_encryption_sdk_test_vectors.internaldafny.generated.WrappedESDKMain as WrappedESDKMain


def new_test_main(args):
    WrappedESDKMain.default__.Main2(args)


module_.default__.Test____Main____ = new_test_main
