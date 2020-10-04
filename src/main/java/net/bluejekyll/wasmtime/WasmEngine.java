package net.bluejekyll.wasmtime;

import javax.annotation.concurrent.NotThreadSafe;
import java.nio.ByteBuffer;

// TODO implement cloneable and clone the underlying engine between threads
@NotThreadSafe
public class WasmEngine extends AbstractOpaquePtr  {
    WasmEngine(long ptr) {
        super(ptr, WasmEngine::freeEngine);
    }

    private static native void freeEngine(long ptr);
    private static native long newStoreNtv(long engine_ptr);
    private static native long newModuleNtv(long engine_ptr, ByteBuffer wasm_bytes) throws WasmtimeException;

    public WasmStore newStore() {
        return new WasmStore(WasmEngine.newStoreNtv(super.getPtr()));
    }

    public WasmModule newModule(ByteBuffer wasm_bytes) throws WasmtimeException {
        if (!wasm_bytes.isDirect()) throw new WasmtimeException("passed in buffer must be direct");
        
        System.out.println("sending bytes: " + wasm_bytes.capacity());
        return new WasmModule(newModuleNtv(super.getPtr(), wasm_bytes.asReadOnlyBuffer()));
    }

    public WasmModule newModule(byte[] wasm_bytes) throws WasmtimeException {
        // ByteBuffers must be direct
        ByteBuffer buf = ByteBuffer.allocateDirect(wasm_bytes.length);
        buf.put(wasm_bytes);
        return new WasmModule(newModuleNtv(super.getPtr(), buf));
    }
}
