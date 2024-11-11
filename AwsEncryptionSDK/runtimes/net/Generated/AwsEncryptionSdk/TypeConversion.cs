// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
using System.Linq;
using System;
namespace AWS.Cryptography.EncryptionSDK
{
    public static class TypeConversion
    {
        public static AWS.Cryptography.EncryptionSDK.AwsEncryptionSdkConfig FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S22_AwsEncryptionSdkConfig(software.amazon.cryptography.encryptionsdk.internaldafny.types._IAwsEncryptionSdkConfig value)
        {
            software.amazon.cryptography.encryptionsdk.internaldafny.types.AwsEncryptionSdkConfig concrete = (software.amazon.cryptography.encryptionsdk.internaldafny.types.AwsEncryptionSdkConfig)value; AWS.Cryptography.EncryptionSDK.AwsEncryptionSdkConfig converted = new AWS.Cryptography.EncryptionSDK.AwsEncryptionSdkConfig(); if (concrete._commitmentPolicy.is_Some) converted.CommitmentPolicy = (AWS.Cryptography.MaterialProviders.ESDKCommitmentPolicy)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S22_AwsEncryptionSdkConfig__M16_commitmentPolicy(concrete._commitmentPolicy);
            if (concrete._maxEncryptedDataKeys.is_Some) converted.MaxEncryptedDataKeys = (long)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S22_AwsEncryptionSdkConfig__M20_maxEncryptedDataKeys(concrete._maxEncryptedDataKeys);
            // BEGIN MANUAL EDIT
            if (concrete._netV4__0__0__RetryPolicy.is_Some) converted.NetV4_0_0_RetryPolicy = (AWS.Cryptography.EncryptionSDK.NetV4_0_0_RetryPolicy)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S22_AwsEncryptionSdkConfig__M21_netV4_0_0_RetryPolicy(concrete._netV4__0__0__RetryPolicy); return converted;
            // END MANUAL EDIT
        }
        public static software.amazon.cryptography.encryptionsdk.internaldafny.types._IAwsEncryptionSdkConfig ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S22_AwsEncryptionSdkConfig(AWS.Cryptography.EncryptionSDK.AwsEncryptionSdkConfig value)
        {
            value.Validate();
            AWS.Cryptography.MaterialProviders.ESDKCommitmentPolicy var_commitmentPolicy = value.IsSetCommitmentPolicy() ? value.CommitmentPolicy : (AWS.Cryptography.MaterialProviders.ESDKCommitmentPolicy)null;
            long? var_maxEncryptedDataKeys = value.IsSetMaxEncryptedDataKeys() ? value.MaxEncryptedDataKeys : (long?)null;
            // BEGIN MANUAL EDIT
            AWS.Cryptography.EncryptionSDK.NetV4_0_0_RetryPolicy var_netV4_0_0_RetryPolicy = value.IsSetNetV4__0__0__RetryPolicy() ? value.NetV4_0_0_RetryPolicy : (AWS.Cryptography.EncryptionSDK.NetV4_0_0_RetryPolicy)null;
            // END MANUAL EDIT
            return new software.amazon.cryptography.encryptionsdk.internaldafny.types.AwsEncryptionSdkConfig(ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S22_AwsEncryptionSdkConfig__M16_commitmentPolicy(var_commitmentPolicy), ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S22_AwsEncryptionSdkConfig__M20_maxEncryptedDataKeys(var_maxEncryptedDataKeys), ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S22_AwsEncryptionSdkConfig__M21_netV4_0_0_RetryPolicy(var_netV4_0_0_RetryPolicy));
        }
        public static AWS.Cryptography.EncryptionSDK.AwsEncryptionSdkException FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S25_AwsEncryptionSdkException(software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_AwsEncryptionSdkException value)
        {
            return new AWS.Cryptography.EncryptionSDK.AwsEncryptionSdkException(
            FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S25_AwsEncryptionSdkException__M7_message(value._message)
            );
        }
        public static software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_AwsEncryptionSdkException ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S25_AwsEncryptionSdkException(AWS.Cryptography.EncryptionSDK.AwsEncryptionSdkException value)
        {

            return new software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_AwsEncryptionSdkException(
            ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S25_AwsEncryptionSdkException__M7_message(value.Message)
            );
        }
        public static AWS.Cryptography.EncryptionSDK.DecryptInput FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput(software.amazon.cryptography.encryptionsdk.internaldafny.types._IDecryptInput value)
        {
            software.amazon.cryptography.encryptionsdk.internaldafny.types.DecryptInput concrete = (software.amazon.cryptography.encryptionsdk.internaldafny.types.DecryptInput)value; AWS.Cryptography.EncryptionSDK.DecryptInput converted = new AWS.Cryptography.EncryptionSDK.DecryptInput(); converted.Ciphertext = (System.IO.MemoryStream)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput__M10_ciphertext(concrete._ciphertext);
            if (concrete._materialsManager.is_Some) converted.MaterialsManager = (AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput__M16_materialsManager(concrete._materialsManager);
            if (concrete._keyring.is_Some) converted.Keyring = (AWS.Cryptography.MaterialProviders.IKeyring)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput__M7_keyring(concrete._keyring);
            if (concrete._encryptionContext.is_Some) converted.EncryptionContext = (System.Collections.Generic.Dictionary<string, string>)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput__M17_encryptionContext(concrete._encryptionContext); return converted;
        }
        public static software.amazon.cryptography.encryptionsdk.internaldafny.types._IDecryptInput ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput(AWS.Cryptography.EncryptionSDK.DecryptInput value)
        {
            value.Validate();
            AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager var_materialsManager = value.IsSetMaterialsManager() ? value.MaterialsManager : (AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager)null;
            AWS.Cryptography.MaterialProviders.IKeyring var_keyring = value.IsSetKeyring() ? value.Keyring : (AWS.Cryptography.MaterialProviders.IKeyring)null;
            System.Collections.Generic.Dictionary<string, string> var_encryptionContext = value.IsSetEncryptionContext() ? value.EncryptionContext : (System.Collections.Generic.Dictionary<string, string>)null;
            return new software.amazon.cryptography.encryptionsdk.internaldafny.types.DecryptInput(ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput__M10_ciphertext(value.Ciphertext), ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput__M16_materialsManager(var_materialsManager), ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput__M7_keyring(var_keyring), ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput__M17_encryptionContext(var_encryptionContext));
        }
        public static AWS.Cryptography.EncryptionSDK.DecryptOutput FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_DecryptOutput(software.amazon.cryptography.encryptionsdk.internaldafny.types._IDecryptOutput value)
        {
            software.amazon.cryptography.encryptionsdk.internaldafny.types.DecryptOutput concrete = (software.amazon.cryptography.encryptionsdk.internaldafny.types.DecryptOutput)value; AWS.Cryptography.EncryptionSDK.DecryptOutput converted = new AWS.Cryptography.EncryptionSDK.DecryptOutput(); converted.Plaintext = (System.IO.MemoryStream)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_DecryptOutput__M9_plaintext(concrete._plaintext);
            converted.EncryptionContext = (System.Collections.Generic.Dictionary<string, string>)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_DecryptOutput__M17_encryptionContext(concrete._encryptionContext);
            converted.AlgorithmSuiteId = (AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_DecryptOutput__M16_algorithmSuiteId(concrete._algorithmSuiteId); return converted;
        }
        public static software.amazon.cryptography.encryptionsdk.internaldafny.types._IDecryptOutput ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_DecryptOutput(AWS.Cryptography.EncryptionSDK.DecryptOutput value)
        {
            value.Validate();

            return new software.amazon.cryptography.encryptionsdk.internaldafny.types.DecryptOutput(ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_DecryptOutput__M9_plaintext(value.Plaintext), ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_DecryptOutput__M17_encryptionContext(value.EncryptionContext), ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_DecryptOutput__M16_algorithmSuiteId(value.AlgorithmSuiteId));
        }
        public static AWS.Cryptography.EncryptionSDK.EncryptInput FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput(software.amazon.cryptography.encryptionsdk.internaldafny.types._IEncryptInput value)
        {
            software.amazon.cryptography.encryptionsdk.internaldafny.types.EncryptInput concrete = (software.amazon.cryptography.encryptionsdk.internaldafny.types.EncryptInput)value; AWS.Cryptography.EncryptionSDK.EncryptInput converted = new AWS.Cryptography.EncryptionSDK.EncryptInput(); converted.Plaintext = (System.IO.MemoryStream)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M9_plaintext(concrete._plaintext);
            if (concrete._encryptionContext.is_Some) converted.EncryptionContext = (System.Collections.Generic.Dictionary<string, string>)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M17_encryptionContext(concrete._encryptionContext);
            if (concrete._materialsManager.is_Some) converted.MaterialsManager = (AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M16_materialsManager(concrete._materialsManager);
            if (concrete._keyring.is_Some) converted.Keyring = (AWS.Cryptography.MaterialProviders.IKeyring)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M7_keyring(concrete._keyring);
            if (concrete._algorithmSuiteId.is_Some) converted.AlgorithmSuiteId = (AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M16_algorithmSuiteId(concrete._algorithmSuiteId);
            if (concrete._frameLength.is_Some) converted.FrameLength = (long)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M11_frameLength(concrete._frameLength); return converted;
        }
        public static software.amazon.cryptography.encryptionsdk.internaldafny.types._IEncryptInput ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput(AWS.Cryptography.EncryptionSDK.EncryptInput value)
        {
            value.Validate();
            System.Collections.Generic.Dictionary<string, string> var_encryptionContext = value.IsSetEncryptionContext() ? value.EncryptionContext : (System.Collections.Generic.Dictionary<string, string>)null;
            AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager var_materialsManager = value.IsSetMaterialsManager() ? value.MaterialsManager : (AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager)null;
            AWS.Cryptography.MaterialProviders.IKeyring var_keyring = value.IsSetKeyring() ? value.Keyring : (AWS.Cryptography.MaterialProviders.IKeyring)null;
            AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId var_algorithmSuiteId = value.IsSetAlgorithmSuiteId() ? value.AlgorithmSuiteId : (AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId)null;
            long? var_frameLength = value.IsSetFrameLength() ? value.FrameLength : (long?)null;
            return new software.amazon.cryptography.encryptionsdk.internaldafny.types.EncryptInput(ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M9_plaintext(value.Plaintext), ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M17_encryptionContext(var_encryptionContext), ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M16_materialsManager(var_materialsManager), ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M7_keyring(var_keyring), ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M16_algorithmSuiteId(var_algorithmSuiteId), ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M11_frameLength(var_frameLength));
        }
        public static AWS.Cryptography.EncryptionSDK.EncryptOutput FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_EncryptOutput(software.amazon.cryptography.encryptionsdk.internaldafny.types._IEncryptOutput value)
        {
            software.amazon.cryptography.encryptionsdk.internaldafny.types.EncryptOutput concrete = (software.amazon.cryptography.encryptionsdk.internaldafny.types.EncryptOutput)value; AWS.Cryptography.EncryptionSDK.EncryptOutput converted = new AWS.Cryptography.EncryptionSDK.EncryptOutput(); converted.Ciphertext = (System.IO.MemoryStream)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_EncryptOutput__M10_ciphertext(concrete._ciphertext);
            converted.EncryptionContext = (System.Collections.Generic.Dictionary<string, string>)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_EncryptOutput__M17_encryptionContext(concrete._encryptionContext);
            converted.AlgorithmSuiteId = (AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId)FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_EncryptOutput__M16_algorithmSuiteId(concrete._algorithmSuiteId); return converted;
        }
        public static software.amazon.cryptography.encryptionsdk.internaldafny.types._IEncryptOutput ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_EncryptOutput(AWS.Cryptography.EncryptionSDK.EncryptOutput value)
        {
            value.Validate();

            return new software.amazon.cryptography.encryptionsdk.internaldafny.types.EncryptOutput(ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_EncryptOutput__M10_ciphertext(value.Ciphertext), ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_EncryptOutput__M17_encryptionContext(value.EncryptionContext), ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_EncryptOutput__M16_algorithmSuiteId(value.AlgorithmSuiteId));
        }
        // BEGIN MANUAL EDIT
        public static AWS.Cryptography.EncryptionSDK.NetV4_0_0_RetryPolicy FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S21_NetV4_0_0_RetryPolicy(software.amazon.cryptography.encryptionsdk.internaldafny.types._INetV4__0__0__RetryPolicy value)
        // END MANUAL EDIT
        {
            if (value.is_FORBID__RETRY) return AWS.Cryptography.EncryptionSDK.NetV4_0_0_RetryPolicy.FORBID_RETRY;
            if (value.is_ALLOW__RETRY) return AWS.Cryptography.EncryptionSDK.NetV4_0_0_RetryPolicy.ALLOW_RETRY;
            throw new System.ArgumentException("Invalid AWS.Cryptography.EncryptionSDK.NetV4_0_0_RetryPolicy value");
        }
        // BEGIN MANUAL EDIT
        public static software.amazon.cryptography.encryptionsdk.internaldafny.types._INetV4__0__0__RetryPolicy ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S21_NetV4_0_0_RetryPolicy(AWS.Cryptography.EncryptionSDK.NetV4_0_0_RetryPolicy value)
        // END MANUAL EDIT
        {
            // BEGIN MANUAL EDIT
            if (AWS.Cryptography.EncryptionSDK.NetV4_0_0_RetryPolicy.FORBID_RETRY.Equals(value)) return software.amazon.cryptography.encryptionsdk.internaldafny.types.NetV4__0__0__RetryPolicy.create_FORBID__RETRY();
            if (AWS.Cryptography.EncryptionSDK.NetV4_0_0_RetryPolicy.ALLOW_RETRY.Equals(value)) return software.amazon.cryptography.encryptionsdk.internaldafny.types.NetV4__0__0__RetryPolicy.create_ALLOW__RETRY();
            // END MANUAL EDIT
            throw new System.ArgumentException("Invalid AWS.Cryptography.EncryptionSDK.NetV4_0_0_RetryPolicy value");
        }
        public static AWS.Cryptography.MaterialProviders.ESDKCommitmentPolicy FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S22_AwsEncryptionSdkConfig__M16_commitmentPolicy(Wrappers_Compile._IOption<software.amazon.cryptography.materialproviders.internaldafny.types._IESDKCommitmentPolicy> value)
        {
            return value.is_None ? (AWS.Cryptography.MaterialProviders.ESDKCommitmentPolicy)null : FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S20_ESDKCommitmentPolicy(value.Extract());
        }
        public static Wrappers_Compile._IOption<software.amazon.cryptography.materialproviders.internaldafny.types._IESDKCommitmentPolicy> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S22_AwsEncryptionSdkConfig__M16_commitmentPolicy(AWS.Cryptography.MaterialProviders.ESDKCommitmentPolicy value)
        {
            return value == null ? Wrappers_Compile.Option<software.amazon.cryptography.materialproviders.internaldafny.types._IESDKCommitmentPolicy>.create_None() : Wrappers_Compile.Option<software.amazon.cryptography.materialproviders.internaldafny.types._IESDKCommitmentPolicy>.create_Some(ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S20_ESDKCommitmentPolicy((AWS.Cryptography.MaterialProviders.ESDKCommitmentPolicy)value));
        }
        public static long? FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S22_AwsEncryptionSdkConfig__M20_maxEncryptedDataKeys(Wrappers_Compile._IOption<long> value)
        {
            return value.is_None ? (long?)null : FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S15_CountingNumbers(value.Extract());
        }
        public static Wrappers_Compile._IOption<long> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S22_AwsEncryptionSdkConfig__M20_maxEncryptedDataKeys(long? value)
        {
            return value == null ? Wrappers_Compile.Option<long>.create_None() : Wrappers_Compile.Option<long>.create_Some(ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S15_CountingNumbers((long)value));
        }
        // BEGIN MANUAL EDIT
        public static AWS.Cryptography.EncryptionSDK.NetV4_0_0_RetryPolicy FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S22_AwsEncryptionSdkConfig__M21_netV4_0_0_RetryPolicy(Wrappers_Compile._IOption<software.amazon.cryptography.encryptionsdk.internaldafny.types._INetV4__0__0__RetryPolicy> value)
        // END MANUAL EDIT
        {
            return value.is_None ? (AWS.Cryptography.EncryptionSDK.NetV4_0_0_RetryPolicy)null : FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S21_NetV4_0_0_RetryPolicy(value.Extract());
        }
        // BEGIN MANUAL EDIT
        public static Wrappers_Compile._IOption<software.amazon.cryptography.encryptionsdk.internaldafny.types._INetV4__0__0__RetryPolicy> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S22_AwsEncryptionSdkConfig__M21_netV4_0_0_RetryPolicy(AWS.Cryptography.EncryptionSDK.NetV4_0_0_RetryPolicy value)
        // END MANUAL EDIT
        {
            // BEGIN MANUAL EDIT
            return value == null ? Wrappers_Compile.Option<software.amazon.cryptography.encryptionsdk.internaldafny.types._INetV4__0__0__RetryPolicy>.create_None() : Wrappers_Compile.Option<software.amazon.cryptography.encryptionsdk.internaldafny.types._INetV4__0__0__RetryPolicy>.create_Some(ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S21_NetV4_0_0_RetryPolicy((AWS.Cryptography.EncryptionSDK.NetV4_0_0_RetryPolicy)value));
            // END MANUAL EDIT
        }
        public static string FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S25_AwsEncryptionSdkException__M7_message(Dafny.ISequence<char> value)
        {
            return FromDafny_N6_smithy__N3_api__S6_String(value);
        }
        public static Dafny.ISequence<char> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S25_AwsEncryptionSdkException__M7_message(string value)
        {
            return ToDafny_N6_smithy__N3_api__S6_String(value);
        }
        public static System.IO.MemoryStream FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput__M10_ciphertext(Dafny.ISequence<byte> value)
        {
            return FromDafny_N6_smithy__N3_api__S4_Blob(value);
        }
        public static Dafny.ISequence<byte> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput__M10_ciphertext(System.IO.MemoryStream value)
        {
            return ToDafny_N6_smithy__N3_api__S4_Blob(value);
        }
        public static AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput__M16_materialsManager(Wrappers_Compile._IOption<software.amazon.cryptography.materialproviders.internaldafny.types.ICryptographicMaterialsManager> value)
        {
            return value.is_None ? (AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager)null : FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S38_CryptographicMaterialsManagerReference(value.Extract());
        }
        public static Wrappers_Compile._IOption<software.amazon.cryptography.materialproviders.internaldafny.types.ICryptographicMaterialsManager> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput__M16_materialsManager(AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager value)
        {
            return value == null ? Wrappers_Compile.Option<software.amazon.cryptography.materialproviders.internaldafny.types.ICryptographicMaterialsManager>.create_None() : Wrappers_Compile.Option<software.amazon.cryptography.materialproviders.internaldafny.types.ICryptographicMaterialsManager>.create_Some(ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S38_CryptographicMaterialsManagerReference((AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager)value));
        }
        public static AWS.Cryptography.MaterialProviders.IKeyring FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput__M7_keyring(Wrappers_Compile._IOption<software.amazon.cryptography.materialproviders.internaldafny.types.IKeyring> value)
        {
            return value.is_None ? (AWS.Cryptography.MaterialProviders.IKeyring)null : FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S16_KeyringReference(value.Extract());
        }
        public static Wrappers_Compile._IOption<software.amazon.cryptography.materialproviders.internaldafny.types.IKeyring> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput__M7_keyring(AWS.Cryptography.MaterialProviders.IKeyring value)
        {
            return value == null ? Wrappers_Compile.Option<software.amazon.cryptography.materialproviders.internaldafny.types.IKeyring>.create_None() : Wrappers_Compile.Option<software.amazon.cryptography.materialproviders.internaldafny.types.IKeyring>.create_Some(ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S16_KeyringReference((AWS.Cryptography.MaterialProviders.IKeyring)value));
        }
        public static System.Collections.Generic.Dictionary<string, string> FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput__M17_encryptionContext(Wrappers_Compile._IOption<Dafny.IMap<Dafny.ISequence<byte>, Dafny.ISequence<byte>>> value)
        {
            return value.is_None ? (System.Collections.Generic.Dictionary<string, string>)null : FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext(value.Extract());
        }
        public static Wrappers_Compile._IOption<Dafny.IMap<Dafny.ISequence<byte>, Dafny.ISequence<byte>>> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_DecryptInput__M17_encryptionContext(System.Collections.Generic.Dictionary<string, string> value)
        {
            return value == null ? Wrappers_Compile.Option<Dafny.IMap<Dafny.ISequence<byte>, Dafny.ISequence<byte>>>.create_None() : Wrappers_Compile.Option<Dafny.IMap<Dafny.ISequence<byte>, Dafny.ISequence<byte>>>.create_Some(ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext((System.Collections.Generic.Dictionary<string, string>)value));
        }
        public static System.IO.MemoryStream FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_DecryptOutput__M9_plaintext(Dafny.ISequence<byte> value)
        {
            return FromDafny_N6_smithy__N3_api__S4_Blob(value);
        }
        public static Dafny.ISequence<byte> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_DecryptOutput__M9_plaintext(System.IO.MemoryStream value)
        {
            return ToDafny_N6_smithy__N3_api__S4_Blob(value);
        }
        public static System.Collections.Generic.Dictionary<string, string> FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_DecryptOutput__M17_encryptionContext(Dafny.IMap<Dafny.ISequence<byte>, Dafny.ISequence<byte>> value)
        {
            return FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext(value);
        }
        public static Dafny.IMap<Dafny.ISequence<byte>, Dafny.ISequence<byte>> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_DecryptOutput__M17_encryptionContext(System.Collections.Generic.Dictionary<string, string> value)
        {
            return ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext(value);
        }
        public static AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_DecryptOutput__M16_algorithmSuiteId(software.amazon.cryptography.materialproviders.internaldafny.types._IESDKAlgorithmSuiteId value)
        {
            return FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S20_ESDKAlgorithmSuiteId(value);
        }
        public static software.amazon.cryptography.materialproviders.internaldafny.types._IESDKAlgorithmSuiteId ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_DecryptOutput__M16_algorithmSuiteId(AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId value)
        {
            return ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S20_ESDKAlgorithmSuiteId(value);
        }
        public static System.IO.MemoryStream FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M9_plaintext(Dafny.ISequence<byte> value)
        {
            return FromDafny_N6_smithy__N3_api__S4_Blob(value);
        }
        public static Dafny.ISequence<byte> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M9_plaintext(System.IO.MemoryStream value)
        {
            return ToDafny_N6_smithy__N3_api__S4_Blob(value);
        }
        public static System.Collections.Generic.Dictionary<string, string> FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M17_encryptionContext(Wrappers_Compile._IOption<Dafny.IMap<Dafny.ISequence<byte>, Dafny.ISequence<byte>>> value)
        {
            return value.is_None ? (System.Collections.Generic.Dictionary<string, string>)null : FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext(value.Extract());
        }
        public static Wrappers_Compile._IOption<Dafny.IMap<Dafny.ISequence<byte>, Dafny.ISequence<byte>>> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M17_encryptionContext(System.Collections.Generic.Dictionary<string, string> value)
        {
            return value == null ? Wrappers_Compile.Option<Dafny.IMap<Dafny.ISequence<byte>, Dafny.ISequence<byte>>>.create_None() : Wrappers_Compile.Option<Dafny.IMap<Dafny.ISequence<byte>, Dafny.ISequence<byte>>>.create_Some(ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext((System.Collections.Generic.Dictionary<string, string>)value));
        }
        public static AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M16_materialsManager(Wrappers_Compile._IOption<software.amazon.cryptography.materialproviders.internaldafny.types.ICryptographicMaterialsManager> value)
        {
            return value.is_None ? (AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager)null : FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S38_CryptographicMaterialsManagerReference(value.Extract());
        }
        public static Wrappers_Compile._IOption<software.amazon.cryptography.materialproviders.internaldafny.types.ICryptographicMaterialsManager> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M16_materialsManager(AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager value)
        {
            return value == null ? Wrappers_Compile.Option<software.amazon.cryptography.materialproviders.internaldafny.types.ICryptographicMaterialsManager>.create_None() : Wrappers_Compile.Option<software.amazon.cryptography.materialproviders.internaldafny.types.ICryptographicMaterialsManager>.create_Some(ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S38_CryptographicMaterialsManagerReference((AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager)value));
        }
        public static AWS.Cryptography.MaterialProviders.IKeyring FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M7_keyring(Wrappers_Compile._IOption<software.amazon.cryptography.materialproviders.internaldafny.types.IKeyring> value)
        {
            return value.is_None ? (AWS.Cryptography.MaterialProviders.IKeyring)null : FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S16_KeyringReference(value.Extract());
        }
        public static Wrappers_Compile._IOption<software.amazon.cryptography.materialproviders.internaldafny.types.IKeyring> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M7_keyring(AWS.Cryptography.MaterialProviders.IKeyring value)
        {
            return value == null ? Wrappers_Compile.Option<software.amazon.cryptography.materialproviders.internaldafny.types.IKeyring>.create_None() : Wrappers_Compile.Option<software.amazon.cryptography.materialproviders.internaldafny.types.IKeyring>.create_Some(ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S16_KeyringReference((AWS.Cryptography.MaterialProviders.IKeyring)value));
        }
        public static AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M16_algorithmSuiteId(Wrappers_Compile._IOption<software.amazon.cryptography.materialproviders.internaldafny.types._IESDKAlgorithmSuiteId> value)
        {
            return value.is_None ? (AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId)null : FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S20_ESDKAlgorithmSuiteId(value.Extract());
        }
        public static Wrappers_Compile._IOption<software.amazon.cryptography.materialproviders.internaldafny.types._IESDKAlgorithmSuiteId> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M16_algorithmSuiteId(AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId value)
        {
            return value == null ? Wrappers_Compile.Option<software.amazon.cryptography.materialproviders.internaldafny.types._IESDKAlgorithmSuiteId>.create_None() : Wrappers_Compile.Option<software.amazon.cryptography.materialproviders.internaldafny.types._IESDKAlgorithmSuiteId>.create_Some(ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S20_ESDKAlgorithmSuiteId((AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId)value));
        }
        public static long? FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M11_frameLength(Wrappers_Compile._IOption<long> value)
        {
            return value.is_None ? (long?)null : FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S11_FrameLength(value.Extract());
        }
        public static Wrappers_Compile._IOption<long> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S12_EncryptInput__M11_frameLength(long? value)
        {
            return value == null ? Wrappers_Compile.Option<long>.create_None() : Wrappers_Compile.Option<long>.create_Some(ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S11_FrameLength((long)value));
        }
        public static System.IO.MemoryStream FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_EncryptOutput__M10_ciphertext(Dafny.ISequence<byte> value)
        {
            return FromDafny_N6_smithy__N3_api__S4_Blob(value);
        }
        public static Dafny.ISequence<byte> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_EncryptOutput__M10_ciphertext(System.IO.MemoryStream value)
        {
            return ToDafny_N6_smithy__N3_api__S4_Blob(value);
        }
        public static System.Collections.Generic.Dictionary<string, string> FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_EncryptOutput__M17_encryptionContext(Dafny.IMap<Dafny.ISequence<byte>, Dafny.ISequence<byte>> value)
        {
            return FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext(value);
        }
        public static Dafny.IMap<Dafny.ISequence<byte>, Dafny.ISequence<byte>> ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_EncryptOutput__M17_encryptionContext(System.Collections.Generic.Dictionary<string, string> value)
        {
            return ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext(value);
        }
        public static AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_EncryptOutput__M16_algorithmSuiteId(software.amazon.cryptography.materialproviders.internaldafny.types._IESDKAlgorithmSuiteId value)
        {
            return FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S20_ESDKAlgorithmSuiteId(value);
        }
        public static software.amazon.cryptography.materialproviders.internaldafny.types._IESDKAlgorithmSuiteId ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S13_EncryptOutput__M16_algorithmSuiteId(AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId value)
        {
            return ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S20_ESDKAlgorithmSuiteId(value);
        }
        public static AWS.Cryptography.MaterialProviders.ESDKCommitmentPolicy FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S20_ESDKCommitmentPolicy(software.amazon.cryptography.materialproviders.internaldafny.types._IESDKCommitmentPolicy value)
        {
            if (value.is_FORBID__ENCRYPT__ALLOW__DECRYPT) return AWS.Cryptography.MaterialProviders.ESDKCommitmentPolicy.FORBID_ENCRYPT_ALLOW_DECRYPT;
            if (value.is_REQUIRE__ENCRYPT__ALLOW__DECRYPT) return AWS.Cryptography.MaterialProviders.ESDKCommitmentPolicy.REQUIRE_ENCRYPT_ALLOW_DECRYPT;
            if (value.is_REQUIRE__ENCRYPT__REQUIRE__DECRYPT) return AWS.Cryptography.MaterialProviders.ESDKCommitmentPolicy.REQUIRE_ENCRYPT_REQUIRE_DECRYPT;
            throw new System.ArgumentException("Invalid AWS.Cryptography.MaterialProviders.ESDKCommitmentPolicy value");
        }
        public static software.amazon.cryptography.materialproviders.internaldafny.types._IESDKCommitmentPolicy ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S20_ESDKCommitmentPolicy(AWS.Cryptography.MaterialProviders.ESDKCommitmentPolicy value)
        {
            if (AWS.Cryptography.MaterialProviders.ESDKCommitmentPolicy.FORBID_ENCRYPT_ALLOW_DECRYPT.Equals(value)) return software.amazon.cryptography.materialproviders.internaldafny.types.ESDKCommitmentPolicy.create_FORBID__ENCRYPT__ALLOW__DECRYPT();
            if (AWS.Cryptography.MaterialProviders.ESDKCommitmentPolicy.REQUIRE_ENCRYPT_ALLOW_DECRYPT.Equals(value)) return software.amazon.cryptography.materialproviders.internaldafny.types.ESDKCommitmentPolicy.create_REQUIRE__ENCRYPT__ALLOW__DECRYPT();
            if (AWS.Cryptography.MaterialProviders.ESDKCommitmentPolicy.REQUIRE_ENCRYPT_REQUIRE_DECRYPT.Equals(value)) return software.amazon.cryptography.materialproviders.internaldafny.types.ESDKCommitmentPolicy.create_REQUIRE__ENCRYPT__REQUIRE__DECRYPT();
            throw new System.ArgumentException("Invalid AWS.Cryptography.MaterialProviders.ESDKCommitmentPolicy value");
        }
        public static long FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S15_CountingNumbers(long value)
        {
            return value;
        }
        public static long ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S15_CountingNumbers(long value)
        {
            return value;
        }
        public static string FromDafny_N6_smithy__N3_api__S6_String(Dafny.ISequence<char> value)
        {
            return new string(value.Elements);
        }
        public static Dafny.ISequence<char> ToDafny_N6_smithy__N3_api__S6_String(string value)
        {
            return Dafny.Sequence<char>.FromString(value);
        }
        public static System.IO.MemoryStream FromDafny_N6_smithy__N3_api__S4_Blob(Dafny.ISequence<byte> value)
        {
            return new System.IO.MemoryStream(value.Elements);
        }
        public static Dafny.ISequence<byte> ToDafny_N6_smithy__N3_api__S4_Blob(System.IO.MemoryStream value)
        {
            if (value.ToArray().Length == 0 && value.Length > 0)
            {
                throw new System.ArgumentException("Fatal Error: MemoryStream instance not backed by an array!");
            }
            return Dafny.Sequence<byte>.FromArray(value.ToArray());

        }
        public static AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S38_CryptographicMaterialsManagerReference(software.amazon.cryptography.materialproviders.internaldafny.types.ICryptographicMaterialsManager value)
        {
            // This is converting a reference type in a dependant module.
            // Therefore it defers to the dependant module for conversion
            return AWS.Cryptography.MaterialProviders.TypeConversion.FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S38_CryptographicMaterialsManagerReference(value);
        }
        public static software.amazon.cryptography.materialproviders.internaldafny.types.ICryptographicMaterialsManager ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S38_CryptographicMaterialsManagerReference(AWS.Cryptography.MaterialProviders.ICryptographicMaterialsManager value)
        {
            // This is converting a reference type in a dependant module.
            // Therefore it defers to the dependant module for conversion
            return AWS.Cryptography.MaterialProviders.TypeConversion.ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S38_CryptographicMaterialsManagerReference(value);
        }
        public static AWS.Cryptography.MaterialProviders.IKeyring FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S16_KeyringReference(software.amazon.cryptography.materialproviders.internaldafny.types.IKeyring value)
        {
            // This is converting a reference type in a dependant module.
            // Therefore it defers to the dependant module for conversion
            return AWS.Cryptography.MaterialProviders.TypeConversion.FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S16_KeyringReference(value);
        }
        public static software.amazon.cryptography.materialproviders.internaldafny.types.IKeyring ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S16_KeyringReference(AWS.Cryptography.MaterialProviders.IKeyring value)
        {
            // This is converting a reference type in a dependant module.
            // Therefore it defers to the dependant module for conversion
            return AWS.Cryptography.MaterialProviders.TypeConversion.ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S16_KeyringReference(value);
        }
        public static System.Collections.Generic.Dictionary<string, string> FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext(Dafny.IMap<Dafny.ISequence<byte>, Dafny.ISequence<byte>> value)
        {
            return value.ItemEnumerable.ToDictionary(pair => FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext__M3_key(pair.Car), pair => FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext__M5_value(pair.Cdr));
        }
        public static Dafny.IMap<Dafny.ISequence<byte>, Dafny.ISequence<byte>> ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext(System.Collections.Generic.Dictionary<string, string> value)
        {
            return Dafny.Map<Dafny.ISequence<byte>, Dafny.ISequence<byte>>.FromCollection(value.Select(pair =>
               new Dafny.Pair<Dafny.ISequence<byte>, Dafny.ISequence<byte>>(ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext__M3_key(pair.Key), ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext__M5_value(pair.Value))
           ));
        }
        public static AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S20_ESDKAlgorithmSuiteId(software.amazon.cryptography.materialproviders.internaldafny.types._IESDKAlgorithmSuiteId value)
        {
            if (value.is_ALG__AES__128__GCM__IV12__TAG16__NO__KDF) return AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_NO_KDF;
            if (value.is_ALG__AES__192__GCM__IV12__TAG16__NO__KDF) return AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_NO_KDF;
            if (value.is_ALG__AES__256__GCM__IV12__TAG16__NO__KDF) return AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_NO_KDF;
            if (value.is_ALG__AES__128__GCM__IV12__TAG16__HKDF__SHA256) return AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256;
            if (value.is_ALG__AES__192__GCM__IV12__TAG16__HKDF__SHA256) return AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256;
            if (value.is_ALG__AES__256__GCM__IV12__TAG16__HKDF__SHA256) return AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256;
            if (value.is_ALG__AES__128__GCM__IV12__TAG16__HKDF__SHA256__ECDSA__P256) return AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256;
            if (value.is_ALG__AES__192__GCM__IV12__TAG16__HKDF__SHA384__ECDSA__P384) return AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384;
            if (value.is_ALG__AES__256__GCM__IV12__TAG16__HKDF__SHA384__ECDSA__P384) return AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384;
            if (value.is_ALG__AES__256__GCM__HKDF__SHA512__COMMIT__KEY) return AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY;
            if (value.is_ALG__AES__256__GCM__HKDF__SHA512__COMMIT__KEY__ECDSA__P384) return AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384;
            throw new System.ArgumentException("Invalid AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId value");
        }
        public static software.amazon.cryptography.materialproviders.internaldafny.types._IESDKAlgorithmSuiteId ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S20_ESDKAlgorithmSuiteId(AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId value)
        {
            if (AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_NO_KDF.Equals(value)) return software.amazon.cryptography.materialproviders.internaldafny.types.ESDKAlgorithmSuiteId.create_ALG__AES__128__GCM__IV12__TAG16__NO__KDF();
            if (AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_NO_KDF.Equals(value)) return software.amazon.cryptography.materialproviders.internaldafny.types.ESDKAlgorithmSuiteId.create_ALG__AES__192__GCM__IV12__TAG16__NO__KDF();
            if (AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_NO_KDF.Equals(value)) return software.amazon.cryptography.materialproviders.internaldafny.types.ESDKAlgorithmSuiteId.create_ALG__AES__256__GCM__IV12__TAG16__NO__KDF();
            if (AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256.Equals(value)) return software.amazon.cryptography.materialproviders.internaldafny.types.ESDKAlgorithmSuiteId.create_ALG__AES__128__GCM__IV12__TAG16__HKDF__SHA256();
            if (AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA256.Equals(value)) return software.amazon.cryptography.materialproviders.internaldafny.types.ESDKAlgorithmSuiteId.create_ALG__AES__192__GCM__IV12__TAG16__HKDF__SHA256();
            if (AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA256.Equals(value)) return software.amazon.cryptography.materialproviders.internaldafny.types.ESDKAlgorithmSuiteId.create_ALG__AES__256__GCM__IV12__TAG16__HKDF__SHA256();
            if (AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_128_GCM_IV12_TAG16_HKDF_SHA256_ECDSA_P256.Equals(value)) return software.amazon.cryptography.materialproviders.internaldafny.types.ESDKAlgorithmSuiteId.create_ALG__AES__128__GCM__IV12__TAG16__HKDF__SHA256__ECDSA__P256();
            if (AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_192_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384.Equals(value)) return software.amazon.cryptography.materialproviders.internaldafny.types.ESDKAlgorithmSuiteId.create_ALG__AES__192__GCM__IV12__TAG16__HKDF__SHA384__ECDSA__P384();
            if (AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_256_GCM_IV12_TAG16_HKDF_SHA384_ECDSA_P384.Equals(value)) return software.amazon.cryptography.materialproviders.internaldafny.types.ESDKAlgorithmSuiteId.create_ALG__AES__256__GCM__IV12__TAG16__HKDF__SHA384__ECDSA__P384();
            if (AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY.Equals(value)) return software.amazon.cryptography.materialproviders.internaldafny.types.ESDKAlgorithmSuiteId.create_ALG__AES__256__GCM__HKDF__SHA512__COMMIT__KEY();
            if (AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId.ALG_AES_256_GCM_HKDF_SHA512_COMMIT_KEY_ECDSA_P384.Equals(value)) return software.amazon.cryptography.materialproviders.internaldafny.types.ESDKAlgorithmSuiteId.create_ALG__AES__256__GCM__HKDF__SHA512__COMMIT__KEY__ECDSA__P384();
            throw new System.ArgumentException("Invalid AWS.Cryptography.MaterialProviders.ESDKAlgorithmSuiteId value");
        }
        public static long FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S11_FrameLength(long value)
        {
            return value;
        }
        public static long ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S11_FrameLength(long value)
        {
            return value;
        }
        public static string FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext__M3_key(Dafny.ISequence<byte> value)
        {
            return FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S9_Utf8Bytes(value);
        }
        public static Dafny.ISequence<byte> ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext__M3_key(string value)
        {
            return ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S9_Utf8Bytes(value);
        }
        public static string FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext__M5_value(Dafny.ISequence<byte> value)
        {
            return FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S9_Utf8Bytes(value);
        }
        public static Dafny.ISequence<byte> ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S17_EncryptionContext__M5_value(string value)
        {
            return ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S9_Utf8Bytes(value);
        }
        public static string FromDafny_N3_aws__N12_cryptography__N17_materialProviders__S9_Utf8Bytes(Dafny.ISequence<byte> value)
        {
            System.Text.UTF8Encoding utf8 = new System.Text.UTF8Encoding(false, true);
            return utf8.GetString(value.Elements);
        }
        public static Dafny.ISequence<byte> ToDafny_N3_aws__N12_cryptography__N17_materialProviders__S9_Utf8Bytes(string value)
        {
            System.Text.UTF8Encoding utf8 = new System.Text.UTF8Encoding(false, true);
            return Dafny.Sequence<byte>.FromArray(utf8.GetBytes(value));
        }
        public static System.Exception FromDafny_CommonError(software.amazon.cryptography.encryptionsdk.internaldafny.types._IError value)
        {
            switch (value)
            {
                case software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_AwsCryptographyMaterialProviders dafnyVal:
                    return AWS.Cryptography.MaterialProviders.TypeConversion.FromDafny_CommonError(
                      dafnyVal._AwsCryptographyMaterialProviders
                    );
                case software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_AwsCryptographyPrimitives dafnyVal:
                    return AWS.Cryptography.Primitives.TypeConversion.FromDafny_CommonError(
                      dafnyVal._AwsCryptographyPrimitives
                    );
                case software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_AwsEncryptionSdkException dafnyVal:
                    return FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S25_AwsEncryptionSdkException(dafnyVal);
                case software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_CollectionOfErrors dafnyVal:
                    return new CollectionOfErrors(
                        new System.Collections.Generic.List<Exception>(dafnyVal.dtor_list.CloneAsArray()
                          .Select(x => TypeConversion.FromDafny_CommonError(x))),
                        new string(dafnyVal.dtor_message.Elements));
                case software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_Opaque dafnyVal:
                    return new OpaqueError(dafnyVal._obj);
                default:
                    // The switch MUST be complete for _IError, so `value` MUST NOT be an _IError. (How did you get here?)
                    return new OpaqueError();
            }
        }
        public static software.amazon.cryptography.encryptionsdk.internaldafny.types._IError ToDafny_CommonError(System.Exception value)
        {
            switch (value.GetType().Namespace)
            {
                case "AWS.Cryptography.MaterialProviders":
                    return software.amazon.cryptography.encryptionsdk.internaldafny.types.Error.create_AwsCryptographyMaterialProviders(
                      AWS.Cryptography.MaterialProviders.TypeConversion.ToDafny_CommonError(value)
                    );
            }
            switch (value)
            {
                case AWS.Cryptography.EncryptionSDK.AwsEncryptionSdkException exception:
                    return ToDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S25_AwsEncryptionSdkException(exception);
                case CollectionOfErrors collectionOfErrors:
                    return new software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_CollectionOfErrors(
                      Dafny.Sequence<software.amazon.cryptography.encryptionsdk.internaldafny.types._IError>
                        .FromArray(
                          collectionOfErrors.list.Select
                              (x => TypeConversion.ToDafny_CommonError(x))
                            .ToArray()),
                      Dafny.Sequence<char>.FromString(collectionOfErrors.Message)
                    );
                // OpaqueError is redundant, but listed for completeness.
                case OpaqueError exception:
                    return new software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_Opaque(exception);
                case System.Exception exception:
                    return new software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_Opaque(exception);
                default:
                    // The switch MUST be complete for System.Exception, so `value` MUST NOT be an System.Exception. (How did you get here?)
                    return new software.amazon.cryptography.encryptionsdk.internaldafny.types.Error_Opaque(value);
            }
        }
    }
}
