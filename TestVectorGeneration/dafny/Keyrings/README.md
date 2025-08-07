Demo for setting up test vector generation for raw AES keyrings.

Here, "test vectors for raw AES keyrings" means partitioning the input space of the CreateRawAesKeyring operation,
selecting representative value(s) for each input member,
enumerating all combinations of input members,
pruning redundant configurations,
then writing these input configurations to JSON.

In addition, this writes a separate JSON file
containing the expected output of a test CreateRawAesKeyringSmokeTest operation
that wraps the CreateRawAesKeyring operation
and translates successes/failures to the strings expected by the JSON.

Generate JSON with

```
make transpile_python
make test_python
```

or any language other than Python.