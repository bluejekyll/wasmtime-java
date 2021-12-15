package net.bluejekyll.wasmtime;

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

    private static native Object callNtv(long func_ptr, long instance_pointer, long store_ptr, Class<?> returnType,
            Object... args)
            throws WasmtimeException;

    public static WasmFunction newFunc(WasmStore store, Object target, String methodName, Class<?>... args)
            throws WasmtimeException, NoSuchMethodException {
        Method method = target.getClass().getMethod(methodName, args);
        return WasmFunction.newFunc(store, method, target);
    }

    public static WasmFunction newFunc(WasmStore store, Method method, Object obj) throws WasmtimeException {
        List<Class<?>> parameters = new ArrayList<>(5);
        for (Parameter param : method.getParameters()) {
            Class<?> ty = param.getType();

            System.out.println("ty class: " + ty.getCanonicalName());
            parameters.add(ty);
        }

        // validate that the type is something we support
        Class<?> returnType = method.getReturnType();
        System.out.println("return class: " + returnType.getCanonicalName());

        long ptr = createFunc(store.getPtr(), method, obj, returnType, parameters);
        return new WasmFunction(ptr);
    }

    /**
     * 
     * @param instance   the linked and compiled instance to call this function
     *                   agains
     * @param returnType the class of the return type, Void for no return
     * @param args       list of arguments for the function, must match those of the
     *                   "wrapped" function
     * @param <T>        return type matching the wrapped functions return type
     * @return If there is a return value for the function, otherwise Void
     * @throws WasmtimeException If any exception is thrown byt the underlying
     *                           function
     */
    @SuppressWarnings("unchecked")
    public <T> T call(WasmInstance instance, WasmStore store, Class<T> returnType, Object... args)
            throws WasmtimeException {
        return (T) callNtv(this.getPtr(), instance.getPtr(), store.getPtr(), returnType, args);
    }

    /**
     *
     * @param instance the linked and compiled instance to call this function agains
     * @param args     list of arguments for the function, must match those of the
     *                 "wrapped" function
     * @return If there is a return value for the function, otherwise Void
     * @throws WasmtimeException If any exception is thrown byt the underlying
     *                           function
     */
    @SuppressWarnings("unchecked")
    public void call(WasmInstance instance, WasmStore store, Object... args) throws WasmtimeException {
        callNtv(this.getPtr(), instance.getPtr(), store.getPtr(), Void.class, args);
    }

    /**
     * WARNING: this is really only useful in tests, Instance will be null in the
     * native call, which is bad for any non-native types, like Strings arrays or
     * ByteBuffers.
     */
    @SuppressWarnings("unchecked")
    <T> T call_for_tests(WasmStore store, Class<T> returnType, Object... args) throws WasmtimeException {
        return (T) callNtv(this.getPtr(), 0, store.getPtr(), returnType, args);
    }

    /**
     * WARNING: this is really only useful in tests, Instance will be null in the
     * native call, which is bad for any non-native types, like Strings arrays or
     * ByteBuffers.
     */
    void call_for_tests(WasmStore store, Object... args) throws WasmtimeException {
        callNtv(this.getPtr(), 0, store.getPtr(), Void.class, args);
    }
}
