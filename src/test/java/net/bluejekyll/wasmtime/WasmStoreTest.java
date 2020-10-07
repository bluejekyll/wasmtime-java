package net.bluejekyll.wasmtime;

import static org.junit.Assert.fail;

import org.junit.Test;

public class WasmStoreTest {
    @Test
    public void testNewWasmStore() {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine(); WasmStore store = engine.newStore()) {
            System.out.println("new store succeeded");
        } catch (Exception e) {
            fail();
        }
    }
}
