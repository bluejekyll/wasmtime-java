use std::convert::TryFrom;
use std::sync::Arc;

use anyhow::{anyhow, Context, Error};
use jni::objects::{JClass, JMethodID, JObject, JValue};
use jni::signature::JavaType;
use jni::sys::{jlong, jobject, jobjectArray};
use jni::JNIEnv;
use log::debug;
use log::warn;
use wasmtime::{Caller, Extern, Func, FuncType, Instance, Store, Trap, Val, ValType};

use crate::wasm_exception;
use crate::wasm_value::{self, WasmVal};
use crate::{opaque_ptr::OpaquePtr, wasm_value::WasmTy};

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
    // Read this to help understand String and array types https://github.com/rustwasm/wasm-bindgen/blob/d54340e5a220953651555f45f90061499dc0ac92/guide/src/contributing/design/exporting-rust.md

    wasm_exception::attempt(&env, |env| {
        let method = env.new_global_ref(method)?;
        let obj = env.new_global_ref(obj)?;
        let jvm = env.get_java_vm()?;

        //
        // determine the return type
        let java_ret = wasm_value::from_java_class(&env, return_ty)
            .context("error converting type to wasm")?;
        debug!(
            "Mapping return value from {:?} to {:?}",
            wasm_value::get_class_name(&env, return_ty)?,
            java_ret
        );

        let ret: Box<[ValType]> = java_ret.clone().map_or_else(
            || Box::new([]) as Box<_>,
            |v| {
                if let WasmTy::ValType(v) = v {
                    Box::new([v]) as Box<_>
                } else {
                    unimplemented!("need to implement return type conversion for slices")
                }
            },
        );

        // collect all the arguments from
        let param_list = env.get_list(param_tys)?;
        let mut wasm_args: Vec<ValType> = Vec::with_capacity(param_list.size()? as usize);
        let mut java_args: Vec<WasmTy> = Vec::with_capacity(wasm_args.len());

        for class in param_list.iter()? {
            // this is a list of classes
            let val = wasm_value::from_java_class(&env, class.into())
                .context("error converting type to wasm")?;
            let val = val.ok_or_else(|| anyhow!("Null parameters not allowed"))?;
            debug!(
                "Mapping parameter from {:?} to {:?}",
                wasm_value::get_class_name(env, class.into())?,
                val
            );

            val.push_arg_tys(&mut wasm_args);
            java_args.push(val);
        }

        // need shared refs for the func to the args
        let java_args = Arc::new(java_args);
        let java_ret = Arc::new(java_ret);

        let func = move |caller: Caller, inputs: &[Val], outputs: &mut [Val]| -> Result<(), Trap> {
            let java_args = &java_args;
            let java_ret = &java_ret;
            let memory = caller.get_export("memory").and_then(Extern::into_memory);

            debug!(
                "Calling Java method args {} and return {} with WASM {} inputs and {} outputs",
                java_args.len(),
                java_ret.as_ref().as_ref().map_or(0, |_| 1),
                inputs.len(),
                outputs.len()
            );

            let mut input_ty_iter = inputs.iter().map(|v| v.ty());
            for java_arg in java_args.iter() {
                java_arg
                    .matches_arg_tys(&mut input_ty_iter)
                    .with_context(|| {
                        format!("Expected arguments to line up with java arg: {}", java_arg)
                    })?;
            }

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
                .new_object_array(java_args.len() as i32, arg_class, JObject::null())
                .map_err(Error::from)
                .context("Could not create empty array?")?;

            // set the parameters
            let mut input_iter = inputs.iter().cloned();
            for (i, java_arg) in java_args.iter().enumerate() {
                let jvalue = unsafe {
                    java_arg
                        .load_from_args(&env, &mut input_iter, memory.as_ref())
                        .with_context(|| format!("Failed to get Java arg from: {}", java_arg))?
                };

                debug!(
                    "Setting parameter {}: {:?} as {:?}",
                    i,
                    java_arg,
                    wasm_value::get_class_name_obj(&env, jvalue)
                );

                env.set_object_array_element(method_args, i as i32, jvalue)
                    .map_err(Error::from)
                    .context("Failed to add value to array?")?;
            }

            debug!("Calling Java method");

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
                .context("Call to Java method failed!");

            // Check if Java threw an exception.
            if val.is_err()
                && env
                    .exception_check()
                    .context("Failed to check for exception")?
            {
                // get the exception
                let exception = env
                    .exception_occurred()
                    .context("Failed to get exception")?;
                // clear the exception so that we can make additional java calls
                env.exception_clear().context("Failed to clear exception")?;

                let err = wasm_exception::exception_to_err(&env, exception);
                return Err(err.into());
            }

            // unwrap the exception
            let val = val?;

            // Now get the return value
            let val = wasm_value::from_jvalue(&env, val)?;
            let result = outputs.get_mut(0);

            match (val, result) {
                (Some(WasmVal::Val(val)), Some(result)) => {
                    debug!("associating {:?} with result", val);
                    *result = val;
                }
                (Some(WasmVal::ByteBuffer(_val)), Some(_result)) => {
                    unimplemented!("need to implement return value translation for WasmTy");
                    //debug!("associating {:?} with result", val);
                    //*result = val;
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
/// * Signature: (JJ[Ljava/lang/Object;)Ljava/lang/Object;
/// */
/// JNIEXPORT jobject JNICALL Java_net_bluejekyll_wasmtime_WasmFunction_callNtv
/// (JNIEnv *, jclass, jlong, jlong, jobjectArray);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmFunction_callNtv<'j>(
    env: JNIEnv<'j>,
    _class: JClass<'j>,
    func: OpaquePtr<'j, Func>,
    instance: OpaquePtr<'j, Instance>,
    args: jobjectArray,
) -> jobject {
    // Read this to help understand String and array types https://github.com/rustwasm/wasm-bindgen/blob/d54340e5a220953651555f45f90061499dc0ac92/guide/src/contributing/design/exporting-rust.md

    wasm_exception::attempt_or_else(
        &env,
        || JObject::null().into_inner(),
        |env| {
            let len = env.get_array_length(args)?;
            let len = usize::try_from(len)?;
            let mut wasm_args = Vec::with_capacity(len);

            let (memory, allocator) = if instance.is_null() {
                (None, None)
            } else {
                (
                    instance.get_memory("memory"),
                    instance.get_func("__alloc_bytes"),
                )
            };

            // we need to convert all the parameters to WASM vals for the call
            debug!("got {} args for function", len);
            for i in 0..len {
                let obj = env
                    .get_object_array_element(args, i32::try_from(i)?)
                    .with_context(|| format!("could not get array index: {} len: {}", i, len))?;

                let val = wasm_value::from_java(env, obj)
                    .with_context(|| format!("failed to convert value at index: {}", i))?;

                debug!("adding arg: {}", val.ty());

                val.store_to_args(env, &mut wasm_args, memory.as_ref(), allocator.as_ref())?;
            }

            // call the function
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
