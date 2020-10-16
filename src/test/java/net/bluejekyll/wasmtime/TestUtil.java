package net.bluejekyll.wasmtime;

import java.io.File;

public class TestUtil {
    public final static File WASM_TARGET_DIR = new File("target/wasm32-wasi/debug");
    public final static File MATH_PATH = new File(WASM_TARGET_DIR, "math.wasm");
    public final static File SLICES_PATH = new File(WASM_TARGET_DIR, "slices.wasm");
    public final static File STRINGS_PATH = new File(WASM_TARGET_DIR, "strings.wasm");
}
