package net.bluejekyll.wasmtime;

import static org.junit.Assert.assertTrue;

import java.lang.reflect.Method;
import java.util.Optional;

import org.junit.Test;

public class WasmLinkerTest {
    public final void helloWorld() {
        System.out.println("Hello World");
    }

    @Test
    public void testLinking() throws Exception {
        String call_hello_world = "(module\n" + " (import \"hello\" \"world\" (func $host_hello))\n"
                + " (func (export \"hello\")\n" + " call $host_hello)\n" + " )";

        Wasmtime wasm = new Wasmtime();
        try (WasmEngine engine = wasm.newWasmEngine();
                WasmStore store = engine.newStore();
                WasmLinker linker = store.newLinker();) {

            // define the Java hello world function
            Method method = this.getClass().getMethod("helloWorld");
            WasmFunction func = WasmFunction.newFunc(store, method, this);

            // add it to the linker
            linker.defineFunction("hello", "world", func);

            // compile the calling module and then link it
            WasmModule module = engine.newModule(call_hello_world.getBytes());
            WasmInstance instance = linker.instantiate(module);
            Optional<WasmFunction> function = instance.getFunction("hello");

            assertTrue(function.isPresent());

            function.ifPresent(f -> {
                try {
                    f.call(new Object[0]);
                } catch (Exception e) {
                    throw new RuntimeException(e);
                }
            });
        }
    }
}
