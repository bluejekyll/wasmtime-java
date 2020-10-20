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

    public void link(WasmStore store, WasmLinker linker) throws WasmtimeException, NoSuchMethodException {
        WasmFunction hello_to_java = WasmFunction.newFunc(store, this, "hello_to_java", ByteBuffer.class);
        linker.defineFunction("test", "hello_to_java", hello_to_java);
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

            link(store, linker);

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

            link(store, linker);

            WasmInstance instance = linker.instantiate(module);
            Optional<WasmFunction> func = instance.getFunction("print_bytes");

            assertTrue("print_bytes isn't present in the module", func.isPresent());
            WasmFunction function = func.get();

            byte[] bytes = new byte[] { 0, 1, 2, 3 };
            ByteBuffer buffer = ByteBuffer.allocateDirect(bytes.length);
            buffer.put(bytes);

            function.call(instance, buffer);
        }
    }

    @Test
    public void testReverseBytes() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
                WasmModule module = engine.newModule(TestUtil.SLICES_PATH);
                WasmStore store = engine.newStore();
                WasmLinker linker = store.newLinker()) {
            System.out.println("slices compiled");
            assertNotNull(module);

            link(store, linker);

            WasmInstance instance = linker.instantiate(module);
            Optional<WasmFunction> func = instance.getFunction("reverse_bytes");

            assertTrue("print_bytes isn't present in the module", func.isPresent());
            WasmFunction function = func.get();

            byte[] bytes = new byte[] { 0, 1, 2, 3 };
            ByteBuffer buffer = ByteBuffer.allocateDirect(bytes.length);
            buffer.put(bytes);

            ByteBuffer ret = function.call(instance, ByteBuffer.class, buffer);
            assertNotNull(ret);
            assertEquals(bytes.length, ret.remaining());

            byte[] reversed = new byte[bytes.length];
            ret.get(reversed);
            assertArrayEquals(reversed, new byte[] { 3, 2, 1, 0 });
        }
    }
}
