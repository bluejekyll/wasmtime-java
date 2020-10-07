package net.bluejekyll.wasmtime;

import static org.junit.Assert.assertNotNull;
import static org.junit.Assert.fail;

import org.junit.Test;

public class WasmModuleTest {
    @Test
    public void testNewWasmModule() throws Exception {
        String good = "(module\n" + " (import \"\" \"\" (func $host_hello (param i32)))\n" + "\n"
                + " (func (export \"hello\")\n" + " i32.const 3\n" + " call $host_hello)\n" + " )";

        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine(); WasmModule module = engine.newModule(good.getBytes())) {
            System.out.println("module compiled");
            assertNotNull(module);
        }
    }
}
