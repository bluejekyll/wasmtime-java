package net.bluejekyll.wasmtime;

import java.io.File;
import java.io.FileOutputStream;
import java.io.InputStream;
import java.io.OutputStream;
import java.nio.ByteBuffer;

/**
 * Wasmtime
 * <p>
 * A wrapper over the FFI of the Rust wasmtime library, which uses Wasmtime for
 * the WASM runtime
 */
public class Wasmtime {
    private static final String NATIVE_LIB = "wasmtime_jni";
    private static volatile boolean libraryLoaded = false;

    private static void loadNative() {
        if (Wasmtime.libraryLoaded) {
            return;
        }

        try {
            // This actually loads the shared object that we'll be creating.
            // The actual location of the .so or .dll may differ based on your
            // platform.
            System.loadLibrary(NATIVE_LIB);
            System.out.printf("loadLibrary succeeded for %s%n", NATIVE_LIB);
            libraryLoaded = true;
        } catch (UnsatisfiedLinkError e) {
            System.out.print("Failed to loadLibrary %s, will try from classpath");
        }

        String osName = System.getProperty("os.name");
        String osArch = System.getProperty("os.arch");

        if (osName.contains("Mac OS X")) {
            osName = "Darwin";
        }

        final String libName = System.mapLibraryName(NATIVE_LIB);
        final String libPath = String.format("NATIVE/%s/%s/%s", osName, osArch, libName);

        // open a temporary file for the native_lib
        final String tmpDir = System.getProperty("java.io.tmpdir");

        if (tmpDir == null) {
            throw new RuntimeException("java.io.tmpdir is null?");
        }

        final File libFile = new File(tmpDir, libName);

        try (
                OutputStream os = new FileOutputStream(libFile, false);
                InputStream in = ClassLoader.getSystemResourceAsStream(libPath);
        ) {
            if (in == null) throw new RuntimeException(String.format("could not find %s in classpath", libPath));
            long length = in.transferTo(os);
            System.out.printf("Created temporary library sized %d at %s%n", length, libFile);
            os.flush();
        } catch (Exception e) {
            System.err.printf("Failed to write %s to %s library: %s%n", libPath, libFile, e.getMessage());
            throw new RuntimeException(e);
        }

        final String path = libFile.getAbsolutePath();
        try {
            // This actually loads the shared object that we'll be creating.
            // The actual location of the .so or .dll may differ based on your
            // platform.
            System.load(path);
            System.out.printf("load succeeded for %s%n", path);
            libraryLoaded = true;
        } catch (UnsatisfiedLinkError e) {
            System.out.printf("Failed to load %s%n", path);
        }
    }

    public Wasmtime() throws WasmtimeException {
        try {
            loadNative();
        } catch (Exception e) {
            throw new WasmtimeException("Failed to load native library for Wasmtime", e);
        }
    }

    private static native long newWasmEngineNtv();

    public WasmEngine newWasmEngine() {
        return new WasmEngine(newWasmEngineNtv());
    }
}
