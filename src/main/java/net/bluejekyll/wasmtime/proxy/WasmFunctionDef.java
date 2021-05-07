package net.bluejekyll.wasmtime.proxy;

import net.bluejekyll.wasmtime.WasmFunction;

public class WasmFunctionDef {
    private final String moduleName;
    private final String functionName;
    private final WasmFunction function;

    public WasmFunctionDef(String moduleName, String functionName, WasmFunction function) {
        this.moduleName = moduleName;
        this.functionName = functionName;
        this.function = function;
    }

    public String getModuleName() {
        return this.moduleName;
    }

    public String getFunctionName() {
        return this.functionName;
    }

    public WasmFunction getFunction() {
        return this.function;
    }
}
