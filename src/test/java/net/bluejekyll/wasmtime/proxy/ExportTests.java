package net.bluejekyll.wasmtime.proxy;

import static org.junit.Assert.assertArrayEquals;
import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.assertTrue;

import java.io.IOException;
import java.util.Optional;

import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import net.bluejekyll.wasmtime.TestUtil;
import net.bluejekyll.wasmtime.WasmEngine;
import net.bluejekyll.wasmtime.WasmFunction;
import net.bluejekyll.wasmtime.WasmInstance;
import net.bluejekyll.wasmtime.WasmLinker;
import net.bluejekyll.wasmtime.WasmStore;
import net.bluejekyll.wasmtime.WasmModule;
import net.bluejekyll.wasmtime.Wasmtime;
import net.bluejekyll.wasmtime.WasmtimeException;

/**
 * Tests for validating Class (module) exports to WASM
 */
public class ExportTests {
    private Wasmtime wasmtime;
    private WasmEngine engine;
    private WasmModule module;
    private WasmStore store;
    private WasmLinker linker;

    @Before
    public void setup() throws WasmtimeException, IOException {
        this.wasmtime = new Wasmtime();
        this.engine = wasmtime.newWasmEngine();
        this.module = engine.newModule(TestUtil.SLICES_PATH);
        System.out.println("slices compiled");
        assertNotNull(this.module);

        this.store = engine.newStore();
        this.linker = engine.newLinker();

        // this could be a test instead...
        this.linker.defineFunctions(this.store, new TestExport());
    }

    @After
    public void tearDown() {
        this.linker.close();
        this.store.close();
        this.module.close();
        this.engine.close();
    }

    @Test
    public void testHelloToJavaWasmModule() throws Exception {
        WasmInstance instance = linker.instantiate(store, module);
        Optional<WasmFunction> func = instance.getFunction(store, "say_hello_to_java");

        assertTrue("say_hello_to_java isn't present in the module", func.isPresent());
        WasmFunction function = func.get();

        function.call(instance, store);

    }

    @Test
    public void testSlicesWasmModule() throws Exception {
        WasmInstance instance = linker.instantiate(store, module);
        Optional<WasmFunction> func = instance.getFunction(store, "print_bytes");

        assertTrue("print_bytes isn't present in the module", func.isPresent());
        WasmFunction function = func.get();

        byte[] bytes = new byte[] { 0, 1, 2, 3 };

        function.call(instance, store, bytes);
    }

    @Test
    public void testReverseBytes() throws Exception {
        WasmInstance instance = linker.instantiate(store, module);
        Optional<WasmFunction> func = instance.getFunction(store, "reverse_bytes");

        assertTrue("print_bytes isn't present in the module", func.isPresent());
        WasmFunction function = func.get();

        byte[] bytes = new byte[] { 0, 1, 2, 3 };

        byte[] ret = function.call(instance, store, byte[].class, bytes);
        assertNotNull(ret);
        assertEquals(bytes.length, ret.length);

        assertArrayEquals(ret, new byte[] { 3, 2, 1, 0 });
    }

    @Test
    public void testReverseBytesInJava() throws Exception {
        WasmInstance instance = linker.instantiate(store, module);
        Optional<WasmFunction> func = instance.getFunction(store, "reverse_bytes_in_java");

        assertTrue("print_bytes isn't present in the module", func.isPresent());
        WasmFunction function = func.get();

        byte[] bytes = new byte[] { 0, 1, 2, 3 };

        byte[] ret = function.call(instance, store, byte[].class, bytes);
        assertNotNull(ret);
        assertEquals(bytes.length, ret.length);

        assertArrayEquals(ret, new byte[] { 3, 2, 1, 0 });

    }
}
