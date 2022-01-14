package net.bluejekyll.wasmtime.tests;

import net.bluejekyll.wasmtime.*;
import net.bluejekyll.wasmtime.ty.*;

import org.junit.Test;

import java.io.UnsupportedEncodingException;
import java.nio.ByteBuffer;
import java.util.Optional;

import static org.junit.Assert.*;
import static net.bluejekyll.wasmtime.ty.WasmTypeUtil.*;

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
                WasmLinker linker = engine.newLinker()) {
            System.out.println("slices compiled");
            assertNotNull(module);

            WasmInstance instance = linker.instantiate(store, module);
            Optional<WasmFunction> func = instance.getFunction(store, "add_i32");

            assertTrue("add_i32 isn't present in the module", func.isPresent());
            WasmFunction function = func.get();

            I32 ret = function.call(instance, store, I32.class, i32(3), i32(2));
            assertEquals(ret.field, 5);
        }
    }

    @Test
    public void testAddI64() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
                WasmModule module = engine.newModule(TestUtil.MATH_PATH);
                WasmStore store = engine.newStore();
                WasmLinker linker = engine.newLinker()) {
            System.out.println("slices compiled");
            assertNotNull(module);

            WasmInstance instance = linker.instantiate(store, module);
            Optional<WasmFunction> func = instance.getFunction(store, "add_i64");

            assertTrue("add_i64 isn't present in the module", func.isPresent());
            WasmFunction function = func.get();

            I64 ret = function.call(instance, store, I64.class, i64(3), i64(2));
            assertEquals(ret.field, 5);
        }
    }

    @Test
    public void testAddF32() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
                WasmModule module = engine.newModule(TestUtil.MATH_PATH);
                WasmStore store = engine.newStore();
                WasmLinker linker = engine.newLinker()) {
            System.out.println("slices compiled");
            assertNotNull(module);

            WasmInstance instance = linker.instantiate(store, module);
            Optional<WasmFunction> func = instance.getFunction(store, "add_f32");

            assertTrue("add_f32 isn't present in the module", func.isPresent());
            WasmFunction function = func.get();

            F32 ret = function.call(instance, store, F32.class, f32((float) 1.1), f32((float) 2.2));
            assertEquals(ret.field, (float) 3.3, 0.1);
        }
    }

    @Test
    public void testAddF64() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
                WasmModule module = engine.newModule(TestUtil.MATH_PATH);
                WasmStore store = engine.newStore();
                WasmLinker linker = engine.newLinker()) {
            System.out.println("slices compiled");
            assertNotNull(module);

            WasmInstance instance = linker.instantiate(store, module);
            Optional<WasmFunction> func = instance.getFunction(store, "add_f64");

            assertTrue("add_f64 isn't present in the module", func.isPresent());
            WasmFunction function = func.get();

            F64 ret = function.call(instance, store, F64.class, f64((double) 1.1), f64((double) 2.2));
            assertEquals(ret.field, (double) 3.3, 0.1);
        }
    }
}
