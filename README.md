[![test](https://github.com/bluejekyll/wasmtime-java/workflows/test/badge.svg?branch=main)](https://github.com/bluejekyll/wasmtime-java/actions?query=workflow%3Atest)

# Java bindings to the Wasmtime runtime engine

WASM, web-assembly, is a low-level stack execution engine. This library gives a JNI based binding to the Wasmtime WASM runtime.

For more information, please see [github.com/bytecodealliance/wasmtime](https://github.com/bytecodealliance/wasmtime).

This is very early stages. All interfaces are subject to change. Supported platforms, x86_64 for Linux and macOS. Windows should also be supported, but currently not due to lack of resources.

## Building

Use the provided Makefile. You need Java 11 (not 17), Maven and Rustup (the error messages will prompt you to install that if you don't have it).

Ensure tools are installed:

```shell
> make init
```

Run tests:

```shell
> make test
```

Install the maven packages locally:

```shell
> make install
```

## Using in Java

Maven co-ordinates for the installed artifacts: `net.bluejekyll:wasmtime-java:1.0-SNAPSHOT`.

### Initializing the Wasmtime runtime

Initialize the WASM runtime. Much of this is not threadsafe, so be aware...

```java
// Initiale the Wasmtime JNI bindings (this will happen once per ClassLoader)
Wasmtime wasmtime = new Wasmtime();
// Get a new Engine (one per thread)
WasmEngine engine = wasmtime.newWasmEngine();
// Compile The module, this can be reused, i.e. is cacheable for multiple executions
WasmModule module = engine.newModule(${PATH_TO_MODULE});
```

Once the runtime is initialized, create a new instance. This requires a linker, and exported functions if the module has imports that need to be met.

```java
// create a new store and linker
WasmStore store = engine.newStore();
WasmLinker linker = store.newLinker();

// This will link in exports from Java to the WASM module, if that module has imports, see below
linker.defineFunctions(this.store, new TestExport());
```

### Exporting functions to Webassembly

To export functions into WASM, that are bound to the WASM module's imports, a class must implement the `WasmExportable` interface. Each method to export and be bound to functions the corresponding function in WASM must be annotated with `WasmExport`.

See [`TestExport`](https://github.com/bluejekyll/wasmtime-java/blob/fd075fe88ba106409d10a1c985f8573f2d7936e2/src/test/java/net/bluejekyll/wasmtime/proxy/TestExport.java) for example:

```java
// Specify the module name that we need to export functions into (imports in the WASM)
@WasmModule(name = "test")
public class TestExport implements WasmExportable {

    // Export the function to WASM and give the name that corresponds to the import in WASM
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
```

After Wasmtime is initiated and the module compiled, then it can be linked and an instance created:

```java
linker.defineFunctions(this.store, new TestExport());
WasmInstance instance = linker.instantiate(module);
```

### Importing functions implemented in Webassembly

`WasmImportProxy` To work with the Proxy builder, an `interface` must extend the `WasmImportable` interface. It should be annotated to specify the WASM module being bound to.

See [`TestImportProxy`](https://github.com/bluejekyll/wasmtime-java/blob/fd075fe88ba106409d10a1c985f8573f2d7936e2/src/test/java/net/bluejekyll/wasmtime/proxy/TestImportProxy.java) for example:


```java
// This annotation allows for the WASM module name to be specified
@WasmModule(name = "test")
public interface TestImportProxy extends WasmImportable {
    // This annotation allows for the function name to be set as it is defined in WASM
    @WasmImport(name = "add_i32")
    int addInteger(int a, int b);
}
```

This can now be used with `WasmImportProxy` to create a proxy that will call into the specified functions in WASM, see [`ImportTests`](https://github.com/bluejekyll/wasmtime-java/blob/fd075fe88ba106409d10a1c985f8573f2d7936e2/src/test/java/net/bluejekyll/wasmtime/proxy/ImportTests.java):

```java
WasmInstance instance = linker.instantiate(this.module);
WasmImportProxy importProxy = new WasmImportProxy(instance);
TestImportProxy proxy = importProxy.newWasmProxy(TestImportProxy.class);

int ret = proxy.addInteger(3, 2);
```

## Structure

The Java is meant to be as minimal as possible. All Wasmtime object references are stored in Java objects as opaque pointers (longs). The safety in this area has not yet been proven. Thread safety in particular is non-existent. The WasmEngine should be safe to share across threads, though we should most likely introduce a custom clone method for this purpose.

### Adding new native methods

The Java compiler will automatically output the headers for the JNI bindings based on the native methods defined in the various classes. While the headers generated by the Java compiler aren't directly used during the Rust JNI compilation, they are useful for seeing the C signature that the Rust needs to export. The files can be found in `target/generated-sources/*.h`.

## Debugging

The tests should all run regardless of platform. Windows hasn't been fully tested due to lack of resources. If tests fail to run, there are a few different environments at play which will make discovering which component failed and why difficult. At the moment, all output from Java is captured in `txt` files at in the `target/surfire-reports/{CLASS_NAME}-output.txt`. All output from the JNI bindings is currently captured in `target/wasm-logs/java_{DATE}.log`, this may be combined into the same place in the future. In the `pom.xml` Maven project file, the `surfire` test configuration has `RUST_LOG` set to `debug` by default. This can be set to any other value to increase or decrease logging output.

## libc like support with WASI

[WASI](https://wasi.dev/) is supported for things like printing to stdout. This is supplied during linking in the Java bindings. It is not required, in Rust this can be targeted with `cargo build --target wasm32-wasi`, that target must be installed with `rustup` before hand.
