
# Guide to the Dafny Core Library

The Dafny programming language and tool set includes
libraries that provide functionality in various domains. This document describes 
the _Dafny Core Library_. Dafny programs are composed of modules; to ensure
verification soundness, Dafny imposes a strict non-circular dependency relationship
among modules. This Core library contains functionality that may be needed
by other libraries, but it itself is standalone.

Our goal is that, eventually, all the supported, delivered libraries will
- be stable --- a minimum of backwards incompatible changes
- be verified --- proved to meet specifications and tested with expected use cases,
with each release of the library and of the Dafny toolset
- be efficient --- efficient in both proofs and executables that use the libraries
- be compilable for every supported target language by default, with exceptions being documented and for exceptional reasons

The Dafny code for the library is written to be understandable with a minimum of inline documentation.
It is open source and available [on github](https://github.com/dafny-lang/libraries). 
This document is meant to give an overall introduction. Also helpful are the annotated examples
in the github repository [here](https://github.com/dafny-lang/libraries/tree/master/examples).

## Content of the Core Library

The sections below describe how to use each of the principal modules of the library:
- [Dafny.Wrappers](Wrappers) -- simple datatypes to support common patterns, such as optional values or the result of operations that can fail
- [Dafny.BoundedInts](BoundedInts) -- definitions of types and constants for bounded integer arithmetic
- [Dafny.FileIO](FileIO/FileIO) -- basic input and output operations
- [Dafny.Math](Math) --- basic mathematical operations and lemmas
- [Dafny.Unicode](Unicode/Unicode) --- tools to manipulate unicode characters and strings
- [Dafny.Collections](Collections/Collections) -- functions and lemmas about sequences, sets and maps
- [Dafny.NonlinearArith](NonlinearArithmetic/NonlinearArithmetic) --  lemmas and functions for non-linear arithmetic
- [Dafny.Relations](Relations) -- predicates and lemmas about properties of functions
- [Dafny.BinaryOperations](BinaryOperations) -- properties of binary operations


## How to use the library

One uses the library on the command-line by including a path to the library as an argument to the `--library` option.

At present, you must have a copy of the library in your file system to use the library. Either
- obtain a release zip file from [the library github release page](https://github.com/dafny-lang/libraries/releases) and unzip it in a folder of your choice, or
- clone the library repository (`git clone https://github.com/dafny-lang/libraries.git`) to a location of your choice

In either case, the relevant directory to point the `--library` option to is named `dafny`, either at the top of the zip file or at `libraries/src/dafny` in the clone, or to the `DafnyCore.dfy` file in that directory. 

Some functionality is implemented using native target language capabilities (e.g. FileIO). In these cases, a compiled program must also include the relevant target
language files that are included in this Dafny Core distribution. See the FileIO examples and documentation
for an instance of this requirement.

The procedure for using library code directly with VSCode is not yet settled. For now make a copy of the library in your system and  'include' the `DafnyCore.dfy` file.

