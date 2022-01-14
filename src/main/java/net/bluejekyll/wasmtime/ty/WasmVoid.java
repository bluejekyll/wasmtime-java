package net.bluejekyll.wasmtime.ty;

public class WasmVoid implements WasmType {
    public final java.lang.Void field = null;

    @Override
    public java.lang.Void getField() {
        return this.field;
    }
}
