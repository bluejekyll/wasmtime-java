package net.bluejekyll.wasmtime;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;

import java.lang.reflect.Method;

import org.junit.Test;

public class WasmFuntionTest {
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
}
