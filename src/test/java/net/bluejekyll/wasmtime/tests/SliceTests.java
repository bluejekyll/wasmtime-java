package net.bluejekyll.wasmtime.tests;

import net.bluejekyll.wasmtime.*;
import org.junit.Test;

import java.io.UnsupportedEncodingException;
import java.nio.ByteBuffer;
import java.util.Optional;

import static org.junit.Assert.*;

/**
 * Tests corresponding to the Rust based WASM programs in /tests/slices
 */
public class SliceTests {
    // public void hello_to_java(byte[] hello_bytes) {
    // final String hello = "Hello Java!";

    // System.out.printf("Hello length: %d%n", hello_bytes.length);

    // final String from_wasm;
    // try {
    // from_wasm = new String(hello_bytes, "UTF-8");
    // } catch (UnsupportedEncodingException e) {
    // // this should never happen for UTF-8
    // throw new RuntimeException(e);
    // }

    // System.out.printf("Hello: %s%n", from_wasm);
    // assertEquals(hello, from_wasm);
    // }

    // public final byte[] reverse_bytes_java(byte[] buffer) {
    // byte[] bytes = new byte[buffer.length];

    // for (int i = bytes.length - 1; i >= 0; i--) {
    // bytes[i] = buffer[buffer.length - 1 - i];
    // }

    // return bytes;
    // }

    // public void link(WasmStore store, WasmLinker linker) throws
    // WasmtimeException, NoSuchMethodException {
    // WasmFunction hello_to_java = WasmFunction.newFunc(store, this,
    // "hello_to_java", byte[].class);
    // linker.defineFunction("test", "hello_to_java", hello_to_java);

    // WasmFunction reverse_bytes_java = WasmFunction.newFunc(store, this,
    // "reverse_bytes_java", byte[].class);
    // linker.defineFunction("test", "reverse_bytes_java", reverse_bytes_java);
    // }

    // @Test
    // public void testHelloToJavaWasmModule() throws Exception {
    // Wasmtime wasm = new Wasmtime();
    // try (WasmEngine engine = wasm.newWasmEngine();
    // WasmModule module = engine.newModule(TestUtil.SLICES_PATH);
    // WasmStore store = engine.newStore();
    // WasmLinker linker = engine.newLinker()) {
    // System.out.println("slices compiled");
    // assertNotNull(module);

    // link(store, linker);

    // WasmInstance instance = linker.instantiate(store, module);
    // Optional<WasmFunction> func = instance.getFunction(store,
    // "say_hello_to_java");

    // assertTrue("say_hello_to_java isn't present in the module",
    // func.isPresent());
    // WasmFunction function = func.get();

    // function.call(instance, store);
    // }
    // }

    // @Test
    // public void testSlicesWasmModule() throws Exception {
    // Wasmtime wasm = new Wasmtime();
    // try (WasmEngine engine = wasm.newWasmEngine();
    // WasmModule module = engine.newModule(TestUtil.SLICES_PATH);
    // WasmStore store = engine.newStore();
    // WasmLinker linker = engine.newLinker()) {
    // System.out.println("slices compiled");
    // assertNotNull(module);

    // link(store, linker);

    // WasmInstance instance = linker.instantiate(store, module);
    // Optional<WasmFunction> func = instance.getFunction(store, "print_bytes");

    // assertTrue("print_bytes isn't present in the module", func.isPresent());
    // WasmFunction function = func.get();

    // byte[] bytes = new byte[] { 0, 1, 2, 3 };
    // function.call(instance, store, bytes);
    // }
    // }

    // @Test
    // public void testReverseBytes() throws Exception {
    // Wasmtime wasm = new Wasmtime();
    // try (WasmEngine engine = wasm.newWasmEngine();
    // WasmModule module = engine.newModule(TestUtil.SLICES_PATH);
    // WasmStore store = engine.newStore();
    // WasmLinker linker = engine.newLinker()) {
    // System.out.println("slices compiled");
    // assertNotNull(module);

    // link(store, linker);

    // WasmInstance instance = linker.instantiate(store, module);
    // Optional<WasmFunction> func = instance.getFunction(store, "reverse_bytes");

    // assertTrue("print_bytes isn't present in the module", func.isPresent());
    // WasmFunction function = func.get();

    // byte[] bytes = new byte[] { 0, 1, 2, 3 };
    // byte[] ret = function.call(instance, store, byte[].class, bytes);
    // assertNotNull(ret);
    // assertEquals(bytes.length, ret.length);

    // assertArrayEquals(ret, new byte[] { 3, 2, 1, 0 });
    // }
    // }

    // @Test
    // public void testReverseBytesInJava() throws Exception {
    // Wasmtime wasm = new Wasmtime();
    // try (WasmEngine engine = wasm.newWasmEngine();
    // WasmModule module = engine.newModule(TestUtil.SLICES_PATH);
    // WasmStore store = engine.newStore();
    // WasmLinker linker = engine.newLinker()) {
    // System.out.println("slices compiled");
    // assertNotNull(module);

    // link(store, linker);

    // WasmInstance instance = linker.instantiate(store, module);
    // Optional<WasmFunction> func = instance.getFunction(store,
    // "reverse_bytes_in_java");

    // assertTrue("print_bytes isn't present in the module", func.isPresent());
    // WasmFunction function = func.get();

    // byte[] bytes = new byte[] { 0, 1, 2, 3 };
    // byte[] ret = function.call(instance, store, byte[].class, bytes);
    // assertNotNull(ret);
    // assertEquals(bytes.length, ret.length);

    // assertArrayEquals(ret, new byte[] { 3, 2, 1, 0 });
    // }
    // }
}
