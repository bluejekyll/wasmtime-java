use jni::objects::JClass;
use jni::JNIEnv;
use wasmtime::Module;

use crate::opaque_ptr::OpaquePtr;

/// /*
///  * Class:     net_bluejekyll_wasmtime_WasmModule
///  * Method:    freeModule
///  * Signature: (J)V
///  */
///  JNIEXPORT void JNICALL Java_net_bluejekyll_wasmtime_WasmModule_freeModule
///  (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmModule_freeModule<'j>(
    _env: JNIEnv<'j>,
    _class: JClass<'j>,
    module: OpaquePtr<'j, Module>,
) {
    drop(module.take());
}
