package net.bluejekyll.wasmtime.tests;

import net.bluejekyll.wasmtime.*;
import org.junit.Test;

import java.io.UnsupportedEncodingException;
import java.nio.ByteBuffer;
import java.util.Optional;

import static org.junit.Assert.*;

/**
 * Tests corresponding to the Rust based WASM programs in /tests/math
 */
public class MathTests {
    @Test
    public void testAddI32() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
                WasmModule module = engine.newModule(TestUtil.MATH_PATH);
                WasmStore store = engine.newStore();
                WasmLinker linker = store.newLinker()) {
            System.out.println("slices compiled");
            assertNotNull(module);

            WasmInstance instance = linker.instantiate(module);
            Optional<WasmFunction> func = instance.getFunction("add_i32");

            assertTrue("add_i32 isn't present in the module", func.isPresent());
            WasmFunction function = func.get();

            int ret = function.call(instance, Integer.TYPE, 3, 2);
            assertEquals(ret, 5);
        }
    }

    @Test
    public void testAddU32() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
                WasmModule module = engine.newModule(TestUtil.MATH_PATH);
                WasmStore store = engine.newStore();
                WasmLinker linker = store.newLinker()) {
            System.out.println("slices compiled");
            assertNotNull(module);

            WasmInstance instance = linker.instantiate(module);
            Optional<WasmFunction> func = instance.getFunction("add_u32");

            assertTrue("add_u32 isn't present in the module", func.isPresent());
            WasmFunction function = func.get();

            int ret = function.call(instance, Integer.TYPE, 3, 2);
            assertEquals(ret, 5);
        }
    }

    @Test
    public void testAddI64() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
                WasmModule module = engine.newModule(TestUtil.MATH_PATH);
                WasmStore store = engine.newStore();
                WasmLinker linker = store.newLinker()) {
            System.out.println("slices compiled");
            assertNotNull(module);

            WasmInstance instance = linker.instantiate(module);
            Optional<WasmFunction> func = instance.getFunction("add_i64");

            assertTrue("add_i64 isn't present in the module", func.isPresent());
            WasmFunction function = func.get();

            long ret = function.call(instance, Long.TYPE, (long)3, (long)2);
            assertEquals(ret, 5);
        }
    }

    @Test
    public void testAddU64() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
                WasmModule module = engine.newModule(TestUtil.MATH_PATH);
                WasmStore store = engine.newStore();
                WasmLinker linker = store.newLinker()) {
            System.out.println("slices compiled");
            assertNotNull(module);

            WasmInstance instance = linker.instantiate(module);
            Optional<WasmFunction> func = instance.getFunction("add_u64");

            assertTrue("add_u64 isn't present in the module", func.isPresent());
            WasmFunction function = func.get();

            long ret = function.call(instance, Long.TYPE, (long)3, (long)2);
            assertEquals(ret, 5);
        }
    }

    @Test
    public void testAddF32() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
                WasmModule module = engine.newModule(TestUtil.MATH_PATH);
                WasmStore store = engine.newStore();
                WasmLinker linker = store.newLinker()) {
            System.out.println("slices compiled");
            assertNotNull(module);

            WasmInstance instance = linker.instantiate(module);
            Optional<WasmFunction> func = instance.getFunction("add_f32");

            assertTrue("add_f32 isn't present in the module", func.isPresent());
            WasmFunction function = func.get();

            float ret = function.call(instance, Float.TYPE, (float)1.1, (float)2.2);
            assertEquals(ret, (float)3.3, 0.1);
        }
    }

    @Test
    public void testAddF64() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
                WasmModule module = engine.newModule(TestUtil.MATH_PATH);
                WasmStore store = engine.newStore();
                WasmLinker linker = store.newLinker()) {
            System.out.println("slices compiled");
            assertNotNull(module);

            WasmInstance instance = linker.instantiate(module);
            Optional<WasmFunction> func = instance.getFunction("add_f64");

            assertTrue("add_f64 isn't present in the module", func.isPresent());
            WasmFunction function = func.get();

            double ret = function.call(instance, Double.TYPE, (double)1.1, (double)2.2);
            assertEquals(ret, (double)3.3, 0.1);
        }
    }
}
