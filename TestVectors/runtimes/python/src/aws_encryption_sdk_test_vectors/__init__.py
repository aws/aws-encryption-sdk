# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
import sys

"""
ESDK's Dafny code parses the TestVectors JSON recursively.
i.e. GetTests(Json) = (Json[0], GetTests(Json[1:]))
The legacy test vectors in TestVectors/aws-encryption-sdk-test-vectors/vectors/awses-decrypt/*.zip
have way more than the default recursion of 1000.
Legacy test vectors exceeds Python's recursion limit when parsing the JSON, which needs >1 call per test vector.
(Other Crypto Tools languages are limited by memory; Python's explicit limit on function calls is unique.)
When using this internal Crypto Tools TestVectors library, set recursion limit to 10,000.
(This value is totally arbitrary and should be increased if this isn't enough.)
"""
sys.setrecursionlimit(10000)

# Initialize generated Dafny
from .internaldafny.generated import module_

# Initialize externs
from .internaldafny import extern
