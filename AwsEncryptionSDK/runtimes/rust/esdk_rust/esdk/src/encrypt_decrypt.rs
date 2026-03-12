// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::*;
use crate::message::body::*;
use crate::message::encryption_context::*;
use crate::message::header::*;
use crate::message::header_auth::*;
use crate::message::header_types::*;
use crate::message::serializable_types::*;
use crate::message::serialize_functions::*;
use crate::message::v2_header_body::get_hkdf;
use crate::message::*;
use aws_mpl_legacy::EncryptedDataKey;
use aws_mpl_legacy::suites::AlgorithmSuite;

use aws_mpl_legacy::primitives::ecdsa_verify_context;
use aws_mpl_legacy::primitives::{EcdsaSignatureAlgorithm, aes_encrypt, generate_random_bytes};

const RESERVED_ENCRYPTION_CONTEXT: &str = "aws-crypto-";
const MAX_DATA: usize = (1usize << 36) - 32;

/// Input for constructing a single frame (regular or final).
struct ConstructFrameInput<'a> {
    alg: aws_mpl_legacy::primitives::AesGcm,
    key: &'a [u8],
    plaintext: &'a [u8],
    message_id: &'a [u8],
    aad_content: BodyAADContent,
    sequence_number: u32,
    is_final: bool,
}

