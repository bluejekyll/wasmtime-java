package net.bluejekyll.wasmtime;

import static org.junit.Assert.assertTrue;

import org.junit.Test;
import jnr.ffi.Pointer;


/**
 * Unit test for simple App.
 */
public class WasmtimeTest 
{
    /**
     * Rigorous Test :-)
     */
    @Test
    public void testNewWasmEngine()
    {
        Wasmtime wasm = new Wasmtime();
        Pointer engine = wasm.newWasmEngine();
        wasm.freeWasmEngine(engine);
    }
}
