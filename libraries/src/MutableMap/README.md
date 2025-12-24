# Mutable Map

The `MutableMap` module introduces mutable maps for Dafny. At the moment, the API is intentionally limited in scope, and only supports compilation to Java.  For the future, we plan to extend both the functionality and the range of supported languages.

To use `MutableMap` in your code, you must:

1. `include` and `import` the `MutableMap` module as you would any other library module
2. incorporate the corresponding language-specific implementation file (that is, currently, `MutableMap.java`) when building or running your program

For example, to run the `MutableMapExamples.dfy` file in the `examples/MutableMap` directory that depends on the `MutableMap` module, run the following.

```bash
# Java
$ dafny run MutableMapExamples.dfy --target:java --input ../../src/MutableMap/MutableMap.java
```

(If you aren't using `dafny run` to run your program,
then you should instead integrate the appropriate language-specific implementation file in your build system.)

The `examples/MutableMap` directory contains more detailed examples of how to use the `MutableMap` module.