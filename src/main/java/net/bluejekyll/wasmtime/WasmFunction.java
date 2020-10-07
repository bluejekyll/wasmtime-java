package net.bluejekyll.wasmtime;

import net.bluejekyll.wasmtime.types.WasmType;

import javax.annotation.concurrent.NotThreadSafe;
import java.lang.reflect.Method;
import java.lang.reflect.Parameter;
import java.util.ArrayList;
import java.util.List;

@NotThreadSafe
public class WasmFunction extends AbstractOpaquePtr {
    WasmFunction(long ptr) {
        super(ptr, WasmFunction::freeFunc);
    }

    private static native void freeFunc(long ptr);

    private static native long createFunc(long store_ptr, Method method, Object obj, Class<?> returnType,
            List<Class<?>> paramTypes) throws WasmtimeException;

    private static native Object callNtv(long func_ptr, Object... args) throws WasmtimeException;

    public static WasmFunction newFunc(WasmStore store, Method method, Object obj) throws WasmtimeException {
        List<Class<?>> parameters = new ArrayList<>(5);
        for (Parameter param : method.getParameters()) {
            Class<?> ty = param.getType();

            // this validates that it's a type we support... maybe just remove this and do
            // it in Rust.
            // WasmType wty = WasmType.fromClass(ty);

            // // void params, there are no params
            // if (wty == null)
            // break;
            System.out.println("ty class: " + ty.getCanonicalName());
            parameters.add(ty);
        }

        // validate that the type is something we support
        Class<?> returnValue = method.getReturnType();
        System.out.println("return class: " + returnValue.getCanonicalName());

        // WasmType.fromClass(returnValue);

        long ptr = createFunc(store.getPtr(), method, obj, returnValue, parameters);
        return new WasmFunction(ptr);
    }

    @SuppressWarnings("unchecked")
    public <T> T call(Object... args) throws WasmtimeException {
        return (T) callNtv(this.getPtr(), args);
    }
}
