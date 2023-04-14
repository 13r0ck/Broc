#!/bin/sh

# https://vaneyckt.io/posts/safer_bash_scripts_with_set_euxo_pipefail/
set -euxo pipefail

# don't forget to validate that $JAVA_HOME is defined, the following would not work without it!
# set it either globally or here
# export JAVA_HOME=/your/java/installed/dir
# in nixos, to set it globally, i needed to say `programs.java.enable = true;` in `/etc/nixos/configuration.nix`


# if broc is in your path, you could
# broc build impl.broc --no-link
# else, assuming in broc repo and that you ran `cargo run --release`
../../target/release/broc build impl.broc --no-link


# make jvm look here to see libinterop.so
export LD_LIBRARY_PATH=$(pwd):$LD_LIBRARY_PATH

# needs jdk10 +
# "-h ." is for placing the jni.h header in the cwd.
# the "javaSource" directory may seem redundant (why not just a plain java file),
# but this is the way of java packaging
# we could go without it with an "implicit" package, but that would ache later on,
# especially with other JVM langs
javac -h . javaSource/Demo.java


clang \
    -g -Wall \
    -fPIC \
    -I"$JAVA_HOME/include" \
    # -I"$JAVA_HOME/include/darwin" # for macos
    -I"$JAVA_HOME/include/linux" \
    # -shared -o libinterop.dylib \ # for macos
    -shared -o libinterop.so \
    brocdemo.o bridge.c


# then run
java javaSource.Demo
