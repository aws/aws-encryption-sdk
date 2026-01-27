// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/*
  Intended usage :

  For encryption
    footer = Footer::new();
    footerAttribute = footer.make_terminal();

  For decryption
    footer :- Footer::deserialize();
    footer.validate()
*/

use crate::serialize::*;
use crate::utils::*;
use crate::*;

#[allow(unused)]
pub(crate) fn foo(
    enc: &aws_mpl_rs::EncryptionMaterials,
    dec: &aws_mpl_rs::DecryptionMaterials,
    data: &[CanonCryptoItem],
    header: &[u8],
    edks: &[aws_mpl_rs::EncryptedDataKey],
) -> Result<(), Error> {
    let footer = Footer::new(enc, data, header)?;
    let footer_attribute = footer.make_terminal();
    let footer2 = Footer::deserialize(&footer_attribute.value, true)?;
    footer2.validate(dec, edks, data, header);
    Ok(())
}

const RECIPIENT_TAG_SIZE: usize = 48;
const SIGNATURE_SIZE: usize = 103;
type RecipientTag = [u8; RECIPIENT_TAG_SIZE];
type Signature = [u8; SIGNATURE_SIZE];
const ENCRYPTED: &[u8] = b"ENCRYPTED";
const PLAINTEXT: &[u8] = b"PLAINTEXT";

//= specification/structured-encryption/footer.md#footer-format
//= type=implication
//# The [Terminal Value](./structures.md#terminal-value) of the footer MUST be
// | Field | Length (bytes) | Interpreted as |
// | ----- | -------------- | -------------- |
// | [Recipient Tags](#recipient-tags) | Variable. 48 bytes per Encrypted Data Key in the header | Bytes |
// | [Signature](#signature) | 0 or 103 | Signature, if signatures are enabled |
pub(crate) struct Footer {
    tags: Vec<RecipientTag>,
    sig: Option<Signature>,
}

impl Footer {
    fn serialize(&self, out: &mut Vec<u8>) {
        serialize_tags(out, &self.tags);
        serialize_sig(out, self.sig);
    }

    pub(crate) fn make_terminal(&self) -> StructuredDataTerminal {
        //= specification/structured-encryption/encrypt-path-structure.md#footer-field
        //= type=implication
        //# The Footer Field TypeID MUST be 0xFFFF
        //= specification/structured-encryption/encrypt-path-structure.md#footer-field
        //= type=implication
        //# The Footer Field Value MUST be the serialized [footer](footer.md).
        let mut out: Vec<_> = Vec::new();
        self.serialize(&mut out);
        value_to_data(out, BINARY)
    }

    pub(crate) fn validate(
        &self,
        mat: &aws_mpl_rs::DecryptionMaterials,
        edks: &[aws_mpl_rs::EncryptedDataKey],
        data: &[CanonCryptoItem],
        header: &[u8],
    ) -> Result<(), Error>
// requires Materials.DecryptionMaterialsWithPlaintextDataKey(mat)
        // requires ValidSuite(mat.algorithmSuite)
        // requires Header.ValidEncryptionContext(mat.encryptionContext)
        // //= specification/structured-encryption/decrypt-path-structure.md#verify-signatures
        // //= type=implication
        // //# The number of [HMACs in the footer](./footer.md#hmacs)
        // //# MUST be the number of [Encrypted Data Keys in the header](./header.md#encrypted-data-keys).
        // ensures ret.Success? ==>
        //           |edks| == |tags|
    {
        need(
            edks.len() == self.tags.len(),
            "There are a different number of recipient tags in the stored header than there are in the decryption materials.",
        )?;
        let canonical_hash = canon_hash(data, header, &mat.encryption_context);
        let hash = aws_mpl_primitives::hmac(
            get_hmac_alg(mat.algorithm_suite.symmetric_signature),
            &mat.symmetric_signing_key.as_ref().unwrap().0,
            &canonical_hash,
        );
        need(hash.len() == 48, "Bad hash length")?;

        let mut found_tag = false;
        for tag in &self.tags {
            //= specification/structured-encryption/footer.md#recipient-tag-verification
            //# Recipient Tag comparisons MUST be constant time operations.
            if aws_mpl_primitives::constant_time_equal(&hash, tag) {
                found_tag = true;
                break;
            }
        }

        //= specification/structured-encryption/footer.md#recipient-tag-verification
        //# Verification MUST fail unless at least one of the [Recipient Tags](#recipient-tags)
        //# matches a calculated recipient tag using the provided symmetricSigningKey.
        need(
            found_tag,
            "Signature of record does not match the signature computed when the record was encrypted.",
        )?;

        // need(self.sig.is_some() == mat.algorithm_suite.signature.Ecdsa.is_some(), E("Internal error. Signature both does and does not exist."))?;
        //= specification/structured-encryption/footer.md#signature-verification
        //# If the footer contains a signature, this signature MUST be verified using the
        //# [asymmetric signature algorithm](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/algorithm-suites.md#algorithm-suites-signature-settings)
        //# indicated by the algorithm suite.
        if let Some(sig) = self.sig.as_ref() {
            aws_mpl_primitives::ecdsa_verify(
                get_ecdsa_alg(mat.algorithm_suite.signature),
                &mat.verification_key.as_ref().unwrap().0,
                &canonical_hash,
                sig,
            )?;
        }
        Ok(())
    }

