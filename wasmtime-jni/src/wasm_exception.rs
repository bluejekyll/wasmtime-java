use std::borrow::Cow;

use anyhow::{anyhow, Context, Error};
use jni::objects::{JString, JThrowable};
use jni::JNIEnv;
use log::warn;

use crate::wasm_value;

#[track_caller]
pub fn attempt<R, F>(env: &JNIEnv, f: F) -> R
where
    R: Default,
    F: FnOnce(&JNIEnv) -> Result<R, Error>,
{
    attempt_or_else(env, R::default, f)
}

#[track_caller]
pub fn attempt_or_else<R, F, D>(env: &JNIEnv, or: D, f: F) -> R
where
    D: FnOnce() -> R,
    F: FnOnce(&JNIEnv) -> Result<R, Error>,
{
    let r = f(env);

    match r {
        Ok(ok) => ok,
        Err(err) => {
            let msg = format!("Error in WASM Binding: {:?}", err);
            warn!("{}", msg);
            env.throw_new("net/bluejekyll/wasmtime/WasmtimeException", msg)
                .expect("failed to throw exception");
            or()
        }
    }
}

pub fn exception_to_err<'j>(env: &JNIEnv<'j>, exception: JThrowable<'j>) -> Result<Error, Error> {
    // TODO: if this is an invocation exception, we want to get the value...
    let clazz = env
        .get_object_class(exception)
        .context("Failed to get exceptions class")?;

    let clazz = wasm_value::get_class_name(&env, clazz).context("Failed to lookup class name")?;

    // TODO: get entire thread
    let message = env
        .call_method(exception, "getMessage", "()Ljava/lang/String;", &[])
        .context("Failed to getMessage on Throwable")?;

    let message = message
        .l()
        .context("Expected a String Object from Throwable.getMessage")?;

    let err = if !message.is_null() {
        let message = JString::from(message);
        let message = env
            .get_string(message)
            .with_context(|| format!("Failed to get_string for Exception: {}", clazz))?;
        let message = Cow::from(&message);

        warn!("Method call threw an exception: {}: {}", clazz, message);
        anyhow!("Method call threw an exception: {}: {}", clazz, message)
    } else {
        warn!("Method call threw an exception: {}", clazz);
        anyhow!("Method call threw an exception: {}", clazz)
    };

    Ok(err)
}
