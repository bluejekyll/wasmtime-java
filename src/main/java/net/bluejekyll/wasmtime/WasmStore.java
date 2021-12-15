package net.bluejekyll.wasmtime;

public class WasmStore extends AbstractOpaquePtr {
    // Store is !Send and !Sync in Rust, we will enforce that with a ThreadLocal

    WasmStore(long ptr) {
        super(ptr, WasmStore::freeStore);
    }

    private static native void freeStore(long ptr);
}
