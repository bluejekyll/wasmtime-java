package net.bluejekyll.wasmtime;

import static org.junit.Assert.assertTrue;

import java.lang.reflect.Method;
import java.nio.ByteBuffer;
import java.util.function.Function;

import org.junit.Assert;
import org.junit.Test;


/**
 * Unit test for simple App.
 */
public class WasmtimeTest {
    @Test
    public void testNewWasmEngine() {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine()) {
            System.out.println("new engine succeeded");
        }
    }

    @Test
    public void testNewWasmStore() {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
             WasmStore store = engine.newStore()) {
            System.out.println("new store succeeded");
        }
    }

    @Test(expected = WasmtimeException.class)
    public void testNewWasmBadModule() throws Exception {
        byte[] bad = {0, 1, 2};

        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
             WasmModule module = engine.newModule(bad)) {
            System.out.println("bad, module should have failed");
        }
    }

    @Test
    public void testWasmWorksAfterException() throws Exception {
        byte[] bad = {0, 1, 2};

        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine()) {
            try (WasmModule module = engine.newModule(bad)) {
                Assert.assertTrue("bad, module should have failed", false);
            } catch (WasmtimeException e) {
                // cool
            }

            // check that things are still working
            try (WasmStore store = engine.newStore()) {
                System.out.println("engine still functions");
            }
        }
    }

    @Test
    public void testNewWasmModule() throws Exception {
        String good = "(module\n" +
                "            (import \"\" \"\" (func $host_hello (param i32)))\n" +
                "\n" +
                "            (func (export \"hello\")\n" +
                "                i32.const 3\n" +
                "                call $host_hello)\n" +
                "        )";

        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
             WasmModule module = engine.newModule(good.getBytes())) {
            System.out.println("module compiled");
        }
    }

    public final void helloWorld() {
        System.out.println("Hello World");
    }

    @Test
    public void testFunction() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
             WasmStore store = engine.newStore()) {

            Method method = this.getClass().getMethod("helloWorld", new Class[]{});
            WasmFunction func = WasmFunction.newFunc(store, method, this);

            try (func) {
                System.out.println("new store succeeded");
            }
        }
    }
}
