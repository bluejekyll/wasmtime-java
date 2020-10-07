package net.bluejekyll.wasmtime;

import java.lang.reflect.Method;

public class WasmStore extends AbstractOpaquePtr {
    WasmStore(long ptr) {
        super(ptr, WasmStore::freeStore);
    }

    private static native void freeStore(long ptr);

    private static native long newLinkerNtv(long store_ptr);

    public WasmLinker newLinker() {
        return new WasmLinker(WasmStore.newLinkerNtv(this.getPtr()));
    }
}