/// Construct and serialize a single frame (regular or final).
///
/// Shared implementation of the "Construct a frame" step from
/// `encrypt.md#construct-a-frame`. Annotations live on the callers
/// where the actual values are chosen.
fn construct_frame(
    input: &ConstructFrameInput<'_>,
    iv: &mut [u8],
    aad: &mut Vec<u8>,
    w: &mut Vec<u8>,
    out: &mut dyn SafeWrite,
    dw: &mut DigestWriter,
) -> Result<(), Error> {
    w.clear();

    body_aad(
        //= specification/client-apis/encrypt.md#construct-a-frame
        //# - The [message ID](../data-format/message-body-aad.md#message-id) MUST be the same as the
        //# [message ID](../data-frame/message-header.md#message-id) serialized in the header of this message.
        input.message_id,
        //= specification/client-apis/encrypt.md#construct-a-frame
        //# - The [Body AAD Content](../data-format/message-body-aad.md#body-aad-content) MUST be the structure defined in
        //# [Message Body AAD](../data-format/message-body-aad.md).
        input.aad_content,
        //= specification/client-apis/encrypt.md#construct-a-frame
        //# - The [sequence number](../data-format/message-body-aad.md#sequence-number) MUST be the sequence
        //# number of the frame being encrypted.
        input.sequence_number,
        //= specification/client-apis/encrypt.md#construct-a-frame
        //# - The [content length](../data-format/message-body-aad.md#content-length) MUST have a value
        //# equal to the length of the plaintext being encrypted.
        input.plaintext.len() as u64,
        // aad is the output buffer
        aad
    );

    // Serialize the frame.

    //= specification/client-apis/encrypt.md#construct-a-frame
    //# This operation MUST serialize a regular frame or final frame with the following specifics:

    //= specification/client-apis/encrypt.md#construct-a-frame
    //# - The IV MUST be the [sequence number](../data-format/message-body-aad.md#sequence-number)
    //# used in the message body AAD above,
    //# padded to the [IV length](../data-format/message-header.md#iv-length).
    iv_seq(
        input.sequence_number,
        // iv is the output buffer
        iv
    );

    //= specification/data-format/message-body.md#regular-frame
    //# A regular frame MUST be serialized as, in order,
    //# Sequence Number,
    //# IV,
    //# Encrypted Content,
    //# and Authentication Tag.

    //= specification/data-format/message-body.md#final-frame
    //# A final frame MUST be serialized as, in order,
    //# Sequence Number End,
    //# Sequence Number,
    //# IV,
    //# Encrypted Content Length,
    //# Encrypted Content,
    //# and Authentication Tag.

    //= specification/data-format/message-body.md#final-frame
    //# This means a final frame MUST be a regular frame with the addition of the serialized
    //# Sequence Number End
    //# and Encrypted Content Length.

    //= specification/client-apis/encrypt.md#construct-a-frame
    //# - Final frame only: [Sequence Number End](../data-format/message-body.md#sequence-number-end) field: This MUST
    //# be the sequence number end value.
    if input.is_final {
        //= specification/client-apis/encrypt.md#construct-a-frame
        //# The Sequence Number End MUST only be serialized for the final frame.
        write_u32(w, ENDFRAME_SEQUENCE_NUMBER)?;
    }
    //= specification/client-apis/encrypt.md#construct-a-frame
    //# - [Sequence Number](../data-format/message-body.md#sequence-number): MUST be the sequence number of this frame,
    //# as determined above.
    write_u32(w, input.sequence_number)?;
    //= specification/client-apis/encrypt.md#construct-a-frame
    //# - [IV](../data-format/message-body.md#iv): MUST be the IV used when calculating the encrypted content above
    write_bytes(w, iv)?;
    //= specification/client-apis/encrypt.md#construct-a-frame
    //# - Final frame only: [Encrypted Content Length](../data-format/message-body.md#encrypted-content-length) field: This MUST
    //# be the encrypted content length value
    if input.is_final {
        //= specification/client-apis/encrypt.md#construct-a-frame
        //# The Encrypted Content Length MUST only be serialized for the final frame.
        write_u32(w, input.plaintext.len() as u32)?;
    }

    
    //= specification/client-apis/encrypt.md#construct-a-frame
    //# To construct a regular or final frame that represents the next frame in the encrypted message's body,
    //# this operation MUST calculate the encrypted content and an authentication tag using the
    //# [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md),
    //# with the following inputs:
    aes_encrypt(
        input.alg,
        //= specification/client-apis/encrypt.md#construct-a-frame
        //# - The AAD MUST be the serialized [message body AAD](../data-format/message-body-aad.md),
        //# constructed as follows:
        aad,
        //= specification/client-apis/encrypt.md#construct-a-frame
        //# - The IV MUST be the [sequence number](../data-format/message-body-aad.md#sequence-number)
        //# used in the message body AAD above,
        //# padded to the [IV length](../data-format/message-header.md#iv-length).
        iv,
        //= specification/client-apis/encrypt.md#construct-a-frame
        //# - The cipherkey MUST be the derived data key
        input.key,
        //= specification/client-apis/encrypt.md#construct-a-frame
        //# - The plaintext MUST be the next subsequence of consumable plaintext bytes
        //# that have not yet been encrypted.
        input.plaintext,
        w
    )?;

    // Write encrypted content
    //= specification/client-apis/encrypt.md#construct-a-frame
    //# - [Encrypted Content](../data-format/message-body.md#encrypted-content): MUST be the encrypted content calculated above.
    write_bytes(out, w)?;
    // Write authentication tag
    //= specification/client-apis/encrypt.md#construct-a-frame
    //# - [Authentication Tag](../data-format/message-body.md#authentication-tag): MUST be the authentication tag
    //# output when calculating the encrypted content above.
    write_bytes(dw, w)?;

    Ok(())
}

/// Serialize the message header to the output stream.
pub(crate) fn serialize_header(
    header: &HeaderInfo,
    out: &mut dyn SafeWrite,
    dw: &mut DigestWriter,
) -> Result<(), Error> {
    let mut w = Vec::new();
    //= specification/data-format/message.md#structure
    //# - The message MUST begin with [Message Header](message-header.md)
    write_bytes(&mut w, &header.raw_header)?;
    write_header_auth_tag(&mut w, &header.header_auth, &header.suite)?;
    write_bytes(out, &w)?;
    write_bytes(dw, &w)?;
    //= specification/data-format/message.md#structure
    //# - The [Message Body](message-body.md) MUST follow the Message Header
    Ok(())
}

