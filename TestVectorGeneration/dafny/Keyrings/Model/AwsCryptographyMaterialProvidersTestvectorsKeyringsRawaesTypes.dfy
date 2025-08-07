// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0
// Do not modify this file. This file is machine generated, and any changes to it will be overwritten.
include "../../../../mpl/StandardLibrary/src/Index.dfy"
include "../../../../mpl/AwsCryptographicMaterialProviders/dafny/AwsCryptographicMaterialProviders/src/Index.dfy"
module {:extern "software.amazon.cryptography.materialproviders.testvectors.keyrings.rawaes.internaldafny.types" } AwsCryptographyMaterialProvidersTestvectorsKeyringsRawaesTypes
{
  import opened Wrappers
  import opened StandardLibrary.UInt
  import opened UTF8
  import AwsCryptographyMaterialProvidersTypes
  // Generic helpers for verification of mock/unit tests.
  datatype DafnyCallEvent<I, O> = DafnyCallEvent(input: I, output: O)

  // Begin Generated Types

  datatype CreateRawAesKeyringSmokeTestOutput = | CreateRawAesKeyringSmokeTestOutput (

                                                )
  datatype KeyringsTestConfig = | KeyringsTestConfig (

                                )
  class IKeyringsTestServiceClientCallHistory {
    ghost constructor() {
      CreateRawAesKeyringSmokeTest := [];
    }
    ghost var CreateRawAesKeyringSmokeTest: seq<DafnyCallEvent<AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput, Result<CreateRawAesKeyringSmokeTestOutput, Error>>>
  }
  trait {:termination false} IKeyringsTestServiceClient
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
    ghost const History: IKeyringsTestServiceClientCallHistory
    predicate CreateRawAesKeyringSmokeTestEnsuresPublicly(input: AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput , output: Result<CreateRawAesKeyringSmokeTestOutput, Error>)
    // The public method to be called by library consumers
    method CreateRawAesKeyringSmokeTest ( input: AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput )
      returns (output: Result<CreateRawAesKeyringSmokeTestOutput, Error>)
      requires
        && ValidState()
      modifies Modifies - {History} ,
               History`CreateRawAesKeyringSmokeTest
      // Dafny will skip type parameters when generating a default decreases clause.
      decreases Modifies - {History}
      ensures
        && ValidState()
      ensures CreateRawAesKeyringSmokeTestEnsuresPublicly(input, output)
      ensures History.CreateRawAesKeyringSmokeTest == old(History.CreateRawAesKeyringSmokeTest) + [DafnyCallEvent(input, output)]

  }
  datatype Error =
      // Local Error structures are listed here

      // Any dependent models are listed here
    | AwsCryptographyMaterialProviders(AwsCryptographyMaterialProviders: AwsCryptographyMaterialProvidersTypes.Error)
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
      // A better Opaque, with a visible string representation.
    | OpaqueWithText(obj: object, objMessage : string)
  type OpaqueError = e: Error | e.Opaque? || e.OpaqueWithText? witness *
  // This dummy subset type is included to make sure Dafny
  // always generates a _ExternBase___default.java class.
  type DummySubsetType = x: int | IsDummySubsetType(x) witness 1
  predicate method IsDummySubsetType(x: int) {
    0 < x
  }

}
abstract module AbstractAwsCryptographyMaterialProvidersTestvectorsKeyringsRawaesService
{
  import opened Wrappers
  import opened StandardLibrary.UInt
  import opened UTF8
  import opened Types = AwsCryptographyMaterialProvidersTestvectorsKeyringsRawaesTypes
  import Operations : AbstractAwsCryptographyMaterialProvidersTestvectorsKeyringsRawaesOperations
  function method DefaultKeyringsTestConfig(): KeyringsTestConfig
  method KeyringsTest(config: KeyringsTestConfig := DefaultKeyringsTestConfig())
    returns (res: Result<KeyringsTestClient, Error>)
    ensures res.Success? ==>
              && fresh(res.value)
              && fresh(res.value.Modifies)
              && fresh(res.value.History)
              && res.value.ValidState()

  // Helper functions for the benefit of native code to create a Success(client) without referring to Dafny internals
  function method CreateSuccessOfClient(client: IKeyringsTestServiceClient): Result<IKeyringsTestServiceClient, Error> {
    Success(client)
  }
  function method CreateFailureOfError(error: Error): Result<IKeyringsTestServiceClient, Error> {
    Failure(error)
  }
  class KeyringsTestClient extends IKeyringsTestServiceClient
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
    predicate CreateRawAesKeyringSmokeTestEnsuresPublicly(input: AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput , output: Result<CreateRawAesKeyringSmokeTestOutput, Error>)
    {Operations.CreateRawAesKeyringSmokeTestEnsuresPublicly(input, output)}
    // The public method to be called by library consumers
    method CreateRawAesKeyringSmokeTest ( input: AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput )
      returns (output: Result<CreateRawAesKeyringSmokeTestOutput, Error>)
      requires
        && ValidState()
      modifies Modifies - {History} ,
               History`CreateRawAesKeyringSmokeTest
      // Dafny will skip type parameters when generating a default decreases clause.
      decreases Modifies - {History}
      ensures
        && ValidState()
      ensures CreateRawAesKeyringSmokeTestEnsuresPublicly(input, output)
      ensures History.CreateRawAesKeyringSmokeTest == old(History.CreateRawAesKeyringSmokeTest) + [DafnyCallEvent(input, output)]
    {
      output := Operations.CreateRawAesKeyringSmokeTest(config, input);
      History.CreateRawAesKeyringSmokeTest := History.CreateRawAesKeyringSmokeTest + [DafnyCallEvent(input, output)];
    }

  }
}
abstract module AbstractAwsCryptographyMaterialProvidersTestvectorsKeyringsRawaesOperations {
  import opened Wrappers
  import opened StandardLibrary.UInt
  import opened UTF8
  import opened Types = AwsCryptographyMaterialProvidersTestvectorsKeyringsRawaesTypes
  type InternalConfig
  predicate ValidInternalConfig?(config: InternalConfig)
  function ModifiesInternalConfig(config: InternalConfig): set<object>
  predicate CreateRawAesKeyringSmokeTestEnsuresPublicly(input: AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput , output: Result<CreateRawAesKeyringSmokeTestOutput, Error>)
  // The private method to be refined by the library developer


  method CreateRawAesKeyringSmokeTest ( config: InternalConfig , input: AwsCryptographyMaterialProvidersTypes.CreateRawAesKeyringInput )
    returns (output: Result<CreateRawAesKeyringSmokeTestOutput, Error>)
    requires
      && ValidInternalConfig?(config)
    modifies ModifiesInternalConfig(config)
    // Dafny will skip type parameters when generating a default decreases clause.
    decreases ModifiesInternalConfig(config)
    ensures
      && ValidInternalConfig?(config)
    ensures CreateRawAesKeyringSmokeTestEnsuresPublicly(input, output)
}
