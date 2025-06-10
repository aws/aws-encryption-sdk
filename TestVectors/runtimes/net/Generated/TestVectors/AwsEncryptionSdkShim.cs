// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
namespace AWS.Cryptography.EncryptionSDK.Wrapped
{
    public class AwsEncryptionSdkShim : software.amazon.cryptography.encryptionsdk.internaldafny.types.IAwsEncryptionSdkClient
    {
        public AWS.Cryptography.EncryptionSDK.ESDK _impl;
        public AwsEncryptionSdkShim(AWS.Cryptography.EncryptionSDK.ESDK impl)
        {
            this._impl = impl;
        }
        public Wrappers_Compile._IResult<software.amazon.cryptography.encryptionsdk.internaldafny.types._IEncryptOutput, software.amazon.cryptography.encryptionsdk.internaldafny.types._IError> Encrypt(software.amazon.cryptography.encryptionsdk.internaldafny.types._IEncryptInput request)
        {
            try
            {
                AWS.Cryptography.EncryptionSDK.EncryptInput unWrappedRequest = TypeConversion.FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput(request);
                AWS.Cryptography.EncryptionSDK.EncryptOutput wrappedResponse =
                this._impl.Encrypt(unWrappedRequest);
                return Wrappers_Compile.Result<software.amazon.cryptography.encryptionsdk.internaldafny.types._IEncryptOutput, software.amazon.cryptography.encryptionsdk.internaldafny.types._IError>.create_Success(TypeConversion.ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_EncryptOutput(wrappedResponse));
            }
            catch (System.Exception ex)
            {
                return Wrappers_Compile.Result<software.amazon.cryptography.encryptionsdk.internaldafny.types._IEncryptOutput, software.amazon.cryptography.encryptionsdk.internaldafny.types._IError>.create_Failure(this.ConvertError(ex));
            }

        }
        public Wrappers_Compile._IResult<software.amazon.cryptography.encryptionsdk.internaldafny.types._IDecryptOutput, software.amazon.cryptography.encryptionsdk.internaldafny.types._IError> Decrypt(software.amazon.cryptography.encryptionsdk.internaldafny.types._IDecryptInput request)
        {
            try
            {
                AWS.Cryptography.EncryptionSDK.DecryptInput unWrappedRequest = TypeConversion.FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput(request);
                AWS.Cryptography.EncryptionSDK.DecryptOutput wrappedResponse =
                this._impl.Decrypt(unWrappedRequest);
                return Wrappers_Compile.Result<software.amazon.cryptography.encryptionsdk.internaldafny.types._IDecryptOutput, software.amazon.cryptography.encryptionsdk.internaldafny.types._IError>.create_Success(TypeConversion.ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_DecryptOutput(wrappedResponse));
            }
            catch (System.Exception ex)
            {
                return Wrappers_Compile.Result<software.amazon.cryptography.encryptionsdk.internaldafny.types._IDecryptOutput, software.amazon.cryptography.encryptionsdk.internaldafny.types._IError>.create_Failure(this.ConvertError(ex));
            }

        }
        private software.amazon.cryptography.encryptionsdk.internaldafny.types._IError ConvertError(System.Exception error)
        {
            switch (error.GetType().Namespace)
            {
                case "AWS.Cryptography.MaterialProviders":
                    return software.amazon.cryptography.encryptionsdk.internaldafny.types.Error.create_AwsCryptographyMaterialProviders(
                      AWS.Cryptography.MaterialProviders.TypeConversion.ToDafny_CommonError(error)
                    );
            }
            switch (error)
            {
                case AWS.Cryptography.EncryptionSDK.AwsEncryptionSdkException e:
                    return TypeConversion.ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S25_AwsEncryptionSdkException(e);

                case CollectionOfErrors collectionOfErrors:
                    return new software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_CollectionOfErrors(
                      Dafny.Sequence<software.amazon.cryptography.encryptionsdk.internaldafny.types._IError>
                        .FromArray(
                          collectionOfErrors.list.Select
                              (x => TypeConversion.ToDafny_CommonError(x))
                            .ToArray()),
                      Dafny.Sequence<char>.FromString(collectionOfErrors.Message)
                    );
                default:
                    return new software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_Opaque(error);

            }
        }
    }
}
