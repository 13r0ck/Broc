# Broc installation guide for x86_64 Linux systems

## How to install Broc

In order to develop in Broc, you need to install the Broc CLI,
which includes the Broc compiler and some helpful utilities.

1. Download the latest nightly from the assets [here](https://github.com/roc-lang/broc/releases).

1. Untar the archive:

    ```sh
    tar -xf broc_nightly-linux_x86_64-<VERSION>.tar.gz
    cd broc_night<TAB TO AUTOCOMPLETE>
    ```

1. To be able to run the `broc` command anywhere on your system; add the line below to your shell startup script (.profile, .zshrc, ...):
    ```sh
    export PATH=$PATH:~/path/to/broc_nightly-linux_x86_64-<VERSION>
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

1. Install the Zig compiler, for apps with Zig-based platforms:

    ```sh
    wget https://ziglang.org/download/0.9.1/zig-linux-x86_64-0.9.1.tar.xz
    tar -xf zig-linux-x86_64-0.9.1.tar.xz
    sudo ln -s  $(pwd)/zig-linux-x86_64-0.9.1/zig /usr/local/bin/zig
    ```

1. Install a C compiler, for apps with C-based platforms:

    ```sh
    # On a Debian-based distro like Ubuntu
    sudo apt update && sudo apt install build-essential clang
    
    # On an RPM-based distro like Fedora
    sudo dnf check-update && sudo dnf install clang
    ```

    ```sh
    # Note: If you installed Rust in this terminal session, you'll need to open a new one first!
    ./broc examples/platform-switching/brocLovesRust.broc

    ./broc examples/platform-switching/brocLovesZig.broc

    ./broc examples/platform-switching/brocLovesC.broc
    ```
