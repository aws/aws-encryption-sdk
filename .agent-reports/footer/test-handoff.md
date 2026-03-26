### Test Handoff

**Spec**: `aws-encryption-sdk-specification/data-format/message-footer.md#signature-length`

**Files Modified**:
- `AwsEncryptionSDK/runtimes/rust/esdk_rust/esdk/src/message/footer.rs`

**Commit Message**:
```
fix(footer): add signature-length duvet annotations to footer serialization
```

**Notes**: All annotations are `type=implication` — no runtime tests needed. Duvet snapshot confirms both requirements covered.
