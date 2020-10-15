package net.bluejekyll.wasmtime;

import java.io.File;

public class TestUtil {
    final static File WASM_TARGET_DIR = new File("target/wasm32-wasi/debug");
    final static File MATH_PATH = new File(WASM_TARGET_DIR, "math.wasm");
    final static File SLICES_PATH = new File(WASM_TARGET_DIR, "slices.wasm");
    final static File STRINGS_PATH = new File(WASM_TARGET_DIR, "strings.wasm");
}
