# CMake integration for the AWS Encryption SDK C++ bindings (aws-esdk-cpp).
#
# Consumers do:
#     add_subdirectory(aws-esdk-cpp)
#     target_link_libraries(my_app PRIVATE AWS::esdk)
#     #include <aws/esdk/EncryptionSDK.h>
#
# This drives `cargo build` to produce the Rust staticlib and the cxx-generated
# C++ sources, then exposes them behind the AWS::esdk target.
cmake_minimum_required(VERSION 3.20)
project(aws_esdk_cpp LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(ESDK_ROOT ${CMAKE_CURRENT_SOURCE_DIR})
set(ESDK_PROFILE debug)
set(ESDK_TARGET_DIR ${ESDK_ROOT}/target)

# Build the Rust staticlib + cxx-generated sources before we reference them.
execute_process(
    COMMAND cargo build
    WORKING_DIRECTORY ${ESDK_ROOT}
    RESULT_VARIABLE _esdk_cargo_result
)
if(NOT _esdk_cargo_result EQUAL 0)
    message(FATAL_ERROR "cargo build failed (exit ${_esdk_cargo_result})")
endif()

set(ESDK_STATICLIB ${ESDK_TARGET_DIR}/${ESDK_PROFILE}/libaws_esdk_cpp.a)
set(ESDK_BRIDGE_CC ${ESDK_TARGET_DIR}/cxxbridge/aws-esdk-cpp/src/lib.rs.cc)

# The cxx runtime archive lives in a hashed build dir; locate it post-build.
file(GLOB_RECURSE _esdk_cxxbridge_rt
     ${ESDK_TARGET_DIR}/${ESDK_PROFILE}/build/cxx-*/out/libcxxbridge1.a)
list(GET _esdk_cxxbridge_rt 0 ESDK_CXXBRIDGE_RT)

# The public target: compiles the generated C++ bridge and links the Rust lib.
add_library(aws_esdk_cpp STATIC ${ESDK_BRIDGE_CC})
target_include_directories(aws_esdk_cpp PUBLIC
    ${ESDK_ROOT}/include
    ${ESDK_TARGET_DIR}/cxxbridge
)
target_link_libraries(aws_esdk_cpp PUBLIC
    ${ESDK_STATICLIB}
    ${ESDK_CXXBRIDGE_RT}
    pthread
    ${CMAKE_DL_LIBS}
)
if(APPLE)
    target_link_libraries(aws_esdk_cpp PUBLIC
        "-framework Security"
        "-framework CoreFoundation"
        "-framework SystemConfiguration"
    )
endif()
add_library(AWS::esdk ALIAS aws_esdk_cpp)

# Build the integration test only when this is the top-level project.
if(CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME)
    add_executable(esdk_integration_test test_esdk.cpp)
    target_link_libraries(esdk_integration_test PRIVATE AWS::esdk)
endif()
