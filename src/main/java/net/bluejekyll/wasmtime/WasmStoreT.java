package net.bluejekyll.wasmtime;

import jnr.ffi.Pointer;

public class WasmStoreT implements AutoCloseable {
    private final Wasmtime.WasmtimeFFI ffi;
    private final Pointer ptr;

    WasmStoreT(Wasmtime.WasmtimeFFI ffi, Pointer wasm_engine_t) {
        this.ffi = ffi;
        this.ptr = wasm_engine_t;
    }

    @Override
    public void close() {
        ffi.wasm_store_delete(this.ptr);
    }
}
