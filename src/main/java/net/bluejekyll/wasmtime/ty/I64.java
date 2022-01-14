package net.bluejekyll.wasmtime.ty;

public class I64 implements WasmType {
    public final long field;

    public I64(long field) {
        this.field = field;
    }

    @Override
    public Long getField() {
        return this.field;
    }

    public long longValue() {
        return field;
    }
}
