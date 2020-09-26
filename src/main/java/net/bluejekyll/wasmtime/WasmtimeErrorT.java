package net.bluejekyll.wasmtime;

import jnr.ffi.Pointer;

public class WasmtimeErrorT implements AutoCloseable {
    private final Wasmtime.WasmtimeFFI ffi;
    private final Pointer ptr;

    WasmtimeErrorT(Wasmtime.WasmtimeFFI ffi, Pointer ptr) {
        this.ffi = ffi;
        this.ptr = ptr;
    }

    public String getMessage() {
        String msg = new String();
        this.ffi.wasmtime_error_message(this.ptr, msg);

        return msg;
    }

    public void checkThrow() throws WasmtimeException {
        if (this.ptr != null) {
            throw WasmtimeException.fromErrorT(this);
        }
    }

    @Override
    public void close() {
        if (this.ptr != null) {
            ffi.wasmtime_error_delete(this.ptr);
        }
    }
}
