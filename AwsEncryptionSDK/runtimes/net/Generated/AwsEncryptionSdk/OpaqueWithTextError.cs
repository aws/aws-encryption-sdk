// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
using System;
using AWS.Cryptography.EncryptionSDK;
namespace AWS.Cryptography.EncryptionSDK
{
    public class OpaqueWithTextError : Exception
    {
        public readonly object obj;
        public readonly string objMessage;
        public OpaqueWithTextError(Exception ex) : base("OpaqueError:", ex) { this.obj = ex; this.objMessage = obj.ToString(); }
        public OpaqueWithTextError() : base("Unknown Unexpected Error") { }
        public OpaqueWithTextError(object obj, string objMessage) : base(obj is Exception ? "OpaqueWithTextError:" : "Opaque obj is not an Exception.", obj as Exception) { this.obj = obj; this.objMessage = objMessage; }
    }

}
