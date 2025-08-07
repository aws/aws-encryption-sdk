Demo for setting up test vector generation for SimpleStrings.

This partitions strings into empty, ASCII, and non-ASCII.

This is not particuarly useful on its own; its utility is in 

1. Being reused by other test vector generation modules that want to exhaustively test strings (and get benefits of fuzzed strings later on)
2. Demoing basic test vector generation framework setup for a shape/operation/service.