/// Encrypt plaintext and serialize the message body (framed) to the output stream.
pub(crate) fn encrypt_and_serialize_body(
    plaintext: &mut dyn SafeRead,
    header: &HeaderInfo,
    key: &[u8],
    out: &mut dyn SafeWrite,
    dw: &mut DigestWriter,
) -> Result<(), Error> {
    let mut total_data_size: usize = 0;
    let frame_length = header.body.frame_length() as usize;
    let iv_len = get_iv_length(&header.suite) as usize;
    let auth_len = get_tag_length(&header.suite) as usize;
    let frame_len = frame_length + iv_len + auth_len + 4;
    let mut w = Vec::with_capacity(frame_len);

    //= specification/data-format/message-body.md#regular-frame-sequence-number
    //= type=implementation
    //# Framed Data MUST start at Sequence Number 1.
    //= specification/client-apis/encrypt.md#construct-a-frame
    //# If this is the first frame sequentially, the sequence number value MUST be 1.
    let mut sequence_number = START_SEQUENCE_NUMBER;
    let alg = get_encrypt(&header.suite);

    let mut iv = vec![0; iv_len];
    let mut plaintext_frame = vec![0; frame_length];
    let mut aad = Vec::new();
    let mut in_size: usize;
    let mut next_char: Option<u8> = None;

    //= specification/client-apis/encrypt.md#construct-the-body
    //= type=implementation
    //# Before the end of the input is indicated,
    //# this operation MUST process as much of the consumable bytes as possible
    //# by [constructing regular frames](#construct-a-frame).
    //# When the end of the input is indicated,
    //# this operation MUST perform the following until all consumable plaintext bytes are processed:
    loop {
        in_size = read_up_to_peek(plaintext, &mut plaintext_frame, next_char)?;

        if in_size != frame_length {
            //= specification/client-apis/encrypt.md#construct-the-body
            //= type=implementation
            //# - If there are not enough input consumable plaintext bytes to create a new regular frame,
            //# then this operation MUST [construct a final frame](#construct-a-frame)
            break;
        }
        //= specification/client-apis/encrypt.md#construct-the-body
        //= type=implementation
        //# - If there are exactly enough consumable plaintext bytes to create one regular frame,
        //# such that creating a regular frame processes all consumable bytes,
        //# then this operation MUST [construct either a final frame or regular frame](#construct-a-frame)
        //# with the remaining plaintext.
        //= specification/client-apis/encrypt.md#construct-the-body
        //= type=implementation
        //# - If there are enough input plaintext bytes consumable to create a new regular frame,
        //# such that creating a regular frame does not processes all consumable bytes,
        //# then this operation MUST [construct a regular frame](#construct-a-frame)
        //# using the consumable plaintext bytes.
        next_char = read_opt_u8(plaintext)?;
        if next_char.is_none() {
            //= specification/client-apis/encrypt.md#construct-the-body
            //= type=implementation
            //# If an end to the input has been indicated, there are no more consumable plaintext bytes to process,
            //# and a final frame has not yet been constructed,
            //# this operation MUST [construct an empty final frame](#construct-a-frame).
            break;
        }

        //= specification/data-format/message-body.md#framed-data
        //= type=implementation
        //# - The number of frames in a single message MUST be less than or equal to `2^32 - 1`.
        if sequence_number == ENDFRAME_SEQUENCE_NUMBER {
            return Err("too many frames".into());
        }

        //= specification/data-format/message-body.md#regular-frame-encrypted-content
        //= type=implementation
        //# The length of the encrypted content of a Regular Frame MUST be equal to the Frame Length.
        total_data_size += frame_length;
        if total_data_size > MAX_DATA {
            return Err("Plain text too large".into());
        }

        // Construct a regular frame
        construct_frame(
            &ConstructFrameInput {
                alg,
                key,
                //= specification/client-apis/encrypt.md#construct-a-frame
                //# - For a regular frame the length of this plaintext MUST equal the frame length.
                //= specification/client-apis/encrypt.md#construct-a-frame
                //# - For a regular frame the length of this plaintext subsequence MUST equal the frame length.
                plaintext: &plaintext_frame,
                message_id: header.body.message_id(),
                aad_content: BodyAADContent::RegularFrame,
                sequence_number,
                is_final: false,
            },
            &mut iv, &mut aad, &mut w, out, dw,
        )?;

        //= specification/data-format/message-body.md#regular-frame-sequence-number
        //= type=implementation
        //# Subsequent frames MUST be in order and MUST contain an increment of 1 from the previous frame.
        //= specification/client-apis/encrypt.md#construct-a-frame
        //# Otherwise, the sequence number value MUST be 1 greater than the value of the sequence number
        //# of the previous frame.
        sequence_number += 1;
    }

    // Final frame
    total_data_size += in_size;
    if total_data_size > MAX_DATA {
        return Err("Plain text too large".into());
    }

    //= specification/data-format/message-body.md#final-frame
    //# The length of the plaintext to be encrypted in the Final Frame MUST be
    //# greater than or equal to 0 and less than or equal to the [Frame Length](message-header.md#frame-length).
    debug_assert!(in_size <= frame_length);
    //= specification/data-format/message-body.md#final-frame
    //# - When the length of the Plaintext is an exact multiple of the Frame Length
    //# (including if it is equal to the frame length),
    //# the Final Frame encrypted content length SHOULD be equal to the frame length but MAY be 0.
    debug_assert!(in_size > 0 || sequence_number == START_SEQUENCE_NUMBER,
        "empty final frame only allowed when entire plaintext is empty");
    //= specification/data-format/message-body.md#final-frame
    //# - When the length of the Plaintext is less than the Frame Length,
    //# the body MUST contain exactly one frame and that frame MUST be a Final Frame.
    debug_assert!((sequence_number == START_SEQUENCE_NUMBER && in_size < frame_length) || in_size == frame_length);

    construct_frame(
        &ConstructFrameInput {
            alg,
            key,
            //= specification/client-apis/encrypt.md#construct-a-frame
            //# - For a final frame this MUST be the length of the remaining plaintext bytes
            //# which have not yet been encrypted,
            //# whose length MUST be equal to or less than the frame length.
            //= specification/client-apis/encrypt.md#construct-a-frame
            //# - For a final frame this MUST be the remaining plaintext bytes which have not yet been encrypted,
            //# whose length MUST be equal to or less than the frame length.
            plaintext: &plaintext_frame[0..in_size],
            message_id: header.body.message_id(),
            aad_content: BodyAADContent::FinalFrame,
            sequence_number,
            is_final: true,
        },
        &mut iv, &mut aad, &mut w, out, dw,
    )?;

    //= specification/client-apis/encrypt.md#construct-the-body
    //# The encrypted message output by the Encrypt operation MUST have a message body equal
    //# to the message body calculated in this step.
    //# If the message bodies are not equal, the Encrypt operation MUST fail.
    Ok(())
}

