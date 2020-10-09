package net.bluejekyll.wasmtime;

import java.lang.reflect.Method;
import java.nio.ByteBuffer;
import java.util.Optional;
import java.util.function.Function;

import org.junit.Assert;
import org.junit.Test;

import static org.junit.Assert.*;

/**
 * Unit test for simple App.
 */
public class WasmtimeTest {
    @Test
    public void testWasmtimeLibraryLoads() throws Exception {
        new Wasmtime();
    }
}
