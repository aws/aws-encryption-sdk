// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Credential-free tests that drive the real rpcv2Cbor wire path over HTTP
// against an ephemeral localhost port. No KMS or network beyond the loopback.

using System.Formats.Cbor;
using System.Net;
using System.Net.Sockets;
using Xunit;

namespace EsdkTestServer.Tests;

public class ProtocolTests
{
    private const string SmithyProtocol = "rpc-v2-cbor";
    private const string CborContentType = "application/cbor";
    private const string GenericErrorType = "aws.cryptography.esdk.testserver#GenericServerError";

    private sealed class TestServer : IDisposable
    {
        internal readonly HttpClient Client = new();
        internal readonly string BaseUrl;
        private readonly WireServer _server;

        internal TestServer()
        {
            for (var attempt = 0; ; attempt++)
            {
                var port = FreeTcpPort();
                var server = new WireServer(port, new Handlers());
                try
                {
                    server.Start();
                    _server = server;
                    BaseUrl = $"http://127.0.0.1:{port}";
                    return;
                }
                catch (HttpListenerException) when (attempt < 9)
                {
                    server.Dispose();
                }
            }
        }

        public void Dispose()
        {
            Client.Dispose();
            _server.Dispose();
        }

        private static int FreeTcpPort()
        {
            var listener = new TcpListener(IPAddress.Loopback, 0);
            listener.Start();
            var port = ((IPEndPoint)listener.LocalEndpoint).Port;
            listener.Stop();
            return port;
        }
    }

    // Sends one rpcv2Cbor request and returns the HTTP status and decoded CBOR
    // body. smithyProtocol replaces the smithy-protocol header value; null sends
    // no such header. Asserts the response protocol headers on every exchange.
    private static (HttpStatusCode Status, Dictionary<string, object> Body) Post(
        TestServer server, string operation, Dictionary<string, object> body, string smithyProtocol = SmithyProtocol)
    {
        var request = new HttpRequestMessage(
            HttpMethod.Post, $"{server.BaseUrl}/service/ESDKTestServer/operation/{operation}")
        {
            Content = new ByteArrayContent(EncodeCbor(body)),
        };
        request.Content.Headers.Remove("Content-Type");
        request.Content.Headers.TryAddWithoutValidation("Content-Type", CborContentType);
        if (smithyProtocol != null)
        {
            request.Headers.TryAddWithoutValidation("smithy-protocol", smithyProtocol);
        }
        using var response = server.Client.Send(request);
        Assert.Equal(SmithyProtocol, Assert.Single(response.Headers.GetValues("smithy-protocol")));
        Assert.Equal(CborContentType, response.Content.Headers.ContentType?.ToString());
        using var stream = response.Content.ReadAsStream();
        using var buffer = new MemoryStream();
        stream.CopyTo(buffer);
        var reader = new CborReader(buffer.ToArray(), CborConformanceMode.Lax);
        var decoded = Assert.IsType<Dictionary<string, object>>(ReadValue(reader));
        return (response.StatusCode, decoded);
    }

    private static byte[] EncodeCbor(Dictionary<string, object> value)
    {
        var writer = new CborWriter();
        WriteValue(writer, value);
        return writer.Encode();
    }

    private static void WriteValue(CborWriter writer, object value)
    {
        switch (value)
        {
            case string s:
                writer.WriteTextString(s);
                break;
            case byte[] bytes:
                writer.WriteByteString(bytes);
                break;
            case int i:
                writer.WriteInt64(i);
                break;
            case long l:
                writer.WriteInt64(l);
                break;
            case Dictionary<string, object> map:
                writer.WriteStartMap(map.Count);
                foreach (var entry in map)
                {
                    writer.WriteTextString(entry.Key);
                    WriteValue(writer, entry.Value);
                }
                writer.WriteEndMap();
                break;
            case List<object> list:
                writer.WriteStartArray(list.Count);
                foreach (var entry in list)
                {
                    WriteValue(writer, entry);
                }
                writer.WriteEndArray();
                break;
            default:
                throw new ArgumentException($"unsupported CBOR test value: {value?.GetType().ToString() ?? "null"}");
        }
    }

    private static object ReadValue(CborReader reader)
    {
        switch (reader.PeekState())
        {
            case CborReaderState.TextString:
                return reader.ReadTextString();
            case CborReaderState.ByteString:
                return reader.ReadByteString();
            case CborReaderState.UnsignedInteger:
            case CborReaderState.NegativeInteger:
                return reader.ReadInt64();
            case CborReaderState.Boolean:
                return reader.ReadBoolean();
            case CborReaderState.Null:
                reader.ReadNull();
                return null;
            case CborReaderState.StartMap:
                var map = new Dictionary<string, object>();
                reader.ReadStartMap();
                while (reader.PeekState() != CborReaderState.EndMap)
                {
                    map[reader.ReadTextString()] = ReadValue(reader);
                }
                reader.ReadEndMap();
                return map;
            case CborReaderState.StartArray:
                var list = new List<object>();
                reader.ReadStartArray();
                while (reader.PeekState() != CborReaderState.EndArray)
                {
                    list.Add(ReadValue(reader));
                }
                reader.ReadEndArray();
                return list;
            default:
                throw new InvalidOperationException($"unsupported CBOR state: {reader.PeekState()}");
        }
    }

    private static string Text(Dictionary<string, object> map, string key)
    {
        Assert.True(map.ContainsKey(key), $"member {key} missing");
        return Assert.IsType<string>(map[key]);
    }

