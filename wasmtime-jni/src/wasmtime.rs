use std::ffi::c_void;

//use env_logger::{self, Target};
use flexi_logger::{opt_format, Logger};
use jni::objects::JClass;
use jni::sys::{jint, jlong, JavaVM, JNI_VERSION_1_8};
use jni::JNIEnv;
use log::info;
use wasmtime::Engine;

use crate::opaque_ptr::OpaquePtr;

/// Optional function defined by dynamically linked libraries. The VM calls JNI_OnLoad when the native library is loaded (for example, through System.loadLibrary).
///
/// In order to make use of functions defined at a certain version of the JNI API, JNI_OnLoad must return a constant defining at least that version. For example, libraries wishing to use AttachCurrentThreadAsDaemon function introduced in JDK 1.4, need to return at least JNI_VERSION_1_4. If the native library does not export a JNI_OnLoad function, the VM assumes that the library only requires JNI version JNI_VERSION_1_1. If the VM does not recognize the version number returned by JNI_OnLoad, the VM will unload the library and act as if the library was never loaded.
/// LINKAGE:
///
/// Exported from dynamically linked native libraries that contain native method implementations.
/// PARAMETERS:
///
/// vm: a pointer to the current VM structure.
///
/// reserved: unused pointer.
/// RETURNS:
///
/// Return the required JNI_VERSION constant (see also GetVersion).
/// jint JNI_OnLoad(JavaVM *vm, void *reserved);
#[no_mangle]
pub extern "system" fn JNI_OnLoad(_vm: JavaVM, _reserved: *mut c_void) -> jint {
    Logger::with_env_or_str("wasmtime=info,wasmtime-jni=info")
        .log_to_file()
        .directory("target/wasm-logs")
        .format(opt_format)
        .start()
        .ok();

    // env_logger::builder().target(Target::Stdout).init();

    info!("wasmtime JNI loaded");
    JNI_VERSION_1_8
}

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
    info!("wasmtime-jni: getting engine");

    // Box it...
    let engine = Engine::default();
    OpaquePtr::from(engine).make_opaque()
}
