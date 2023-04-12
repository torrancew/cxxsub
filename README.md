# cxxsub

This repo contains examples of emulating C++ subclasses via cxx and Rust

## Structure

The repo contains the following:
* `build.rs` -- build script to handle codegen for the various FFI interactions
* `cpp` -- C++ library and shim code
* `src` -- Rust bridge and main driver
