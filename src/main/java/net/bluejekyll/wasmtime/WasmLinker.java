package net.bluejekyll.wasmtime;

public class WasmLinker extends AbstractOpaquePtr {
    WasmLinker(long ptr) {
        super(ptr, WasmLinker::freeLinker);
    }

    private static native void freeLinker(long ptr);

    private static native void defineFunc(long ptr, String module, String name, long func_ptr);

    private static native long instantiateNtv(long linker_ptr, long module_ptr) throws WasmtimeException;

    /**
     * @param module name of the module in which this function should be defined
     *               (like a class)
     * @param name   for the function to use
     */
    public void defineFunction(String module, String name, WasmFunction function) throws WasmtimeException {
        WasmLinker.defineFunc(this.getPtr(), module, name, function.getPtr());
    }

    public WasmInstance instantiate(WasmModule module) throws WasmtimeException {
        return new WasmInstance(WasmLinker.instantiateNtv(this.getPtr(), module.getPtr()));
    }
}
