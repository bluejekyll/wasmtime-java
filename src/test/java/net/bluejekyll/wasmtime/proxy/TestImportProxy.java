package net.bluejekyll.wasmtime.proxy;

@WasmModule
public interface TestImportProxy extends WasmImportable {
    @WasmImport(name = "add_i32")
    int addInteger(int a, int b);

    @WasmImport(name = "add_f32")
    float addFloats(float a, float b);
}
