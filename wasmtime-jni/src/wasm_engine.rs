use jni::objects::{JByteBuffer, JClass};
use jni::sys::jlong;
use jni::JNIEnv;
use log::{debug, warn};
use wasmtime::{Engine, Module, Store};

use crate::opaque_ptr::OpaquePtr;

/// /*
///  * Class:     net_bluejekyll_wasmtime_WasmEngine
///  * Method:    freeEngine
///  * Signature: (J)V
///  */
///  JNIEXPORT void JNICALL Java_net_bluejekyll_wasmtime_WasmEngine_freeEngine
///  (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmEngine_freeEngine<'j>(
    _env: JNIEnv<'j>,
    _class: JClass<'j>,
    engine: OpaquePtr<'j, Engine>,
) {
    drop(engine.take());
}

/// /*
///  * Class:     net_bluejekyll_wasmtime_WasmEngine
///  * Method:    newStoreNtv
///  * Signature: ()J
///  */
///  JNIEXPORT jlong JNICALL Java_net_bluejekyll_wasmtime_WasmEngine_newStoreNtv
///  (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmEngine_newStoreNtv<'j>(
    _env: JNIEnv<'j>,
    _class: JClass<'j>,
    engine: OpaquePtr<'j, Engine>,
) -> jlong {
    let store = Store::new(&engine);
    OpaquePtr::from(store).make_opaque()
}

/// /*
///  * Class:     net_bluejekyll_wasmtime_WasmEngine
///  * Method:    newModuleNtv
///  * Signature: (JLjava/nio/ByteBuffer;)J
///  */
///  JNIEXPORT jlong JNICALL Java_net_bluejekyll_wasmtime_WasmEngine_newModuleNtv
///  (JNIEnv *, jclass, jlong, jobject);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmEngine_newModuleNtv(
    env: JNIEnv,
    _class: JClass,
    engine: OpaquePtr<Engine>,
    wat: JByteBuffer,
) -> jlong {
    let wat_bytes = match env.get_direct_buffer_address(wat) {
        Err(err) => {
            warn!("Error accessing byte buffer: {}", err);
            env.throw_new("net/bluejekyll/wasmtime/WasmtimeException", err.to_string())
                .expect("failed to throw exception");
            return 0;
        }
        Ok(ok) => ok,
    };

    debug!("compiling wasm module from bytes: {}", wat_bytes.len());
    let module = match Module::new(&engine, wat_bytes) {
        Err(err) => {
            env.throw_new("net/bluejekyll/wasmtime/WasmtimeException", err.to_string())
                .expect("failed to throw exception");
            return 0;
        }
        Ok(ok) => ok,
    };

    if log::log_enabled!(log::Level::Debug) {
        for import in module.imports() {
            debug!("Import module: {:?}", import);
        }

        for export in module.exports() {
            debug!("Export module: {:?}", export);
        }
    }

    OpaquePtr::from(module).make_opaque()
}
