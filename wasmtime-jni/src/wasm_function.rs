use std::borrow::Cow;
use std::convert::TryFrom;

use anyhow::{anyhow, Context, Error};
use jni::objects::{JClass, JMethodID, JObject, JString, JValue};
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

                // get the invoke method on the method, the result is always Object
                let ret = JavaType::Object(String::from("java/lang/Object"));
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

                // setup the arguments for the call
                let method_args = JObject::from(method_args);

                let val = env
                    .call_method_unchecked(
                        method.as_obj(),
                        method_id,
                        ret,
                        &[JValue::Object(obj.as_obj()), JValue::Object(method_args)],
                    )
                    .map_err(Error::from)
                    .context("Call to Java method failed!")?;

                // Check if Java threw an exception.
                if env
                    .exception_check()
                    .context("Failed to check for exception")?
                {
                    let exception = env
                        .exception_occurred()
                        .context("Failed to get exception")?;

                    let clazz = env
                        .get_object_class(exception)
                        .context("Failed to get exceptions class")?;
                    let clazz = wasm_value::get_class_name(&env, clazz)
                        .context("Failed to lookup class name")?;

                    // TODO: get entire thread
                    let message = env
                        .call_method(exception, "getMessage", "()Ljava/lang/String", &[])
                        .context("Gailed to getMessage on Throwable")?;

                    let message = message
                        .l()
                        .context("Expected a String Object from Throwable.getMessage")?;

                    let err = if !message.is_null() {
                        let message = JString::from(message);
                        let message = env.get_string(message).with_context(|| {
                            format!("Failed to get_string for Exception: {}", clazz)
                        })?;
                        let message = Cow::from(&message);

                        warn!("Method call threw an exception: {}: {}", clazz, message);
                        anyhow!("Method call threw an exception: {}: {}", clazz, message)
                    } else {
                        warn!("Method call threw an exception: {}", clazz);
                        anyhow!("Method call threw an exception: {}", clazz)
                    };

                    // clear the exception
                    env.exception_clear().context("Failed to clear exception")?;
                    return Err(err.into());
                }

                // Now get the return value
                let val = wasm_value::from_jvalue(&env, val)?;
                let result = outputs.get_mut(0);

                match (val, result) {
                    (Some(val), Some(result)) => {
                        debug!("associating {:?} with result", val);
                        *result = val;
                    }
                    (None, Some(result)) => {
                        debug!("associating null with result");
                        *result = Val::null();
                    }
                    (Some(val), None) => {
                        warn!("WASM expected no result, but Java supplied: {:?}", val);
                    }
                    (None, None) => {
                        debug!("returning no result");
                    }
                }

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
