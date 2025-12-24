[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Basic raw RSA keyring example

Implementations of this example MUST follow the rules defined in
[Example Templates](../../../examples.md#example-templates).

## Implementations

- [Python (DEV)](https://github.com/aws/aws-encryption-sdk-python/blob/keyring/examples/src/keyring/raw_rsa/keypair.py)
- [Java (DEV)](https://github.com/aws/aws-encryption-sdk-java/blob/keyring/src/examples/java/com/amazonaws/crypto/examples/keyring/rawrsa/RawRsa.java)
- [Java Master Key Provider (DEV)](https://github.com/aws/aws-encryption-sdk-java/blob/keyring/src/examples/java/com/amazonaws/crypto/examples/masterkeyprovider/rawrsa/RawRsa.java)

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Header

```python
# This example shows how to configure and use a raw RSA keyring using a pre-loaded RSA keypair.
#
# If your RSA key is in PEM or DER format,
# see the ``keyring/raw_rsa/private_key_only_from_pem`` example.
#
# https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/choose-keyring.html#use-raw-rsa-keyring
#
# In this example, we use the one-step encrypt and decrypt APIs.
```

## Summary

```python
# Demonstrate an encrypt/decrypt cycle using a raw RSA keyring.
```

## Inputs

- **source plaintext** :
  Plaintext to encrypt

## Steps

1. Define encryption context.

   ```python
   # Prepare your encryption context.
   # Remember that your encryption context is NOT SECRET.
   # https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/concepts.html#encryption-context
   encryption_context = {
       "encryption": "context",
       "is not": "secret",
       "but adds": "useful metadata",
       "that can help you": "be confident that",
       "the data you are handling": "is what you think it is",
   }
   ```

1. Generate the wrapping key.

   ```python
   # Generate an RSA private key to use with your keyring.
   # In practice, you should get this key from a secure key management system such as an HSM.
   #
   # The National Institute of Standards and Technology (NIST) recommends a minimum of 2048-bit keys for RSA.
   # https://www.nist.gov/publications/transitioning-use-cryptographic-algorithms-and-key-lengths
   #
   # Why did we use this public exponent?
   # https://crypto.stanford.edu/~dabo/pubs/papers/RSA-survey.pdf
   private_key = rsa.generate_private_key(public_exponent=65537, key_size=4096, backend=default_backend())
   ```

1. Create keyring.

   ```python
   # Create the keyring that determines how your data keys are protected.
   keyring = RawRSAKeyring(
       # The key namespace and key name are defined by you
       # and are used by the raw RSA keyring
       # to determine whether it should attempt to decrypt
       # an encrypted data key.
       #
       # https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/choose-keyring.html#use-raw-rsa-keyring
       key_namespace="some managed raw keys",
       key_name="my RSA wrapping key",
       private_wrapping_key=private_key,
       public_wrapping_key=private_key.public_key(),
       # The wrapping algorithm tells the raw RSA keyring
       # how to use your wrapping key to encrypt data keys.
       #
       # We recommend using RSA_OAEP_SHA256_MGF1.
       # You should not use RSA_PKCS1 unless you require it for backwards compatibility.
       wrapping_algorithm=WrappingAlgorithm.RSA_OAEP_SHA256_MGF1,
   )
   ```

1. Encrypt plaintext data.

   ```python
   # Encrypt your plaintext data.
   ciphertext, _encrypt_header = aws_encryption_sdk.encrypt(
       source=source_plaintext, encryption_context=encryption_context, keyring=keyring
   )
   ```

1. Compare ciphertext to plaintext.

   ```python
   # Demonstrate that the ciphertext and plaintext are different.
   assert ciphertext != source_plaintext
   ```

1. Decrypt encrypted data.

   ```python
   # Decrypt your encrypted data using the same keyring you used on encrypt.
   #
   # You do not need to specify the encryption context on decrypt
   # because the header of the encrypted message includes the encryption context.
   decrypted, decrypt_header = aws_encryption_sdk.decrypt(source=ciphertext, keyring=keyring)
   ```

1. Compare the decrypted plaintext and original plaintext.

   ```python
   # Demonstrate that the decrypted plaintext is identical to the original plaintext.
   assert decrypted == source_plaintext
   ```

1. Verify the encryption context.

   ```python
   # Verify that the encryption context used in the decrypt operation includes
   # the encryption context that you specified when encrypting.
   # The AWS Encryption SDK can add pairs, so don't require an exact match.
   #
   # In production, always use a meaningful encryption context.
   assert set(encryption_context.items()) <= set(decrypt_header.encryption_context.items())
   ```
