# Testing the libraries

The code in these libraries is tested both with deductive verification, using dafny,
and with traditional runtime testing of compilations by dafny. 
The latter is important as a check that dafny compiled the (verified) dafny code correctly.
Also, some functionality (e.g. FileIO) is partially implemented using
custom code in the target programming language, which is not verified by dafny.
All files are also checked that they conform to standard dafny formatting,
using `dafny format`.

The tests are run using `lit`. The configuration of the tests is controlled by
(a) the lit.site.cfg file and (b) the yml files in .github/workflows.

The tests are run on each push to a pull_request and are required to pass to enable merging.
They are also run on a nightly schedule.

To run the files manually: run `lit .` in this directory (`libraries`, at the top
of the clone of the repo). You can also manually run individual files or directories,
but `lit` must be invoked in the `libraries` directory. You can also trigger the workflow
using `workflow_dispatch`. To do a manual check of formatting, use `dafny format --check <file or folder>`.

The tests themselves are specially formatted lines (those beginning with `// RUN:`) in
each .dfy file. The tests use dafny's new command-line syntax, using the lit
macros `%verify` and `%run` and the like.

The versions of dafny being run are set in the `tests.yml` file. The versions tested
must be recent enough to support the new CLI syntax, so 3.11 and later. Also, if 
verification logging is desired, the version must be at least that of 3/18/2023 when
the logging parameters were implemented in the new CLI.

Testing can be run on older versions if (a) the lit macros like `%verify` are redefined using
old CLI syntax and (b) the runtime testing tests are rewritten to not use `dafny run`.
