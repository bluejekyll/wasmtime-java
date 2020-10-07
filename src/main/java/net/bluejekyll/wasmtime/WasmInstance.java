package net.bluejekyll.wasmtime;

import java.util.Optional;

public class WasmInstance extends AbstractOpaquePtr {
    WasmInstance(long ptr) {
        super(ptr, WasmInstance::freeInstance);
    }

    private static native void freeInstance(long ptr);
    private static native long getFunctionNtv(long ptr, String name);

    public Optional<WasmFunction> getFunction(String name) {
        long func = WasmInstance.getFunctionNtv(this.getPtr(), name);
        if (func == 0) {
            return Optional.empty();
        } else {
            return Optional.of(new WasmFunction(func));
        }
    }
}
