# Simple Makefile - all C++ bridge code is compiled into the Rust static library.

# Idiomatic example. Requires C++17 (the idiomatic header uses std::string_view).
# Needs live AWS resources / ESDK_TEST_* env vars to run; builds without them.
example: examples/basic_hierarchical_keyring_example.cpp include/aws/esdk/EncryptionSDK.h src/lib.rs target/debug/libaws_esdk_cpp.a
	g++ examples/basic_hierarchical_keyring_example.cpp target/cxxbridge/aws-esdk-cpp/src/lib.rs.cc \
		$$(find target/debug/build -path "*/cxx-*/out/libcxxbridge1.a" | head -n1) \
		-Ltarget/debug -laws_esdk_cpp \
		-Itarget/cxxbridge -Iinclude \
		-lpthread -ldl \
		-std=c++17 \
		-o basic_hierarchical_keyring_example

# Idiomatic-facade integration tests. Requires C++17 (the idiomatic header uses
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

clean:
	rm -f basic_hierarchical_keyring_example test_esdk

clippy:
	cargo clippy --tests --examples

test:
	cargo build
	cargo build --release
	make example
	make test_esdk

duvet:
	@test -e spec || ln -s ../../spec spec
	rm -rf .duvet/reports .duvet/requirements
	duvet report

.PHONY: all clean duvet test clippy
