package net.bluejekyll.wasmtime;

import org.junit.Test;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNull;
import static org.junit.Assert.assertTrue;
import static org.junit.Assert.fail;

import java.lang.reflect.Method;

public class WasmtimeExceptionTest {
    @Test(expected = WasmtimeException.class)
    public void testNewWasmBadModule() throws Exception {
        byte[] bad = { 0, 1, 2 };

        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine(); WasmModule module = engine.newModule(bad)) {
            System.out.println("bad, module should have failed");
        }
    }

    @Test
    public void testWasmWorksAfterException() throws Exception {
        byte[] bad = { 0, 1, 2 };

        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine()) {
            try (WasmModule module = engine.newModule(bad)) {
                fail("bad, module should have failed");
            } catch (WasmtimeException e) {
                // cool
            }

            // check that things are still working
            try (WasmStore store = engine.newStore()) {
                System.out.println("engine still functions");
            }
        }
    }

    public void iThrowForFun() {
        System.out.println("I throw for fun!");
        throw new RuntimeException("I throw for fun!");
    }

    public void iWork() {
        System.out.println("I work!");
    }

    @Test
    public void testExceptionInJavaFunc() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine(); WasmStore store = engine.newStore()) {

            Method method = this.getClass().getMethod("iThrowForFun");
            WasmFunction func = WasmFunction.newFunc(store, method, this);

            try (func) {
                System.out.println("running function");
                func.call_for_tests();
            } catch (Exception e) {
                // TODO: we eventually want to look for the RuntimeException
                assertTrue(e.getMessage().contains("InvocationTargetException"));
            }

            // double check the exception is cleared...
            Method method2 = this.getClass().getMethod("iWork");
            WasmFunction func2 = WasmFunction.newFunc(store, method2, this);

            try (func2) {
                func2.call_for_tests();
                assertTrue(true);
            }
        }
    }
}
