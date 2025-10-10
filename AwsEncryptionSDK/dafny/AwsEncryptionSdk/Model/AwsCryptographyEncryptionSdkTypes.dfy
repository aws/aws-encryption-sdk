// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
include "../../../../mpl/StandardLibrary/src/Index.dfy"
include "../../../../mpl/AwsCryptographicMaterialProviders/dafny/AwsCryptographicMaterialProviders/src/Index.dfy"
include "../../../../mpl/AwsCryptographyPrimitives/src/Index.dfy"
module {:extern "software.amazon.cryptography.encryptionsdk.internaldafny.types" } AwsCryptographyEncryptionSdkTypes
{
  import opened Wrappers
  import opened StandardLibrary.UInt
  import opened UTF8
  import AwsCryptographyMaterialProvidersTypes
  import AwsCryptographyPrimitivesTypes
    // Generic helpers for verification of mock/unit tests.
  datatype DafnyCallEvent<I, O> = DafnyCallEvent(input: I, output: O)

  // Begin Generated Types

  class IAwsEncryptionSdkClientCallHistory {
    ghost constructor() {
      Encrypt := [];
      Decrypt := [];
    }
    ghost var Encrypt: seq<DafnyCallEvent<EncryptInput, Result<EncryptOutput, Error>>>
    ghost var Decrypt: seq<DafnyCallEvent<DecryptInput, Result<DecryptOutput, Error>>>
  }
  trait {:termination false} IAwsEncryptionSdkClient
  {
    // Helper to define any additional modifies/reads clauses.
    // If your operations need to mutate state,
    // add it in your constructor function:
    // Modifies := {your, fields, here, History};
    // If you do not need to mutate anything:
    // Modifies := {History};

    ghost const Modifies: set<object>
    // For an unassigned field defined in a trait,
    // Dafny can only assign a value in the constructor.
    // This means that for Dafny to reason about this value,
    // it needs some way to know (an invariant),
    // about the state of the object.
    // This builds on the Valid/Repr paradigm
    // To make this kind requires safe to add
    // to methods called from unverified code,
    // the predicate MUST NOT take any arguments.
    // This means that the correctness of this requires
    // MUST only be evaluated by the class itself.
    // If you require any additional mutation,
    // then you MUST ensure everything you need in ValidState.
    // You MUST also ensure ValidState in your constructor.
    predicate ValidState()
      ensures ValidState() ==> History in Modifies
    ghost const History: IAwsEncryptionSdkClientCallHistory
    predicate EncryptEnsuresPublicly(input: EncryptInput , output: Result<EncryptOutput, Error>)
    // The public method to be called by library consumers
    method Encrypt ( input: EncryptInput )
      returns (output: Result<EncryptOutput, Error>)
      requires
        && ValidState() && ( input.materialsManager.Some? ==>
                               && input.materialsManager.value.ValidState()
                               && input.materialsManager.value.Modifies !! {History}
           ) && ( input.keyring.Some? ==>
                    && input.keyring.value.ValidState()
                    && input.keyring.value.Modifies !! {History}
           )
      modifies Modifies - {History} ,
               (if input.materialsManager.Some? then input.materialsManager.value.Modifies else {}) ,
               (if input.keyring.Some? then input.keyring.value.Modifies else {}) ,
               History`Encrypt
      // Dafny will skip type parameters when generating a default decreases clause.
      decreases Modifies - {History} ,
                (if input.materialsManager.Some? then input.materialsManager.value.Modifies else {}) ,
                (if input.keyring.Some? then input.keyring.value.Modifies else {})
      ensures
        && ValidState()
      ensures EncryptEnsuresPublicly(input, output)
      ensures History.Encrypt == old(History.Encrypt) + [DafnyCallEvent(input, output)]

    predicate DecryptEnsuresPublicly(input: DecryptInput , output: Result<DecryptOutput, Error>)
    // The public method to be called by library consumers
    method Decrypt ( input: DecryptInput )
      returns (output: Result<DecryptOutput, Error>)
      requires
        && ValidState() && ( input.materialsManager.Some? ==>
                               && input.materialsManager.value.ValidState()
                               && input.materialsManager.value.Modifies !! {History}
           ) && ( input.keyring.Some? ==>
                    && input.keyring.value.ValidState()
                    && input.keyring.value.Modifies !! {History}
           )
      modifies Modifies - {History} ,
               (if input.materialsManager.Some? then input.materialsManager.value.Modifies else {}) ,
               (if input.keyring.Some? then input.keyring.value.Modifies else {}) ,
               History`Decrypt
      // Dafny will skip type parameters when generating a default decreases clause.
      decreases Modifies - {History} ,
                (if input.materialsManager.Some? then input.materialsManager.value.Modifies else {}) ,
                (if input.keyring.Some? then input.keyring.value.Modifies else {})
      ensures
        && ValidState()
      ensures DecryptEnsuresPublicly(input, output)
      ensures History.Decrypt == old(History.Decrypt) + [DafnyCallEvent(input, output)]

  }
  datatype AwsEncryptionSdkConfig = | AwsEncryptionSdkConfig (
    nameonly commitmentPolicy: Option<AwsCryptographyMaterialProvidersTypes.ESDKCommitmentPolicy> := Option.None ,
    nameonly maxEncryptedDataKeys: Option<CountingNumbers> := Option.None ,
    nameonly netV4_0_0_RetryPolicy: Option<NetV4_0_0_RetryPolicy> := Option.None
  )
  type CountingNumbers = x: int64 | IsValid_CountingNumbers(x) witness *
  predicate method IsValid_CountingNumbers(x: int64) {
    ( 1 <= x  )
  }
  datatype DecryptInput = | DecryptInput (
    nameonly ciphertext: seq<uint8> ,
    nameonly materialsManager: Option<AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager> := Option.None ,
    nameonly keyring: Option<AwsCryptographyMaterialProvidersTypes.IKeyring> := Option.None ,
    nameonly encryptionContext: Option<AwsCryptographyMaterialProvidersTypes.EncryptionContext> := Option.None
  )
  datatype DecryptOutput = | DecryptOutput (
    nameonly plaintext: seq<uint8> ,
    nameonly encryptionContext: AwsCryptographyMaterialProvidersTypes.EncryptionContext ,
    nameonly algorithmSuiteId: AwsCryptographyMaterialProvidersTypes.ESDKAlgorithmSuiteId
  )
  datatype EncryptInput = | EncryptInput (
    nameonly plaintext: seq<uint8> ,
    nameonly encryptionContext: Option<AwsCryptographyMaterialProvidersTypes.EncryptionContext> := Option.None ,
    nameonly materialsManager: Option<AwsCryptographyMaterialProvidersTypes.ICryptographicMaterialsManager> := Option.None ,
    nameonly keyring: Option<AwsCryptographyMaterialProvidersTypes.IKeyring> := Option.None ,
    nameonly algorithmSuiteId: Option<AwsCryptographyMaterialProvidersTypes.ESDKAlgorithmSuiteId> := Option.None ,
    nameonly frameLength: Option<FrameLength> := Option.None
  )
  datatype EncryptOutput = | EncryptOutput (
    nameonly ciphertext: seq<uint8> ,
    nameonly encryptionContext: AwsCryptographyMaterialProvidersTypes.EncryptionContext ,
    nameonly algorithmSuiteId: AwsCryptographyMaterialProvidersTypes.ESDKAlgorithmSuiteId
  )
  type FrameLength = x: int64 | IsValid_FrameLength(x) witness *
  predicate method IsValid_FrameLength(x: int64) {
    ( 1 <= x <= 4294967296 )
  }
  datatype NetV4_0_0_RetryPolicy =
    | FORBID_RETRY
    | ALLOW_RETRY
  datatype Error =
      // Local Error structures are listed here
    | AwsEncryptionSdkException (
        nameonly message: string
      )
      // Any dependent models are listed here
    | AwsCryptographyMaterialProviders(AwsCryptographyMaterialProviders: AwsCryptographyMaterialProvidersTypes.Error)
    | AwsCryptographyPrimitives(AwsCryptographyPrimitives: AwsCryptographyPrimitivesTypes.Error)
      // The Collection error is used to collect several errors together
      // This is useful when composing OR logic.
      // Consider the following method:
      // 
      // method FN<I, O>(n:I)
      //   returns (res: Result<O, Types.Error>)
      //   ensures A(I).Success? ==> res.Success?
      //   ensures B(I).Success? ==> res.Success?
      //   ensures A(I).Failure? && B(I).Failure? ==> res.Failure?
      // 
      // If either A || B is successful then FN is successful.
      // And if A && B fail then FN will fail.
      // But what information should FN transmit back to the caller?
      // While it may be correct to hide these details from the caller,
      // this can not be the globally correct option.
      // Suppose that A and B can be blocked by different ACLs,
      // and that their representation of I is only eventually consistent.
      // How can the caller distinguish, at a minimum for logging,
      // the difference between the four failure modes?
    // || (!access(A(I)) && !access(B(I)))
    // || (!exit(A(I)) && !exit(B(I)))
    // || (!access(A(I)) && !exit(B(I)))
    // || (!exit(A(I)) && !access(B(I)))
    | CollectionOfErrors(list: seq<Error>, nameonly message: string)
      // The Opaque error, used for native, extern, wrapped or unknown errors
    | Opaque(obj: object)
  type OpaqueError = e: Error | e.Opaque? witness *
}
abstract module AbstractAwsCryptographyEncryptionSdkService
{
  import opened Wrappers
  import opened StandardLibrary.UInt
  import opened UTF8
  import opened Types = AwsCryptographyEncryptionSdkTypes
  import Operations : AbstractAwsCryptographyEncryptionSdkOperations
  function method DefaultAwsEncryptionSdkConfig(): AwsEncryptionSdkConfig
  method ESDK(config: AwsEncryptionSdkConfig := DefaultAwsEncryptionSdkConfig())
    returns (res: Result<ESDKClient, Error>)
    ensures res.Success? ==>
              && fresh(res.value)
              && fresh(res.value.Modifies)
              && fresh(res.value.History)
              && res.value.ValidState()

