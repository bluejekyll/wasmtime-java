package net.bluejekyll.wasmtime.ty;

public class I32 implements WasmType {
    public final int field;

    public I32(int field) {
        this.field = field;
    }

    @Override
    public Integer getField() {
        return this.field;
    }

    public int intValue() {
        return this.field;
    }
}
