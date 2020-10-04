package net.bluejekyll.wasmtime;

public class WasmInstance extends AbstractOpaquePtr {
    WasmInstance(long ptr) {
        super(ptr, WasmInstance::freeInstance);
    }

    private static native void freeInstance(long ptr);
}
