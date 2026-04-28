Redshift wants a performant C++ ESDK with hierarchical keyring support. At a high level we’ll deliver this by shipping a natively-written Rust ESDK and exposing it to C++ via bindings. This doc defines the scope of those deliverables.
Q1: What will we deliver for Redshift?
Andy previously decided this and we won’t backtrack since Option 1 is mostly implemented, but it’s worth stating the options again to understand how we got here.
1. C++ bindings to a native Rust ESDK that uses the Dafny-Rust MPL
Recommended and explored in this doc.
2. Option 1 but with a new Native Rust MPL
Not worth the effort. Most of the performance benefit from moving to native comes from eliminating conversions to/from Dafny shapes for large objects. The MPL handles orders of magnitude fewer bytes than the ESDK in typical operation, so moving it to native is less impactful.
3. Option 1 but with the existing Dafny Rust ESDK
Not performant. Early performance tests of a native Rust ESDK with the Dafny Rust MPL show:
•	>4x speedup in wall-clock time for single-threaded encrypt/decrypt
o	50 MB payload, time dropped from 37 ms to 9 ms
•	Memory overhead reduced from O(n) to O(1) relative to input size
o	50 MB payload, additional memory overhead dropped from 48 MB to 0.02 MB (not re-creating the entire input/output buffer as Dafny types)
4. Native C++ hierarchy keyring
We want to minimize the amount of C/C++ business logic we own moving forward; this would not help.
Q2: What is the long-term goal?
What did Andy leave behind
Andy left behind some partial Rust crates. For a full writeup, see the Appendix. Two relevant crates for this project are a native ESDK Rust crate and a C++ bindings crate.
More detail:
Native Rust ESDK
Implementation: Encrypt/decrypt are working and pass some interop testing.
Duvet: About 18% annotated, but all are implications (i.e. untested).
MPL: Uses a combination of the existing Dafny-Rust MPL and the native Rust MPL. (Native Rust MPL is only for type definitions – all methods are stubbed). We would refactor this to only use the Dafny-Rust MPL.
Tests: Implements most ESDK Dafny integ tests, passes some test vectors.
C++ Bindings
Implementation: Some Rust code re-defines shapes (ex. EncryptInput, AlgorithmSuiteId) and methods (ex. create_hierarchical_keyring(), encrypt()) that are exported to C++. The shapes exported to C++ are different than the Rust shapes; the exposed methods are responsible for converting from C++ shapes to Rust shapes and calling the Rust methods. A crate called `cxx-build` compiles this to C++ bindings.
Tests: None
Deliverables
We will deliver only the functionality required by Redshift. This means
1.	An internal-Amazon native Rust ESDK that only supports hierarchy keyrings, and
2.	C++ bindings for this native Rust ESDK.
This means not immediately delivering more fully-featured options:
•	Support all MPL keyrings
o	Redshift only needs HKeyring support. We can add others later.
•	Release native Rust ESDK publicly
o	Launching an “HKeyring-only” ESDK conflicts with the keyring support for the current public library. We should support all MPL keyrings before releasing this publicly, and we don’t need to do that now to deliver for Redshift.
o	Probably dependent on post-Dafny strategy
o	Public release requires docs, examples
Once we have demand for more keyrings or want to launch the native Rust ESDK publicly, we would start from this internal build.
Spec Compliance
The native Rust ESDK/C++ bindings will fully implement Duvet coverage for the ESDK spec, meaning files in either the data-format and client-apis folders. They do not need to implement coverage for any files for the MPL spec (the framework folder) since they are re-using the Dafny-Rust MPL components.
Duvet’ing the Rust code is straightforward, but I’ll need to figure out what Duvet’ing C++ bindings looks like in practice (out of scope for this doc).
As an aside, no ESDK implementation is actively Duvet’ed today; see the appendix. I won’t Duvet these other implementations for reasons described in the appendix. Not Duvet’ing other implementations increases the risk of implementation inconsistencies, but I suggest we accept this risk and mitigate it with testing.
Testing
Defining the testing bar for a native library isn’t straightforward because we don’t have a testing spec/test plan for the ESDK to follow. I’m not planning to create a testing spec for the ESDK for the same reasons as I’m not planning to Duvet existing ESDK implementations.
I’ll suggest bounding testing to the following:
•	Unit tests
o	Rust: ESDK implementation
o	C++: Shim logic
•	Integ tests
o	Rust: Re-implement all Dafny ESDK integ tests (luckily, there aren’t a ton) scoped down/modified to test with HKeyring
	These are all of the non-proof tests in the Dafny ESDK.  By implementing this, the Rust ESDK would have “at least as many” integ tests as the Dafny ESDK, which sounds like a nice bar
