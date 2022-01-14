package net.bluejekyll.wasmtime;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

import java.io.UnsupportedEncodingException;
import java.lang.reflect.Method;
import java.nio.ByteBuffer;
import java.util.Optional;

import org.junit.Test;

import net.bluejekyll.wasmtime.ty.*;

import static net.bluejekyll.wasmtime.ty.WasmTypeUtil.*;

public class WasmFunctionTest {
    public final void helloWorld() {
        System.out.println("Hello World");
    }

    @Test
    public void testFunction() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
                WasmStore store = engine.newStore()) {

            Method method = this.getClass().getMethod("helloWorld");
            WasmFunction func = WasmFunction.newFunc(store, method, this);

            try (func) {
                System.out.println("new store succeeded");
                func.call_for_tests(store);
                assertNotNull(func);
            }
        }
    }

    public final I32 addI32s(I32 a, I32 b) {
        return i32(a.field + b.field);
    }

    @Test
    public void testParamsAndReturn() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
                WasmStore store = engine.newStore()) {

            Method method = this.getClass().getMethod("addI32s", I32.class, I32.class);
            WasmFunction func = WasmFunction.newFunc(store, method, this);

            try (func) {
                System.out.println("running function");
                I32 val = func.call_for_tests(store, I32.class, i32(1), i32(2));
                assertEquals(3, val.field);
            }
        }
    }

    public final I64 addI64s(I64 a, I64 b) {
        return i64(a.field + b.field);
    }

    @Test
    public void testLongParamsAndReturn() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine(); WasmStore store = engine.newStore()) {

            Method method = this.getClass().getMethod("addI64s", I64.class, I64.class);
            WasmFunction func = WasmFunction.newFunc(store, method, this);

            try (func) {
                System.out.println("running function");
                I64 val = func.call_for_tests(store, I64.class, i64(1), i64(2));
                assertEquals(3, val.field);
            }
        }
    }

    public final F32 addF32s(F32 a, F32 b) {
        return f32(a.field + b.field);
    }

    @Test
    public void testFloatParamsAndReturn() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine(); WasmStore store = engine.newStore()) {

            Method method = this.getClass().getMethod("addF32s", F32.class, F32.class);
            WasmFunction func = WasmFunction.newFunc(store, method, this);

            try (func) {
                System.out.println("running function");
                F32 val = func.call_for_tests(store, F32.class, f32((float) 1.1), f32((float) 1.2));
                assertTrue(2.29 < val.field);
                assertTrue(2.31 > val.field);
            }
        }
    }

    public final F64 addF64(F64 a, F64 b) {
        return f64(a.field + b.field);
    }

    @Test
    public void testDoubleParamsAndReturn() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine(); WasmStore store = engine.newStore()) {

            Method method = this.getClass().getMethod("addF64", F64.class, F64.class);
            WasmFunction func = WasmFunction.newFunc(store, method, this);

            try (func) {
                System.out.println("running function");
                F64 val = func.call_for_tests(store, F64.class, f64((double) 1.1), f64((double) 1.2));
                assertTrue(2.29 < val.field);
                assertTrue(2.31 > val.field);
            }
        }
    }
}
