package net.bluejekyll.wasmtime.proxy;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

/**
 * Use to annotate methods that will be exported for use in WebAssembly from
 * Java.
 */
@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.METHOD)
public @interface WasmImport {
    /** Name in the Webassembly ABI */
    public String name() default "";
}
