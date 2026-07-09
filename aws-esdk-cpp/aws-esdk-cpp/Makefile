# Simple Makefile - all C++ bridge code is now compiled into the Rust static library

main: main.cpp src/lib.rs target/debug/libaws_esdk_cpp.a
	g++ main.cpp target/cxxbridge/aws-esdk-cpp/src/lib.rs.cc \
		$$(find target/debug/build -path "*/cxx-*/out/libcxxbridge1.a" | head -n1) \
		-Ltarget/debug -laws_esdk_cpp \
		-Itarget/cxxbridge \
		-lpthread -ldl \
		-std=c++11 \
		-o main


main_release: main.cpp src/lib.rs target/release/libaws_esdk_cpp.a
	g++ main.cpp target/cxxbridge/aws-esdk-cpp/src/lib.rs.cc \
		$$(find target/release/build -path "*/cxx-*/out/libcxxbridge1.a" | head -n1) \
		-Ltarget/release -laws_esdk_cpp \
		-Itarget/cxxbridge \
		-lpthread -ldl \
		-std=c++11 \
		-o main_release

clean:
	rm -f main

.PHONY: all clean duvet test clippy

# Idiomatic-facade integration tests. Requires C++17 (aws_esdk.hpp uses
# std::string_view). Run ./test_esdk with ESDK_TEST_* env vars set (it skips
# cleanly if they are unset).
test_esdk: test_esdk.cpp include/aws/esdk/EncryptionSDK.h src/lib.rs target/debug/libaws_esdk_cpp.a
	g++ test_esdk.cpp target/cxxbridge/aws-esdk-cpp/src/lib.rs.cc \
		$$(find target/debug/build -path "*/cxx-*/out/libcxxbridge1.a" | head -n1) \
		-Ltarget/debug -laws_esdk_cpp \
		-Itarget/cxxbridge -Iinclude \
		-lpthread -ldl \
		-std=c++17 \
		-o test_esdk

clippy:
	cargo clippy --tests --examples

test:
	cargo build
	cargo build --release
	make
	./main
	make main_release
	./main_release

duvet:
	@test -e spec || ln -s ../../spec spec
	rm -rf .duvet/reports .duvet/requirements
	duvet report
