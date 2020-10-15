use std::borrow::Cow;

use jni::objects::{JClass, JString};
use jni::sys::jlong;
use jni::JNIEnv;
use wasmtime::{Func, Linker, Module};
use wasmtime_wasi::{Wasi, WasiCtx};

use crate::opaque_ptr::OpaquePtr;
use crate::wasm_exception;

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
    ptr: OpaquePtr<'j, Linker>,
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
    mut linker: OpaquePtr<'j, Linker>,
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
/// * Signature: (JJ)J
/// */
/// JNIEXPORT jlong JNICALL Java_net_bluejekyll_wasmtime_WasmLinker_instantiateNtv
///  (JNIEnv *, jclass, jlong, jlong);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmLinker_instantiateNtv<'j>(
    env: JNIEnv<'j>,
    _class: JClass<'j>,
    mut linker: OpaquePtr<'j, Linker>,
    module: OpaquePtr<'j, Module>,
) -> jlong {
    wasm_exception::attempt(&env, |_env| {
        let store = linker.store();
        let wasi = Wasi::new(&store, WasiCtx::new(std::env::args())?);
        wasi.add_to_linker(&mut linker)?;

        let instance = linker.instantiate(&module)?;
        Ok(OpaquePtr::from(instance).make_opaque())
    })
}
