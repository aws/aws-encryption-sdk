#!/bin/bash -eu

# Check if exactly one argument is provided
if [ "$#" -ne 1 ]; then
    echo 1>&2 "USAGE: test_published.sh N.N.N"
    exit 1
fi

# Go to the directory of this script
cd $( dirname ${BASH_SOURCE[0]} )

# Check if the provided argument matches the version pattern
REGEX_VERSION='^\d+\.\d+\.\d+$'
echo "$1" | egrep -q $REGEX_VERSION
if [ $? -ne 0 ]; then
   echo 1>&2 "Version \"$1\" must be N.N.N"
   exit 1
fi

VERSION=$1

# Update examples in test_examples directory
rm -rf test_examples/src
cp -r examples test_examples/src/
cd test_examples

# Add aws-esdk
cargo add aws-esdk

# Check if the added version matches the provided version
MATCH=$(fgrep "aws-esdk = \"$VERSION\"" Cargo.toml | wc -l)
if [ $MATCH -eq "0" ]; then
    echo Version $VERSION of aws-esdk not the most recent
    egrep '^aws-esdk' Cargo.toml
    exit 1
fi

# Run the cargo project
cargo test --release
