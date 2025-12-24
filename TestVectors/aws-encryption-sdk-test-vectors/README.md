# Test Vectors for the AWS Encryption SDK

This repository contains known-good static test files for testing compatibility of AWS Encryption
SDK clients.

[For security issue notifications, see CONTRIBUTING.md.](./CONTRIBUTING.md#security-issue-notifications)

# Vectors

The contents of `vectors/awses-decrypt` conform to the new rules for test vectors
[as defined here](https://github.com/awslabs/aws-crypto-tools-test-vector-framework).
An implementation of these rules can be found [here](https://github.com/aws/aws-encryption-sdk-python/tree/master/test_vector_handlers)
and this readme will be updated soon once the rules themselves are published.

# Legacy Vectors

The `vectors/awses-legacy` directory contains known good test vectors using the legacy format.
These test vectors are contained in zip files that identify the client and version that were used to generate them.

The zip filename format used is `IMPLEMENTATION-VERSION.zip`. So, for example, a zip file
containing test vectors generated using version 1.3.0 of the AWS Encryption SDK for Python
would be named: `python-1.3.0.zip`.

The contents of each zip file follow the below format:
```
manifests/ciphertext.manifest
manifests/test_case.manifest
plaintext/<text size name>
ciphertext/<algorithm ID (hex string)>/<text size name>/<frame length (0 == non-framed)>/<test case uuid>
```

The Test Case manifest is provided for informational purposes and describes all test cases
used to generate the included test vectors.

The Ciphertext manifest describes all static test cases contained in the zip.  It provides
the necessary information to set up each test case and locate the associated plaintext and
ciphertext files relative to the zip file root.  The implied information in the ciphertext
filepath is not necessary in order to run the test case. The ciphertext files are broken
up simply to ease any necessary human interaction and to guard against possible filesystem
limits.
