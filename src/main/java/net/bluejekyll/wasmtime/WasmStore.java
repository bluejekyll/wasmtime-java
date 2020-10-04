package net.bluejekyll.wasmtime;

import java.lang.reflect.Method;

public class WasmStore extends AbstractOpaquePtr {
    WasmStore(long ptr) {
        super(ptr, WasmStore::freeStore);
    }

    private static native void freeStore(long ptr);
    private static native long newLinker(long store_ptr);

    
}
