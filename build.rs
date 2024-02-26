fn main() {
    cxx_build::bridge("src/lib.rs")
        .file("src/fibonacci.cpp")
        .flag_if_supported("-std=c++11")
        .compile("rust_cpp_fibonacci");

    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/fibonacci.cpp");
}