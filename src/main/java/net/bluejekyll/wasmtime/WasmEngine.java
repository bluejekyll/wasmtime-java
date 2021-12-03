package net.bluejekyll.wasmtime;

import javax.annotation.concurrent.NotThreadSafe;

import java.io.File;
import java.io.FileInputStream;
import java.io.IOError;
import java.io.IOException;
import java.io.InputStream;
import java.nio.ByteBuffer;
import java.nio.file.CopyOption;

// TODO implement cloneable and clone the underlying engine between threads
@NotThreadSafe
public class WasmEngine extends AbstractOpaquePtr implements Cloneable {
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
        if (!wasm_bytes.isDirect())
            throw new WasmtimeException("passed in buffer must be direct");

        return new WasmModule(newModuleNtv(super.getPtr(), wasm_bytes.asReadOnlyBuffer()));
    }

    public WasmModule newModule(byte[] wasm_bytes) throws WasmtimeException {
        // ByteBuffers must be direct
        ByteBuffer buf = ByteBuffer.allocateDirect(wasm_bytes.length);
        buf.put(wasm_bytes);
        return new WasmModule(newModuleNtv(super.getPtr(), buf));
    }

    public WasmModule newModule(File wasm_file) throws WasmtimeException, IOException {
        try (InputStream in = new FileInputStream(wasm_file)) {
            return this.newModule(in.readAllBytes());
        }
    }

    /** Need to support clone for safe copies being used across threads */
    @Override
    protected Object clone() throws CloneNotSupportedException {
        // TODO Auto-generated method stub
        return super.clone();
    }
}
