package net.bluejekyll.wasmtime;

import jnr.ffi.Pointer;

public class WasmModuleT implements AutoCloseable {
    private final Wasmtime.WasmtimeFFI ffi;
    private final Pointer ptr;

    WasmModuleT(Wasmtime.WasmtimeFFI ffi, Pointer wasm_module_t) {
        this.ffi = ffi;
        this.ptr = wasm_module_t;
    }

    @Override
    public void close() {
        ffi.wasm_module_delete(this.ptr);
    }
}
