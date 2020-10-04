use std::fmt::Display;

use anyhow::Error;
use jni::objects::{JByteBuffer, JClass};
use jni::sys::{jbyteArray, jlong, jobject};
use jni::JNIEnv;
use log::{debug, warn};
use wasmtime::{Engine, Module, Store};

#[track_caller]
pub fn attempt<R, F>(env: &JNIEnv, f: F) -> R
where
    R: Default,
    F: FnOnce(&JNIEnv) -> Result<R, Error>,
{
    attempt_or_else(env, || R::default(), f)
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
            warn!("Error accessing byte buffer: {}", err);
            env.throw_new("net/bluejekyll/wasmtime/WasmtimeException", err.to_string())
                .expect("failed to throw exception");
            or()
        }
    }
}
