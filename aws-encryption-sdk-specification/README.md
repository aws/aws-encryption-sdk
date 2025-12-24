[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# AWS Encryption SDK Specification

## Overview

This repository contains the AWS Encryption SDK Specification.  
The primary goal of this specification is to define a standard,
language independent, description of the AWS Encryption SDK features.  
It serves as the source of truth for the features that make up the AWS Encryption SDK
and the details of their behavior.
It is intended to promote consistency and interoperability
across implementations of the AWS Encryption SDK.  
This GitHub project is also intended to track issues and feature requests,
and to collect feedback pertaining to the AWS Encryption SDK.

[Security issue notifications](./CONTRIBUTING.md#security-issue-notifications)

### Current Implementations

Below is the list of current implementation of this specification:

- [C](https://github.com/aws/aws-encryption-sdk-c)
- [Java](https://github.com/aws/aws-encryption-sdk-java)
- [Python](https://github.com/aws/aws-encryption-sdk-python)
- [CLI](https://github.com/aws/aws-encryption-sdk-cli)
- [Javascript](https://github.com/awslabs/aws-encryption-sdk-javascript)

## License Summary

The documentation is made available under the Creative Commons Attribution-ShareAlike 4.0 International License. See the LICENSE file.

The sample code within this documentation is made available under the MIT-0 license. See the LICENSE-SAMPLECODE file.

## Editing

We use `prettier` to maintain consistent formatting.
Our CI will stop PRs that do not match our formatting requirements,
but to easily apply them,
run `./ci/prettify.sh write`.
If you want to check them without writing,
run `./ci/prettify.sh check`.

## Generate Duvet Reports

[Duvet](https://github.com/awslabs/aws-encryption-sdk-specification/issues/240) is a tool that can be used to ensure specification is documented alongside code.

This repo contains helpful scripts for installing and using Duvet with this specification.

To install Duvet:

```
./util/install-duvet
```

To generate a report on what portions of this specification are covered in code, use the `report` script.
For example:

```
./util/report.js 'src/main/**/*.java' 'src/test/**/*.java'
```

## Extract `compliance` from Specification

The Specification is written in Markdown.
Our compliance tooling needs RFC formatted text files.
As such, we have a tool that extracts the RFC spec from the Markdown.

### RFC Scope

We do not include every Markdown file in the RFC spec;
Nor do we include all sections of the Markdown files
(i.e. we exclude legacy, changes, etc.).

The directories in scope for RFC specifications are those listed
in `util/specification_extract.sh`.

### Running `extract`

#### Running on A change

If you have changed any Markdown in scope for compliance, you MUST run the extract utility.
There are two ways of doing this. If you have changed only one file, you may run `extract` against just that file.

```
./util/extract.js PATH_TO_CHANGED_MARKEDOWN
```

For example, if ONLY the Default CMM Markdown was updated, we would run:

```
./util/extract.js framework/default-cmm.md
```

#### Running on all

Alternatively, the entire specification may be extracted at once. Run:

```
./util/specification_extract.sh
```

### Installing dependencies

The utility/script `util/extract.js` depends on four run
times: `node`, `python`, `ruby`, and `rust`
(No, this is not ideal, but Crypto Tools is pushing the "spec to code" boundary;
we are ahead of the tooling).

#### Set Up Python & `xml2rfc`

Follow [AWS Crypto Tools Getting Started with Python instructions](https://github.com/aws/crypto-tools/blob/master/getting-started/python/README.md#local-development-setup) to install `pyenv`.

Then, in this repository, run `pyenv local 3.9.7; pyenv exec python -m pip install xml2rfc==3.5.0 markupsafe==2.0.1`.

#### Set up `kramdown-rfc2629`

This is the Ruby dependency. Unfortunately, we have not figured out
a good way of installing this, so we do a bad way:

```
sudo gem install kramdown-rfc2629
```

#### Node

Follow
[Installing Node.js with `nvm` macOS by Daniel Schildt](https://gist.github.com/d2s/372b5943bce17b964a79)
to get `nvm` and `node` working.

#### Rust

Installing Duvet will install rust.