    pub(crate) fn deserialize(data: &[u8], has_sig: bool) -> Result<Self, Error> {
        if has_sig {
            need(
                data.len() >= RECIPIENT_TAG_SIZE + SIGNATURE_SIZE,
                "Footer too short.",
            )?;
            need(
                (data.len() - SIGNATURE_SIZE).is_multiple_of(RECIPIENT_TAG_SIZE),
                "Mangled signed footer has strange size",
            )?;
            Ok(Self {
                tags: gather_tags(&data[..data.len() - SIGNATURE_SIZE]),
                sig: Some(data[data.len() - SIGNATURE_SIZE..].try_into().unwrap()),
            })
        } else {
            need(
                data.len().is_multiple_of(RECIPIENT_TAG_SIZE),
                "Mangled unsigned footer has strange size",
            )?;
            need(data.len() >= RECIPIENT_TAG_SIZE, "Footer too short.")?;
            Ok(Self {
                tags: gather_tags(data),
                sig: None,
            })
        }
    }

    // return the footer value for the StructuredData
    pub(crate) fn new(
        mat: &aws_mpl_rs::EncryptionMaterials,
        data: &[CanonCryptoItem],
        header: &[u8],
    ) -> Result<Self, Error>
// requires ValidSuite(mat.algorithmSuite)
        // requires Materials.EncryptionMaterialsHasPlaintextDataKey(mat)
        // requires Header.ValidEncryptionContext(mat.encryptionContext)

        // ensures (ret.Success? && mat.algorithmSuite.signature.ECDSA?) ==>
        //           //= specification/structured-encryption/footer.md#signature
        //           //= type=implication
        //           //# The `signature`, if it exists, MUST be calculated using the
        //           //# [asymmetric signature algorithm](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/algorithm-suites.md#algorithm-suites-signature-settings)
        //           //# indicated by the algorithm suite.
        //           && var history = client.History.ECDSASign;
        //           && 0 < |history|
        //           && var signInput = Seq.Last(history).input;
        //           && signInput.signatureAlgorithm == mat.algorithmSuite.signature.ECDSA.curve
    {
        let canonical_hash = canon_hash(data, header, &mat.encryption_context);
        let mut tags: Vec<RecipientTag> = Vec::new();
        for key in &mat.symmetric_signing_keys {
            //= specification/structured-encryption/footer.md#recipient-tags
            //# the Recipient Tag MUST be MUST be calculated over the [Canonical Hash](#canonical-hash)
            //# using the [symmetric signature algorithm](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/algorithm-suites.md#algorithm-suites-signature-settings)
            //# indicated in the algorithm suite,
            //# and the
            //# [symmetric signing keys](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/structures.md#symmetric-signing-keys)
            //# in the encryption materials.
            let hmac = aws_mpl_primitives::hmac(
                get_hmac_alg(mat.algorithm_suite.symmetric_signature),
                &key.0,
                &canonical_hash,
            );
            // :- Need(|hash| as u64 == 48, E("Bad hash length"));

            //= specification/structured-encryption/footer.md#recipient-tags
            //# the HMAC values MUST have the same order as the
            //# [symmetric signing keys](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/structures.md#symmetric-signing-keys)
            //# used to calculate them.
            tags.push((&hmac[..]).try_into().unwrap());
        }
        //= specification/structured-encryption/footer.md#recipient-tags
        //= type=implication
        //# There MUST be one Recipient Tag for each Encrypted Data Key in the [header](./header.md#encrypted-data-keys)
        debug_assert!(tags.len() == mat.encrypted_data_keys.len());

        match mat.algorithm_suite.signature {
            aws_mpl_rs::suites::SignatureAlgorithm::Ecdsa(x) => {
                //= specification/structured-encryption/footer.md#signature
                //# The `signature`, if it exists, MUST be calculated over the [Canonical Hash](#canonical-hash),
                //# using the asymmetric signing key in the encryption materials.
                let sig = aws_mpl_primitives::ecdsa_sign(
                    x,
                    &mat.signing_key.as_ref().unwrap().0,
                    &canonical_hash,
                )?;
                need(
                    sig.len() == SIGNATURE_SIZE,
                    format!(
                        "Signature is {} bytes, should have been {SIGNATURE_SIZE} bytes.",
                        sig.len()
                    ),
                )?;
                Ok(Self {
                    tags,
                    sig: Some(sig.try_into().unwrap()),
                })
            }
            aws_mpl_rs::suites::SignatureAlgorithm::None => Ok(Self { tags, sig: None }),
            _ => panic!(),
        }
    }
}

