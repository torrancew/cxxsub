fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=src/ffi.rs");
    println!("cargo:rerun-if-changed=cpp/lib.h");
    println!("cargo:rerun-if-changed=cpp/lib.cpp");

    cxx_build::bridge("src/ffi.rs")
        .file("cpp/bridge.cpp")
        .flag("-std=c++20")
        .flag("-Wno-cpp")
        .compile("cpp-lib");

    Ok(())
}
