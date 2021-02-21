fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/rango.cpp")
        .flag_if_supported("-std=c++14")
        .compile("rango-demo");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/rango.cpp");
    println!("cargo:rerun-if-changed=include/rango.h");
}
