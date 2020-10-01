package net.bluejekyll.wasmtime.types;

import javax.annotation.Nullable;

/**
 * Long -> i64
 */
public enum WasmType {
    i32,
    i64,
    f32,
    f64,
    byteArray;

    /**
     * Null if and only if the Class is Void or void
     * @param ty
     * @return
     * @throws UnsupportedOperationException
     */
    @Nullable
    public static WasmType fromClass(Class<?> ty) throws UnsupportedOperationException {
        if (ty.isAssignableFrom(Long.class)) {
            return WasmType.i64;
        } else if (ty.isAssignableFrom(Integer.class)) {
            return WasmType.i32;
        } else if (ty.isAssignableFrom(Double.class)) {
            return WasmType.f64;
        } else if (ty.isAssignableFrom(Float.class)) {
            return WasmType.f32;
        } else if (ty.isAssignableFrom(void.class) || ty.isAssignableFrom(Void.class)) {
            return null;
        }

        throw new UnsupportedOperationException("Unsupported class for WASM: " + ty);
    }
}

