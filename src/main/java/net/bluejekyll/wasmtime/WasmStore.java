package net.bluejekyll.wasmtime;

import java.lang.reflect.Method;

public class WasmStore extends AbstractOpaquePtr {
    WasmStore(long ptr) {
        super(ptr);
    }

    private static native void freeStore(long ptr);
    //private static native long newFuncNtv(long storePtr);
    
    @Override
    public void free(long ptr) {
        WasmStore.freeStore(ptr);
    }
}