/// Serialize header + encrypt and serialize body.
pub(crate) fn encrypt_and_serialize(
    plaintext: &mut dyn SafeRead,
    header: &HeaderInfo,
    key: &[u8],
    out: &mut dyn SafeWrite,
    dw: &mut DigestWriter,
) -> Result<(), Error> {
    serialize_header(header, out, dw)?;
    encrypt_and_serialize_body(plaintext, header, key, out, dw)
}

pub(crate) fn get_ecdsa_alg(
    alg: aws_mpl_legacy::suites::SignatureAlgorithm,
) -> Result<EcdsaSignatureAlgorithm, Error> {
    match alg {
        aws_mpl_legacy::suites::SignatureAlgorithm::Ecdsa(x) => Ok(x),
        _ => Err("UnsupportedAlgorithm".into()),
    }
}

pub(crate) fn verify_signature(
    r: &mut dyn SafeRead,
    context: aws_mpl_legacy::primitives::DigestContext,
    dec_mat: aws_mpl_legacy::DecryptionMaterials,
    raw: &mut dyn SafeWrite,
) -> Result<(), Error> {
    if dec_mat.verification_key.is_none() {
        return Ok(());
    }

    let signature = footer::read_footer(r, raw)?;
    let ecdsa_params = get_ecdsa_alg(dec_mat.algorithm_suite.signature)?;
    //= specification/client-apis/decrypt.md#verify-the-signature
    //# If this verification is not successful, this operation MUST immediately halt and fail.
    let valid = ecdsa_verify_context(
        ecdsa_params,
        &dec_mat.verification_key.unwrap().0,
        context,
        &signature,
    )?;

    if !valid {
        return Err("InvalidSignature".into());
    }
    Ok(())
}

