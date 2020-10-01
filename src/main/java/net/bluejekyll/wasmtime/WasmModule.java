package net.bluejekyll.wasmtime;

public class WasmModule extends AbstractOpaquePtr {
    WasmModule(long ptr) {
        super(ptr);
    }

    private static native void freeModule(long ptr);
    
    @Override
    public void free(long ptr) {
        WasmModule.freeModule(ptr);
    }
}
