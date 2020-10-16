# Java bindings to the Wasmtime runtime engine

WASM, web-assembly, is a low-level stack execution engine. This library gives a JNI based binding to the Wasmtime WASM runtime.

For more information, please see [github.com/bytecodealliance/wasmtime](https://github.com/bytecodealliance/wasmtime).

This is very early stages. All interfaces are subject to change. Supported platforms, x86_64 for Linux and macOS. Windows should also be supported, but currently not due to lack of resources.

## Building

Use the provided Makefile.

Ensure tools are installed:

```shell
> make init
```

Run tests:

```shell
> make test
```

## Structure

The Java is meant to be as minimal as possible. All Wasmtime object references are stored in Java objects as opaque pointers (longs). The safety in this area has not yet been proven. Thread safety in particular is non-existent. The WasmEngine should be safe to share across threads, though we should most likely introduce a custom clone method for this purpose.

### Adding new native methods

The Java compiler will automatically output the headers for the JNI bindings based on the native methods defined in the various classes. While the headers generated by the Java compiler aren't directly used during the Rust JNI compilation, they are useful for seeing the C signature that the Rust needs to export. The files can be found in `target/generated-sources/*.h`.

## Debugging

The tests should all run regardless of platform. Windows hasn't been fully tested due to lack of resources. If tests fail to run, there are a few different environments at play which will make discovering which component failed and why difficult. At the moment, all output from Java is captured in `txt` files at in the `target/surfire-reports/{CLASS_NAME}-output.txt`. All output from the JNI bindings is currently captured in `target/wasm-logs/java_{DATE}.log`, this may be combined into the same place in the future. In the `pom.xml` Maven project file, the `surfire` test configuration has `RUST_LOG` set to `debug` by default. This can be set to any other value to increase or decrease logging output.

## libc like support with WASI

[WASI](https://wasi.dev/) is supported for things like printing to stdout. This is supplied during linking in the Java bindings. It is not required, in Rust this can be targeted with `cargo build --target wasm32-wasi`, that target must be installed with `rustup` before hand.