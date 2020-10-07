use anyhow::Error;
use jni::JNIEnv;
use log::warn;

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
            warn!("Converting error to exception: {:?}", err);
            env.throw_new("net/bluejekyll/wasmtime/WasmtimeException", err.to_string())
                .expect("failed to throw exception");
            or()
        }
    }
}
