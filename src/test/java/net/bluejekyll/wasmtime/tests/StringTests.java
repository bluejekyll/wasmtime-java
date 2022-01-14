package net.bluejekyll.wasmtime.tests;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

import java.util.Optional;

import org.junit.Test;

import net.bluejekyll.wasmtime.*;

/**
 * Tests corresponding to the Rust based WASM programs in /tests/strings
 */
public class StringTests {
    // public final String say_hello_to_java(String name) {
    // return String.format("Hello, %s!", name);
    // }

    // public void link(WasmStore store, WasmLinker linker) throws
    // WasmtimeException, NoSuchMethodException {
    // WasmFunction hello_to_java = WasmFunction.newFunc(store, this,
    // "say_hello_to_java", String.class);
    // linker.defineFunction("test", "say_hello_to_java", hello_to_java);
    // }

    // @Test
    // public void testHello() throws Exception {
    // Wasmtime wasm = new Wasmtime();
    // try (WasmEngine engine = wasm.newWasmEngine();
    // WasmModule module = engine.newModule(TestUtil.STRINGS_PATH);
    // WasmStore store = engine.newStore();
    // WasmLinker linker = engine.newLinker()) {
    // System.out.println("slices compiled");
    // assertNotNull(module);

    // link(store, linker);
    // WasmInstance instance = linker.instantiate(store, module);
    // Optional<WasmFunction> func = instance.getFunction(store, "say_hello_to");

    // assertTrue("say_hello_to isn't present in the module", func.isPresent());
    // WasmFunction function = func.get();

    // String name = this.getClass().getName();
    // String ret = function.call(instance, store, String.class, name);
    // assertNotNull(ret);

    // String expected = String.format("Hello, %s!", name);
    // assertEquals(expected, ret);
    // }
    // }

    // @Test
    // public void testHelloToJava() throws Exception {
    // Wasmtime wasm = new Wasmtime();
    // try (WasmEngine engine = wasm.newWasmEngine();
    // WasmModule module = engine.newModule(TestUtil.STRINGS_PATH);
    // WasmStore store = engine.newStore();
    // WasmLinker linker = engine.newLinker()) {
    // System.out.println("slices compiled");
    // assertNotNull(module);

    // link(store, linker);
    // WasmInstance instance = linker.instantiate(store, module);

    // Optional<WasmFunction> func = instance.getFunction(store,
    // "say_hello_in_java");

    // assertTrue("say_hello_to isn't present in the module", func.isPresent());
    // WasmFunction function = func.get();

    // String name = this.getClass().getName();
    // String ret = function.call(instance, store, String.class, name);
    // assertNotNull(ret);

    // String expected = String.format("Hello, %s!", name);
    // assertEquals(expected, ret);
    // }
    // }
}
