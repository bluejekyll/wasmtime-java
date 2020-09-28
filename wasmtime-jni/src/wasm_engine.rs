use jni::objects::JClass;
use jni::sys::{jbyteArray, jlong};
use jni::JNIEnv;
use log::debug;
use wasmtime::{Engine, Module, Store};

use crate::opaque_ptr;

/// /*
///  * Class:     net_bluejekyll_wasmtime_WasmEngine
///  * Method:    freeEngine
///  * Signature: (J)V
///  */
///  JNIEXPORT void JNICALL Java_net_bluejekyll_wasmtime_WasmEngine_freeEngine
///  (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmEngine_freeEngine(
    _env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    unsafe {
        let engine = opaque_ptr::box_from_jlong::<Engine>(ptr);
        drop(engine);
    }
}

/// /*
///  * Class:     net_bluejekyll_wasmtime_WasmEngine
///  * Method:    newStoreNtv
///  * Signature: ()J
///  */
///  JNIEXPORT jlong JNICALL Java_net_bluejekyll_wasmtime_WasmEngine_newStoreNtv
///  (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmEngine_newStoreNtv(
    env: JNIEnv,
    _class: JClass,
    engine_ptr: jlong,
) -> jlong {
    let engine: &Engine = unsafe { opaque_ptr::ref_from_jlong(&env, engine_ptr) };
    let store = Store::new(engine);
    opaque_ptr::to_jlong(store)
}

/// /*
///  * Class:     net_bluejekyll_wasmtime_WasmEngine
///  * Method:    newModuleNtv
///  * Signature: (J[B)J
///  */
///  JNIEXPORT jlong JNICALL Java_net_bluejekyll_wasmtime_WasmEngine_newModuleNtv
///  (JNIEnv *, jclass, jlong, jbyteArray);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmEngine_newModuleNtv(
    env: JNIEnv,
    _class: JClass,
    engine_ptr: jlong,
    wat: jbyteArray,
) -> jlong {
    let engine: &Engine = unsafe { opaque_ptr::ref_from_jlong(&env, engine_ptr) };

    debug!("getting wasm bytes");
    let wat_bytes: Vec<u8> = match env.convert_byte_array(wat) {
        Err(err) => {
            env.throw_new("net/bluejekyll/wasmtime/WasmtimeException", err.to_string())
                .expect("failed to throw exception");
            return 0;
        }
        Ok(ok) => ok,
    };

    debug!("compiling wasm module");
    let module = match Module::new(engine, wat_bytes) {
        Err(err) => {
            env.throw_new("net/bluejekyll/wasmtime/WasmtimeException", err.to_string())
                .expect("failed to throw exception");
            return 0;
        }
        Ok(ok) => ok,
    };

    opaque_ptr::to_jlong(module)
}
