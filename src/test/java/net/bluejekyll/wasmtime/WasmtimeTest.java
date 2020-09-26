package net.bluejekyll.wasmtime;

import static org.junit.Assert.assertTrue;

import org.junit.Test;
import jnr.ffi.Pointer;


/**
 * Unit test for simple App.
 */
public class WasmtimeTest 
{
    @Test
    public void testNewWasmEngine()
    {
        Wasmtime wasm = new Wasmtime();
        try(WasmEngineT engine = wasm.newWasmEngine()) {
            System.out.println("new engine succeeded");
        }
    }

    @Test
    public void testNewWasmStore()
    {
        Wasmtime wasm = new Wasmtime();
        try(WasmEngineT engine = wasm.newWasmEngine();
            WasmStoreT store = engine.newStore()) {
            System.out.println("new store succeeded");
        }
    }

    @Test(expected = WasmtimeException.class)
    public void testNewWasmBadModule() throws Exception
    {
        byte[] bad = {};

        Wasmtime wasm = new Wasmtime();
        try(WasmEngineT engine = wasm.newWasmEngine();
            WasmModuleT module = engine.newModule(bad)) {
            System.out.println("bad, module should have failed");
        }
    }

    @Test(expected = WasmtimeException.class)
    public void testNewWasmModule() throws Exception
    {
        String good = "(module\n" +
                "            (import \"\" \"\" (func $host_hello (param i32)))\n" +
                "\n" +
                "            (func (export \"hello\")\n" +
                "                i32.const 3\n" +
                "                call $host_hello)\n" +
                "        )";

        Wasmtime wasm = new Wasmtime();
        try(WasmEngineT engine = wasm.newWasmEngine();
            WasmModuleT module = engine.newModule(good.getBytes())) {
            System.out.println("bad, module should have failed");
        }
    }
}
