// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Runnable entry point for the .NET Language_Server: binds an rpcv2Cbor HTTP
// endpoint on a port taken from (in order) the first CLI argument, the
// ESDK_TESTSERVER_PORT env var, or the default 8097.

using EsdkTestServer;

var port = 8097;
var raw = Environment.GetEnvironmentVariable("ESDK_TESTSERVER_PORT");
if (args.Length > 0)
{
    raw = args[0];
}
if (int.TryParse(raw, out var parsed) && parsed is > 0 and <= 65535)
{
    port = parsed;
}

var server = new WireServer(port, new Handlers());
server.Start();
Console.Error.WriteLine($"listening at http://127.0.0.1:{port}");
await server.AcceptLoop;
