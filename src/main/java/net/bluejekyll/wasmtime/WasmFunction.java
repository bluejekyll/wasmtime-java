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

    private static native long createFunc(long store_ptr, Method method, Object obj, WasmType returnType,
            List<WasmType> paramTypes) throws WasmtimeException;

    private static native Object callNtv(long func_ptr, Object[] args) throws WasmtimeException;

    public static WasmFunction newFunc(WasmStore store, Method method, Object obj) throws WasmtimeException {
        List<WasmType> parameters = new ArrayList<>(5);
        for (Parameter param : method.getParameters()) {
            Class<?> ty = param.getType();

            WasmType wty = WasmType.fromClass(ty);

            // void params, there are no params
            if (wty == null)
                break;
            parameters.add(wty);
        }

        WasmType returnValue = WasmType.fromClass(method.getReturnType());

        long ptr = createFunc(store.getPtr(), method, obj, returnValue, parameters);
        return new WasmFunction(ptr);
    }

    public <T> T call(Object[] args) throws WasmtimeException {
        return (T) callNtv(this.getPtr(), args);
    }
}
