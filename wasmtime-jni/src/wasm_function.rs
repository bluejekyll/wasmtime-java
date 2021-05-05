use std::borrow::Cow;
use std::convert::TryFrom;
use std::slice;

use anyhow::{anyhow, Context, Error};
use jni::objects::{JClass, JMethodID, JObject, JValue, ReleaseMode};
use jni::signature::JavaType;
use jni::sys::{jlong, jobject, jobjectArray};
use jni::JNIEnv;
use log::debug;
use log::warn;
use wasmtime::{Caller, Func, FuncType, Instance, Store, Trap, Val, ValType};

use crate::opaque_ptr::OpaquePtr;
use crate::ty::{WasmAlloc, WasmSlice};
use crate::wasm_exception;
use crate::wasm_value::{self, WasmTy, WasmVal};

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

        // collect all the arguments from
        let param_list = env.get_list(param_tys)?;
        let mut wasm_args: Vec<ValType> = Vec::with_capacity(param_list.size()? as usize);
        let mut java_args: Vec<WasmTy> = Vec::with_capacity(wasm_args.len());

        for class in param_list.iter()? {
            // this is a list of classes
            let val = wasm_value::from_java_class(&env, class.into(), false)
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

        // determine the return type
        let java_ret = wasm_value::from_java_class(&env, return_ty, true)
            .context("error converting type to wasm")?;
        debug!(
            "Mapping return value from {:?} to {:?}",
            wasm_value::get_class_name(&env, return_ty)?,
            java_ret
        );

        let wasm_ret = if let Some(java_ret) = &java_ret {
            java_ret.return_or_push_arg_tys(&mut wasm_args)
        } else {
            None
        };

        let wasm_ret: Vec<ValType> = wasm_ret.map_or_else(|| vec![], |v| vec![v]);

        let func = move |caller: Caller, inputs: &[Val], outputs: &mut [Val]| -> Result<(), Trap> {
            let java_args = &java_args;
            let java_ret = &java_ret;
            let wasm_alloc = WasmAlloc::from_caller(&caller);

            debug!(
                "Calling Java method args {} and return {} with WASM {} inputs and {} outputs",
                java_args.len(),
                java_ret.as_ref().as_ref().map_or(0, |_| 1),
                inputs.len(),
                outputs.len()
            );

            // validate the parameters
            let mut input_ty_iter = inputs.iter().map(|v| v.ty());
            for java_arg in java_args.iter() {
                java_arg
                    .matches_arg_tys(&mut input_ty_iter)
                    .with_context(|| {
                        format!("Expected arguments to line up with java arg: {}", java_arg)
                    })?;
            }

            // TODO: this validation fails, b/c the ty from WASM is ExternRef... not sure why?
            // validate the return
            // if let Some(java_ret) = java_ret {
            //     java_ret
            //         .matches_return_or_arg_tys(outputs.get(0).map(Val::ty), &mut input_ty_iter)?;
            // }

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
                        .load_from_args(&env, &mut input_iter, wasm_alloc.as_ref())
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

            // get the optional pointer to the arg to store a byte return by ref
            let ret_by_ref_ptr = java_ret
                .as_ref()
                .and_then(|v| v.get_return_by_ref_arg(input_iter));

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

            // TODO: much of this logic is duplicitive with that in wasm_value::WasmVal::store_to_args
            match (val, result) {
                (Some(WasmVal::Val(val)), Some(result)) => {
                    debug!("associating {:?} with result", val);
                    *result = val;
                }
                (Some(WasmVal::ByteBuffer(val)), None) => {
                    debug!("allocating space and associating bytes for return by ref");
                    let ptr = ret_by_ref_ptr
                        .ok_or_else(|| anyhow!("expected return by ref argument pointer"))?;
                    let wasm_alloc = wasm_alloc.ok_or_else(|| anyhow!("WasmAlloc is required"))?;

                    let bytes = env
                        .get_direct_buffer_address(val)
                        .context("could not get bytes from address")?;

                    // get mutable reference to the return by ref pointer and then store
                    unsafe {
                        let bytes = wasm_alloc.alloc_bytes(bytes)?;
                        let ret_by_ref_loc = wasm_alloc.obj_as_mut::<WasmSlice>(ptr);
                        *ret_by_ref_loc = bytes.wasm_slice();
                    }
                }
                (Some(WasmVal::ByteArray { jarray, .. }), None) => {
                    debug!("allocating space and associating bytes for return by ref");
                    let ptr = ret_by_ref_ptr
                        .ok_or_else(|| anyhow!("expected return by ref argument pointer"))?;
                    let wasm_alloc = wasm_alloc.ok_or_else(|| anyhow!("WasmAlloc is required"))?;

                    let len = env
                        .get_array_length(jarray)
                        .context("failed to get Java array length")?;
                    let jbytes = env
                        .get_byte_array_elements(jarray, ReleaseMode::NoCopyBack)
                        .context("failed to get java array elements")?;
                    let byte_array: &[u8] = unsafe {
                        slice::from_raw_parts(jbytes.as_ptr() as *const u8, len as usize)
                    };

                    let bytes = wasm_alloc.alloc_bytes(byte_array)?;

                    // get mutable reference to the return by ref pointer and then store
                    unsafe {
                        let ret_by_ref_loc = wasm_alloc.obj_as_mut::<WasmSlice>(ptr);
                        *ret_by_ref_loc = bytes.wasm_slice();
                    }
                }
                (Some(WasmVal::String(string)), None) => {
                    debug!("allocating space and associating string for return by ref");
                    let ptr = ret_by_ref_ptr
                        .ok_or_else(|| anyhow!("expected return by ref argument pointer"))?;
                    let wasm_alloc = wasm_alloc.ok_or_else(|| anyhow!("WasmAlloc is required"))?;

                    let jstr = env
                        .get_string(string)
                        .context("failed to get Java String")?;
                    let cow = Cow::from(&jstr);
                    let cow_bytes = cow.as_bytes();

                    // the module might not have the memory exported
                    let wasm_slice = wasm_alloc.alloc_bytes(cow_bytes)?;

                    // get mutable reference to the return by ref pointer and then store
                    unsafe {
                        let ret_by_ref_loc = wasm_alloc.obj_as_mut::<WasmSlice>(ptr);
                        *ret_by_ref_loc = wasm_slice.wasm_slice();
                    }
                }
                (Some(WasmVal::ByteBuffer(_val)), Some(_result)) => {
                    return Err(anyhow!(
                        "Unexpected WASM return value, should have been return by reference"
                    )
                    .into());
                }
                (Some(WasmVal::ByteArray { .. }), Some(_result)) => {
                    return Err(anyhow!(
                        "Unexpected WASM return value, should have been return by reference"
                    )
                    .into());
                }
                (Some(WasmVal::String(_)), Some(_result)) => {
                    return Err(anyhow!(
                        "Unexpected WASM return value, should have been return by reference"
                    )
                    .into());
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

        let func = Func::new(&store, FuncType::new(wasm_args, wasm_ret), func);

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
///  * Class:     net_bluejekyll_wasmtime_WasmFunction
///  * Method:    callNtv
///  * Signature: (JJLjava/lang/Class;[Ljava/lang/Object;)Ljava/lang/Object;
///  */
///  JNIEXPORT jobject JNICALL Java_net_bluejekyll_wasmtime_WasmFunction_callNtv
///  (JNIEnv *, jclass, jlong, jlong, jclass, jobjectArray);
#[no_mangle]
pub extern "system" fn Java_net_bluejekyll_wasmtime_WasmFunction_callNtv<'j>(
    env: JNIEnv<'j>,
    _class: JClass<'j>,
    func: OpaquePtr<'j, Func>,
    instance: OpaquePtr<'j, Instance>,
    return_type: JClass<'j>,
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

            let wasm_alloc = if !instance.is_null() {
                WasmAlloc::from_instance(&instance)
            } else {
                None
            };

            // let droppers will cleanup allocated memory in the WASM module after the function call
            //   or should the callee drop?? hmm...
            let mut wasm_droppers = Vec::with_capacity(len);

            // we need to convert all the parameters to WASM vals for the call
            debug!("got {} args for function", len);
            for i in 0..len {
                let obj = env
                    .get_object_array_element(args, i32::try_from(i)?)
                    .with_context(|| format!("could not get array index: {} len: {}", i, len))?;

                let val = wasm_value::from_java(env, obj)
                    .with_context(|| format!("failed to convert argument at index: {}", i))?;

                debug!("adding arg: {}", val.ty());
                if let Some(dropper) =
                    val.store_to_args(env, &mut wasm_args, wasm_alloc.as_ref())?
                {
                    wasm_droppers.push(dropper);
                }
            }

            // now we may need to add a return_by_ref parameter
            let wasm_return_ty = wasm_value::from_java_class(&env, return_type, true)?;
            debug!("return ty: {:?}", wasm_return_ty);

            let maybe_ret_by_ref = if let Some(wasm_return_ty) = &wasm_return_ty {
                wasm_return_ty
                    .clone()
                    .return_or_store_to_arg(&mut wasm_args, wasm_alloc.as_ref())?
            } else {
                None
            };

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

            let ret = if let Some(wasm_return_ty) = wasm_return_ty {
                unsafe {
                    wasm_value::return_or_load_or_from_arg(
                        env,
                        wasm_return_ty,
                        val.get(0),
                        maybe_ret_by_ref,
                        wasm_alloc.as_ref(),
                    )?
                }
            } else {
                JObject::null()
            };

            // Giving the return result to java
            Ok(ret.into_inner())
        },
    )
}
