package net.bluejekyll.wasmtime;

public class WasmStore extends AbstractOpaquePtr {
    // Store is !Send and !Sync in Rust, we will enforce that with a ThreadLocal
    // TODO: do we need stackable stores?
    private static final ThreadLocal<Long> STORE = new ThreadLocal<Long>();

    WasmStore(long ptr) {
        super(ptr, WasmStore::freeStoreChecked);
        if (STORE.get() != null) {
            throw new IllegalStateException("Previous STORE not cleared for this thread");
        }

        STORE.set(ptr);
    }

    private static native void freeStore(long ptr);

    private static void verifyStore(long ptr) {
        if (STORE.get() != ptr) {
            throw new IllegalStateException(String.format("STORE expected %d got %d", STORE.get(), ptr));
        }
    }

    @Override
    public long getPtr() {
        long ptr = super.getPtr();
        verifyStore(ptr);

        return ptr;
    }

    public static void freeStoreChecked(long ptr) {
        if (STORE.get() != ptr) {
            throw new IllegalStateException(String.format("STORE expected %d got %d", STORE.get(), ptr));
        }

        WasmStore.freeStore(ptr);
        STORE.remove();
    }
}