    private static byte[] Blob(Dictionary<string, object> map, string key)
    {
        Assert.True(map.ContainsKey(key), $"member {key} missing");
        return Assert.IsType<byte[]>(map[key]);
    }

    // Asserts the modeled GenericServerError wire form and returns its message.
    private static string RequireGenericError(HttpStatusCode status, Dictionary<string, object> body)
    {
        Assert.Equal(HttpStatusCode.BadRequest, status);
        Assert.Equal(GenericErrorType, Text(body, "__type"));
        return Text(body, "message");
    }

    // The offline Raw-AES / Default-CMM CreateClient request the commons
    // BlobRoundTrip behaviors use.
    private static Dictionary<string, object> RawAesCreateClientBody(long? maxEncryptedDataKeys)
    {
        var wrappingKey = new byte[32];
        for (var i = 0; i < wrappingKey.Length; i++)
        {
            wrappingKey[i] = (byte)i;
        }
        var config = new Dictionary<string, object>
        {
            ["commitmentPolicy"] = "REQUIRE_ENCRYPT_REQUIRE_DECRYPT",
            ["cmm"] = new Dictionary<string, object>
            {
                ["Default"] = new Dictionary<string, object>
                {
                    ["keyring"] = new Dictionary<string, object>
                    {
                        ["RawAes"] = new Dictionary<string, object>
                        {
                            ["keyNamespace"] = "esdk-test-server",
                            ["keyName"] = "raw-aes-round-trip-key",
                            ["wrappingKey"] = wrappingKey,
                            ["wrappingAlg"] = "ALG_AES256_GCM_IV12_TAG16",
                        },
                    },
                },
            },
        };
        if (maxEncryptedDataKeys is { } max)
        {
            config["maxEncryptedDataKeys"] = max;
        }
        return new Dictionary<string, object> { ["config"] = config };
    }

    private static string CreateRawAesClient(TestServer server)
    {
        var (status, body) = Post(server, "CreateClient", RawAesCreateClientBody(null));
        Assert.Equal(HttpStatusCode.OK, status);
        var clientId = Text(body, "clientId");
        Assert.NotEqual("", clientId);
        return clientId;
    }

    [Fact]
    public void RawAesRoundTrip()
    {
        using var server = new TestServer();
        var clientId = CreateRawAesClient(server);

        var plaintext = System.Text.Encoding.UTF8.GetBytes("Hello ESDK TestServer round trip.");
        const string suite = "ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY";
        var (status, encrypted) = Post(server, "Encrypt", new Dictionary<string, object>
        {
            ["clientId"] = clientId,
            ["plaintext"] = plaintext,
            ["encryptionContext"] = new Dictionary<string, object> { ["purpose"] = "round-trip" },
            ["algorithmSuiteId"] = suite,
            ["frameLength"] = 1024L,
        });
        Assert.Equal(HttpStatusCode.OK, status);
        var ciphertext = Blob(encrypted, "ciphertext");
        Assert.NotEqual(plaintext, ciphertext);

        var (decryptStatus, decrypted) = Post(server, "Decrypt", new Dictionary<string, object>
        {
            ["clientId"] = clientId,
            ["ciphertext"] = ciphertext,
        });
        Assert.Equal(HttpStatusCode.OK, decryptStatus);
        Assert.Equal(plaintext, Blob(decrypted, "plaintext"));
        var encryptionContext = Assert.IsType<Dictionary<string, object>>(decrypted["encryptionContext"]);
        Assert.Equal("round-trip", Text(encryptionContext, "purpose"));
        Assert.Equal(suite, Text(decrypted, "algorithmSuiteId"));

        // A second client built from the same config decrypts the same message
        // without being given the encryption context: it travels in the header.
        var secondId = CreateRawAesClient(server);
        var (secondStatus, second) = Post(server, "Decrypt", new Dictionary<string, object>
        {
            ["clientId"] = secondId,
            ["ciphertext"] = ciphertext,
        });
        Assert.Equal(HttpStatusCode.OK, secondStatus);
        Assert.Equal(plaintext, Blob(second, "plaintext"));
    }

    [Fact]
    public void MissingOrWrongSmithyProtocolHeader()
    {
        using var server = new TestServer();

        var (status, body) = Post(server, "CreateClient", RawAesCreateClientBody(null), smithyProtocol: null);
        RequireGenericError(status, body);

        (status, body) = Post(server, "CreateClient", RawAesCreateClientBody(null), smithyProtocol: "rpc-v2-json");
        RequireGenericError(status, body);
    }

    [Fact]
    public void UnknownOperation()
    {
        using var server = new TestServer();
        var (status, body) = Post(server, "Frobnicate", new Dictionary<string, object>());
        RequireGenericError(status, body);
    }

    [Fact]
    public void EncryptStreamUnsupported()
    {
        using var server = new TestServer();
        var (status, body) = Post(server, "EncryptStream", new Dictionary<string, object>());
        var message = RequireGenericError(status, body);
        Assert.Equal("streaming operations are not supported by the net language server", message);
    }

    [Fact]
    public void EncryptUnknownClientId()
    {
        using var server = new TestServer();
        var (status, body) = Post(server, "Encrypt", new Dictionary<string, object>
        {
            ["clientId"] = "no-such-client",
            ["plaintext"] = System.Text.Encoding.UTF8.GetBytes("plaintext"),
        });
        RequireGenericError(status, body);
    }

    [Fact]
    public void CreateClientRejectsZeroMaxEncryptedDataKeys()
    {
        using var server = new TestServer();
        var (status, body) = Post(server, "CreateClient", RawAesCreateClientBody(0));
        RequireGenericError(status, body);
    }
}
