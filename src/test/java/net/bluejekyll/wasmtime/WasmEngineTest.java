package net.bluejekyll.wasmtime;

import static org.junit.Assert.fail;

import org.junit.Test;

public class WasmEngineTest {
    @Test
    public void testNewWasmEngine() throws Exception {
        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine()) {
            System.out.println("new engine succeeded");
        } catch (Exception e) {
            fail();
        }
    }
}
