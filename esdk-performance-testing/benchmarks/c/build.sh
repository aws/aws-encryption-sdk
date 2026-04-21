#!/bin/bash
# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0
#
# Builds the AWS C SDK dependencies and then the benchmark.
# Usage: ./build.sh [--clean]

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DEPS_DIR="$SCRIPT_DIR/deps"
INSTALL_DIR="$SCRIPT_DIR/deps/install"
BUILD_DIR="$SCRIPT_DIR/build"

if [ "$1" = "--clean" ]; then
    echo "Cleaning build artifacts..."
    rm -rf "$DEPS_DIR" "$BUILD_DIR"
    echo "Done."
    exit 0
fi

mkdir -p "$DEPS_DIR" "$INSTALL_DIR" "$BUILD_DIR"

CMAKE_COMMON_ARGS="-DCMAKE_BUILD_TYPE=Release -DBUILD_SHARED_LIBS=OFF -DCMAKE_INSTALL_PREFIX=$INSTALL_DIR -DCMAKE_PREFIX_PATH=$INSTALL_DIR"

build_dep() {
    local name=$1
    local repo=$2
    local branch=${3:-main}

    echo ""
    echo "=== Building $name ==="

    if [ ! -d "$DEPS_DIR/$name" ]; then
        git clone --depth 1 --branch "$branch" "$repo" "$DEPS_DIR/$name"
    fi

    mkdir -p "$DEPS_DIR/$name/build"
    cmake -S "$DEPS_DIR/$name" -B "$DEPS_DIR/$name/build" $CMAKE_COMMON_ARGS
    cmake --build "$DEPS_DIR/$name/build" --target install -j "$(nproc 2>/dev/null || sysctl -n hw.logicalcpu)"
}

# Build dependencies in order
build_dep aws-c-common   https://github.com/awslabs/aws-c-common.git
build_dep aws-c-cal      https://github.com/awslabs/aws-c-cal.git
build_dep aws-c-io       https://github.com/awslabs/aws-c-io.git
build_dep aws-c-compression https://github.com/awslabs/aws-c-compression.git
build_dep aws-c-http     https://github.com/awslabs/aws-c-http.git
build_dep aws-c-sdkutils https://github.com/awslabs/aws-c-sdkutils.git
build_dep aws-c-auth     https://github.com/awslabs/aws-c-auth.git
build_dep aws-encryption-sdk-c https://github.com/aws/aws-encryption-sdk-c.git master

# Build the benchmark
echo ""
echo "=== Building benchmark ==="
cmake -S "$SCRIPT_DIR" -B "$BUILD_DIR" \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_PREFIX_PATH="$INSTALL_DIR;$INSTALL_DIR/lib/aws-encryption-sdk"
cmake --build "$BUILD_DIR" -j "$(nproc 2>/dev/null || sysctl -n hw.logicalcpu)"

echo ""
echo "Build complete: $BUILD_DIR/esdk_benchmark"
