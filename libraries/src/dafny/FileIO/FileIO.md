
## The `FileIO` module {#sec-fileio}

_The FileIO methods currently only work with `--unicode-char:false`, which is not the default for Dafny 4._

The `FileIO` module provides basic file I/O operations.
Right now, these are limited to reading bytes from a file and writing bytes to a file.
The API is intentionally limited in scope and will be expanded later.

Unlike other modules in the `libraries` repository,
the `FileIO` module will not compile or run correctly without a language-specific implementation file.
Language-specific implementation files are provided for C#/.NET, Java, and Javascript.
(Python and Golang support are planned.)
Concretely, to use `FileIO` in your code, you must:

1. `include` and `import` the `FileIO` module as you would any other library module
2. incorporate the corresponding language-specific implementation file (e.g. `FileIO.cs`) when building or running your program

For example, to run a `Program.dfy` file that depends on the `FileIO` module, run the following.
(This assumes you have the `libraries` repository checked out within the working directory.)

```bash
# C#/.NET
$ dafny run Program.dfy --input libraries/src/FileIO/FileIO.cs

# Java
$ dafny run Program.dfy --target:java --input libraries/src/FileIO/FileIO.java

# Javascript
$ dafny run Program.dfy --target:js --input libraries/src/FileIO/FileIO.js
```

(If you aren't using `dafny run` to run your program,
then you should instead integrate the appropriate language-specific implementation file in your build system.)

These sample programs from the examples folder illustrate how to read and write files:

<!-- %check-resolve -->
```dafny
module ReadBytesFromFile {
  import Dafny.FileIO

  method Main(args: seq<string>) {
    expect |args| > 0;
    expect |args| == 3, "usage: " + args[0] + " DATA_PATH EXPECTED_ERROR_PREFIX";
    var dataPath := args[1];
    var expectedErrorPrefix := args[2];

    // Happy path: read from the data file, and check that we see the expected content.
    {
      var expectedStr := "Hello world\nGoodbye\n";
      // This conversion is safe only for ASCII values. For Unicode conversions, see the Unicode modules.
      var expectedBytes := seq(|expectedStr|, i requires 0 <= i < |expectedStr| => expectedStr[i] as int);

      var res := FileIO.ReadBytesFromFile(dataPath);
      expect res.Success?, "unexpected failure: " + res.error;

      var readBytes := seq(|res.value|, i requires 0 <= i < |res.value| => res.value[i] as int);
      expect readBytes == expectedBytes, "read unexpected byte sequence";
    }

    // Failure path: attempting to read from a blank file path should never work.
    {
      var res := FileIO.ReadBytesFromFile("");
      expect res.Failure?, "unexpected success";
      expect expectedErrorPrefix <= res.error, "unexpected error message: " + res.error;
    }
  }
}
```

<!-- %check-resolve -->
```dafny
module WriteBytesToFile {
  import Dafny.FileIO

  method Main(args: seq<string>) {
    expect |args| > 0;
    expect |args| == 3, "usage: " + args[0] + " OUTPUT_DIR EXPECTED_ERROR_PREFIX";
    var outputDir := args[1];
    var expectedErrorPrefix := args[2];

    // Happy paths: write files to the output dir. (The %diff LIT commands check that we wrote the correct content.)
    {
      // Ideally we would define `str` as a constant and compute `bytes` automatically.
      // To do so, we would need to convert each `char` in `str` to a `bv8` value, by using `as bv8`.
      // But that triggers a bug in the Java compiler: <https://github.com/dafny-lang/dafny/issues/2942>.
      // So for now we settle for manually writing out the desired `bytes`,
      // and statically verifying that these values match the characters of `str`.
      ghost var str := "Hello world\nGoodbye\n";
      var bytes: seq<bv8> := [
        // "Hello world\n"
        0x48, 0x65, 0x6c, 0x6c, 0x6f, 0x20, 0x77, 0x6f, 0x72, 0x6c, 0x64, 0x0a,
        // "Goodbye\n"
        0x47, 0x6f, 0x6f, 0x64, 0x62, 0x79, 0x65, 0x0a
      ];
      assert forall i | 0 <= i < |bytes| :: bytes[i] as int == str[i] as int;

      // Write directly into the output directory
      {
        var res := FileIO.WriteBytesToFile(outputDir + "/output_plain", bytes);
        expect res.Success?, "unexpected failure writing to output_plain: " + res.error;
      }
      // Ensure that nonexistent parent directories are created as necessary
      {
        var res := FileIO.WriteBytesToFile(outputDir + "/foo/bar/output_nested", bytes);
        expect res.Success?, "unexpected failure writing to output_nested: " + res.error;
      }
      // Ensure that file paths with .. are handled correctly
      {
        var res := FileIO.WriteBytesToFile(outputDir + "/foo/bar/../output_up", bytes);
        expect res.Success?, "unexpected failure writing to output_up: " + res.error;
      }
    }

    // Failure path: attempting to write to a blank file path should never work.
    {
      var res := FileIO.WriteBytesToFile("", []);
      expect res.Failure?, "unexpected success";
      expect expectedErrorPrefix <= res.error, "unexpected error message: " + res.error;
    }
  }
}
```
