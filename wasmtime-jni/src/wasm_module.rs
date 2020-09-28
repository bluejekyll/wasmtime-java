use jni::objects::JClass;
use jni::sys::jlong;
use jni::JNIEnv;
use wasmtime::{Engine, Module, Store};

use crate::opaque_ptr;

/// /*
///  * Class:     net_bluejekyll_wasmtime_WasmModule
///  * Method:    freeModule
///  * Signature: (J)V
///  */
///  JNIEXPORT void JNICALL Java_net_bluejekyll_wasmtime_WasmModule_freeModule
///  (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmModule_freeModule(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    unsafe {
        let module = opaque_ptr::box_from_jlong::<Module>(ptr);
        drop(module);
    }
}
