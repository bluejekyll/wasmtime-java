use std::convert::TryFrom;

use anyhow::{anyhow, Context, Error};
use jni::objects::{JClass, JMethodID, JObject, JValue};
use jni::signature::JavaType;
use jni::sys::{jlong, jobject, jobjectArray};
use jni::JNIEnv;
use log::debug;
use log::warn;
use wasmtime::{Caller, Func, FuncType, Store, Trap, Val, ValType};

use crate::opaque_ptr::OpaquePtr;
use crate::wasm_exception;
use crate::wasm_value;

/// /*
/// * Class:     net_bluejekyll_wasmtime_WasmFunction
/// * Method:    createFunc
/// * Signature: (JLjava/lang/reflect/Method;Ljava/lang/Object;Ljava/lang/Class;Ljava/util/List;)J
/// */
/// JNIEXPORT jlong JNICALL Java_net_bluejekyll_wasmtime_WasmFunction_createFunc
///  (JNIEnv *, jclass, jlong, jobject, jobject, jclass, jobject);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmFunction_createFunc<'j>(
    env: JNIEnv<'j>,
    _class: JClass<'j>,
    store: OpaquePtr<'j, Store>,
    method: JObject<'j>,
    obj: JObject<'j>,
    return_ty: JClass<'j>,
    param_tys: JObject<'j>,
) -> jlong {
    wasm_exception::attempt(&env, |env| {
        let method = env.new_global_ref(method)?;
        let obj = env.new_global_ref(obj)?;
        let jvm = env.get_java_vm()?;

        let func =
            move |_caller: Caller, inputs: &[Val], outputs: &mut [Val]| -> Result<(), Trap> {
                debug!(
                    "calling java method with {} inputs and {} outputs",
                    inputs.len(),
                    outputs.len()
                );

                let env = jvm
                    .get_env()
                    .map_err(Error::from)
                    .context("Error accessing JNIEnv in WASM")?;

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

                // build up parameters
                let arg_class = env
                    .find_class("java/lang/Object")
                    .map_err(Error::from)
                    .context("Could not find Object?")?;

                let method_args = env
                    .new_object_array(inputs.len() as i32, arg_class, JObject::null())
                    .map_err(Error::from)
                    .context("Could not create empty array?")?;

                // set the parameters
                for (i, val) in inputs.into_iter().enumerate() {
                    let jvalue = wasm_value::to_java(&env, val)?;
                    debug!("Setting parameter {}: {:?} as {:?}", i, val, jvalue);

                    env.set_object_array_element(method_args, i as i32, jvalue)
                        .map_err(Error::from)
                        .context("Failed to add value to array?")?;
                }

                let method_args = JObject::from(method_args);

                let ret = JavaType::Object(String::from("java/lang/Object"));
                let val = env
                    .call_method_unchecked(
                        method.as_obj(),
                        method_id,
                        ret,
                        &[JValue::Object(obj.as_obj()), JValue::Object(method_args)],
                    )
                    .map_err(Error::from)
                    .context("Call to Java method failed!")?;

                let val = wasm_value::from_jvalue(&env, val)?;
                if let Some(v) = outputs.get_mut(0) {
                    *v = val;
                }

                // FIXME: check for exception
                // FIXME: handle exception by converting to error
                // FIXME: return Trap error in case of exception

                Ok(())
            };

        let ret = wasm_value::from_java_class(&env, return_ty.into())
            .context("error converting type to wasm")?;
        debug!(
            "Mapping return value from {:?} to {:?}",
            wasm_value::get_class_name(env, return_ty)?,
            ret
        );

        let ret: Box<[ValType]> =
            ret.map_or_else(|| Box::new([]) as Box<_>, |v| Box::new([v]) as Box<_>);

        let param_list = env.get_list(param_tys)?;

        let mut wasm_args = Vec::with_capacity(param_list.size()? as usize);

        for class in param_list.iter()? {
            // this is a list of classes
            let val = wasm_value::from_java_class(&env, class.into())
                .context("error converting type to wasm")?;
            let val = val.ok_or_else(|| anyhow!("Null parameters not allowed"))?;
            debug!(
                "Mapping parameter from {:?} to {:?}",
                wasm_value::get_class_name(env, return_ty)?,
                ret
            );

            wasm_args.push(val);
        }

        let func = Func::new(
            &store,
            FuncType::new(wasm_args.into_boxed_slice(), ret),
            func,
        );

        Ok(OpaquePtr::from(func).make_opaque())
    })
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

            debug!("got {} args for function", len);
            for i in 0..len {
                let obj = env
                    .get_object_array_element(args, i32::try_from(i)?)
                    .with_context(|| format!("could not get array index: {} len: {}", i, len))?;
                let val = wasm_value::from_java(&env, obj)
                    .with_context(|| format!("failed to convert value at index: {}", i))?;

                debug!("adding arg: {}", val.ty());

                wasm_args.push(val);
            }

            // we need to convert all the parameters to WASM vals for the call
            let val = func
                .call(&wasm_args)
                .with_context(|| format!("failed to execute wasm function: {:?}", *func))?;

            if val.len() > 1 {
                return Err(anyhow!(
                    "multiple return values not supported, expected 0 or 1 found: {}",
                    val.len()
                ));
            }

            let ret = match val.first() {
                Some(val) => wasm_value::to_java(&env, val).with_context(|| {
                    format!("failed to convert WASM return value to Java: {}", val.ty())
                })?,
                None => JObject::null(),
            };

            // Giving the return result to java
            Ok(ret.into_inner())
        },
    )
}
