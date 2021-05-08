package net.bluejekyll.wasmtime.proxy;

import java.lang.reflect.InvocationHandler;
import java.lang.reflect.InvocationTargetException;
import java.lang.reflect.Method;
import java.lang.reflect.Proxy;
import java.util.HashMap;
import java.util.Map;
import java.util.Optional;

import net.bluejekyll.wasmtime.WasmFunction;
import net.bluejekyll.wasmtime.WasmInstance;
import net.bluejekyll.wasmtime.WasmtimeException;

public class WasmImportProxy {
    private final WasmInstance instance;

    public WasmImportProxy(WasmInstance instance) {
        this.instance = instance;
    }

    private static class WasmInvocationHandler implements InvocationHandler {
        final WasmInstance instance;
        final Map<String, WasmFunction> functions;

        WasmInvocationHandler(WasmInstance instance, Map<String, WasmFunction> functions) {
            this.instance = instance;
            this.functions = functions;
        }

        @Override
        public Object invoke(Object proxy, Method method, Object[] args) throws Throwable {
            WasmFunction function = functions.get(method.getName());

            if (function != null) {
                return function.call(this.instance, method.getReturnType(), args);
            }

            // TODO: add this back with Java 1.16
            // if (method.isDefault()) {
            // return InvocationHandler.invokeDefault​(proxy, method, args);
            // }

            throw new RuntimeException(String.format("Method is not defined for WASM module: %s", method.getName()));
        }

    }

    public <T extends WasmImportable> T newWasmProxy(Class<T> proxyClass)
            throws IllegalArgumentException, WasmtimeException {
        final Method[] methods = proxyClass.getMethods();

        final HashMap<String, WasmFunction> functions = new HashMap<>();

        for (Method method : methods) {
            final WasmImport wasmImport = method.getAnnotation(WasmImport.class);
            final String methodName = method.getName();

            // we will only support annotated methods
            if (wasmImport == null)
                continue;

            String functionName = wasmImport.name();
            if (functionName.isEmpty()) {
                functionName = methodName;
            }

            Optional<WasmFunction> func = this.instance.getFunction(functionName);

            if (!func.isPresent()) {
                throw new WasmtimeException(String.format("Function not present in WASM Module: %s", functionName));
            }

            // we use the Java method name here because that's what will be passed into the
            // invocation handler.
            WasmFunction existing = functions.get(methodName);
            if (existing != null) {
                throw new WasmtimeException(
                        String.format("Function %s already has method %s bound", functionName, methodName));
            }

            functions.put(methodName, func.get());
        }

        WasmInvocationHandler invocationHandler = new WasmInvocationHandler(this.instance, functions);
        return (T) Proxy.newProxyInstance(proxyClass.getClassLoader(), new Class<?>[] { proxyClass },
                invocationHandler);
    }
}
