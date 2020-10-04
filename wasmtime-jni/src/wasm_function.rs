use std::convert::TryFrom;
use std::sync::Arc;

use anyhow::anyhow;
use jni::objects::{JByteBuffer, JClass, JList, JMethodID, JObject, JValue};
use jni::signature::JavaType;
use jni::sys::{jbyteArray, jclass, jlong, jobject, jobjectArray};
use jni::JNIEnv;
use log::{debug, warn};
use wasmtime::{Caller, Engine, Func, FuncType, Module, Store, Trap, Val};

use crate::opaque_ptr::OpaquePtr;
use crate::wasm_exception;
use crate::wasm_value;

/// /*
/// * Class:     net_bluejekyll_wasmtime_WasmFunction
/// * Method:    createFunc
///  * Signature: (JLjava/lang/reflect/Method;Ljava/lang/Object;Lnet/bluejekyll/wasmtime/types/WasmType;Ljava/util/List;)J
/// */
/// JNIEXPORT jlong JNICALL Java_net_bluejekyll_wasmtime_WasmFunction_createFunc
/// (JNIEnv *, jclass, jlong, jobject, jobject, jobject);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmFunction_createFunc<'j>(
    env: JNIEnv<'j>,
    _class: JClass<'j>,
    store: OpaquePtr<'j, Store>,
    method: JObject<'j>,
    obj: JObject<'j>,
    return_ty: JObject<'j>,
    param_tys: JObject<'j>,
) -> jlong {
    let method = env
        .new_global_ref(method)
        .expect("could not create GlobalRef");
    let obj = env.new_global_ref(obj).expect("could not create GlobalRef");
    let jvm = env.get_java_vm().expect("No JVM?");

    let func = move |_caller: Caller, inputs: &[Val], outputs: &mut [Val]| -> Result<(), Trap> {
        let env = jvm
            .get_env()
            .map_err(|e| format!("Error accessing JNIEnv in WASM context: {}", e))
            .map_err(|e| Trap::new(e))?;

        let method_id: JMethodID = match env.get_method_id(
            "java/lang/reflect/Method",
            "invoke",
            "(Ljava/lang/Object;[Ljava/lang/Object;)Ljava/lang/Object;",
        ) {
            Err(err) => {
                let warning = format!("Error accessing byte buffer: {}", err);
                warn!("{}", warning);
                return Err(Trap::new(warning));
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

        let ret = JavaType::Object(String::from("java/lang/Object"));
        let val = env
            .call_method_unchecked(
                method.as_obj(),
                method_id,
                ret,
                &[JValue::Object(obj.as_obj()), JValue::Object(method_args)],
            )
            .expect("Cat ate my homework!");

        // FIXME: check for exception
        // FIXME: handle exception by converting to error
        // FIXME: return Trap error in case of exception

        Ok(())
    };

    let func = Func::new(&store, FuncType::new(Box::new([]), Box::new([])), func);
    OpaquePtr::from(func).make_opaque()
}

/// /*
///  * Class:     net_bluejekyll_wasmtime_WasmFunction
///  * Method:    freeFunc
///  * Signature: (J)V
///  */
///  JNIEXPORT void JNICALL Java_net_bluejekyll_wasmtime_WasmFunction_freeFunc
///  (JNIEnv *, jclass, jlong);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmFunction_freeFunc<'j>(
    _env: JNIEnv<'j>,
    _class: JClass<'j>,
    func: OpaquePtr<'j, Func>,
) {
    drop(func.take());
}

/// /*
/// * Class:     net_bluejekyll_wasmtime_WasmFunction
/// * Method:    callNtv
/// * Signature: (J[Ljava/lang/Object;)Ljava/lang/Object;
/// */
/// JNIEXPORT jobject JNICALL Java_net_bluejekyll_wasmtime_WasmFunction_callNtv
/// (JNIEnv *, jclass, jlong, jobjectArray);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmFunction_callNtv<'j>(
    env: JNIEnv<'j>,
    _class: JClass<'j>,
    func: OpaquePtr<'j, Func>,
    args: jobjectArray,
) -> jobject {
    wasm_exception::attempt_or_else(
        &env,
        || JObject::null().into_inner(),
        |env| {
            let len = env.get_array_length(args)?;
            let len = usize::try_from(len)?;
            let mut wasm_args = Vec::with_capacity(len);

            for i in 0..len {
                let obj = env.get_object_array_element(args, i32::try_from(i)?)?;
                let val = wasm_value::from_java(&env, obj)?;

                wasm_args.push(val);
            }

            // we need to convert all the parameters to WASM vals for the call
            let val = func.call(&wasm_args)?;

            if val.len() > 1 {
                return Err(anyhow!(
                    "multiple return values not supported, expected 0 or 1 found: {}",
                    val.len()
                ));
            }

            let ret = match val.first() {
                Some(val) => wasm_value::to_java(&env, val)?,
                None => JObject::null(),
            };

            // Giving the return result to java
            Ok(ret.into_inner())
        },
    )
}
