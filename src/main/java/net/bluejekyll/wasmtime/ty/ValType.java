package net.bluejekyll.wasmtime.ty;

public enum ValType {
    I32,
    I64,
    F32,
    F64,
    V128,
    FuncRef,
    ExternRef;

    public boolean is_num() {
        switch (this) {
            case I32:
                ;
            case I64:
                ;
            case F32:
                ;
            case V128:
                return true;
            default:
                return false;
        }
    }

    public boolean is_ref() {
        switch (this) {
            case FuncRef:
                ;
            case ExternRef:
                return true;
            default:
                return false;
        }
    }
}