o	C++: Minimal happy-path smoke testing. ex. No need to re-implement all of the required EC CMM test cases
•	ESDK encrypt/decrypt interop tests for HKeyring
o	Both Rust and C++ (to test shim layer correctness)
o	This should test the real HKeyring, not the fake HKeyring that the Dafny test vectors test
•	Performance tests
o	Ideally both Rust and C++ (to test shim layer overhead)
Development
Rust: Develop in a private Github repo (create a aws/aws-encryption-sdk-rust private repo).
•	Testing is much easier if it’s in Github and not Brazil
•	Why call it `aws/aws-encryption-sdk-rust`? I expect this project will evolve into the native ESDK rust we launch publicly. There’s a reasonable chance that product will live in the aws/aws-encryption-sdk-rust repo
o	Even if the Native Builds work suggests putting this in a `aws/aws-encryption-sdk-rust`isn’t our repo organization approach moving forward, this aligns with our organization strategy today. We can migrate this repo along with our other repos once our organization strategy is defined.
•	Can be imported into Brazil via a package we maintain + PRs to that package
C++: Develop in a private Github repo (create a aws/aws-encryption-sdk-cpp private repo)
•	Why separate repo from ESDK Rust? It’s a different UX/language for customers, which seems like the right boundary for creating a new repo/product
•	Can be imported into Brazil via a package we maintain + PRs to that package??
•	Tony wants them to own this
Alternatives:
•	Develop in Brazil – testing is more complicated, harder to make public eventually
•	Develop in some alternative repo – not sure which one, open to suggestions
Appendix
The spec compliance story for other languages is complicated. An ESDK spec exists, but no ESDK implementation is validated against the spec with Duvet today:
•	Dafny: Some comments exist but Duvet does not run. All comments are in an old format and must be updated before Duvet can run.
•	Java: No comments.
•	Python: No comments.
•	C: Some comments exist but Duvet does not run. All comments are in an old format and must be updated before Duvet can run.
•	JavaScript: Comments for HKeyring implementation exist but Duvet does not run. 
I’m not planning to Duvet any existing ESDK implementations. Achieving real spec alignment goes beyond simply adding annotations and enabling Duvet. It requires validating that code precisely implements the specification. Prior rushed batch spec updates led to missed nuances around UTF encoding (COE). Even if this alignment process were done carefully, it would probably find implementation inconsistencies that would need to be considered and addressed, delaying timelines (remembering FireEgg). Fully reconciling the spec and existing behavior is out of scope due to high cost and low near-term ROI. This requires a separate effort.

Appendix
Andy left behind a lot of partial Rust crates: 
•	prim/ (aws-mpl-primitives) — Native Rust wrappers around aws-lc-rs. Covers AES-GCM encrypt/decrypt, HKDF (full, extract, expand), ECDSA sign/verify (including streaming DigestContext), ECDH, AES-KDF-CTR, HMAC, Digest, random bytes, constant-time compare, and a memory tracker. This maps to the full AwsCryptographyPrimitives Smithy model — all operations from the Smithy specs (aes, hkdf, signature, digest, random, ecdh) are covered. Nothing obviously missing.
•	mpl/ (aws-mpl-rs) — Types and interfaces complete, implementations stubbed. All the core types are defined: EncryptionMaterials, DecryptionMaterials, EncryptedDataKey, Secret, AlgorithmSuite (all 11 ESDK + 2 DBE suites), CommitmentPolicy, EncryptionContext, Keyring and CryptographicMaterialsManager. Input/output structs for every keyring constructor exist (KMS strict, MRK, discovery, hierarchical, RSA, ECDH, raw AES, raw RSA, raw ECDH, multi-keyring). But every single function and method returns not_implemented(). So: the API surface is fully designed, but there's zero working keyring or CMM logic. You can't actually create any keyring from this crate.
•	esdk/ (aws-esdk) – full encrypt/decrypt, including streaming support.
•	aws-esdk-cxx/ — single lib.rs file exposing encrypt/decrypt, hierarchical keyring creation, keystore, KMS/DDB client lifecycle to C++ via the cxx crate. This can be used today to create and use a hierarchy keyring in C++. 
•	aws_structured_encryption/ — Partial rewrite, early stage. 9 source files covering encrypt/decrypt for structured data, canonization, header/footer serialization, crypto operations, path handling. Depends on the native MPL for types and the native Primitives for crypto. Compared to the full Dafny StructuredEncryption module, this has the core encrypt/decrypt path and the canonization logic, but it's missing the broader DB-ESDK integration (DynamoDB item encryptor, searchable encryption, beacons). It's the structured encryption core without the full DB-ESDK wrapper.

