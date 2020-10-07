package net.bluejekyll.wasmtime;

import java.lang.reflect.Method;
import java.nio.ByteBuffer;
import java.util.Optional;
import java.util.function.Function;

import org.junit.Assert;
import org.junit.Test;

import static org.junit.Assert.*;

/**
 * Unit test for simple App.
 */
public class WasmtimeTest {
    // @Test
    // public void testNewWasmEngine() {
    // Wasmtime wasm = new Wasmtime();
    // try (WasmEngine engine = wasm.newWasmEngine()) {
    // System.out.println("new engine succeeded");
    // }
    // }

    // @Test
    // public void testNewWasmStore() {
    // Wasmtime wasm = new Wasmtime();
    // try (WasmEngine engine = wasm.newWasmEngine(); WasmStore store =
    // engine.newStore()) {
    // System.out.println("new store succeeded");
    // }
    // }

    // @Test(expected = WasmtimeException.class)
    // public void testNewWasmBadModule() throws Exception {
    // byte[] bad = { 0, 1, 2 };

    // Wasmtime wasm = new Wasmtime();
    // try (WasmEngine engine = wasm.newWasmEngine(); WasmModule module =
    // engine.newModule(bad)) {
    // System.out.println("bad, module should have failed");
    // }
    // }

    // @Test
    // public void testWasmWorksAfterException() throws Exception {
    // byte[] bad = { 0, 1, 2 };

    // Wasmtime wasm = new Wasmtime();
    // try (WasmEngine engine = wasm.newWasmEngine()) {
    // try (WasmModule module = engine.newModule(bad)) {
    // fail("bad, module should have failed");
    // } catch (WasmtimeException e) {
    // // cool
    // }

    // // check that things are still working
    // try (WasmStore store = engine.newStore()) {
    // System.out.println("engine still functions");
    // }
    // }
    // }

    // @Test
    // public void testNewWasmModule() throws Exception {
    // String good = "(module\n" + " (import \"\" \"\" (func $host_hello (param
    // i32)))\n" + "\n"
    // + " (func (export \"hello\")\n" + " i32.const 3\n"
    // + " call $host_hello)\n" + " )";

    // Wasmtime wasm = new Wasmtime();
    // try (WasmEngine engine = wasm.newWasmEngine(); WasmModule module =
    // engine.newModule(good.getBytes())) {
    // System.out.println("module compiled");
    // assertNotNull(module);
    // }
    // }

    // public final void helloWorld() {
    // System.out.println("Hello World");
    // }

    // @Test
    // public void testFunction() throws Exception {
    // Wasmtime wasm = new Wasmtime();
    // try (WasmEngine engine = wasm.newWasmEngine(); WasmStore store =
    // engine.newStore()) {

    // Method method = this.getClass().getMethod("helloWorld");
    // WasmFunction func = WasmFunction.newFunc(store, method, this);

    // try (func) {
    // System.out.println("new store succeeded");
    // func.call();
    // assertNotNull(func);
    // }
    // }
    // }

    // @Test
    // public void testLinking() throws Exception {
    // String call_hello_world = "(module\n" + " (import \"hello\" \"world\" (func
    // $host_hello))\n"
    // + " (func (export \"hello\")\n" + " call $host_hello)\n" + " )";

    // Wasmtime wasm = new Wasmtime();
    // try (WasmEngine engine = wasm.newWasmEngine();
    // WasmStore store = engine.newStore();
    // WasmLinker linker = store.newLinker();) {

    // // define the Java hello world function
    // Method method = this.getClass().getMethod("helloWorld");
    // WasmFunction func = WasmFunction.newFunc(store, method, this);

    // // add it to the linker
    // linker.defineFunction("hello", "world", func);

    // // compile the calling module and then link it
    // WasmModule module = engine.newModule(call_hello_world.getBytes());
    // WasmInstance instance = linker.instantiate(module);
    // Optional<WasmFunction> function = instance.getFunction("hello");

    // assertTrue(function.isPresent());

    // function.ifPresent(f -> {
    // try {
    // f.call(new Object[0]);
    // } catch (Exception e) {
    // throw new RuntimeException(e);
    // }
    // });
    // }
    // }

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
