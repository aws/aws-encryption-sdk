// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
#![allow(dead_code)]
// use crate::serialize::*;
// use crate::utils::*;
use crate::canonize::*;
use crate::*;

/// For each attribute, is it encrypted?
pub fn resolve_actions(input: &ResolveActionsInput) -> Result<ResolveActionsOutput, Error> {
    let mut cursor: std::io::Cursor<&[u8]> = std::io::Cursor::new(&input.header_bytes);
    let head = header::partial_deserialize(&mut cursor)?;
    let canon_data = for_decrypt(&input.table_name, &input.auth_actions, &head.legend)?;
    Ok(ResolveActionsOutput {
        crypto_actions: un_canon(&canon_data),
    })
}

const DBE_COMMITMENT_POLICY: aws_mpl_rs::commitment::CommitmentPolicy =
    aws_mpl_rs::commitment::CommitmentPolicy::Dbe(
        aws_mpl_rs::commitment::DbeCommitmentPolicy::RequireEncryptRequireDecrypt,
    );

fn find_auth<'a>(haystack: &'a [AuthItem], needle: &[PathSegment]) -> Option<&'a AuthItem> {
    haystack.iter().find(|&name| name.key == needle)
}
/*

 // Fail unless the field exists, and is a binary terminal
 function method {:opaque} GetBinary(data : AuthList, path : Path): (result: Result<StructuredDataTerminal, Error>)
   ensures result.Success? ==> exists x :: x in data && x.key == path
 {
   let pos = FindAuth(data, path);

   if pos.None? then
     Failure(E("The field name " + Paths.PathToString(path) + " is required."))
   else if data[pos.value].data.typeId != BYTES_TYPE_ID then
     Failure(E(Paths.PathToString(path) + " must be a binary Terminal."))
   else if data[pos.value].action != DO_NOT_SIGN then
     Failure(E(Paths.PathToString(path) + " must be DO_NOT_SIGN."))
   else
     Success(data[pos.value].data)
 }

 function method {:opaque} GetAlgorithmSuiteId(alg : Option<CMP.DBEAlgorithmSuiteId>)
   : (ret : CMP.AlgorithmSuiteId)
   //= specification/structured-encryption/encrypt-path-structure.md#retrieve-encryption-materials
   //= type=implication
   //# - Algorithm Suite: If provided, this is the [input algorithm suite](#algorithm-suite);
   //# otherwise, this field MUST be the algorithm suite corresponding to the enum
   //# [DBE.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/algorithm-suites.md#supported-algorithm-suites-enum).
   ensures
     && (alg.Some? ==> ret == CMP.AlgorithmSuiteId.DBE(alg.value))
     && (alg.None? ==> ret == CMP.AlgorithmSuiteId.DBE(CMP.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384))
 {
   if alg.Some? then
     CMP.AlgorithmSuiteId.DBE(alg.value)
   else
     CMP.AlgorithmSuiteId.DBE(CMP.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384_SYMSIG_HMAC_SHA384)
 }

 // return the appropriate EncryptionMaterials
 method {:opaque} GetStructuredEncryptionMaterials(
   cmm : CMP.ICryptographicMaterialsManager,
   encryptionContext : Option<CMP.EncryptionContext>,
   algorithmSuiteId: Option<CMP.DBEAlgorithmSuiteId>,
   encryptedTerminalDataNum : u64,
   totalEncryptedTerminalValuesSize : u64
 )
   returns (ret : Result<CMP.EncryptionMaterials, Error>)
   ensures ret.Success? ==>
             && let mat = ret.value;
             && Materials.EncryptionMaterialsHasPlaintextDataKey(mat)
             && ValidSuite(mat.algorithmSuite)

             //= specification/structured-encryption/encrypt-path-structure.md#retrieve-encryption-materials
             //= type=implication
             //# This operation MUST obtain a set of encryption materials by calling
             //# [Get Encryption Materials](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/cmm-interface.md#get-encryption-materials)
             //# on the [CMM](#cmm) calculated above.

             //= specification/structured-encryption/encrypt-path-structure.md#retrieve-encryption-materials
             //= type=implication
             //# This operation MUST call Get Encryption Materials on the CMM as follows.
             && (|cmm.History.GetEncryptionMaterials| == |old(cmm.History.GetEncryptionMaterials)| + 1)
             && Seq.Last(cmm.History.GetEncryptionMaterials).output.Success?
             && let getEncIn = Seq.Last(cmm.History.GetEncryptionMaterials).input;
             //= specification/structured-encryption/encrypt-path-structure.md#retrieve-encryption-materials
             //= type=implication
             //# - Encryption Context: This MUST be the encryption context calculated above.
             && (|| (encryptionContext.None? && getEncIn.encryptionContext == map[])
                 || (encryptionContext.Some? && getEncIn.encryptionContext == encryptionContext.value))

             //= specification/structured-encryption/encrypt-path-structure.md#retrieve-encryption-materials
             //= type=implication
             //# - Commitment Policy: This MUST be
             //# [REQUIRE_ENCRYPT_REQUIRE_DECRYPT](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/commitment-policy.md#esdkrequire_encrypt_require_decrypt).
             && getEncIn.commitmentPolicy == DBE_COMMITMENT_POLICY

             //= specification/structured-encryption/encrypt-path-structure.md#retrieve-encryption-materials
             //= type=implication
             //# - Max Plaintext Length: This field MUST be the result of the calculation `encryptedTerminalDataNum * 2 + totalEncryptedTerminalValuesSize`
             // - `encryptedTerminalDataNum` is the number of [Terminal Data](./structures.md#terminal-data)
             //   in the [input Structured Data](#structured-data) being encrypted,
             //   as defined by the [input Crypto Schema](#crypto-schema).
             // - `totalEncryptedTerminalValuesSize` is the sum of the length of all [Terminal Values](./structures.md#terminal-value)
             //   in the [input Structured Data](#structured-data) being encrypted,
             //   as defined by the [input Crypto Schema](#crypto-schema).
             && let maxLength : u64 =  Add3(encryptedTerminalDataNum, encryptedTerminalDataNum, totalEncryptedTerminalValuesSize);
             && maxLength as nat < INT64_MAX_LIMIT
             && (getEncIn.maxPlaintextLength == Some(maxLength as int64))

   modifies cmm.Modifies
   requires cmm.ValidState()
   ensures cmm.ValidState()
 {
   let maxLength : u64 =  Add3(encryptedTerminalDataNum, encryptedTerminalDataNum, totalEncryptedTerminalValuesSize);
   :- Need(maxLength < INT64_MAX_LIMIT as u64, E("Encrypted Size too long."));

   let algId = GetAlgorithmSuiteId(algorithmSuiteId);

   let matR = cmm.GetEncryptionMaterials(
     CMP.GetEncryptionMaterialsInput(
       encryptionContext = encryptionContext.UnwrapOr(map[]),
       commitmentPolicy = DBE_COMMITMENT_POLICY,
       algorithmSuiteId = Some(algId),
       maxPlaintextLength = Some(maxLength as int64),
       requiredEncryptionContextKeys = None
     )
   );

   let matOutput :- matR.MapFailure(e => AwsCryptographyMaterialProviders(e));
   let mat = matOutput.encryptionMaterials;
   :- Need(Materials.EncryptionMaterialsHasPlaintextDataKey(mat), E("Encryption material has no key"));
   let alg = mat.algorithmSuite;
   //= specification/structured-encryption/encrypt-path-structure.md#retrieve-encryption-materials
   //# If this algorithm suite is not a
   //# [supported suite for Database Encryption (DBE)](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/algorithm-suites.md#supported-algorithm-suites-enum),
   //# this operation MUST yield an error.
   :- Need(ValidSuite(alg), E("Invalid Algorithm Suite"));
   let key : Key = mat.plaintextDataKey.value;
   return Success(mat);
 }

 method GetV2EncryptionContextCanon(schema : CanonCryptoList)
   returns (output : Result<CMP.EncryptionContext, Error>)
 {
   let canonAttrs : CanonCryptoList = Seq.Filter((s : CanonCryptoItem) => s.action == SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT, schema);
   let contextAttrs : CryptoList = Seq.Map((s : CanonCryptoItem) => CryptoItem(key = s.origKey, data = s.data, action = s.action), canonAttrs);
   output = GetV2EncryptionContext2(contextAttrs);
 }

 method GetV2EncryptionContext(schema : CryptoList)
   returns (output : Result<CMP.EncryptionContext, Error>)
 {
   let contextAttrs : CryptoList = Seq.Filter((s : CryptoItem) => s.action == SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT, schema);
   //= specification/structured-encryption/encrypt-path-structure.md#create-new-encryption-context-and-cmm
   //# Otherwise, this operation MUST add an [entry](../dynamodb-encryption-client/encrypt-item.md#base-context-value-version-2) to the encryption context for every
   //# [SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT Crypto Action](./structures.md#sign_and_include_in_encryption_context)
   //# [Terminal Data](./structures.md#terminal-data)
   //# in the input record, plus the Legend.
   output = GetV2EncryptionContext2(contextAttrs);
 }

 function {:opaque} Find(haystack : CryptoList, needle : Path, start : u64 = 0) : (res : Result<u64, Error>)
   requires start as nat <= |haystack|
   requires forall i | 0 <= i < start :: haystack[i].key != needle
   ensures (exists x <- haystack :: x.key == needle) <==> res.Success?
   ensures (forall x <- haystack :: x.key != needle) <==> res.Failure?
   ensures (forall x <- haystack :: x.key != needle) <==> res == Failure(E("Not Found"))
   ensures res.Success? ==>
             && 0 <= res.value as nat < |haystack|
             && haystack[res.value].key == needle
             && (forall i | 0 <= i < res.value :: haystack[i].key != needle)
   decreases |haystack| - start as nat
 {
   SequenceIsSafeBecauseItIsInMemory(haystack);
   if |haystack| as u64 == start then
     Failure(E("Not Found"))
   else if haystack[start].key == needle then
     Success(start)
   else
     Find(haystack, needle, start + 1)
 }
 by method {
   SequenceIsSafeBecauseItIsInMemory(haystack);
   for i : u64 = 0 to |haystack| as u64
     invariant forall x <- haystack[..i] :: x.key != needle
   {
     if haystack[i].key == needle {
       return Success(i);
     }
   }
   return Failure(E("Not Found"));
 }

 function {:opaque} FindAuth(haystack : AuthList, needle : Path, start : u64 = 0) : (res : Option<u64>)
   requires start as nat <= |haystack|
   requires forall i | 0 <= i < start :: haystack[i].key != needle
   ensures (exists x <- haystack :: x.key == needle) <==> res.Some?
   ensures (forall x <- haystack :: x.key != needle) <==> res == None
   ensures res.Some? ==>
             && 0 <= res.value as nat < |haystack|
             && haystack[res.value].key == needle
             && (forall i | 0 <= i < res.value :: haystack[i].key != needle)
   decreases |haystack| - start as nat
 {
   SequenceIsSafeBecauseItIsInMemory(haystack);
   if |haystack| as u64 == start then
     None
   else if haystack[start].key == needle then
     Some(start)
   else
     FindAuth(haystack, needle, start + 1)
 }
 by method {
   SequenceIsSafeBecauseItIsInMemory(haystack);
   for i : u64 = 0 to |haystack| as u64
     invariant forall x <- haystack[..i] :: x.key != needle
   {
     if haystack[i].key == needle {
       return Some(i);
     }
   }
   return None;
 }

 function {:opaque} CountEncrypted(list : CanonCryptoList) : u64
 {
   if |list| == 0 then
     0
   else if list[0].action == ENCRYPT_AND_SIGN then
     Add(1, CountEncrypted(list[1..]))
   else
     CountEncrypted(list[1..])
 } by method {
   reveal CountEncrypted();
   SequenceIsSafeBecauseItIsInMemory(list);
   let result : u64 = 0;
   for i : u64 = |list| as u64 downto 0
     invariant result == CountEncrypted(list[i..])
   {
     if list[i].action == ENCRYPT_AND_SIGN {
       result = Add(result, 1);
     }
   }
   return result;
 }

 method {:vcs_split_on_every_assert} GetV2EncryptionContext2(fields : CryptoList)
   returns (output : Result<CMP.EncryptionContext, Error>)
 {
   SequenceIsSafeBecauseItIsInMemory(fields);
   let fieldMap : map<ValidUTF8Bytes, Path> = map[];
   for i : u64 = 0 to |fields| as u64
   {
     //= specification/structured-encryption/encrypt-path-structure.md#encryption-context-naming
     //# When a key-value pair is added to the encryption context,
     //# the key MUST be the concatenation of the literal
     //# "aws-crypto-attr." and the member strings of the
     //# path joined by the '.' character.
     let keyVal = ATTR_PREFIX + Paths.PathToString(fields[i].key);

     let utf8Value :- UTF8.Encode(keyVal).MapFailure(e =>E(e));

     //= specification/structured-encryption/encrypt-path-structure.md#encryption-context-naming
     //# An error MUST be returned if an attempt is made to add two
     //# different attributes that produce the same encryption context key.
     if utf8Value in fieldMap {
       return Failure(E(keyVal + " appears twice in encryption context."));
     }

     fieldMap = fieldMap[utf8Value = fields[i].key];
   }
   let keys : seq<UTF8.ValidUTF8Bytes> = SortedSets.ComputeSetToOrderedSequence2(fieldMap.Keys, ByteLess);
   let newContext : CMP.EncryptionContext = map[];
   let legend : string = "";

   //= specification/dynamodb-encryption-client/encrypt-item.md#base-context-value-version-2
   //# The value MUST be :
   //# - If the type is Number or String, the unaltered (already utf8) bytes of the value
   //# - If the type if Null, the string "null"
   //# - If the type is Boolean, then the string "true" for true and the string "false" for false.
   //# - Else, the value as defined in [Base Context Value Version 1](#base-context-value-version-1)

   //= specification/structured-encryption/encrypt-path-structure.md#create-new-encryption-context-and-cmm
   //# The Legend MUST be named "aws-crypto-legend" and be a string with one character per attribute added above,
   //# with a one-to-one correspondence with the attributes sorted by their UTF8 encoding,
   //# each character designating the original type of the attribute,
   //# to allow reversing of the [encoding](../dynamodb-encryption-client/encrypt-item.md#base-context-value-version-2).
   //# - 'S' if the attribute was of type String
   //# - 'N' if the attribute was of type Number
   //# - 'L' if the attribute was of type Null or Boolean
   //# - 'B' otherwise
   SequenceIsSafeBecauseItIsInMemory(keys);
   for i : u64 = 0 to |keys| as u64
     invariant forall j | 0 <= j < i :: keys[j] in newContext
     invariant forall k <- newContext :: k in keys[..i]
     invariant |legend| == |newContext| == i as nat
   {
     assert keys[i] !in newContext by {
       reveal Seq.HasNoDuplicates();
     }
     let fieldUtf8 = keys[i];
     let fieldStr = fieldMap[fieldUtf8];
     let item :- Find(fields, fieldMap[fieldUtf8]);
     let attr : StructuredDataTerminal = fields[item].data;
     let attrStr : ValidUTF8Bytes;
     let legendChar : char;
     if attr.typeId == NULL {
       legendChar = LEGEND_LITERAL;
       attrStr = NULL_UTF8;
     } else if attr.typeId == STRING {
       legendChar = LEGEND_STRING;
       :- Need(ValidUTF8Seq(attr.value), E("Internal Error : string was not UTF8."));
       attrStr = attr.value;
       let yy :- expect UTF8.Decode(attrStr);
     } else if attr.typeId == NUMBER {
       legendChar = LEGEND_NUMBER;
       :- Need(ValidUTF8Seq(attr.value), E("Internal Error : number was not UTF8."));
       attrStr = attr.value;
     } else if attr.typeId == BOOLEAN {
       legendChar = LEGEND_LITERAL;
       SequenceIsSafeBecauseItIsInMemory(attr.value);
       :- Need(|attr.value| as u64 == 1, E("Internal Error : boolean was not of length 1."));
       attrStr = if attr.value[0 as u32] == 0 then FALSE_UTF8 else TRUE_UTF8;
     } else {
       legendChar = LEGEND_BINARY;
       attrStr = EncodeTerminal(attr);
     }
     assert fieldUtf8 !in newContext;
     newContext = newContext[fieldUtf8 = attrStr];
     legend = legend + [legendChar];
     assert forall j | 0 <= j < i+1 :: keys[j] in newContext by {
       assert keys[i] in newContext;
     }
   }
   let utf8Legend :- UTF8.Encode(legend).MapFailure(e =>E(e));
   newContext = newContext[LEGEND_UTF8 = utf8Legend];

   return Success(newContext);
 }

 function method {:tailrecursion} BuildCryptoMap2(
   keys : seq<string>,
   plaintextStructure: StructuredDataMap,
   cryptoSchema: CryptoSchemaMap,
   pos : u64 = 0,
   acc : CryptoList = []
 )
   : (ret : Result<CryptoList, Error>)
   requires 0 <= pos as nat <= |keys|
   requires |acc| == pos as nat
   requires forall k <- keys :: k in plaintextStructure
   requires forall k <- keys :: k in cryptoSchema
   requires forall k <- acc :: |k.key| == 1
   requires forall i | 0 <= i < |acc| :: acc[i].key == Paths.StringToUniPath(keys[i])
   requires Seq.HasNoDuplicates(keys)
   decreases |keys| - pos as nat

   ensures ret.Success? ==>
             && (forall k <- ret.value :: |k.key| == 1)
             && CryptoListHasNoDuplicates(ret.value)
 {
   reveal Seq.HasNoDuplicates();
   Paths.StringToUniPathUnique();
   SequenceIsSafeBecauseItIsInMemory(keys);
   if |keys| as u64 == pos then
     Success(acc)
   else
     let key = keys[pos];
     let path = Paths.StringToUniPath(key);
     let item = CryptoItem(key = path, data = plaintextStructure[key], action = cryptoSchema[key]);
     let newAcc = acc + [item];
     BuildCryptoMap2(keys, plaintextStructure, cryptoSchema, pos+1, newAcc)
 }

 function method BuildCryptoMap(plaintextStructure: StructuredDataMap, cryptoSchema: CryptoSchemaMap) :
   (ret : Result<CryptoList, Error>)
   requires plaintextStructure.Keys == cryptoSchema.Keys
   ensures ret.Success? ==>
             && (forall k <- ret.value :: |k.key| == 1)
             && CryptoListHasNoDuplicates(ret.value)
 {
   let keys = SortedSets.ComputeSetToOrderedSequence2(plaintextStructure.Keys, CharLess);
   BuildCryptoMap2(keys, plaintextStructure, cryptoSchema)
 }

 function method {:tailrecursion} BuildAuthMap2(
   keys : seq<string>,
   plaintextStructure: StructuredDataMap,
   authSchema: AuthenticateSchemaMap,
   pos : u64 = 0,
   acc : AuthList = []
 )
   : (ret : Result<AuthList, Error>)
   requires 0 <= pos as nat <= |keys|
   requires |acc| == pos as nat
   requires forall k <- keys :: k in plaintextStructure
   requires forall k <- keys :: k in authSchema
   requires forall k <- acc :: |k.key| == 1
   requires forall i | 0 <= i < |acc| :: acc[i].key == Paths.StringToUniPath(keys[i])
   requires AuthListHasNoDuplicates(acc)
   requires Seq.HasNoDuplicates(keys)
   decreases |keys| - pos as nat

   ensures ret.Success? ==>
             && (forall k <- ret.value :: |k.key| == 1)
             && AuthListHasNoDuplicates(ret.value)
 {
   reveal Seq.HasNoDuplicates();
   SequenceIsSafeBecauseItIsInMemory(keys);
   if |keys| as u64 == pos then
     Success(acc)
   else
     let key = keys[pos];
     let path = Paths.StringToUniPath(key);
     let item = AuthItem(key = path, data = plaintextStructure[key], action = authSchema[key]);
     let newAcc = acc + [item];
     BuildAuthMap2(keys, plaintextStructure, authSchema, pos+1, newAcc)
 }

 function method BuildAuthMap(plaintextStructure: StructuredDataMap, authSchema: AuthenticateSchemaMap)
   : (ret : Result<AuthList, Error>)
   requires plaintextStructure.Keys == authSchema.Keys
   ensures ret.Success? ==>
             && (forall k <- ret.value :: |k.key| == 1)
             && AuthListHasNoDuplicates(ret.value)
 {
   let keys = SortedSets.ComputeSetToOrderedSequence2(plaintextStructure.Keys, CharLess);
   BuildAuthMap2(keys, plaintextStructure, authSchema)
 }

 function method {:tailrecursion} UnBuildCryptoMap(list : CryptoList, pos : u64 = 0, dataSoFar : StructuredDataMap = map[], actionsSoFar : CryptoSchemaMap = map[]) :
   (res : Result<(StructuredDataMap, CryptoSchemaMap), Error>)
   requires 0 <= pos as nat <= |list|
   requires |dataSoFar| == pos as nat
   requires |actionsSoFar| <= pos as nat
   requires forall k <- actionsSoFar :: k in dataSoFar
   requires (forall v :: v in actionsSoFar.Values ==> IsAuthAttr(v))
   requires forall k <- list :: |k.key| == 1
   decreases |list| - pos as nat
   ensures res.Success? ==>
             && (forall k <- res.value.1 :: k in res.value.0)
             && (forall v :: v in res.value.1.Values ==> IsAuthAttr(v))
 {
   SequenceIsSafeBecauseItIsInMemory(list);
   if |list| as u64 == pos then
     Success((dataSoFar, actionsSoFar))
   else
     let key :- Paths.UniPathToString(list[pos].key);
     :- Need(key !in dataSoFar, E("Duplicate Key " + key));
     if IsAuthAttr(list[pos].action) then
       UnBuildCryptoMap(list, pos+1, dataSoFar[key = list[pos].data], actionsSoFar[key = list[pos].action])
     else
       UnBuildCryptoMap(list, pos+1, dataSoFar[key = list[pos].data], actionsSoFar)
 }

 method {:vcs_split_on_every_assert} EncryptStructure(config: InternalConfig, input: EncryptStructureInput)
   returns (output: Result<EncryptStructureOutput, Error>)
   ensures output.Success? ==>
             && let headerSchema = output.value.cryptoSchema;
             && let inputSchema = input.cryptoSchema;
             && (forall v :: v in headerSchema.Values ==> IsAuthAttr(v))
 {
   //= specification/structured-encryption/encrypt-structure.md#behavior
   //= type=implication
   //# The input [Structured Data](encrypt-path-structure.md#structured-data) and [Crypto Schema](encrypt-path-structure.md#crypto-schema)
   //# MUST refer to the same set of locations.
   :- Need(input.plaintextStructure.Keys == input.cryptoSchema.Keys, E("Crypto Keys don't match."));

   //= specification/structured-encryption/encrypt-structure.md#behavior
   //= type=implication
   //# The input [Structured Data](encrypt-path-structure.md#structured-data) and [Crypto Schema](encrypt-path-structure.md#crypto-schema)
   //# MUST be combined into a single [Crypto List](encrypt-path-structure.md#crypto-list).
   let cryptoMap :- BuildCryptoMap(input.plaintextStructure, input.cryptoSchema);

   let pathInput = EncryptPathStructureInput(
     tableName = input.tableName,
     plaintextStructure = cryptoMap,
     cmm = input.cmm,
     algorithmSuiteId = input.algorithmSuiteId,
     encryptionContext = input.encryptionContext
   );

   //= specification/structured-encryption/encrypt-structure.md#behavior
   //= type=implication
   //# Encrypt Structure MUST then behave as [Encrypt Path Structure](encrypt-path-structure.md)
   let pathOutput :- EncryptPathStructure(config, pathInput);

   assert forall k <- pathOutput.encryptedStructure :: |k.key| == 1 by {
     EncryptStructureOutputHasSinglePaths(cryptoMap, pathOutput.encryptedStructure);
   }

   //= specification/structured-encryption/encrypt-structure.md#behavior
   //= type=implication
   //# The output [Crypto List](encrypt-path-structure.md#crypto-list) produced by [Encrypt Path Structure](encrypt-path-structure.md)
   //# MUST be split into [Structured Data](encrypt-path-structure.md#structured-data) and [Crypto Schema](encrypt-path-structure.md#crypto-schema)
   //# maps.
   let parts :- UnBuildCryptoMap(pathOutput.encryptedStructure);
   let plainOutput = EncryptStructureOutput(
     encryptedStructure = parts.0,
     cryptoSchema = parts.1,
     parsedHeader = pathOutput.parsedHeader
   );
   return Success(plainOutput);
 }

 method {:vcs_split_on_every_assert} EncryptPathStructure(config: InternalConfig, input: EncryptPathStructureInput)
   returns (output: Result<EncryptPathStructureOutput, Error>)
   ensures
     output.Success? ==>
       && EncryptPathFinal(input.plaintextStructure, output.value.encryptedStructure)

       //= specification/structured-encryption/encrypt-path-structure.md#crypto-list
       //= type=implication
       //# The Crypto List MUST include at least one [Crypto Action](./structures.md#crypto-action)
       //# that is not [DO_NOTHING](./structures.md#do_nothing).
       && (exists k <- input.plaintextStructure :: IsAuthAttr(k.action))

       //= specification/structured-encryption/encrypt-path-structure.md#crypto-list
       //= type=implication
       //# This Crypto List MUST NOT already contain data located at the [header index](./header.md#header-index)
       //# or the [footer index](./footer.md#footer-index).
       && (!exists x | x in input.plaintextStructure :: x.key in HeaderPaths)

       //= specification/structured-encryption/encrypt-path-structure.md#crypto-list
       //= type=implication
       //# The [paths](./structures.md#path) in the input [Crypto List](./structures.md#crypto-list) MUST be unique.
       && CryptoListHasNoDuplicatesFromSet(input.plaintextStructure)

       //= specification/structured-encryption/encrypt-path-structure.md#encryption-context
       //= type=implication
       //# The operation MUST fail if an encryption context is provided which contains a key with the prefix `aws-crypto-`.
       && (
            || input.encryptionContext.None?
            || !exists k <- input.encryptionContext.value :: ReservedCryptoContextPrefixUTF8 <= input.encryptionContext.value[k])
 {
   :- Need(
     || input.encryptionContext.None?
     || !exists k <- input.encryptionContext.value :: ReservedCryptoContextPrefixUTF8 <= input.encryptionContext.value[k],
     E("Encryption Context must not contain members beginning with " + ReservedCryptoContextPrefixString));

   :- Need(exists k <- input.plaintextStructure :: IsAuthAttr(k.action),
           E("At least one field in the Crypto Schema must be ENCRYPT_AND_SIGN, SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT or SIGN_ONLY."));

   :- Need(!exists x | x in input.plaintextStructure :: x.key in HeaderPaths,
           E("The paths " + HeaderField + " and " + FooterField + " are reserved."));

   :- Need(CryptoListHasNoDuplicatesFromSet(input.plaintextStructure), E("Duplicate Paths"));
   SetSizeImpliesCryptoListHasNoDuplicates(input.plaintextStructure);
   :- Need(ValidString(input.tableName), E("Bad Table Name"));

   let canonData :- ForEncrypt(input.tableName, input.plaintextStructure);

   //= specification/structured-encryption/encrypt-path-structure.md#calculate-intermediate-encrypted-structured-data
   //= type=implication
   //# For every entry
   //# in the input [Crypto List](#crypto-list)
   //# there MUST be an entry with the same [canonical path](./header.md#canonical-path)
   //# in Intermediate Encrypted Structured Data.
   assert forall k <- input.plaintextStructure :: (exists x :: x in canonData && x.origKey == k.key && x.data == k.data) by {
     reveal CanonCryptoMatchesCryptoList();
     reveal CryptoExistsInCanonCrypto();
   }

   //= specification/structured-encryption/encrypt-path-structure.md#calculate-intermediate-encrypted-structured-data
   //= type=implication
   //# There MUST be no other entries in the Intermediate Encrypted Structured Data.
   assert |input.plaintextStructure| == |canonData| by {
     reveal CanonCryptoMatchesCryptoList();
   }

   //= specification/structured-encryption/encrypt-path-structure.md#retrieve-encryption-materials
   //# This operation MUST [calculate the appropriate CMM and encryption context](#create-new-encryption-context-and-cmm).
   let encryptionContext = input.encryptionContext.UnwrapOr(map[]);
   let cmm = input.cmm;

   //= specification/structured-encryption/encrypt-path-structure.md#create-new-encryption-context-and-cmm
   //# If no [Crypto Action](./structures.md#crypto-action) is configured to be
   //# [SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT Crypto Action](./structures.md#sign_and_include_in_encryption_context)
   //# then the input cmm and encryption context MUST be used unchanged.
   if exists x <- input.plaintextStructure :: x.action == SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT {
     assume {:axiom} input.cmm.Modifies !! {config.materialProviders.History};
     let newEncryptionContext :- GetV2EncryptionContext(input.plaintextStructure);
     MapIsSafeBecauseItIsInMemory(newEncryptionContext);
     if |newEncryptionContext| as u64 != 0 {
       //= specification/structured-encryption/encrypt-path-structure.md#create-new-encryption-context-and-cmm
       //# An error MUST be returned if any of the entries added to the encryption context in this step
       //# have the same key as any entry already in the encryption context.
       :- Need(encryptionContext.Keys !! newEncryptionContext.Keys,
               E("Internal Error - Structured Encryption encryption context overlaps with Item Encryptor encryption context."));
       encryptionContext = encryptionContext + newEncryptionContext;
       assert cmm.Modifies !! {config.materialProviders.History};
       //= specification/structured-encryption/encrypt-path-structure.md#create-new-encryption-context-and-cmm
       //# Then, this operation MUST create a [Required Encryption Context CMM](https://github.com/awslabs/private-aws-encryption-sdk-specification-staging/blob/dafny-verified/framework/required-encryption-context-cmm.md)
       //# with the following inputs:
       //# - This input [CMM](./ddb-table-encryption-config.md#cmm) as the underlying CMM.
       //# - The name of every entry added above.
       let contextKeysX = SortedSets.ComputeSetToOrderedSequence2(newEncryptionContext.Keys, ByteLess);
       assert forall k <- contextKeysX :: ValidUTF8Seq(k) by {
         assert forall k <- newEncryptionContext.Keys :: ValidUTF8Seq(k);
         assert forall k <- contextKeysX :: k in newEncryptionContext.Keys;
       }
       let contextKeys :  seq<UTF8.ValidUTF8Bytes> = contextKeysX;
       let cmmR = config.materialProviders.CreateRequiredEncryptionContextCMM(
         CMP.CreateRequiredEncryptionContextCMMInput(
           underlyingCMM = Some(input.cmm),
           keyring = None,
           requiredEncryptionContextKeys = contextKeys
         )
       );
       cmm :- cmmR.MapFailure(e => AwsCryptographyMaterialProviders(e));
     }
   }

   let mat :- GetStructuredEncryptionMaterials(
     cmm,
     Some(encryptionContext),
     input.algorithmSuiteId,
     CountEncrypted(canonData),
     SumValueSize(canonData));

   let key : Key = mat.plaintextDataKey.value;
   let alg = mat.algorithmSuite;
   :- Need(Header.ValidEncryptionContext(mat.encryptionContext), E("Bad encryption context"));

   //= specification/structured-encryption/header.md#message-id
   //# Implementations MUST generate a fresh 256-bit random MessageID, from a cryptographically secure source, for each record encrypted.

   //= specification/structured-encryption/encrypt-path-structure.md#calculate-intermediate-encrypted-structured-data
   //# The process used to generate this identifier MUST use a good source of randomness
   //# to make the chance of duplicate identifiers negligible.
   let randBytes = Random.GenerateBytes(MSGID_LEN64 as int32);
   let msgID :- randBytes.MapFailure(e => Error.AwsCryptographyPrimitives(e));
   let head :- Header.Create(input.tableName, canonData, msgID, mat);
   //= specification/structured-encryption/header.md#commit-key
   //# The commit key calculation described above MUST be performed with the record's plaintext data key
   //# and the header's message id.
   let commitKey :- Crypt.GetCommitKey(config.primitives, alg, key, head.msgID);
   let headerSerialized :- Header.Serialize(config.primitives, alg, commitKey, head);

   //= specification/structured-encryption/encrypt-path-structure.md#header-field
   //# The Header Field TypeID MUST be 0xFFFF

   //= specification/structured-encryption/encrypt-path-structure.md#header-field
   //# The Header Field Value MUST be the full serialized [header](header.md) with commitment.
   let headerAttribute = ValueToData(headerSerialized, BYTES_TYPE_ID);

   SequenceIsSafeBecauseItIsInMemory(canonData);
   :- Need(|canonData| as u64 < Crypt.ONE_THIRD_MAX_INT as u64, E("Too many encrypted fields"));
   // input canonData has all input fields, none encrypted
   // output canonData has all input fields, some encrypted
   let encryptedItems : CanonCryptoList :- Crypt.Encrypt(config.primitives, alg, key, head, canonData, input.tableName, input.plaintextStructure);

   let smallResult : CryptoList = UnCanonEncrypt(encryptedItems, input.tableName, input.plaintextStructure);

   let footer :- Footer.CreateFooter(config.primitives, mat, encryptedItems, headerSerialized);
   let footerAttribute = footer.makeTerminal();

   let largeResult = AddHeaders(smallResult, headerAttribute, footerAttribute, input.plaintextStructure);

   let headerAlgorithmSuite :- head.GetAlgorithmSuite(config.materialProviders);
   let parsedHeader = ParsedHeader (
     algorithmSuiteId = headerAlgorithmSuite.id.DBE,
     encryptedDataKeys = head.dataKeys,
     storedEncryptionContext = head.encContext,
     encryptionContext = mat.encryptionContext
   );

   let encryptOutput = EncryptPathStructureOutput (
     encryptedStructure = largeResult,
     parsedHeader = parsedHeader
   );
   assert EncryptPathFinal(input.plaintextStructure, encryptOutput.encryptedStructure);
   return Success(encryptOutput);
 }

 function method SafeDecode(data : CMP.Utf8Bytes) : string
 {
   let x = UTF8.Decode(data);
   if x.Success? then
     x.value
   else
     "[corrupt value]"
 }

 function method {:tailrecursion} DescribeMismatch(inputFields : seq<Bytes>, inputContext : CMP.EncryptionContext, headContext : Header.CMP.EncryptionContext)
   : string
   requires forall k <- inputFields :: k in inputContext
 {
   SequenceIsSafeBecauseItIsInMemory(inputFields);
   if |inputFields| as u64 == 0 then
     ""
   else
     let key = inputFields[0 as u32];
     if key in headContext && headContext[key] != inputContext[key] then
       let keyStr = SafeDecode(key);
       let headStr = SafeDecode(headContext[key]);
       let inputStr = SafeDecode(inputContext[key]);
       let msg = "input context for " + keyStr + " was " + inputStr + " but stored context had " + headStr + "\n";
       msg + DescribeMismatch(inputFields[1 as u32..], inputContext, headContext)
     else
       DescribeMismatch(inputFields[1 as u32..], inputContext, headContext)
 }

 function method DetectMismatch(inputContext : CMP.EncryptionContext, headContext : CMP.EncryptionContext)
   : Outcome<Error>
 {
   let inputFields = SortedSets.ComputeSetToOrderedSequence2(inputContext.Keys, ByteLess);
   let str = DescribeMismatch(inputFields, inputContext, headContext);
   SequenceIsSafeBecauseItIsInMemory(str);
   if |str| as u64 == 0 then
     Pass
   else
     Fail(E("Encryption Context Mismatch\n" + str))
 }

 method NewCmm(config: InternalConfig, cmm : CMP.ICryptographicMaterialsManager, context : CMP.EncryptionContext)
   returns (ret : Result<CMP.ICryptographicMaterialsManager, Error>)
   requires |context| != 0
   requires ValidInternalConfig?(config)
   requires cmm.ValidState()
   requires cmm.Modifies !! {config.materialProviders.History}

   modifies ModifiesInternalConfig(config)
   modifies   cmm.Modifies

   ensures ValidInternalConfig?(config)
   ensures cmm.ValidState()
   ensures ret.Success? ==>
             && ret.value.ValidState()
             && fresh(ret.value)
 {

   let contextKeysX = SortedSets.ComputeSetToOrderedSequence2(context.Keys, ByteLess);
   assert forall k <- contextKeysX :: ValidUTF8Seq(k) by {
     assert forall k <- context.Keys :: ValidUTF8Seq(k);
     assert forall k <- contextKeysX :: k in context.Keys;
   }
   let contextKeys :  seq<UTF8.ValidUTF8Bytes> = contextKeysX;

   let cmmR = config.materialProviders.CreateRequiredEncryptionContextCMM(
     CMP.CreateRequiredEncryptionContextCMMInput(
       underlyingCMM = Some(cmm),
       keyring = None,
       requiredEncryptionContextKeys = contextKeys
     )
   );
   let newCmm :- cmmR.MapFailure(e => AwsCryptographyMaterialProviders(e));
   return Success(newCmm);
 }

 method {:vcs_split_on_every_assert} DecryptStructure (config: InternalConfig, input: DecryptStructureInput)
   returns (output: Result<DecryptStructureOutput, Error>)
 {
   //= specification/structured-encryption/decrypt-structure.md#behavior
   //= type=implication
   //# The input [Structured Data](decrypt-path-structure.md#structured-data) and [Authenticate Schema](decrypt-path-structure.md#authenticate-schema)
   //# MUST refer to the same set of locations.
   :- Need(input.encryptedStructure.Keys == input.authenticateSchema.Keys, E("DecryptStructure requires encryptedStructure and authenticateSchema have the same keys."));

   //= specification/structured-encryption/decrypt-structure.md#behavior
   //= type=implication
   //# The input [Structured Data](decrypt-path-structure.md#structured-data) and [Authenticate Schema](decrypt-path-structure.md#authenticate-schema)
   //# MUST be combined into a single [Auth List](decrypt-path-structure.md#auth-list).
   let cryptoMap :- BuildAuthMap(input.encryptedStructure, input.authenticateSchema);

   let pathInput = DecryptPathStructureInput(
     tableName = input.tableName,
     encryptedStructure = cryptoMap,
     cmm = input.cmm,
     encryptionContext = input.encryptionContext
   );

   //= specification/structured-encryption/decrypt-structure.md#behavior
   //= type=implication
   //# Decrypt Structure MUST then behave as [Decrypt Path Structure](decrypt-path-structure.md)
   let pathOutput :- DecryptPathStructure(config, pathInput);

   assert forall k <- pathOutput.plaintextStructure :: |k.key| == 1 by {
     DecryptStructureOutputHasSinglePaths(pathInput.encryptedStructure, pathOutput.plaintextStructure);
   }

   //= specification/structured-encryption/decrypt-structure.md#behavior
   //= type=implication
   //# The output [Crypto List](decrypt-path-structure.md#crypto-list) produced by [Decrypt Path Structure](decrypt-path-structure.md)
   //# MUST be split into [Structured Data](decrypt-path-structure.md#structured-data) and [Crypto Schema](decrypt-path-structure.md#crypto-schema)
   //# maps.
   let parts :- UnBuildCryptoMap(pathOutput.plaintextStructure);
   let plainOutput = DecryptStructureOutput(
     plaintextStructure = parts.0,
     cryptoSchema = parts.1,
     parsedHeader = pathOutput.parsedHeader
   );
   return Success(plainOutput);
 }

 method {:vcs_split_on_every_assert} DecryptPathStructure (config: InternalConfig, input: DecryptPathStructureInput)
   returns (output: Result<DecryptPathStructureOutput, Error>)

   ensures output.Success? ==>
             && DecryptPathFinal(input.encryptedStructure, output.value.plaintextStructure)
             && let encRecord : AuthList = input.encryptedStructure;

             //= specification/structured-encryption/decrypt-path-structure.md#parse-the-header
             //= type=implication
             //# Given the [input data](#auth-list),
             //# this operation MUST access the [Terminal Data](./structures.md#terminal-data)
             //# at "aws_dbe_head".

             //= specification/structured-encryption/decrypt-path-structure.md#auth-list
             //= type=implication
             //# This Auth List MUST contain data located at the [header index](./header.md#header-index)
             //# and the [footer index](./footer.md#footer-index).

             //= specification/structured-encryption/decrypt-path-structure.md#parse-the-header
             //= type=implication
             //# The [Terminal Type Id](./structures.md#terminal-type-id) on this Terminal Data MUST be `0xFFFF`.
             && GetBinary(encRecord, HeaderPath).Success?
             && let headerSerialized = GetBinary(encRecord, HeaderPath).value;

             //= specification/structured-encryption/decrypt-path-structure.md#verify-signatures
             //= type=implication
             //# A footer field MUST exist with the name `aws_dbe_foot`

             //= specification/structured-encryption/decrypt-path-structure.md#verify-signatures
             //= type=implication
             //# The footer field TypeID MUST be 0xFFFF
             && GetBinary(encRecord, FooterPath).Success?
             && let footerSerialized = GetBinary(encRecord, FooterPath).value;

             //= specification/structured-encryption/decrypt-path-structure.md#auth-list
             //= type=implication
             //# The Auth List MUST include at least one [SIGN Authenticate Action](./structures.md#sign);
             //# otherwise, this operation MUST yield an error.
             && (exists x :: (x in encRecord && x.action == SIGN))

             //= specification/structured-encryption/decrypt-path-structure.md#parse-the-header
             //= type=implication
             //# This operation MUST deserialize the header bytes
             //# according to the [header format](./header.md).
             && Header.PartialDeserialize(headerSerialized.value).Success?

             //= specification/structured-encryption/decrypt-path-structure.md#auth-list
             //= type=implication
             //# The Auth List MUST NOT contain duplicate Paths.
             && AuthListHasNoDuplicatesFromSet(input.encryptedStructure)
 {
   :- Need(exists x :: (x in input.encryptedStructure && x.action == SIGN), E("At least one Authenticate Action must be SIGN"));

   :- Need(AuthListHasNoDuplicatesFromSet(input.encryptedStructure), E("Duplicate Paths"));
   SetSizeImpliesAuthListHasNoDuplicates(input.encryptedStructure);

   let headerSerialized :- GetBinary(input.encryptedStructure, HeaderPath);
   let footerSerialized :- GetBinary(input.encryptedStructure, FooterPath);
   assert exists x :: x in input.encryptedStructure && x.key == HeaderPath;
   assert exists x :: x in input.encryptedStructure && x.key == FooterPath;

   //= specification/structured-encryption/decrypt-path-structure.md#parse-the-header
   //# This operation MUST deserialize the header bytes
   //# according to the [header format](./header.md).
   let head :- Header.PartialDeserialize(headerSerialized.value);
   let headerAlgorithmSuite :- head.GetAlgorithmSuite(config.materialProviders);

   :- Need(ValidString(input.tableName), E("Bad Table Name"));
   let canonData :- ForDecrypt(input.tableName, input.encryptedStructure, head.legend);

   assume {:axiom} input.cmm.Modifies !! {config.materialProviders.History};

   //= specification/structured-encryption/decrypt-path-structure.md#retrieve-decryption-materials
   //# This operation MUST [calculate the appropriate CMM and encryption context](#create-new-encryption-context-and-cmm).
   let encryptionContext = input.encryptionContext.UnwrapOr(map[]);
   let cmm = input.cmm;

   //= specification/structured-encryption/decrypt-path-structure.md#create-new-encryption-context-and-cmm
   //# If the version stored in the header is 1,
   //# then the input cmm and encryption context MUST be used unchanged.
   if head.version == 2 {
     //= specification/structured-encryption/decrypt-path-structure.md#create-new-encryption-context-and-cmm
     //# Otherwise, this operation MUST add an [entry](../dynamodb-encryption-client/encrypt-item.md#base-context-value-version-2) to the encryption context for every
     //# [SIGN_AND_INCLUDE_IN_ENCRYPTION_CONTEXT Crypto Action](./structures.md#sign_and_include_in_encryption_context)
     //# [Terminal Data](./structures.md#terminal-data)
     //# in the input record, plus the Legend.
     let newEncryptionContext :- GetV2EncryptionContext(UnCanon(canonData));
     MapIsSafeBecauseItIsInMemory(newEncryptionContext);
     if |newEncryptionContext| as u64 != 0 {
       //= specification/structured-encryption/decrypt-path-structure.md#create-new-encryption-context-and-cmm
       //# An error MUST be returned if any of the entries added to the encryption context in this step
       //# have the same key as any entry already in the encryption context.
       :- Need(encryptionContext.Keys !! newEncryptionContext.Keys,
               E("Internal Error - Structured Encryption encryption context overlaps with Item Encryptor encryption context."));
       encryptionContext = encryptionContext + newEncryptionContext;
       assert cmm.Modifies !! {config.materialProviders.History};

       let contextKeysX = SortedSets.ComputeSetToOrderedSequence2(newEncryptionContext.Keys, ByteLess);
       assert forall k <- contextKeysX :: ValidUTF8Seq(k) by {
         assert forall k <- newEncryptionContext.Keys :: ValidUTF8Seq(k);
         assert forall k <- contextKeysX :: k in newEncryptionContext.Keys;
       }
       let contextKeys :  seq<UTF8.ValidUTF8Bytes> = contextKeysX;

       //= specification/structured-encryption/decrypt-path-structure.md#create-new-encryption-context-and-cmm
       //# Then, this operation MUST create a [Required Encryption Context CMM](https://github.com/awslabs/private-aws-encryption-sdk-specification-staging/blob/dafny-verified/framework/required-encryption-context-cmm.md)
       //# with the following inputs:
       //# - This input [CMM](./ddb-table-encryption-config.md#cmm) as the underlying CMM.
       //# - The name of every entry added above.
       let cmmR = config.materialProviders.CreateRequiredEncryptionContextCMM(
         CMP.CreateRequiredEncryptionContextCMMInput(
           underlyingCMM = Some(input.cmm),
           keyring = None,
           requiredEncryptionContextKeys = contextKeys
         )
       );
       cmm :- cmmR.MapFailure(e => AwsCryptographyMaterialProviders(e));
     }
   }

   //= specification/structured-encryption/decrypt-path-structure.md#retrieve-decryption-materials
   //# This operation MUST obtain a set of decryption materials by calling
   //# [Decrypt Materials](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/cmm-interface.md#decrypt-materials)
   //# on the [CMM](#cmm) calculated above.

   //= specification/structured-encryption/decrypt-path-structure.md#retrieve-decryption-materials
   //# The call to the CMM's Decrypt Materials operation MUST be constructed as follows:
   // - Encryption Context: The encryption context containing exactly the union of
   //   key-value pairs in the [input Encryption Context](#encryption-context)
   //   and the key-value pairs in the [Encryption Context parsed from the header](./header.md#encryption-context).
   // - Algorithm Suite ID: The algorithm suite [indicated by the Message Format Flavor](./header.md#format-flavor)
   //   parsed in the header.
   // - Encrypted Data Keys: The [Encrypted Data Keys parsed from the header](./header.md#encrypted-data-keys).

   // assert (cmm == input.cmm) || fresh(cmm);
   let matR = cmm.DecryptMaterials(
     CMP.DecryptMaterialsInput (
       algorithmSuiteId = headerAlgorithmSuite.id,
       commitmentPolicy = DBE_COMMITMENT_POLICY,
       encryptedDataKeys = head.dataKeys,
       encryptionContext = head.encContext,
       reproducedEncryptionContext = Some(encryptionContext)
     )
   );

   let matOutput :- matR.MapFailure(e => AwsCryptographyMaterialProviders(e));
   let mat = matOutput.decryptionMaterials;
   :- Need(Header.ValidEncryptionContext(mat.encryptionContext), E("Bad encryption context"));
   :- Need(Materials.DecryptionMaterialsWithPlaintextDataKey(mat), E("Encryption material has no key"));

   //= specification/structured-encryption/decrypt-path-structure.md#retrieve-decryption-materials
   //# The algorithm suite used in all further aspects of this operation MUST be
   //# the algorithm suite in the
   //# [decryption materials](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/structures.md#decryption-materials)
   //# returned from the Decrypt Materials call.

   //= specification/structured-encryption/decrypt-path-structure.md#retrieve-decryption-materials
   //# Note that the algorithm suite in the retrieved decryption materials MAY be different from the input algorithm suite.

   //= specification/structured-encryption/decrypt-path-structure.md#retrieve-decryption-materials
   //# If this algorithm suite is not a
   //# [supported suite for DBE](../../submodules/MaterialProviders/aws-encryption-sdk-specification/framework/algorithm-suites.md#supported-algorithm-suites-enum)
   //# this operation MUST yield an error.
   :- Need(ValidSuite(mat.algorithmSuite), E("Invalid Algorithm Suite"));
   let postCMMAlg = mat.algorithmSuite;
   let key : Key = mat.plaintextDataKey.value;
   let commitKey :- Crypt.GetCommitKey(config.primitives, postCMMAlg, key, head.msgID);
   //= specification/structured-encryption/decrypt-path-structure.md#parse-the-header
   //# The header field value MUST be [verified](header.md#commitment-verification)
   let ok :- head.verifyCommitment(config.primitives, postCMMAlg, commitKey, headerSerialized.value);

   //= specification/structured-encryption/decrypt-path-structure.md#verify-signatures
   //# This operation MUST deserialize the bytes in [Terminal Value](./structures.md#terminal-value)
   //# according to the [footer format](./footer.md).
   let footer :- Footer.DeserializeFooter(footerSerialized.value, postCMMAlg.signature.ECDSA?);

   //= specification/structured-encryption/decrypt-path-structure.md#verify-signatures
   //# The footer field value MUST be [verified](footer.md#footer-verification).

   //= specification/structured-encryption/decrypt-path-structure.md#verify-signatures
   //# Decryption MUST fail immediately if verification fails.
   let _ :- footer.validate(config.primitives, mat, head.dataKeys, canonData, headerSerialized.value);
   let decryptedItems : CanonCryptoList :- Crypt.Decrypt(config.primitives, postCMMAlg, key, head, canonData, input.tableName, input.encryptedStructure);

   let largeResult = UnCanonDecrypt(decryptedItems, input.tableName, input.encryptedStructure);
   let smallResult = RemoveHeaders(largeResult, input.encryptedStructure);

   //= specification/structured-encryption/decrypt-path-structure.md#construct-decrypted-structured-data
   //= type=implication
   //# The output MUST also include a [Parsed Header](#parsed-header) that contains
   //# data that was serialized into the header included in the output Structured Data.
   let parsedHeader = ParsedHeader(
     algorithmSuiteId = headerAlgorithmSuite.id.DBE,
     encryptedDataKeys = head.dataKeys,
     storedEncryptionContext = head.encContext,
     encryptionContext = mat.encryptionContext
   );

   let decryptOutput = DecryptPathStructureOutput(
     plaintextStructure = smallResult,
     parsedHeader = parsedHeader
   );

   assert DecryptPathFinal(input.encryptedStructure, decryptOutput.plaintextStructure);
   output = Success(decryptOutput);
 }

*/
