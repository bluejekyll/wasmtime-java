use jni::objects::JClass;
use jni::sys::jlong;
use jni::JNIEnv;
use wasmtime::Store;

use crate::opaque_ptr::OpaquePtr;

// TODO: consider requiring a background thread per store?

/// /*
///  * Class:     net_bluejekyll_wasmtime_WasmStore
///  * Method:    freeStore
///  * Signature: (J)V
///  */
///  JNIEXPORT void JNICALL Java_net_bluejekyll_wasmtime_WasmStore_freeStore
///  (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmStore_freeStore<'j>(
    _env: JNIEnv<'j>,
    _class: JClass<'j>,
    store: OpaquePtr<'j, Store>,
) {
    drop(store.take());
}
