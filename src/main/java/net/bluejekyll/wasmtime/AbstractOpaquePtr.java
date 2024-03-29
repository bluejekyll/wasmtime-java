package net.bluejekyll.wasmtime;

import javax.annotation.concurrent.NotThreadSafe;
import java.lang.ref.Cleaner;
import java.util.function.Consumer;

@NotThreadSafe
public abstract class AbstractOpaquePtr implements AutoCloseable {
    private static final Cleaner cleaner = Cleaner.create();

    private final long ptr;
    private final Cleaner.Cleanable cleanable;

    /**
     * @param ptr  a valid, non-null pointer to the underlying native type
     * @param free a function to free the pointer, this must be a static method
     */
    protected AbstractOpaquePtr(long ptr, Consumer<Long> free) {
        this.ptr = ptr;
        this.cleanable = cleaner.register(this, new Freedom(ptr, free));

        if (this.ptr == 0) {
            throw new NullPointerException(
                    String.format("Null pointer for %s(%d)", this.getClass().getName(), this.ptr));
        }
    }

    private static class Freedom implements Runnable {
        private final long ptr;
        private final Consumer<Long> free;

        Freedom(long ptr, Consumer<Long> free) {
            this.ptr = ptr;
            this.free = free;
        }

        public void run() {
            this.free.accept(this.ptr);
        }
    }

    protected long getPtr() {
        if (this.ptr == 0) {
            throw new NullPointerException(
                    String.format("Null pointer for %s(%d)", this.getClass().getName(), this.ptr));
        }

        return this.ptr;
    }

    @Override
    public void close() {
        this.cleanable.clean();
    }
}
