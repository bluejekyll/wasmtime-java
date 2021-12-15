package net.bluejekyll.wasmtime.proxy;

import static org.junit.Assert.assertEquals;

import java.io.UnsupportedEncodingException;

@WasmModule(name = "test")
public class TestExport implements WasmExportable {

    @WasmExport(name = "hello_to_java")
    public void helloToJava(byte[] hello_bytes) {
        final String hello = "Hello Java!";

        // System.out.printf("Hello length: %d%n", hello_bytes.length);

        final String from_wasm;
        try {
            from_wasm = new String(hello_bytes, "UTF-8");
        } catch (UnsupportedEncodingException e) {
            // this should never happen for UTF-8
            throw new RuntimeException(e);
        }

        // System.out.printf("Hello: %s%n", from_wasm);
        assertEquals(hello, from_wasm);
    }

    @WasmExport(name = "reverse_bytes_java")
    public final byte[] reverseBytesJava(byte[] buffer) {
        // System.out.printf("reversingBytesJava len: %d%n", buffer.length);

        byte[] bytes = new byte[buffer.length];

        for (int i = bytes.length - 1; i >= 0; i--) {
            bytes[i] = buffer[buffer.length - 1 - i];
        }

        return bytes;
    }

}
