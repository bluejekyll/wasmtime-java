package net.bluejekyll.wasmtime.ty;

public class F32 implements WasmType {
    public final float field;

    public F32(float field) {
        this.field = field;
    }

    @Override
    public Float getField() {
        return this.field;
    }

    public float floatValue() {
        return this.field;
    }
}
