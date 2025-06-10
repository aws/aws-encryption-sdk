#!/bin/bash -eu

rm -rf test_examples/src
cp -r examples test_examples/src/
cd test_examples
cargo upgrade
rm -rf src
