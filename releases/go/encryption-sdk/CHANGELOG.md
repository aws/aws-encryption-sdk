## [releases/go/encryption-sdk/v0.2.1] - 2025-08-12

### 🐛 Bug Fixes

- _(go)_ Fix and refactor go mod (#793)

### 📚 Documentation

- _(Go)_ Add Go to supported languages in the readme (#780)

### ⚙️ Miscellaneous Tasks

- Fail verification on warnings (#766)
- _(dafny)_ Allow FileIO to deal in uint8, rather than bv8 (#770)
- _(CI)_ Fix daily CI (#771)
- _(CI)_ Clean up unused attributes (#772)
- Add C to list of languages checked for interoperability (#773)
- Bump smithy-dafny to latest (#774)
- Bump MPL to latest version (#775)
- Improve performance (#764)
- _(rust)_ Update smithy-dafny and use small-int feature (#777)
- Bump MPL version (#778)
- _(dafny)_ Remove more usage of BigInteger (#781)
- Install polymorph deps in github workflows (#783)
- _(rust)_ Prepare rust 1.1.0 release (#782)
- _(rust)_ Prepare 1.1.1 release (#784)
- _(CI)_ Partially fix nightly build (#785)
- _(dafny)_ Update makefile to only use prettier 3.5.3 (#786)
- Bump mpl (#787)
- _(go)_ Update go test matrix and clean up setup (#788)
- Let interop testing launch N tasks instead of N^2 (#789)
- _(CI)_ Add backwards compatability tests for nightly (#790)
- _(CI)_ Add slack notifications (#797)

# Changelog

# [0.2.0] (2025-03-21)

- Breaks compatibility with v0.1.0 (and v0.1.1) when using chars with unicode codepoints > 65535
- utf8-utf16 encoding fix
- support for utf16 surrogate pairs / chars with unicode codepoints > 65535
- fix for replacement char U+FFFD
- empty byte fix to allow custom keyring wrapping
- other operational improvements

# [0.1.1] (2025-02-14)

- retraction of all versions due to incompatibility with other esdk implementations

# [0.1.0] (2025-01-15)

Semantic version upgrade from v0.0.1 to v0.1.0

## 0.0.1 (2025-01-16)

Initial launch of the AWS Encryption SDK for Go.
