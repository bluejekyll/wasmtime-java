use std::borrow::Cow;

use jni::objects::{JClass, JString};
use jni::sys::jlong;
use jni::JNIEnv;
use log::debug;
use wasmtime::{Instance, Store};

use crate::opaque_ptr::OpaquePtr;
use crate::wasm_exception;
use crate::wasm_state::JavaState;

/// /*
///  * Class:     net_bluejekyll_wasmtime_WasmInstance
///  * Method:    freeInstance
///  * Signature: (J)V
///  */
///  JNIEXPORT void JNICALL Java_net_bluejekyll_wasmtime_WasmInstance_freeInstance
///  (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmInstance_freeInstance<'j>(
    _env: JNIEnv<'j>,
    _class: JClass<'j>,
    instance: OpaquePtr<'j, Instance>,
) {
    drop(instance.take());
}

/// /*
///  * Class:     net_bluejekyll_wasmtime_WasmInstance
///  * Method:    getFunctionNtv
///  * Signature: (JJLjava/lang/String;)J
///  */
///  JNIEXPORT jlong JNICALL Java_net_bluejekyll_wasmtime_WasmInstance_getFunctionNtv
///  (JNIEnv *, jclass, jlong, jlong, jstring);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmInstance_getFunctionNtv<'j>(
    env: JNIEnv<'j>,
    _class: JClass<'j>,
    instance: OpaquePtr<'j, Instance>,
    mut store: OpaquePtr<'j, Store<JavaState>>,
    name: JString<'j>,
) -> jlong {
    wasm_exception::attempt(&env, |env| {
        let name = env.get_string(name)?;
        let name: Cow<str> = Cow::from(&name);

        let func = instance.get_func(&mut *store, &name);

        let func_ptr = if let Some(func) = func {
            debug!(
                "found function in WASM: {}:{:?}",
                name,
                func.ty(&mut *store)
            );
            let func = OpaquePtr::from(func);
            func.make_opaque()
        } else {
            0
        };

        Ok(func_ptr)
    })
}
