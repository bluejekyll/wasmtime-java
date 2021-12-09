use std::borrow::Cow;

use jni::objects::{JClass, JString};
use jni::sys::jlong;
use jni::JNIEnv;
use wasmtime::{Func, Linker, Module, Store};

use crate::opaque_ptr::OpaquePtr;
use crate::wasm_exception;
use crate::wasm_state::JavaState;

/// /*
/// * Class:     net_bluejekyll_wasmtime_WasmLinker
/// * Method:    freeLinker
/// * Signature: (J)V
/// */
/// JNIEXPORT void JNICALL Java_net_bluejekyll_wasmtime_WasmLinker_freeLinker
/// (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmLinker_freeLinker<'j>(
    _env: JNIEnv<'j>,
    _class: JClass<'j>,
    ptr: OpaquePtr<'j, Linker<JavaState>>,
) {
    drop(ptr.take());
}

/// /*
/// * Class:     net_bluejekyll_wasmtime_WasmLinker
/// * Method:    defineFunc
/// * Signature: (JLjava/lang/String;Ljava/lang/String;J)V
/// */
/// JNIEXPORT void JNICALL Java_net_bluejekyll_wasmtime_WasmLinker_defineFunc
///  (JNIaEnv *, jclass, jlong, jstring, jstring, jlong);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmLinker_defineFunc<'j>(
    env: JNIEnv<'j>,
    _class: JClass<'j>,
    mut linker: OpaquePtr<'j, Linker<JavaState>>,
    module: JString<'j>,
    name: JString<'j>,
    func: OpaquePtr<'j, Func>,
) {
    wasm_exception::attempt(&env, |env| {
        let module = env.get_string(module)?;
        let name = env.get_string(name)?;

        let module: Cow<str> = Cow::from(&module);
        let name: Cow<str> = Cow::from(&name);

        let func = func.clone();
        linker.define(&module, &name, func)?;
        Ok(())
    })
}

/// /*
/// * Class:     net_bluejekyll_wasmtime_WasmLinker
/// * Method:    instantiateNtv
/// * Signature: (JJJ)J
/// */
/// JNIEXPORT jlong JNICALL Java_net_bluejekyll_wasmtime_WasmLinker_instantiateNtv
///  (JNIEnv *, jclass, jlong, jlong, jlong);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmLinker_instantiateNtv<'j>(
    env: JNIEnv<'j>,
    _class: JClass<'j>,
    linker: OpaquePtr<'j, Linker<JavaState>>,
    mut store: OpaquePtr<'j, Store<JavaState>>,
    module: OpaquePtr<'j, Module>,
) -> jlong {
    wasm_exception::attempt(&env, |_env| {
        // // TODO: Security considerations here, we don't want to capture the parent processes env
        // //  we probably also want custom filehandles for the stdio of the module as well...
        // let wasi_ctx = WasiCtxBuilder::new().inherit_env()?.inherit_stdio().build();

        // sync::add_to_linker(&mut linker, wasi_ctx)?;

        let instance = linker.instantiate(&mut *store, &module)?;
        Ok(OpaquePtr::from(instance).make_opaque())
    })
}
