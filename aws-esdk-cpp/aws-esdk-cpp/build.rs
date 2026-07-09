fn main() {
    // Generate the cxx bridge code and compile it
    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++11")
        .compile("cxxbridge-aws-mpl-cxx");

    // Tell cargo to link the C++ standard library
    println!("cargo:rustc-link-lib=c++");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
