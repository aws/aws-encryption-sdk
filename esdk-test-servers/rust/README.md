# ESDK TestServer — Dafny-Rust Language_Server

A hand-rolled [rpcv2Cbor](https://smithy.io/2.0/additional-specs/protocols/smithy-rpc-v2.html)
HTTP server that implements the ESDK TestServer Smithy contract and delegates
each operation to the Dafny-generated AWS Encryption SDK for Rust (`aws-esdk`)
committed at `releases/rust/esdk`.

## What it speaks

- `POST /service/ESDKTestServer/operation/{Operation}`
- Header `smithy-protocol: rpc-v2-cbor`, `Content-Type: application/cbor`
- CBOR map request/response bodies; errors as a CBOR map `{__type, message}`
- Operations: `CreateClient`, `Encrypt`, `Decrypt`. `EncryptStream` / `DecryptStream`
  return `GenericServerError` (this server is non-streaming).

## Running

```bash
make run-server PORT=8098        # foreground
# or, orchestrated:
make start-server PORT=8098
make wait-for-server PORT=8098
make stop-server PORT=8098
make check                       # fmt + clippy + build + test (the CI gate)
```

## Full cross-language matrix

```bash
make test-server                 # clone commons and run the orchestrated Tests
                                 # (needs a JDK 21+ and AWS credentials)
```

## Layout

`src/model.rs` (shapes), `src/wire.rs` (protocol), `src/keyring.rs`
(config→MPL mapping), `src/handlers.rs` (delegation + client registry).