//= specification/client-apis/encrypt.md#encryption-context
//= type=implementation
//# If the input encryption context contains any entries with a key beginning with `aws-crypto-`,
//# the encryption operation MUST fail.
pub(crate) fn validate_encryption_context(ec: &EncryptionContext) -> Result<(), Error> {
    for key in ec.keys() {
        if key.starts_with(RESERVED_ENCRYPTION_CONTEXT) {
            return Err(
                "Encryption context keys cannot contain reserved prefix 'aws-crypto-'".into(),
            );
        }
    }
    Ok(())
}

//= specification/data-format/message-header.md#encrypted-data-key-count
//= type=implementation
//# This value MUST be greater than 0.
pub(crate) fn validate_max_encrypted_data_keys(
    max_encrypted_data_keys: Option<std::num::NonZeroUsize>,
    edks: &[EncryptedDataKey],
) -> Result<(), Error> {
    if let Some(max) = max_encrypted_data_keys {
        if edks.len() > max.get() {
            return Err("Encrypted data keys exceed maxEncryptedDataKeys".into());
        }
        if edks.is_empty() {
            return Err("Encrypted data keys is empty.".into());
        }
    }
    Ok(())
}

//= specification/data-format/message-header.md#message-id
//= type=implementation
//# A Message ID MUST uniquely identify the [message](message.md).
//= specification/data-format/message-header.md#message-id
//= type=implementation
//# While implementations cannot guarantee complete uniqueness,
//# implementations MUST use a good source of randomness when generating messages IDs in order to make
//# the chance of duplicate IDs negligible.
pub(crate) fn generate_message_id(suite: &AlgorithmSuite) -> Result<MessageId, Error> {
    let length = if suite.message_version == 1 {
        MESSAGE_ID_LEN_V1
    } else {
        MESSAGE_ID_LEN_V2
    };
    let mut rand_bytes: Vec<u8> = vec![0; length as usize];
    generate_random_bytes(&mut rand_bytes)?;
    Ok(rand_bytes)
}

pub(crate) fn build_header_for_encrypt(
    message_id: &MessageId,
    suite: &AlgorithmSuite,
    encryption_context: &EncryptionContext,
    required_encryption_context_keys: &[String],
    encrypted_data_keys: &[EncryptedDataKey],
    frame_length: u32,
    derived_data_keys: &key_derivation::ExpandedKeyMaterial,
) -> Result<HeaderInfo, Error> {
    let mut stored_encryption_context = encryption_context.clone();
    let mut required_encryption_context_map: EncryptionContext = EncryptionContext::new();
    //= specification/client-apis/encrypt.md#v2-header
    //# - [AAD](../data-format/message-header.md#aad): MUST be the serialization of the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials),
    //# and this serialization MUST NOT contain any key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys).
    for key in required_encryption_context_keys {
        if stored_encryption_context.contains_key(key) {
            required_encryption_context_map
                .insert(key.clone(), stored_encryption_context.remove(key).unwrap());
        }
    }
    let canonical_stored_encryption_context = to_canonical_pairs(stored_encryption_context);

    let body: HeaderBody = build_header_body(
        message_id,
        suite,
        &canonical_stored_encryption_context,
        encrypted_data_keys,
        frame_length,
        derived_data_keys.commitment_key.clone(),
    )?;

    let canonical_req_encryption_context = to_canonical_pairs(required_encryption_context_map);
    let mut serialized_req_encryption_context = Vec::new();
    //= specification/client-apis/encrypt.md#authentication-tag
    //# The encryption context to only authenticate MUST be the [encryption context](../framework/structures.md#encryption-context)
    //# in the [encryption materials](../framework/structures.md#encryption-materials)
    //# filtered to only contain key value pairs listed in
    //# the [encryption material's](../framework/structures.md#encryption-materials)
    //# [required encryption context keys](../framework/structures.md#required-encryption-context-keys)
    //# serialized according to the [encryption context serialization specification](../framework/structures.md#serialization).
    write_empty_ec_or_write_aad(
        &mut serialized_req_encryption_context,
        &canonical_req_encryption_context,
    )?;

    let mut raw_header = Vec::new();
    //= specification/client-apis/encrypt.md#construct-the-header
    //# Before encrypting input plaintext,
    //# this operation MUST serialize the [message header body](../data-format/message-header.md).
    //= specification/client-apis/encrypt.md#construct-the-header
    //# The [message format version](../data-format/message-header.md#supported-versions) MUST be the value associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites).
    write_header_body(&mut raw_header, &body)?;

    //= specification/client-apis/encrypt.md#authentication-tag
    //# After serializing the message header body,
    //# this operation MUST calculate an [authentication tag](../data-format/message-header.md#authentication-tag)
    //# over the message header body.
    let header_auth = build_header_auth_tag(
        suite,
        &derived_data_keys.data_key,
        &raw_header,
        &serialized_req_encryption_context,
    )?;

    Ok(HeaderInfo {
        suite: suite.clone(),
        body,
        encryption_context: encryption_context.clone(),
        header_auth,
        raw_header,
    })
}

