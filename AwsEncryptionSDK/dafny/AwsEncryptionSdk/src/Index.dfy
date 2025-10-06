// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "AwsEncryptionSdkOperations.dfy"

module 
  {:extern "software.amazon.cryptography.encryptionsdk.internaldafny" }
  EncryptionSdk refines AbstractAwsCryptographyEncryptionSdkService {
  import Operations = AwsEncryptionSdkOperations
  import Primitives = AtomicPrimitives
  import MaterialProviders
  import AwsCryptographyMaterialProvidersTypes

  // TODO Post-#619: Formally Verify this section
  // TODO Post-#619: Duvet this section
  function method DefaultAwsEncryptionSdkConfig(): AwsEncryptionSdkConfig
  {
    AwsEncryptionSdkConfig(
      commitmentPolicy := Some(AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy.REQUIRE_ENCRYPT_REQUIRE_DECRYPT),
      maxEncryptedDataKeys := None,
      netV4_0_0_RetryPolicy := Some(NetV4_0_0_RetryPolicy.ALLOW_RETRY)
    )
  }

  method ESDK(config: AwsEncryptionSdkConfig)
    returns (res: Result<ESDKClient, Error>)
  {
    var maybeCrypto := Primitives.AtomicPrimitives();
    var cryptoX: AwsCryptographyPrimitivesTypes.IAwsCryptographicPrimitivesClient :- maybeCrypto
      .MapFailure(e => AwsCryptographyPrimitives(e));
    assert cryptoX is Primitives.AtomicPrimitivesClient;
    var crypto := cryptoX as Primitives.AtomicPrimitivesClient;

    var maybeMpl := MaterialProviders.MaterialProviders();
    var mplX: AwsCryptographyMaterialProvidersTypes.IAwsCryptographicMaterialProvidersClient :- maybeMpl
      .MapFailure(e => AwsCryptographyMaterialProviders(e));
    assert mplX is MaterialProviders.MaterialProvidersClient;
    var mpl := mplX as MaterialProviders.MaterialProvidersClient;

    var internalConfig := Operations.Config(
      crypto := crypto,
      mpl := mpl,
      commitmentPolicy := config.commitmentPolicy.UnwrapOr(AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy.REQUIRE_ENCRYPT_REQUIRE_DECRYPT),
      maxEncryptedDataKeys := config.maxEncryptedDataKeys,
      netV4_0_0_RetryPolicy := config.netV4_0_0_RetryPolicy.UnwrapOr(NetV4_0_0_RetryPolicy.ALLOW_RETRY)
    );
    var client := new ESDKClient(internalConfig);
    return Success(client);
  }

  class ESDKClient... {
    predicate ValidState()
    {
      && Operations.ValidInternalConfig?(config)
      && History !in Operations.ModifiesInternalConfig(config)
      && Modifies == Operations.ModifiesInternalConfig(config) + {History}
    }

    constructor(config: Operations.InternalConfig)
    {
      this.config := config;
      History := new IAwsEncryptionSdkClientCallHistory();
      Modifies := Operations.ModifiesInternalConfig(config) + {History};
    }
  }
}
