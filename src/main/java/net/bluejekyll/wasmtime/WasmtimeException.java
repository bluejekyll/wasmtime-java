package net.bluejekyll.wasmtime;

public class WasmtimeException extends Exception {
    public WasmtimeException(String msg) {
        super(msg);
    }

    public WasmtimeException(String msg, Throwable e) {
        super(msg, e);
    }
}
