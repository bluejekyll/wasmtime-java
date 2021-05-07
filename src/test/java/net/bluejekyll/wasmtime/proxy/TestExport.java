package net.bluejekyll.wasmtime.proxy;

import static org.junit.Assert.assertEquals;

import java.io.UnsupportedEncodingException;
import java.nio.ByteBuffer;

@WasmModule(name = "test")
public class TestExport implements WasmExportable {

    @WasmExport(name = "hello_to_java")
    public void helloToJava(ByteBuffer hello_bytes) {
        final String hello = "Hello Java!";

        System.out.printf("Hello length: %d%n", hello_bytes.capacity());

        final byte[] bytes = new byte[hello_bytes.capacity()];
        hello_bytes.get(bytes);

        final String from_wasm;
        try {
            from_wasm = new String(bytes, "UTF-8");
        } catch (UnsupportedEncodingException e) {
            // this should never happen for UTF-8
            throw new RuntimeException(e);
        }

        System.out.printf("Hello: %s%n", from_wasm);
        assertEquals(hello, from_wasm);
    }

    @WasmExport(name = "reverse_bytes_java")
    public final byte[] reverseBytesJava(ByteBuffer buffer) {
        ByteBuffer toReverse = buffer.duplicate();
        byte[] bytes = new byte[toReverse.remaining()];

        for (int i = bytes.length - 1; i >= 0; i--) {
            bytes[i] = toReverse.get();
        }

        return bytes;
    }

}
