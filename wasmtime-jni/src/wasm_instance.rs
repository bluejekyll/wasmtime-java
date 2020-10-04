use std::sync::Arc;

use jni::objects::{JByteBuffer, JClass, JList, JMethodID, JObject, JValue};
use jni::signature::JavaType;
use jni::sys::{jbyteArray, jclass, jlong, jobject, jobjectArray};
use jni::JNIEnv;
use log::{debug, warn};
use wasmtime::{Caller, Engine, Func, FuncType, Instance, Module, Store, Trap, Val};

use crate::opaque_ptr;

/// /*
///  * Class:     net_bluejekyll_wasmtime_WasmInstance
///  * Method:    freeInstance
///  * Signature: (J)V
///  */
///  JNIEXPORT void JNICALL Java_net_bluejekyll_wasmtime_WasmInstance_freeInstance
///  (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmInstance_freeInstance<'j>(
    env: JNIEnv<'j>,
    _class: JClass<'j>,
    ptr: jlong,
) {
    unsafe {
        drop(opaque_ptr::box_from_jlong::<Instance>(ptr));
    }
}
