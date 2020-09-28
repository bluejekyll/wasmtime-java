package net.bluejekyll.wasmtime;

import jnr.ffi.Pointer;
import jnr.ffi.Runtime;
import jnr.ffi.byref.PointerByReference;

public class WasmEngine extends AbstractOpaquePtr implements AutoCloseable  {
    WasmEngine(long ptr) {
        super(ptr);
    }

    private static native void freeEngine(long ptr);
    private static native long newStoreNtv(long engine_ptr);
    private static native long newModuleNtv(long engine_ptr, byte[] wasm_bytes) throws WasmtimeException;

    public WasmStore newStore() {
        return new WasmStore(this.newStoreNtv(super.getPtr()));
    }

    public WasmModule newModule(byte[] wasm_bytes) throws WasmtimeException {
        return new WasmModule(newModuleNtv(super.getPtr(), wasm_bytes));
    }

    @Override
    public void close() {
        this.freeEngine(super.getPtr());
    }
}
