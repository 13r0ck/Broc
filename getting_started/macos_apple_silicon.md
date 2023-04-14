# Broc installation guide for Apple silicon systems

## How to install Broc

:warning: We do not yet officially support **MacOS 13**. But, as long as you are **not** using a zig or wasm platform most things should work fine.

In order to develop in Broc, you need to install the Broc CLI,
which includes the Broc compiler and some helpful utilities.

1. Download the latest nightly from the assets [here](https://github.com/roc-lang/broc/releases).

1. To prevent "broc can't be opened because Apple can't check it...":

    ```sh
    xattr -d com.apple.quarantine broc_nightly-macos_apple_silicon-<VERSION>.tar.gz
    ```

1. Untar the archive:

    ```sh
    tar xf broc_nightly-macos_apple_silicon-<VERSION>.tar.gz
    cd broc_night<TAB TO AUTOCOMPLETE>
    ```

1. Install llvm 13:

    ```sh
    brew install llvm@13
    ```

1. To be able to run the `broc` command anywhere on your system; add the line below to your shell startup script (.profile, .zshrc, ...):
    ```sh
    export PATH=$PATH:~/path/to/broc_nightly-macos_apple_silicon-<VERSION>
    ```

1. Check everything worked by executing `broc version`

## How to install Broc platform dependencies

This step is not necessary if you only want to use the [basic-cli platform](https://github.com/roc-lang/basic-cli), like in the tutorial.
But, if you want to compile Broc apps with other platforms (either in [`examples/`](https://github.com/roc-lang/broc/tree/main/examples) or in your own projects),
you'll need to install one or more of these platform languages too.

1. Install the Rust compiler, for apps with Rust-based platforms:

    ```sh
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

1. If you'd like to use Zig-based platforms: download [zig 0.9.1](https://ziglang.org/download/0.9.1/zig-macos-aarch64-0.9.1.tar.xz), extract the archive and add `export PATH=$PATH:~/path/to/zig` to your shell startup script (.profile, .zshrc, …). Note: zig 0.9.1 is not available on homebrew.

1. Run examples:

    ```sh
    # Note: If you installed rust in this terminal session, you'll need to open a new one first!
    ./broc examples/platform-switching/brocLovesRust.broc

    ./broc examples/platform-switching/brocLovesZig.broc

    ./broc examples/platform-switching/brocLovesC.broc
    ```
