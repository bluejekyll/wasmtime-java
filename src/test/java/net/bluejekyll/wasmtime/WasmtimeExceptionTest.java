package net.bluejekyll.wasmtime;

import org.junit.Test;

import static org.junit.Assert.fail;

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
}
