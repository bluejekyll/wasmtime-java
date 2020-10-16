package net.bluejekyll.wasmtime;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

import java.io.UnsupportedEncodingException;
import java.lang.reflect.Method;
import java.nio.ByteBuffer;
import java.util.Optional;

import org.junit.Test;

public class WasmFunctionTest {
    public final void helloWorld() {
        System.out.println("Hello World");
    }

    @Test
    public void testFunction() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine(); WasmStore store = engine.newStore()) {

            Method method = this.getClass().getMethod("helloWorld");
            WasmFunction func = WasmFunction.newFunc(store, method, this);

            try (func) {
                System.out.println("new store succeeded");
                func.call_for_tests();
                assertNotNull(func);
            }
        }
    }

    public final int addInts(int a, int b) {
        return a + b;
    }

    @Test
    public void testParamsAndReturn() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine(); WasmStore store = engine.newStore()) {

            Method method = this.getClass().getMethod("addInts", int.class, int.class);
            WasmFunction func = WasmFunction.newFunc(store, method, this);

            try (func) {
                System.out.println("running function");
                int val = func.call_for_tests(1, 2);
                assertEquals(3, val);
            }
        }
    }

    public final Integer addIntegers(Integer a, Integer b) {
        return a + b;
    }

    @Test
    public void testParamsAndReturnObj() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine(); WasmStore store = engine.newStore()) {

            Method method = this.getClass().getMethod("addIntegers", Integer.class, Integer.class);
            WasmFunction func = WasmFunction.newFunc(store, method, this);

            try (func) {
                System.out.println("running function");
                int val = func.call_for_tests(1, 2);
                assertEquals(3, val);
            }
        }
    }

    public final long addLongs(long a, long b) {
        return a + b;
    }

    @Test
    public void testLongParamsAndReturn() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine(); WasmStore store = engine.newStore()) {

            Method method = this.getClass().getMethod("addLongs", long.class, long.class);
            WasmFunction func = WasmFunction.newFunc(store, method, this);

            try (func) {
                System.out.println("running function");
                long val = func.call_for_tests((long) 1, (long) 2);
                assertEquals(3, val);
            }
        }
    }

    public final Long addLongObjs(Long a, Long b) {
        return a + b;
    }

    @Test
    public void testLongObjParamsAndReturn() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine(); WasmStore store = engine.newStore()) {

            Method method = this.getClass().getMethod("addLongObjs", Long.class, Long.class);
            WasmFunction func = WasmFunction.newFunc(store, method, this);

            try (func) {
                System.out.println("running function");
                Long val = func.call_for_tests((long) 1, (long) 2);
                assertEquals(3, val.longValue());
            }
        }
    }

    public final float addFloats(float a, float b) {
        return a + b;
    }

    @Test
    public void testFloatParamsAndReturn() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine(); WasmStore store = engine.newStore()) {

            Method method = this.getClass().getMethod("addFloats", float.class, float.class);
            WasmFunction func = WasmFunction.newFunc(store, method, this);

            try (func) {
                System.out.println("running function");
                float val = func.call_for_tests((float) 1.1, (float) 1.2);
                assertTrue(2.29 < val);
                assertTrue(2.31 > val);
            }
        }
    }

    public final Float addFloats(Float a, Float b) {
        return a + b;
    }

    @Test
    public void testFloatObjParamsAndReturn() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine(); WasmStore store = engine.newStore()) {

            Method method = this.getClass().getMethod("addFloats", Float.class, Float.class);
            WasmFunction func = WasmFunction.newFunc(store, method, this);

            try (func) {
                System.out.println("running function");
                Float val = func.call_for_tests((float) 1.1, (float) 1.2);

                assertTrue(2.29 < val);
                assertTrue(2.31 > val);
            }
        }
    }

    public final double addDoubles(double a, double b) {
        return a + b;
    }

    @Test
    public void testDoubleParamsAndReturn() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine(); WasmStore store = engine.newStore()) {

            Method method = this.getClass().getMethod("addDoubles", double.class, double.class);
            WasmFunction func = WasmFunction.newFunc(store, method, this);

            try (func) {
                System.out.println("running function");
                double val = func.call_for_tests((double) 1.1, (double) 1.2);
                assertTrue(2.29 < val);
                assertTrue(2.31 > val);
            }
        }
    }

    public final Double addDoubles(Double a, Double b) {
        return a + b;
    }

    @Test
    public void testDoubleObjParamsAndReturn() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine(); WasmStore store = engine.newStore()) {
            WasmFunction func = WasmFunction.newFunc(store, this, "addDoubles", Double.class, Double.class);

            try (func) {
                System.out.println("running function");
                Double val = func.call_for_tests((double) 1.1, (double) 1.2);

                assertTrue(2.29 < val);
                assertTrue(2.31 > val);
            }
        }
    }
}
