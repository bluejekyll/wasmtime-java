package net.bluejekyll.wasmtime.proxy;

import java.lang.annotation.Annotation;
import java.lang.reflect.Method;
import java.util.ArrayList;
import java.util.List;

import net.bluejekyll.wasmtime.WasmFunction;
import net.bluejekyll.wasmtime.WasmStore;
import net.bluejekyll.wasmtime.WasmtimeException;

public interface WasmExportable {
    public default List<WasmFunctionDef> defineWasmFunctions(WasmStore store) throws WasmtimeException {
        Class<?> clazz = this.getClass();

        // name will default to the class name if none is offered
        String moduleName = "";
        WasmModule module = clazz.getAnnotation(WasmModule.class);
        if (module != null) {
            moduleName = module.name();
        }

        if (moduleName.equals("")) {
            // TODO: convert to WASM convention
            moduleName = clazz.getName();
        }

        // list the methods annotated with WasmExport
        Method[] methods = clazz.getMethods();
        ArrayList<WasmFunctionDef> functions = new ArrayList<>(methods.length);

        for (Method method : methods) {
            WasmExport export = method.getAnnotation(WasmExport.class);

            // if there was no annotation, we will skip...
            if (export == null)
                continue;

            String exportName = export.name();
            if (exportName.equals("")) {
                exportName = method.getName();
            }

            // get parameters...
            WasmFunction function = WasmFunction.newFunc(store, method, this);
            functions.add(new WasmFunctionDef(moduleName, exportName, function));
        }

        functions.trimToSize();
        return functions;
    }
}
