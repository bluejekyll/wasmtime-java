package net.bluejekyll.wasmtime.ty;

import java.math.BigInteger;

public class WasmTypeUtil {
    public static I32 i32(int val) {
        return new I32(val);
    };

    public static I64 i64(long val) {
        return new I64(val);
    };

    public static F32 f32(float val) {
        return new F32(val);
    };

    public static F64 f64(double val) {
        return new F64(val);
    };

    public static V128 v128(BigInteger val) {
        return new V128(val);
    };

    // FuncRef,
    // ExternRef;
}
