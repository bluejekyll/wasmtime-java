package net.bluejekyll.wasmtime.proxy;

import static org.junit.Assert.assertEquals;
import static org.junit.Assert.assertNotNull;

import java.io.IOException;

import org.junit.After;
import org.junit.Before;
import org.junit.Test;

import net.bluejekyll.wasmtime.TestUtil;
import net.bluejekyll.wasmtime.WasmEngine;
import net.bluejekyll.wasmtime.WasmInstance;
import net.bluejekyll.wasmtime.WasmLinker;
import net.bluejekyll.wasmtime.WasmStore;
import net.bluejekyll.wasmtime.WasmModule;
import net.bluejekyll.wasmtime.Wasmtime;
import net.bluejekyll.wasmtime.WasmtimeException;

public class ImportTests {
    private Wasmtime wasmtime;
    private WasmEngine engine;
    private WasmModule module;
    private WasmStore store;
    private WasmLinker linker;
    private WasmImportProxy importProxy;

    @Before
    public void setup() throws WasmtimeException, IOException {
        this.wasmtime = new Wasmtime();
        this.engine = wasmtime.newWasmEngine();
        this.module = engine.newModule(TestUtil.MATH_PATH);
        System.out.println("slices compiled");
        assertNotNull(this.module);

        this.store = engine.newStore();
        this.linker = store.newLinker();
    }

    @After
    public void tearDown() {
        this.linker.close();
        this.store.close();
        this.module.close();
        this.engine.close();
    }

    @Test
    public void testAddIntegers() throws Exception {
        WasmInstance instance = linker.instantiate(this.module);
        WasmImportProxy importProxy = new WasmImportProxy(instance);
        TestImportProxy proxy = importProxy.newWasmProxy(TestImportProxy.class);

        int ret = proxy.addInteger(3, 2);
        assertEquals(ret, 5);
    }

    @Test
    public void testAddFloats() throws Exception {
        WasmInstance instance = linker.instantiate(module);
        WasmImportProxy importProxy = new WasmImportProxy(instance);
        TestImportProxy proxy = importProxy.newWasmProxy(TestImportProxy.class);

        float ret = proxy.addFloats((float) 1.1, (float) 2.2);
        assertEquals(ret, (float) 3.3, 0.1);
    }
}
