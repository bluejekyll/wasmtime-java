package net.bluejekyll.wasmtime;

import jnr.ffi.Pointer;

public class WasmStore extends AbstractOpaquePtr implements AutoCloseable {
    WasmStore(long ptr) {
        super(ptr);
    }

    private static native void freeStore(long ptr);

    @Override
    public void close() {
        WasmStore.freeStore(super.getPtr());
    }
}
