---
inclusion: auto
---

# Annotation Conventions

When asked to "convert to implication":
1. Remove the test function from the test file (and its spec citation annotations).
2. Find the corresponding annotation in the source code (e.g. `footer.rs`, `encrypt.rs`) and add `//= type=implication` to it.
3. Do NOT place implication annotations in test files — they belong on the source code.
