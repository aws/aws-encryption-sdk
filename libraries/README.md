# libraries

Libraries useful for Dafny programs

## Status

**This repository is now deprecated:** nearly all of this code is now available directly from the Dafny distribution using the flag `--standard-libraries`. See https://github.com/dafny-lang/dafny/tree/master/Source/DafnyStandardLibraries for details.

At the moment, we're just collecting generally useful Dafny code.

Once we have some amount of code which is used successfully by several projects, we might restructure this repo, informed by the concrete use cases.

So, please do use this library, give feedback, and contribute code, but also expect breaking changes.

## Contributions

Any contributions of generally useful code are welcome, just open a pull request!  Please follow the [library style guidelines](STYLE.md).  If the way to use your new code is not obvious, please add some examples in the `examples` directory to illustrate how to use the code.  We use the [LLVM integrated tester (lit)](https://llvm.org/docs/CommandGuide/lit.html) to test the library files and ensure they all verify correctly.  Please see Dafny's documentation on [installation](https://github.com/dafny-lang/dafny/wiki/INSTALL) and [testing](https://github.com/dafny-lang/dafny/wiki/Running-Dafny's-test-suite) for more details.

## Acknowledgements

Much of this code came from or was inspired by code from the following projects:

* [Ironclad Apps](https://github.com/microsoft/Ironclad/tree/main/ironclad-apps)
* [IronFleet](https://github.com/microsoft/Ironclad/tree/main/ironfleet)
* [Vale](https://github.com/project-everest/vale/tree/legacy_dafny)
* [Verified BetrFS](https://github.com/vmware-labs/verified-betrfs)
* [Verifying OpenTitan](https://github.com/secure-foundations/veri-titan)