  // Helper functions for the benefit of native code to create a Success(client) without referring to Dafny internals
  function method CreateSuccessOfClient(client: IAwsEncryptionSdkClient): Result<IAwsEncryptionSdkClient, Error> {
    Success(client)
  }
  function method CreateFailureOfError(error: Error): Result<IAwsEncryptionSdkClient, Error> {
    Failure(error)
  }
  class ESDKClient extends IAwsEncryptionSdkClient
  {
    constructor(config: Operations.InternalConfig)
      requires Operations.ValidInternalConfig?(config)
      ensures
        && ValidState()
        && fresh(History)
        && this.config == config
    const config: Operations.InternalConfig
    predicate ValidState()
      ensures ValidState() ==>
                && Operations.ValidInternalConfig?(config)
                && History !in Operations.ModifiesInternalConfig(config)
                && Modifies == Operations.ModifiesInternalConfig(config) + {History}
    predicate EncryptEnsuresPublicly(input: EncryptInput , output: Result<EncryptOutput, Error>)
    {Operations.EncryptEnsuresPublicly(input, output)}
    // The public method to be called by library consumers
    method Encrypt ( input: EncryptInput )
      returns (output: Result<EncryptOutput, Error>)
      requires
        && ValidState() && ( input.materialsManager.Some? ==>
                               && input.materialsManager.value.ValidState()
                               && input.materialsManager.value.Modifies !! {History}
           ) && ( input.keyring.Some? ==>
                    && input.keyring.value.ValidState()
                    && input.keyring.value.Modifies !! {History}
           )
      modifies Modifies - {History} ,
               (if input.materialsManager.Some? then input.materialsManager.value.Modifies else {}) ,
               (if input.keyring.Some? then input.keyring.value.Modifies else {}) ,
               History`Encrypt
      // Dafny will skip type parameters when generating a default decreases clause.
      decreases Modifies - {History} ,
                (if input.materialsManager.Some? then input.materialsManager.value.Modifies else {}) ,
                (if input.keyring.Some? then input.keyring.value.Modifies else {})
      ensures
        && ValidState()
      ensures EncryptEnsuresPublicly(input, output)
      ensures History.Encrypt == old(History.Encrypt) + [DafnyCallEvent(input, output)]
    {
      output := Operations.Encrypt(config, input);
      History.Encrypt := History.Encrypt + [DafnyCallEvent(input, output)];
    }

    predicate DecryptEnsuresPublicly(input: DecryptInput , output: Result<DecryptOutput, Error>)
    {Operations.DecryptEnsuresPublicly(input, output)}
    // The public method to be called by library consumers
    method Decrypt ( input: DecryptInput )
      returns (output: Result<DecryptOutput, Error>)
      requires
        && ValidState() && ( input.materialsManager.Some? ==>
                               && input.materialsManager.value.ValidState()
                               && input.materialsManager.value.Modifies !! {History}
           ) && ( input.keyring.Some? ==>
                    && input.keyring.value.ValidState()
                    && input.keyring.value.Modifies !! {History}
           )
      modifies Modifies - {History} ,
               (if input.materialsManager.Some? then input.materialsManager.value.Modifies else {}) ,
               (if input.keyring.Some? then input.keyring.value.Modifies else {}) ,
               History`Decrypt
      // Dafny will skip type parameters when generating a default decreases clause.
      decreases Modifies - {History} ,
                (if input.materialsManager.Some? then input.materialsManager.value.Modifies else {}) ,
                (if input.keyring.Some? then input.keyring.value.Modifies else {})
      ensures
        && ValidState()
      ensures DecryptEnsuresPublicly(input, output)
      ensures History.Decrypt == old(History.Decrypt) + [DafnyCallEvent(input, output)]
    {
      output := Operations.Decrypt(config, input);
      History.Decrypt := History.Decrypt + [DafnyCallEvent(input, output)];
    }

  }
}
abstract module AbstractAwsCryptographyEncryptionSdkOperations {
  import opened Wrappers
  import opened StandardLibrary.UInt
  import opened UTF8
  import opened Types = AwsCryptographyEncryptionSdkTypes
  type InternalConfig
  predicate ValidInternalConfig?(config: InternalConfig)
  function ModifiesInternalConfig(config: InternalConfig): set<object>
  predicate EncryptEnsuresPublicly(input: EncryptInput , output: Result<EncryptOutput, Error>)
  // The private method to be refined by the library developer


