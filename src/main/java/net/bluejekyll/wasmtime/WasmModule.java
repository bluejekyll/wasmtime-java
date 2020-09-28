package net.bluejekyll.wasmtime;

import jnr.ffi.Pointer;

public class WasmModule extends AbstractOpaquePtr implements AutoCloseable {
    WasmModule(long ptr) {
        super(ptr);
    }

    private static native void freeModule(long ptr);
    
    @Override
    public void close() {
        freeModule(super.getPtr());
    }
}
