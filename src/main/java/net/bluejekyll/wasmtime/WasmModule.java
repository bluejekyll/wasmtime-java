package net.bluejekyll.wasmtime;

public class WasmModule extends AbstractOpaquePtr {
    WasmModule(long ptr) {
        super(ptr, WasmModule::freeModule);
    }

    private static native void freeModule(long ptr);
}
