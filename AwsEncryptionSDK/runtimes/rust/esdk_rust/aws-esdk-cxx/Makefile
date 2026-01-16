# Simple Makefile - all C++ bridge code is now compiled into the Rust static library

main: main.cpp src/lib.rs
	g++ main.cpp ../target/cxxbridge/aws-esdk-cxx/src/lib.rs.cc \
		$$(find ../target/debug/build -path "*/cxx-*/out/libcxxbridge1.a" | head -n1) \
		-L../target/debug -laws_esdk_cxx \
		-I../target/cxxbridge \
		-lpthread -ldl \
		-std=c++11 \
		-o main


main_release:
	g++ main.cpp ../target/cxxbridge/aws-esdk-cxx/src/lib.rs.cc \
		$$(find ../target/release/build -path "*/cxx-*/out/libcxxbridge1.a" | head -n1) \
		-L../target/release -laws_esdk_cxx \
		-I../target/cxxbridge \
		-lpthread -ldl \
		-std=c++11 \
		-o main




clean:
	rm -f main

.PHONY: all clean
