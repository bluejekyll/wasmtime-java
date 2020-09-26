package net.bluejekyll.wasmtime;

import jnr.ffi.Pointer;
import jnr.ffi.Runtime;
import jnr.ffi.byref.PointerByReference;

public class WasmEngineT implements AutoCloseable  {
    private final Wasmtime.WasmtimeFFI ffi;
    private final Runtime ffiRuntime;
    private final Pointer ptr;

    WasmEngineT(Wasmtime.WasmtimeFFI ffi, Runtime ffiRuntime, Pointer wasm_engine_t) {
        this.ffi = ffi;
        this.ffiRuntime = ffiRuntime;
        this.ptr = wasm_engine_t;
    }

    public WasmStoreT newStore() {
        return new WasmStoreT(this.ffi, this.ffi.wasm_store_new(this.ptr));
    }

    public WasmModuleT newModule(byte[] wasm_bytes) throws WasmtimeException {
        Wasmtime.WasmByteVecT wasm_bytes_t = new Wasmtime.WasmByteVecT(this.ffiRuntime);
        PointerByReference wasm_module_t = new PointerByReference();

        wasm_bytes_t.setData(wasm_bytes);

        Pointer error_t = this.ffi.wasmtime_module_new(this.ptr, wasm_bytes_t, wasm_module_t);
        new WasmtimeErrorT(this.ffi, error_t).checkThrow();

        return new WasmModuleT(this.ffi, wasm_module_t.getValue());
    }


    @Override
    public void close() {
        ffi.wasm_engine_delete(this.ptr);
    }
}
