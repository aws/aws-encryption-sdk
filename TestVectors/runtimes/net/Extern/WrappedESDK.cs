using AwsArnParsing_Compile;
using Wrappers_Compile;

namespace software.amazon.cryptography.encryptionsdk.internaldafny.wrapped
{
    public partial class __default
    {
        public static
            _IResult<
                types.IAwsEncryptionSdkClient,
                types.Error
            >
            WrappedESDK(
                types._IAwsEncryptionSdkConfig config)
        {
            var wrappedConfig = AWS.Cryptography.EncryptionSDK.Wrapped.TypeConversion
                .FromDafny_N3_aws__N12_cryptography__N13_encryptionSdk__S22_AwsEncryptionSdkConfig(config);
            var impl = new AWS.Cryptography.EncryptionSDK.ESDK(wrappedConfig);
            var wrappedClient = new AWS.Cryptography.EncryptionSDK.Wrapped.AwsEncryptionSdkShim(impl);

            return Result<types.IAwsEncryptionSdkClient,
                types.Error>.create_Success(wrappedClient);
        }
    }
}