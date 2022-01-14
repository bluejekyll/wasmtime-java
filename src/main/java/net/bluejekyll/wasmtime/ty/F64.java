package net.bluejekyll.wasmtime.ty;

public class F64 implements WasmType {
    public final double field;

    public F64(double field) {
        this.field = field;
    }

    @Override
    public Double getField() {
        return this.field;
    }

    public double doubleValue() {
        return this.field;
    }
}
