package net.bluejekyll.wasmtime.ty;

import java.math.BigInteger;

public class V128 implements WasmType {
    // TODO: maybe this should be an array of bytes...
    public final BigInteger field;

    public V128(BigInteger field) {
        this.field = field;
    }

    @Override
    public BigInteger getField() {
        return this.field;
    }
}
