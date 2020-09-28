use jni::objects::JClass;
use jni::sys::jlong;
use jni::JNIEnv;
use wasmtime::Store;

use crate::opaque_ptr;

/// /*
///  * Class:     net_bluejekyll_wasmtime_WasmStore
///  * Method:    freeStore
///  * Signature: (J)V
///  */
///  JNIEXPORT void JNICALL Java_net_bluejekyll_wasmtime_WasmStore_freeStore
///  (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmStore_freeStore(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    unsafe {
        let store = opaque_ptr::box_from_jlong::<Store>(ptr);
        drop(store);
    }
}
