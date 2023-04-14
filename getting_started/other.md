# Broc installation guide for other systems

1. [Install Rust](https://rustup.rs/)

1. [Build Broc from source](../BUILDING_FROM_SOURCE.md)

1. Run examples:

    ```sh
    cargo run examples/platform-switching/brocLovesRust.broc

    # This requires installing the Zig compiler, too.
    cargo run examples/platform-switching/brocLovesZig.broc

    # This requires installing the `clang` C compiler, too.
    cargo run examples/platform-switching/brocLovesC.broc
    ```
