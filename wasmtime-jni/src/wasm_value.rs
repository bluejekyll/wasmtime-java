use std::borrow::Cow;

use anyhow::{anyhow, Error};
use jni::objects::{JByteBuffer, JClass, JList, JMethodID, JObject, JString, JValue};
use jni::signature::JavaType;
use jni::sys::{jbyteArray, jclass, jlong, jobject, jobjectArray};
use jni::JNIEnv;
use log::{debug, warn};
use wasmtime::{Caller, Engine, Func, FuncType, Module, Store, Trap, Val};

const LONG: &str = "java/lang/Long";
const INT: &str = "java/lang/Integer";
const FLOAT: &str = "java/lang/Float";
const DOUBLE: &str = "java/lang/Double";

pub fn from_java<'j>(env: &JNIEnv<'j>, obj: JObject<'j>) -> Result<Val, Error> {
    match obj {
        _ if env.is_instance_of(obj, LONG)? => {
            let jvalue = env.call_method(obj, "longValue", "()J", &[])?;
            Ok(Val::I64(jvalue.j()?))
        }
        _ if env.is_instance_of(obj, INT)? => {
            let jvalue = env.call_method(obj, "intValue", "()I", &[])?;
            Ok(Val::I32(jvalue.i()?))
        }
        _ if env.is_instance_of(obj, DOUBLE)? => {
            let jvalue = env.call_method(obj, "doubleValue", "()D", &[])?;
            Ok(Val::F64(jvalue.d()?.to_bits()))
        }
        _ if env.is_instance_of(obj, FLOAT)? => {
            let jvalue = env.call_method(obj, "floatValue", "()F", &[])?;
            Ok(Val::F32(jvalue.f()?.to_bits()))
        }
        _ => {
            let clazz = env.get_object_class(obj)?;
            let name = env.call_method(obj, "getCanonicalName", "()Ljava/lang/String;", &[])?;
            let name = name.l()?;
            let name = JString::from(name);
            let name = env.get_string(name)?;
            let name: Cow<str> = Cow::from(&name);

            Err(anyhow!("Unsupported Java type: {}", name))
        }
    }
}

pub fn to_java<'j, 'w: 'j>(env: &JNIEnv<'j>, val: &'w Val) -> Result<JObject<'j>, Error> {
    match val {
        Val::I64(val) => {
            let jvalue = JValue::Long(*val);
            let obj = env.new_object(LONG, "(J)", &[jvalue])?;
            Ok(obj)
        }
        Val::I32(val) => {
            let jvalue = JValue::Int(*val);
            let obj = env.new_object(INT, "(I)", &[jvalue])?;
            Ok(obj)
        }
        Val::F64(val) => {
            let jvalue = JValue::Double(f64::from_bits(*val));
            let obj = env.new_object(INT, "(D)", &[jvalue])?;
            Ok(obj)
        }
        Val::F32(val) => {
            let jvalue = JValue::Float(f32::from_bits(*val));
            let obj = env.new_object(INT, "(F)", &[jvalue])?;
            Ok(obj)
        }
        _ => Err(anyhow!("Unsupported WASM type: {}", val.ty())),
    }
}
