// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

// Operation handlers and the clientId registry. CreateClient builds the
// materials manager from the modeled config (delegating to the MPL), constructs
// the real ESDK client eagerly, and registers both under a fresh UUID. Encrypt
// and Decrypt resolve the registered client by clientId and forward to the real
// library, always through the materials manager.

using AWS.Cryptography.EncryptionSDK;
using AWS.Cryptography.MaterialProviders;

namespace EsdkTestServer;

internal sealed class Handlers
{
    private readonly MaterialProviders _materialProviders = new(new MaterialProvidersConfig());
    private readonly Dictionary<string, ClientEntry> _clients = new();
    private readonly object _lock = new();

    // One registered ESDK client and the materials manager it encrypts and
    // decrypts with.
    private sealed class ClientEntry
    {
        internal ESDK Esdk;
        internal ICryptographicMaterialsManager Cmm;
    }

    // Eagerly builds the materials manager and the real ESDK client; any
    // construction or validation failure is a GenericServerError. new ESDK(...)
    // runs the config's own Validate() (rejecting maxEncryptedDataKeys < 1)
    // before constructing the client.
    internal CreateClientResponse CreateClient(CreateClientRequest request)
    {
        ClientEntry entry;
        try
        {
            var cmm = KeyringFactory.BuildCmm(_materialProviders, request.Config.Cmm);
            var config = new AwsEncryptionSdkConfig { CommitmentPolicy = request.Config.CommitmentPolicy };
            if (request.Config.MaxEncryptedDataKeys is { } maxEncryptedDataKeys)
            {
                config.MaxEncryptedDataKeys = maxEncryptedDataKeys;
            }
            entry = new ClientEntry { Esdk = new ESDK(config), Cmm = cmm };
        }
        catch (Exception e)
        {
            throw ServerErrorException.Generic($"CreateClient failed to construct the ESDK client: {Describe(e)}");
        }
        var clientId = Guid.NewGuid().ToString();
        lock (_lock)
        {
            _clients[clientId] = entry;
        }
        return new CreateClientResponse { ClientId = clientId };
    }

    internal EncryptResponse Encrypt(EncryptRequest request)
    {
        var entry = Resolve(request.ClientId);
        var input = new EncryptInput
        {
            Plaintext = new MemoryStream(request.Plaintext),
            MaterialsManager = entry.Cmm,
            EncryptionContext = request.EncryptionContext,
            AlgorithmSuiteId = request.AlgorithmSuiteId,
        };
        if (request.FrameLength is { } frameLength)
        {
            input.FrameLength = frameLength;
        }
        EncryptOutput output;
        try
        {
            output = entry.Esdk.Encrypt(input);
        }
        catch (Exception e)
        {
            throw ServerErrorException.EsdkClient(Describe(e));
        }
        return new EncryptResponse { Ciphertext = output.Ciphertext.ToArray() };
    }

    internal DecryptResponse Decrypt(DecryptRequest request)
    {
        var entry = Resolve(request.ClientId);
        var input = new DecryptInput
        {
            Ciphertext = new MemoryStream(request.Ciphertext),
            MaterialsManager = entry.Cmm,
            EncryptionContext = request.EncryptionContext,
        };
        DecryptOutput output;
        try
        {
            output = entry.Esdk.Decrypt(input);
        }
        catch (Exception e)
        {
            throw ServerErrorException.EsdkClient(Describe(e));
        }
        return new DecryptResponse
        {
            Plaintext = output.Plaintext.ToArray(),
            EncryptionContext = output.EncryptionContext,
            AlgorithmSuiteId = output.AlgorithmSuiteId,
        };
    }

    // Looks up the registered client; a missing, empty, or unknown clientId is
    // a GenericServerError.
    private ClientEntry Resolve(string clientId)
    {
        if (string.IsNullOrEmpty(clientId))
        {
            throw ServerErrorException.Generic("clientId must be non-empty");
        }
        lock (_lock)
        {
            if (_clients.TryGetValue(clientId, out var entry))
            {
                return entry;
            }
        }
        throw ServerErrorException.Generic($"unknown clientId: {clientId}");
    }

    // Flattens a library exception to a message. The generated CollectionOfErrors
    // types already embed their collected causes in Message; wrapper exceptions
    // like OpaqueError carry the cause in InnerException, so the chain is
    // appended to keep the underlying failure visible.
    private static string Describe(Exception exception)
    {
        var message = exception.Message;
        if (exception.InnerException != null)
        {
            message = $"{message} [encountered: {Describe(exception.InnerException)}]";
        }
        return message;
    }
}