pub(crate) fn build_header_body(
    message_id: &MessageId,
    suite: &AlgorithmSuite,
    encryption_context: &ESDKCanonicalEncryptionContext,
    encrypted_data_keys: &[EncryptedDataKey],
    frame_length: u32,
    suite_data: Option<Vec<u8>>,
) -> Result<HeaderBody, Error> {
    //= specification/client-apis/encrypt.md#construct-the-header
    //# The [message format version](../data-format/message-header.md#supported-versions) MUST be the value associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites).
    match suite.commitment {
        aws_mpl_legacy::suites::DerivationAlgorithm::Hkdf(h) => {
            if suite_data.is_none()
                || suite_data.as_ref().unwrap().len() != h.output_key_length as usize
            {
                return Err("Validation Error 1".into());
            }
            //= specification/client-apis/encrypt.md#v2-header
            //# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 2.0
            //# then the [message header body](../data-format/message-header.md#header-body-version-1-0) MUST be serialized with the following specifics:
            Ok(HeaderBody::V2Body(V2HeaderBody {
                algorithm_suite: suite.clone(),
                message_id: message_id.clone(),
                encryption_context: encryption_context.clone(),
                encrypted_data_keys: encrypted_data_keys.into(),
                //= specification/client-apis/encrypt.md#un-framed-message-body-encryption
                //= type=implication
                //# Implementations of the AWS Encryption SDK MUST NOT encrypt using the Non-Framed content type.
                content_type: ContentType::Framed,
                frame_length,
                suite_data: suite_data.unwrap(),
            }))
        }
        aws_mpl_legacy::suites::DerivationAlgorithm::Identity => Err("Validation Error 2".into()),
        //= specification/client-apis/encrypt.md#v1-header
        //# If the message format version associated with the [algorithm suite](../framework/algorithm-suites.md#supported-algorithm-suites) is 1.0
        //# then the [message header body](../data-format/message-header.md#header-body-version-1-0) MUST be serialized with the following specifics:
        _ => Ok(HeaderBody::V1Body(V1HeaderBody {
            message_type: MessageType::TypeCustomerAed,
            algorithm_suite: suite.clone(),
            message_id: message_id.clone(),
            encryption_context: encryption_context.clone(),
            encrypted_data_keys: encrypted_data_keys.into(),
            //= specification/client-apis/encrypt.md#un-framed-message-body-encryption
            //= type=implication
            //# Implementations of the AWS Encryption SDK MUST NOT encrypt using the Non-Framed content type.
            content_type: ContentType::Framed,
            header_iv_length: u64::from(get_iv_length(suite)),
            frame_length,
        })),
    }
}

