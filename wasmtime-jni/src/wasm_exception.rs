use std::borrow::Cow;
use std::fmt;

use anyhow::{anyhow, Context, Error};
use jni::objects::{JObject, JString, JThrowable};
use jni::strings::JavaStr;
use jni::sys::jarray;
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

pub fn exception_to_err<'j>(env: &JNIEnv<'j>, throwable: JThrowable<'j>) -> Error {
    let reporter = ReportJThrowable { env, throwable };

    warn!("WASM caught an exception: {}", reporter);
    anyhow!("WASM caught an exception: {}", reporter)
}

struct ReportJThrowable<'l, 'j: 'l> {
    env: &'l JNIEnv<'j>,
    throwable: JThrowable<'j>,
}

impl<'l, 'j: 'l> ReportJThrowable<'l, 'j> {
    fn from(env: &'l JNIEnv<'j>, throwable: JThrowable<'j>) -> Self {
        Self { env, throwable }
    }
}

impl<'l, 'j: 'l> fmt::Display for ReportJThrowable<'l, 'j> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        // just in case...
        if self.throwable.is_null() {
            write!(f, "null throwable thrown")?;
            return Ok(());
        }

        let cause = self
            .env
            .call_method(self.throwable, "getCause", "()Ljava/lang/Throwable;", &[])
            .map_err(|_| fmt::Error)?;

        let cause = cause.l().map_err(|_| fmt::Error)?;
        if !cause.is_null() {
            let reporter = ReportJThrowable::from(&self.env, JThrowable::from(cause));
            write!(f, "cause: {}", reporter)?;
        }

        let clazz = wasm_value::get_class_name_obj(self.env, self.throwable.clone().into())
            .map_err(|_| fmt::Error)?;

        let message =
            call_string_method(self.env, &self.throwable, "getMessage").map_err(|_| fmt::Error)?;

        if let Some(message) = message {
            writeln!(f, "{}: {}", clazz, Cow::from(&message))?;
        } else {
            writeln!(f, "{}", clazz)?;
        };

        let trace = self
            .env
            .call_method(
                self.throwable,
                "getStackTrace",
                "()[Ljava/lang/StackTraceElement;",
                &[],
            )
            .map_err(|_| fmt::Error)?
            .l()
            .map_err(|_| fmt::Error)?;

        if !trace.is_null() {
            let trace = *trace as jarray;
            let len = self.env.get_array_length(trace).map_err(|_| fmt::Error)?;

            for i in 0..len as usize {
                let stack_element = self
                    .env
                    .get_object_array_element(trace, i as i32)
                    .map_err(|_| fmt::Error)?;

                let stack_str = call_string_method(self.env, &stack_element, "toString")
                    .map_err(|_| fmt::Error)?;

                if let Some(stack_str) = stack_str {
                    writeln!(f, "\t{}", Cow::from(&stack_str))?;
                }
            }
        }

        Ok(())
    }
}

fn call_string_method<'j: 'l, 'l>(
    env: &'l JNIEnv<'j>,
    obj: &JObject<'j>,
    method: &str,
) -> Result<Option<JavaStr<'j, 'l>>, anyhow::Error> {
    let jstring = env
        .call_method(*obj, method, "()Ljava/lang/String;", &[])
        .map_err(|_| fmt::Error)?
        .l()
        .map_err(|_| fmt::Error)
        .map(JString::from)?;

    if jstring.is_null() {
        return Ok(None);
    }

    env.get_string(jstring)
        .with_context(|| format!("Failed to get_string for Exception: {:?}", obj))
        .map(Some)
}
