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
