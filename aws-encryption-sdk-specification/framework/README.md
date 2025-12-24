[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Material Providers Library

## Overview

This repository contains the Material Providers Library Specification.
The primary goal of this specification is to define a standard,
language independent, description of the Material Providers Library features.  
It serves as the source of truth for the features that make up the Material Providers Library
and the details of their behavior.

It is intended to promote consistency and interoperability
across implementations of the Material Providers Library.
It also is intended to promote consistency, interoperability, and ease of configuration
across AWS Cryptography Crypto Tools libraries.
By offering a consistency configuration story
this simplifies our documentation
and makes it easier for our customers to use AWS Cryptography Crypto Tools libraries
to actually encrypt/decrypt.

This GitHub project is also intended to track issues and feature requests,
and to collect feedback pertaining to the Material Providers Library.
As well as potential needs for other AWS Cryptography Crypto Tools libraries.

[Security issue notifications](./CONTRIBUTING.md#security-issue-notifications)

## License Summary

The documentation is made available under the Creative Commons Attribution-ShareAlike 4.0 International License.
See the LICENSE file.

The sample code within this documentation is made available under the MIT-0 license.
See the LICENSE-SAMPLECODE file.

## Editing

We use `prettier` to maintain consistent formatting.
Our CI will stop PRs that do not match our formatting requirements,
but to easily apply them,
run `../ci/prettify.sh write`.
If you want to check them without writing,
run `../ci/prettify.sh check`.
