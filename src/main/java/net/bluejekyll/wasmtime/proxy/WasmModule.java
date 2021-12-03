package net.bluejekyll.wasmtime.proxy;

import java.lang.annotation.ElementType;
import java.lang.annotation.Retention;
import java.lang.annotation.RetentionPolicy;
import java.lang.annotation.Target;

/**
 * Use this to indicate the module of class or interface being abstracted over
 * Webassembly for Java
 */
@Retention(RetentionPolicy.RUNTIME)
@Target(ElementType.TYPE)
public @interface WasmModule {
    /**
     * Name in the Webassembly ABI.
     * 
     * This is unnecessary, and ignored, for import proxies.
     */
    public String name() default "";
}