fn get_ecdsa_alg(
    x: aws_mpl_rs::suites::SignatureAlgorithm,
) -> aws_mpl_primitives::EcdsaSignatureAlgorithm {
    match x {
        aws_mpl_rs::suites::SignatureAlgorithm::Ecdsa(x) => x,
        _ => panic!(),
    }
}
fn get_hmac_alg(
    x: aws_mpl_rs::suites::SymmetricSignatureAlgorithm,
) -> aws_mpl_primitives::DigestAlg {
    match x {
        aws_mpl_rs::suites::SymmetricSignatureAlgorithm::Hmac(x) => x,
        _ => panic!(),
    }
}

fn get_canonical_encrypted_field(
    data: &mut Vec<u8>,
    canonical_field_name: &[u8],
    value: &StructuredDataTerminal,
) {
    //= specification/structured-encryption/footer.md#canonical-encrypted-field
    //= type=implication
    //# The canonical form of an encrypted field MUST be
    //# | Field | Length (bytes) | Interpreted as |
    //# | ----- | -------------- | -------------- |
    //# | The [canonical path](./header.md#canonical-path) of the field name | Variable | Bytes |
    //# | encrypted data length - 2 | 8 | 64-bit integer |
    //# | "ENCRYPTED" | 9 | Literal Ascii text |
    //# | TypeID | 2 | the type ID of the unencrypted Terminal |
    //# | value | Variable | the encrypted Terminal value |

    data.extend_from_slice(canonical_field_name);
    write_u64(data, value.value.len() as u64 - 2);
    data.extend_from_slice(ENCRYPTED);
    data.extend_from_slice(&value.value);
}

fn get_canonical_plaintext_field(
    data: &mut Vec<u8>,
    canonical_field_name: &[u8],
    value: &StructuredDataTerminal,
)
//= specification/structured-encryption/footer.md#canonical-plaintext-field
//= type=implication
//# The canonical form of a plaintext field MUST be
//# | Field | Length (bytes) | Interpreted as |
//# | ----- | -------------- | -------------- |
//# | The [canonical path](./header.md#canonical-path) of the field name | Variable | Bytes |
//# | data length | 8 | 64-bit integer |
//# | "PLAINTEXT" | 9 | Literal Ascii text |
//# | TypeID | 2 | the type ID of the Terminal |
//# | value | Variable | the Terminal value |
{
    data.extend_from_slice(canonical_field_name);
    write_u64(data, value.value.len() as u64);
    data.extend_from_slice(PLAINTEXT);
    data.extend_from_slice(&value.type_id);
    data.extend_from_slice(&value.value);
}

// Given a key value pair, return the canonical value for use in the footer checksum calculations
fn get_canonical_item(data: &mut Vec<u8>, item: &CanonCryptoItem) {
    if item.action == CryptoAction::EncryptAndSign {
        get_canonical_encrypted_field(data, &item.key, &item.data);
    } else {
        get_canonical_plaintext_field(data, &item.key, &item.data);
    }
}

fn canon_content(data: &mut Vec<u8>, items: &[CanonCryptoItem]) {
    for item in items {
        if item.action != CryptoAction::DoNothing {
            get_canonical_item(data, item);
        }
    }
}

fn canon_record(
    data: &mut Vec<u8>,
    items: &[CanonCryptoItem],
    header: &[u8],
    enc: &EncryptionContext,
)
//= specification/structured-encryption/footer.md#canonical-record
//= type=implication
//# The canonical form of a record MUST be
//# | Field | Length (bytes) | Interpreted as |
//# | ----- | -------------- | -------------- |
//# | header | Variable | The full serialized header with commitment |
//# | AAD Length | 8 | 64-bit integer, the length of the following AAD data |
//# | AAD | Variable | The serialization of the Encryption Context from the Encryption Materials |
//# | Field Data | Variable | For each [signed field](#signed-fields), ordered lexicographically by [canonical path](./header.md#canonical-path), the [canonical field](#canonical-field).
{
    data.extend_from_slice(header);
    // FIXME -- we can calculate the length without having to serialize the context
    let mut aad = Vec::new();
    header::serialize_context(&mut aad, enc);
    write_u64(data, aad.len() as u64);
    data.append(&mut aad);
    canon_content(data, items);
}

fn canon_hash(items: &[CanonCryptoItem], header: &[u8], enc: &EncryptionContext) -> Vec<u8>
//= specification/structured-encryption/footer.md#hash-calculation
        //= type=implication
        //# The canonical hash of a record MUST be the SHA384 of the canonical form of the record.
{
    let mut data = Vec::new();
    canon_record(&mut data, items, header, enc);
    aws_mpl_primitives::digest(aws_mpl_primitives::DigestAlg::Sha384, &data)
}

fn serialize_tags(data: &mut Vec<u8>, tags: &[RecipientTag]) {
    for tag in tags {
        data.extend_from_slice(tag);
    }
}

fn serialize_sig(data: &mut Vec<u8>, sig: Option<Signature>) {
    if let Some(s) = sig {
        data.extend_from_slice(&s);
    }
}

fn gather_tags(data: &[u8]) -> Vec<RecipientTag> {
    data.chunks_exact(RECIPIENT_TAG_SIZE)
        .map(|chunk| chunk.try_into().unwrap())
        .collect()
}