  method Encrypt ( config: InternalConfig , input: EncryptInput )
    returns (output: Result<EncryptOutput, Error>)
    requires
      && ValidInternalConfig?(config) && ( input.materialsManager.Some? ==>
                                             && input.materialsManager.value.ValidState()
         ) && ( input.keyring.Some? ==>
                  && input.keyring.value.ValidState()
         )
    modifies ModifiesInternalConfig(config) ,
             (if input.materialsManager.Some? then input.materialsManager.value.Modifies else {}) ,
             (if input.keyring.Some? then input.keyring.value.Modifies else {})
    // Dafny will skip type parameters when generating a default decreases clause.
    decreases ModifiesInternalConfig(config) ,
              (if input.materialsManager.Some? then input.materialsManager.value.Modifies else {}) ,
              (if input.keyring.Some? then input.keyring.value.Modifies else {})
    ensures
      && ValidInternalConfig?(config)
    ensures EncryptEnsuresPublicly(input, output)


  predicate DecryptEnsuresPublicly(input: DecryptInput , output: Result<DecryptOutput, Error>)
  // The private method to be refined by the library developer


  method Decrypt ( config: InternalConfig , input: DecryptInput )
    returns (output: Result<DecryptOutput, Error>)
    requires
      && ValidInternalConfig?(config) && ( input.materialsManager.Some? ==>
                                             && input.materialsManager.value.ValidState()
         ) && ( input.keyring.Some? ==>
                  && input.keyring.value.ValidState()
         )
    modifies ModifiesInternalConfig(config) ,
             (if input.materialsManager.Some? then input.materialsManager.value.Modifies else {}) ,
             (if input.keyring.Some? then input.keyring.value.Modifies else {})
    // Dafny will skip type parameters when generating a default decreases clause.
    decreases ModifiesInternalConfig(config) ,
              (if input.materialsManager.Some? then input.materialsManager.value.Modifies else {}) ,
              (if input.keyring.Some? then input.keyring.value.Modifies else {})
    ensures
      && ValidInternalConfig?(config)
    ensures DecryptEnsuresPublicly(input, output)
}
