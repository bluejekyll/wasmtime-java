package net.bluejekyll.wasmtime;

public abstract class AbstractOpaquePtr {
    private final long ptr;

    protected AbstractOpaquePtr(long ptr) {
        this.ptr = ptr;
    }

    protected long getPtr() {
        return this.ptr;
    }
}
