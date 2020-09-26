package net.bluejekyll.wasmtime;

public class WasmtimeException extends Exception {
    public WasmtimeException(String msg) {
        super(msg);
    }

    public static WasmtimeException fromErrorT(WasmtimeErrorT error) {
        String msg = error.getMessage();
        error.close();

        return new WasmtimeException(msg);
    }
}
