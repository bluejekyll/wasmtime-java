use flexi_logger::{opt_format, Logger};
use jni::objects::JClass;
use jni::sys::jlong;
use jni::JNIEnv;
use log::info;
use wasmtime::Engine;

use crate::opaque_ptr::OpaquePtr;

/// /*
///  * Class:     net_bluejekyll_wasmtime_Wasmtime
///  * Method:    newWasmEngineNtv
///  * Signature: ()J
///  */
///  JNIEXPORT jlong JNICALL Java_net_bluejekyll_wasmtime_Wasmtime_newWasmEngineNtv
///  (JNIEnv *, jclass);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_Wasmtime_newWasmEngineNtv(
    _env: JNIEnv,
    _input: JClass,
) -> jlong {
    Logger::with_env_or_str("wasmtime=info,wasmtime-jni=info")
        .log_to_file()
        .directory("target")
        .format(opt_format)
        .start()
        .ok();

    info!("wasmtime-jni: getting engine");

    // Box it...
    let engine = Engine::default();
    OpaquePtr::from(engine).make_opaque()
}
