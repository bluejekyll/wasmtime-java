# Java bindings to the Wasmtime runtime engine

WASM, web-assembly, is a low-level stack execution engine. This library gives a JNI based binding to the Wasmtime WASM runtime.

For more information, please see [github.com/bytecodealliance/wasmtime](https://github.com/bytecodealliance/wasmtime).

This is very early stages. All interfaces are subject to change.

## Structure

The Java is meant to be as minimal as possible. All Wasmtime object references are stored in Java objects as opaque pointers (longs). The safety in this area has not yet been proven. Thread safety in particular is non-existent. The WasmEngine should be safe to share across threads, though we should most likely introduce a custom clone method for this purpose.