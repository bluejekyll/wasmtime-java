use jni::objects::{JByteBuffer, JClass, JList, JMethodID, JObject, JValue};
use jni::signature::JavaType;
use jni::sys::{jbyteArray, jclass, jlong, jobject, jobjectArray};
use jni::JNIEnv;
use log::{debug, warn};
use wasmtime::{Caller, Engine, Func, FuncType, Module, Store, Trap, Val};

use crate::opaque_ptr;

/// /*
/// * Class:     net_bluejekyll_wasmtime_WasmFunction
/// * Method:    createFunc
///  * Signature: (JLjava/lang/reflect/Method;Ljava/lang/Object;Lnet/bluejekyll/wasmtime/types/WasmType;Ljava/util/List;)J
/// */
/// JNIEXPORT jlong JNICALL Java_net_bluejekyll_wasmtime_WasmFunction_createFunc
/// (JNIEnv *, jclass, jlong, jobject, jobject, jobject);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmFunction_createFunc(
    env: JNIEnv<'static>,
    _class: JClass,
    store_ptr: jlong,
    method: JObject<'static>,
    obj: JObject<'static>,
    return_ty: JObject<'static>,
    param_tys: JObject<'static>,
) -> jlong {
    let store: &Store = unsafe { opaque_ptr::ref_from_jlong(&env, store_ptr) };

    let method_id: JMethodID = match env.get_method_id(
        "java/lang/reflect/Method",
        "invoke",
        "(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;",
    ) {
        Err(err) => {
            warn!("Error accessing byte buffer: {}", err);
            env.throw_new("net/bluejekyll/wasmtime/WasmtimeException", err.to_string())
                .expect("failed to throw exception");
            return 0;
        }
        Ok(ok) => ok,
    };

    let arg_class = env
        .find_class("java/lang/Object")
        .expect("there is always an Object");
    let method_args = env
        .new_object_array(0, arg_class, JObject::null())
        .expect("could not create empty array");
    let method_args = JObject::from(method_args);

    let func = move |_caller: Caller, inputs: &[Val], outputs: &mut [Val]| -> Result<(), Trap> {
        let ret = JavaType::Object(String::from("java/lang/Object"));
        let val = env
            .call_method_unchecked(
                method,
                method_id,
                ret,
                &[JValue::Object(obj), JValue::Object(method_args)],
            )
            .expect("Cat ate my homework!");

        // FIXME: check for exception
        // FIXME: handle exception by converting to error
        // FIXME: return Trap error in case of exception

        Ok(())
    };

    let func = Func::new(store, FuncType::new(Box::new([]), Box::new([])), func);

    opaque_ptr::to_jlong(func)
}

/// /*
///  * Class:     net_bluejekyll_wasmtime_WasmFunction
///  * Method:    freeFunc
///  * Signature: (J)V
///  */
///  JNIEXPORT void JNICALL Java_net_bluejekyll_wasmtime_WasmFunction_freeFunc
///  (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmFunction_freeFunc(
    env: JNIEnv,
    _class: JClass,
    ptr: jlong,
) {
    unsafe {
        drop(opaque_ptr::box_from_jlong::<Func>(ptr));
    }
}
