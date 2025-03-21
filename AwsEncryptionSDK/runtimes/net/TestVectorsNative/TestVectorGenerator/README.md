# TestVectorGenerator

This project generates decryption test vectors,
as described in <https://github.com/awslabs/aws-crypto-tools-test-vector-framework/blob/master/features/0006-awses-message-decryption-generation.md>.

## Usage

Prerequisites:

- Set up AWS credentials
- Create an empty output directory, e.g. `generated_vectors`

From this directory, run the following:

```bash
$ dotnet run -- \
    --encrypt-manifest resources/0006-awses-message-decryption-generation.v2.json \
    --output-dir generated_vectors
```

Point your decryption test vector runner to `generated_vectors/manifest.json`
in order to decrypt the generated vectors.
