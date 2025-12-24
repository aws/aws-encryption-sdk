[//]: # "Copyright Amazon.com Inc. or its affiliates. All Rights Reserved."
[//]: # "SPDX-License-Identifier: CC-BY-SA-4.0"

# Raw AES keyring example

Implementations of this example MUST follow the rules defined in
[Example Templates](../../../examples.md#example-templates).

## Implementations

- [Python (DEV)](https://github.com/aws/aws-encryption-sdk-python/blob/keyring/examples/src/keyring/raw_aes/raw_aes.py)
- [Python Master Key Provider (DEV)](https://github.com/aws/aws-encryption-sdk-python/blob/keyring/examples/src/master_key_provider/raw_aes/raw_aes.py)
- [Java (DEV)](https://github.com/aws/aws-encryption-sdk-java/blob/keyring/src/examples/java/com/amazonaws/crypto/examples/keyring/rawaes/RawAes.java)
- [Java Master Key Provider (DEV)](https://github.com/aws/aws-encryption-sdk-java/blob/keyring/src/examples/java/com/amazonaws/crypto/examples/masterkeyprovider/rawaes/RawAes.java)

## Definitions

### Conventions used in this document

The key words
"MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT",
"SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL"
in this document are to be interpreted as described in
[RFC 2119](https://tools.ietf.org/html/rfc2119).

## Header

```python
# This examples shows how to configure and use a raw AES keyring.
#
# https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/choose-keyring.html#use-raw-aes-keyring
#
# In this example, we use the one-step encrypt and decrypt APIs.
```

## Summary

```python
# Demonstrate an encrypt/decrypt cycle using a raw AES keyring.
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
   # Generate a 256-bit (32 byte) AES key to use with your keyring.
   #
   # In practice, you should get this key from a secure key management system such as an HSM.
   key = os.urandom(32)
   ```

1. Create keyring.

   ```python
   # Create the keyring that determines how your data keys are protected.
   keyring = RawAESKeyring(
       # The key namespace and key name are defined by you
       # and are used by the raw AES keyring
       # to determine whether it should attempt to decrypt
       # an encrypted data key.
       #
       # https://docs.aws.amazon.com/encryption-sdk/latest/developer-guide/choose-keyring.html#use-raw-aes-keyring
       key_namespace="some managed raw keys",
       key_name="my AES wrapping key",
       wrapping_key=key,
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
