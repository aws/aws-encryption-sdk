// Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

include "WriteVectors.dfy"
include "EsdkTestManifests.dfy"
include "EsdkManifestOptions.dfy"

module {:options "-functionSyntax:4"} WrappedESDKMain {
  import opened Wrappers
  import WrappedESDK
  import WriteVectors

  import EsdkTestManifests
  import EsdkManifestOptions
  import Seq
  import opened GetOpt

  method Main2(args: seq<string>) {
    var vectorOptions := Options("test-vectors", "?",
                                 [
                                   Param.Command(Options("decrypt", "decrypt command for test-vectors",
                                                         [
                                                           Param.Opt("manifest-path", "relative path to the location of the manifest", unused := Required),
                                                           Param.Opt("test-name", "id of the test to run")
                                                         ])),
                                   Param.Command(Options("encrypt", "encrypt command for test-vectors",
                                                         [
                                                           Param.Opt("manifest-path", "relative path to the location of the manifest", unused := Required),
                                                           Param.Opt("decrypt-manifest-path", "relative path to the location where the decrypted manifest will be written to.", unused := Required),
                                                           Param.Opt("test-name", "id of the test to run")
                                                         ])),
                                   Param.Command(Options("encrypt-manifest", "encryp manifest command for test-vectors",
                                                         [
                                                           Param.Opt("encrypt-manifest-output", "relative path of where to store the encrypt-manifest produced", unused := Required)
                                                         ]))
                                 ]);
    // The expectation is that the first argument
    // is the filename or runtime
    expect 0 < |args|;
    var parsedOptions? := GetOptions(vectorOptions, args);

    if parsedOptions?.Success? {
      var h := NeedsHelp(vectorOptions, parsedOptions?.value);
      if h.Some? {
        print h.value;
        return;
      }
      var op? := ParseCommandLineOptions(parsedOptions?.value);

      if op?.Success? {
        var op := op?.value;
        match op
        case Decrypt(_, _) =>
          var result := EsdkTestManifests.StartDecryptVectors(op);
          if result.Failure? {
            print result.error;
          }
          expect result.Success?;
        case Encrypt(_, _, _, _) =>
          var result := EsdkTestManifests.StartEncryptVectors(op);
          if result.Failure? {
            print result.error;
          }
          expect result.Success?;
        case EncryptManifest(_, _) =>
          var result := WriteVectors.WritetestVectors(op);
          if result.Failure? {
            print result.error;
          }
          expect result.Success?;
      } else {
        print op?.error + "\n";
        print "help\n";
      }
    } else {
      print parsedOptions?.error + "\n";
    }

  }

  function ParseCommandLineOptions(parsedOptions: Parsed)
    : (output: Result<EsdkManifestOptions.ManifestOptions, string>)
  {
    :- Need(parsedOptions.subcommand.Some?, "Must supply subcommand\n");

    match parsedOptions.subcommand.value.command
    case "decrypt" => ParseDecryptCmd(parsedOptions.subcommand.value.params)
    case "encrypt" => ParseEncryptCmd(parsedOptions.subcommand.value.params)
    case "encrypt-manifest" => ParseEncryptManifestCmd(parsedOptions.subcommand.value.params)
    // GetOpt GetOptions actually takes care of this for us but Dafny doesn't know so we must have default case.
    case _ => Failure("Received unknown subcommand")
  }

  function ParseDecryptCmd(params: seq<OneArg>)
    : (output: Result<EsdkManifestOptions.ManifestOptions, string>)
    ensures output.Success? ==> output.value.Decrypt?
  {
    var manifestPath? := OptValue(params, "manifest-path");
    var testName? := OptValue(params, "test-name");

    var manifestPath := if manifestPath?.Some? then manifestPath?.value else ".";
    :- Need(0 < |manifestPath|, "Invalid manifest path length\n");

    Success(EsdkManifestOptions.Decrypt(
              manifestPath := if Seq.Last(manifestPath) == '/' then manifestPath else manifestPath + "/",
              testName := if testName?.Some? then testName?  else None
            ))
  }

  function ParseEncryptCmd(params: seq<OneArg>)
    : (output: Result<EsdkManifestOptions.ManifestOptions, string>)
    ensures output.Success? ==> output.value.Encrypt?
  {
    var manifestPath? := OptValue(params, "manifest-path");
    var manifestName? := OptValue(params, "manifest");
    var decryptManifestPath? := OptValue(params, "decrypt-manifest-path");
    var testName? := OptValue(params, "test-name");

    var manifestPath := if manifestPath?.Some? then manifestPath?.value else ".";
    var manifestName := if manifestName?.Some? then manifestName?.value else "encrypt-manifest.json";
    var decryptManifestPath := if decryptManifestPath?.Some? then decryptManifestPath?.value else ".";
    :- Need(
         && 0 < |manifestPath|
         && 0 < |decryptManifestPath|,
         "Invalid manifest or decrypt manifest path length\n"
       );

    Success(EsdkManifestOptions.Encrypt(
              manifestPath := if Seq.Last(manifestPath) == '/' then manifestPath else manifestPath + "/",
              manifest := manifestName,
              decryptManifestOutput := if Seq.Last(decryptManifestPath) == '/' then decryptManifestPath else decryptManifestPath + "/",
              testName := if testName?.Some? then testName? else None
            ))
  }

  function ParseEncryptManifestCmd(params: seq<OneArg>)
    : (output: Result<EsdkManifestOptions.ManifestOptions, string>)
    ensures output.Success? ==> output.value.EncryptManifest?
  {
    var encryptManifestOutput? := OptValue(params, "encrypt-manifest-output");
    var encryptManifestOutput := if encryptManifestOutput?.Some? then encryptManifestOutput?.value else ".";
    :- Need(0 < |encryptManifestOutput|, "Invalid encrypt manifest output length");

    Success(EsdkManifestOptions.EncryptManifest(
      encryptManifestOutput := if Seq.Last(encryptManifestOutput) == '/' then encryptManifestOutput else encryptManifestOutput + "/",
      version := 5
    ))
  }

}