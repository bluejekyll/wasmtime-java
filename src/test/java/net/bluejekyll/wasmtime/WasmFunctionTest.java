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
                func.call();
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
                int val = func.call(1, 2);
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
                int val = func.call(1, 2);
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
                long val = func.call((long) 1, (long) 2);
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
                Long val = func.call((long) 1, (long) 2);
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
                float val = func.call((float) 1.1, (float) 1.2);
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
                Float val = func.call((float) 1.1, (float) 1.2);

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
                double val = func.call((double) 1.1, (double) 1.2);
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
                Double val = func.call((double) 1.1, (double) 1.2);

                assertTrue(2.29 < val);
                assertTrue(2.31 > val);
            }
        }
    }

    public void hello_to_java(ByteBuffer hello_bytes) {
        final String hello = "Hello Java!";

        System.out.printf("Hello length: %d%n", hello_bytes.capacity());

        final byte[] bytes = new byte[hello_bytes.capacity()];
        hello_bytes.get(bytes);

        final String from_wasm;
        try {
            from_wasm = new String(bytes, "UTF-8");
        } catch (UnsupportedEncodingException e) {
            // this should never happen for UTF-8
            throw new RuntimeException(e);
        }

        System.out.printf("Hello: %s%n", from_wasm);
        assertEquals(hello, from_wasm);
    }

    @Test
    public void testHelloToJavaWasmModule() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
             WasmModule module = engine.newModule(TestUtil.SLICES_PATH);
             WasmStore store = engine.newStore();
             WasmLinker linker = store.newLinker()) {
            System.out.println("slices compiled");
            assertNotNull(module);

            WasmFunction hello_to_java = WasmFunction.newFunc(store, this, "hello_to_java", ByteBuffer.class);
            linker.defineFunction("test", "hello_to_java", hello_to_java);

            WasmInstance instance = linker.instantiate(module);
            Optional<WasmFunction> func = instance.getFunction("say_hello_to_java");

            assertTrue("say_hello_to_java isn't present in the module", func.isPresent());
            WasmFunction function = func.get();

            function.call(instance);
        }
    }

    @Test
    public void testSlicesWasmModule() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
             WasmModule module = engine.newModule(TestUtil.SLICES_PATH);
             WasmStore store = engine.newStore();
             WasmLinker linker = store.newLinker()) {
            System.out.println("slices compiled");
            assertNotNull(module);

            WasmInstance instance = linker.instantiate(module);
            Optional<WasmFunction> func = instance.getFunction("print_bytes");

            assertTrue("print_bytes isn't present in the module", func.isPresent());
            WasmFunction function = func.get();

            byte[] bytes = new byte[]{0,1,2,3};
            ByteBuffer buffer = ByteBuffer.allocateDirect(bytes.length);
            buffer.put(bytes);

            function.call(instance, buffer);
        }
    }
}
