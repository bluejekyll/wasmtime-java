package net.bluejekyll.wasmtime;

import jnr.ffi.LibraryLoader;
import jnr.ffi.Pointer;

/**
 * Wasmtime
 * 
 * A wrapper over the FFI of the Rust wasmtime library, which uses Wasmtime for the WASM runtime
 *
 */
public class Wasmtime 
{
    private final WasmtimeFFI ffi;

    public static interface WasmtimeFFI {
        
    }
    
    public Wasmtime() {
        this.ffi = LibraryLoader.create(WasmtimeFFI.class).library("wasmtime").load();
    }
    
    public Pointer newWasmEngine() {
        return null;
    }

    public void freeWasmEngine(Pointer engine) {
        // do nothing
    }

    public static void main( String[] args )
    {
        System.out.println( "Hello World!" );
    }
}