pub(crate) fn build_header_auth_tag(
    suite: &AlgorithmSuite,
    data_key: &[u8],
    raw_header: &[u8],
    serialized_req_encryption_context: &[u8],
) -> Result<HeaderAuth, Error> {
    let key_length = get_encrypt_key_length(suite);
    if data_key.len() != key_length as usize {
        return Err("Incorrect data key length".into());
    }

    //= specification/client-apis/encrypt.md#authentication-tag
    //# The value of this MUST be the output of the [authenticated encryption algorithm](../framework/algorithm-suites.md#encryption-algorithm)
    //# specified by the [algorithm suite](../framework/algorithm-suites.md), with the following inputs:
    //= specification/client-apis/encrypt.md#authentication-tag
    //# - The IV MUST have a value of 0.
    let iv = vec![0; get_iv_length(suite) as usize];
    let mut auth_tag = Vec::new();
    //= specification/client-apis/encrypt.md#authentication-tag
    //# - The AAD MUST be the concatenation of the serialized [message header body](../data-format/message-header.md#header-body)
    //# and the serialization of encryption context to only authenticate.
    //= specification/client-apis/encrypt.md#authentication-tag
    //# - The cipherkey MUST be the derived data key
    //= specification/client-apis/encrypt.md#authentication-tag
    //# - The plaintext MUST be an empty byte array
    aes_encrypt(
        get_encrypt(suite),
        &iv,
        data_key,
        &[],
        &[raw_header, serialized_req_encryption_context].concat(),
        &mut auth_tag,
    )?;

    Ok(HeaderAuth::AESMac {
        header_iv: iv,
        header_auth_tag: auth_tag,
    })
}

pub(crate) fn validate_suite_data(
    suite: &AlgorithmSuite,
    header: &HeaderBody,
    expected_suite_data: &[u8],
) -> Result<(), Error> {
    if header.suite_data() != expected_suite_data {
        return Err("Commitment key does not match".into());
    }
    //= specification/data-format/message-header.md#algorithm-suite-data
    //# The length of the suite data field MUST be equal to the [Algorithm Suite Data Length](../framework/algorithm-suites.md#algorithm-suite-data-length) value
    //# of the [algorithm suite](../framework/algorithm-suites.md) specified by the [Algorithm Suite ID](#algorithm-suite-id) field.
    if get_hkdf(&suite.commitment).output_key_length != expected_suite_data.len() as u32 {
        return Err("Commitment key is invalid".into());
    }
    Ok(())
}

pub(crate) fn read_and_decrypt_non_framed_message_body(
    r: &mut dyn SafeRead,
    header: &HeaderInfo,
    key: &[u8],
    raw: &mut dyn SafeWrite,
) -> Result<Vec<u8>, Error> {
    //= specification/data-format/message-header.md#frame-length
    //# When the [content type](#content-type) is non-framed, the value of this field MUST be 0.
    if header.body.frame_length() != 0 {
        return Err("Non-framed message contains non-zero frame length.".into());
    }
    let iv = read_vec(r, get_iv_length(&header.suite) as usize, raw)?;
    let enc_content = read_seq_u64_bounded(
        r,
        SAFE_MAX_ENCRYPT,
        "Frame exceeds AES-GCM cryptographic safety for a single key/iv.",
        raw,
    )?;
    let auth_tag = read_vec(r, get_tag_length(&header.suite) as usize, raw)?;
    let mut aad = Vec::new();
    body_aad(
        header.body.message_id(),
        //= specification/data-format/message-body-aad.md#body-aad-content
        //# - [Non-framed data](message-body.md#non-framed-data) MUST use the value `AWSKMSEncryptionClient Single Block`.
        BodyAADContent::SingleBlock,
        //= specification/data-format/message-body-aad.md#sequence-number
        //# For [non-framed data](message-body.md#non-framed-data), the value of this field MUST be `1`.
        NONFRAMED_SEQUENCE_NUMBER,
        //= specification/data-format/message-body-aad.md#content-length
        //# - For [non-framed data](message-body.md#non-framed-data), this value MUST equal the length, in bytes,
        //# of the plaintext data provided to the algorithm for encryption.
        enc_content.len() as u64,
        &mut aad,
    );

    let mut result: Vec<u8> = enc_content.clone();
    aws_mpl_legacy::primitives::aes_decrypt(
        get_encrypt(&header.suite),
        key,
        &enc_content,
        &auth_tag,
        &iv,
        &aad,
        result.as_mut(),
    )?;

    Ok(result)
}
