# Copyright Amazon.com Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

cmake_minimum_required(VERSION 3.10)
project(esdk_benchmark C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

# ---- System dependencies ----

find_package(PkgConfig REQUIRED)
pkg_check_modules(YAML REQUIRED yaml-0.1)
pkg_check_modules(JSON REQUIRED json-c)
find_package(Threads REQUIRED)

# ---- AWS dependencies (must be pre-installed via build.sh) ----

find_package(aws-c-common REQUIRED)
find_package(aws-encryption-sdk REQUIRED)

# ---- Benchmark executable ----

add_executable(esdk_benchmark
    src/main.c
    src/config.c
    src/results.c
    src/benchmark.c
    src/tests.c
)

target_include_directories(esdk_benchmark PRIVATE
    ${YAML_INCLUDE_DIRS}
    ${JSON_INCLUDE_DIRS}
)

target_link_directories(esdk_benchmark PRIVATE
    ${YAML_LIBRARY_DIRS}
    ${JSON_LIBRARY_DIRS}
)

target_link_libraries(esdk_benchmark PRIVATE
    AWS::aws-encryption-sdk
    AWS::aws-c-common
    ${YAML_LIBRARIES}
    ${JSON_LIBRARIES}
    Threads::Threads
)

target_compile_options(esdk_benchmark PRIVATE -Wall -Wextra -O2)
