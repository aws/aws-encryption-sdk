# ESDK TestServer — Go Language_Server

A hand-rolled [rpcv2Cbor](https://smithy.io/2.0/additional-specs/protocols/smithy-rpc-v2.html)
HTTP server that implements the ESDK TestServer Smithy contract and delegates
each operation to the AWS Encryption SDK for Go
(`releases/go/encryption-sdk` in this repository).

## Embedded reference to the commons TestServer

The single source of truth for the wire contract — the Smithy model, the one
generated Test_Client, and the one Tests suite — lives in the commons repo:

- Repository: [`aws/aws-crypto-tools-commons`](https://github.com/aws/aws-crypto-tools-commons)
- Model: `esdk/test-server/model/esdk-test-server.smithy`
- Tests: `esdk/test-server/tests`

This directory hosts only the Go Language_Server; it consumes the commons
contract. The commons coordinates are pinned in `commons-configuration.json`.

## What it speaks

- `POST /service/ESDKTestServer/operation/{Operation}`
- Header `smithy-protocol: rpc-v2-cbor`, `Content-Type: application/cbor`
- CBOR map request/response bodies; errors as a CBOR map `{__type, message}`
- Operations: `CreateClient`, `Encrypt`, `Decrypt`. `EncryptStream` / `DecryptStream`
  return `GenericServerError` (this server is non-streaming).

## Running

```bash
make run-server PORT=8099        # foreground
# or, orchestrated:
make start-server PORT=8099
make wait-for-server PORT=8099
make stop-server PORT=8099
make check                       # fmt + vet + build + test (the CI gate)
```

## Full cross-language matrix

```bash
make test-server                 # clone commons and delegate to its orchestrator
```

Clones the commons repository at the branch pinned in
`commons-configuration.json` (`COMMONS_BRANCH=<b>` overrides) and runs the
orchestrator's full Tests matrix against this server. Needs AWS credentials and
a JDK 21+.

## Layout

`model.go` (shapes + CBOR mapping), `wire.go` (protocol), `keyring.go`
(config→MPL mapping), `handlers.go` (delegation + client registry).
