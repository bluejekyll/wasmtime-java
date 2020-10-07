package net.bluejekyll.wasmtime.types;

import javax.annotation.Nullable;

/**
 * Long -> i64
 */
public enum WasmType {
    i32, i64, f32, f64, byteArray;

    /**
     * Null if and only if the Class is Void or void
     * 
     * @param ty
     * @return
     * @throws UnsupportedOperationException
     */
    @Nullable
    public static WasmType fromClass(Class<?> ty) throws UnsupportedOperationException {
        if ((Long.class.isAssignableFrom(ty)) || long.class.isAssignableFrom(ty)) {
            return WasmType.i64;
        } else if (Integer.class.isAssignableFrom(ty) || int.class.isAssignableFrom(ty)) {
            return WasmType.i32;
        } else if (Double.class.isAssignableFrom(ty) || double.class.isAssignableFrom(ty)) {
            return WasmType.f64;
        } else if (Float.class.isAssignableFrom(ty) || float.class.isAssignableFrom(ty)) {
            return WasmType.f32;
        } else if (Void.class.isAssignableFrom(ty) || void.class.isAssignableFrom(ty)) {
            return null;
        }

        throw new UnsupportedOperationException("Unsupported class for WASM: " + ty);
    }
}
