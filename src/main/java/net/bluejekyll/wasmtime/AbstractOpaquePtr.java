package net.bluejekyll.wasmtime;

import javax.annotation.concurrent.NotThreadSafe;

@NotThreadSafe
public abstract class AbstractOpaquePtr implements AutoCloseable {
    private final long ptr;

    protected AbstractOpaquePtr(long ptr) {
        this.ptr = ptr;
    }

    protected long getPtr() {
        return this.ptr;
    }

    protected abstract void free(long ptr);

    @Override
    public void close() {
        this.free(this.ptr);
    }
}
