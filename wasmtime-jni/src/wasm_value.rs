use std::borrow::Cow;

use anyhow::{anyhow, Context, Error};
use jni::objects::{JClass, JObject, JString, JValue};
use jni::JNIEnv;
use wasmtime::{Val, ValType};

const CLASS: &str = "Ljava/lang/Class;";
const LONG: &str = "java/lang/Long";
const INTEGER: &str = "java/lang/Integer";
const DOUBLE: &str = "java/lang/Double";
const FLOAT: &str = "java/lang/Float";
const VOID: &str = "java/lang/Void";
const PRIMITIVE: &str = "TYPE";

pub fn get_class_name<'j>(env: &'j JNIEnv<'j>, clazz: JClass<'j>) -> Result<String, Error> {
    let name = env.call_method(clazz, "getCanonicalName", "()Ljava/lang/String;", &[])?;
    let name = name.l()?;
    let name = JString::from(name);
    let name = env.get_string(name)?;
    Ok(Cow::from(&name).to_string())
}

pub fn from_java_class<'j>(env: &JNIEnv<'j>, clazz: JClass<'j>) -> Result<Option<ValType>, Error> {
    if clazz.is_null() {
        return Ok(None);
    }

    let longp: JClass = env.get_static_field(LONG, PRIMITIVE, CLASS)?.l()?.into();
    let intp: JClass = env.get_static_field(INTEGER, PRIMITIVE, CLASS)?.l()?.into();
    let doublep: JClass = env.get_static_field(DOUBLE, PRIMITIVE, CLASS)?.l()?.into();
    let floatp: JClass = env.get_static_field(FLOAT, PRIMITIVE, CLASS)?.l()?.into();
    let voidp: JClass = env.get_static_field(VOID, PRIMITIVE, CLASS)?.l()?.into();

    let ty = match clazz {
        _ if env.is_assignable_from(clazz, LONG)? => ValType::I64,
        _ if env.is_assignable_from(clazz, longp)? => ValType::I64,
        _ if env.is_assignable_from(clazz, INTEGER)? => ValType::I32,
        _ if env.is_assignable_from(clazz, intp)? => ValType::I32,
        _ if env.is_assignable_from(clazz, DOUBLE)? => ValType::F64,
        _ if env.is_assignable_from(clazz, doublep)? => ValType::F64,
        _ if env.is_assignable_from(clazz, FLOAT)? => ValType::F32,
        _ if env.is_assignable_from(clazz, floatp)? => ValType::F32,
        _ if env.is_assignable_from(clazz, VOID)? => return Ok(None),
        _ if env.is_assignable_from(clazz, voidp)? => return Ok(None),
        _ => {
            let name = get_class_name(env, clazz)?;
            return Err(anyhow!("Unsupported Java type: {}", name));
        }
    };

    Ok(Some(ty))
}

pub fn from_java<'j>(env: &JNIEnv<'j>, obj: JObject<'j>) -> Result<Val, Error> {
    match obj {
        _ if env.is_instance_of(obj, LONG)? => {
            let jvalue = env.call_method(obj, "longValue", "()J", &[])?;
            Ok(Val::I64(jvalue.j()?))
        }
        _ if env.is_instance_of(obj, INTEGER)? => {
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
            let name = get_class_name(env, clazz)?;
            Err(anyhow!("Unsupported Java type: {}", name))
        }
    }
}

pub fn from_jvalue<'j>(env: &JNIEnv<'j>, val: JValue) -> Result<Val, Error> {
    let val = match val {
        JValue::Object(obj) => return from_java(env, obj),
        JValue::Long(v) => Val::I64(v),
        JValue::Int(v) => Val::I32(v),
        JValue::Double(v) => Val::F64(f64::to_bits(v)),
        JValue::Float(v) => Val::F32(f32::to_bits(v)),
        _ => return Err(anyhow!("Unsuppored return type: {}", val.type_name())),
    };

    Ok(val)
}

pub fn to_java<'j, 'w: 'j>(env: &JNIEnv<'j>, val: &'w Val) -> Result<JObject<'j>, Error> {
    let obj = match val {
        Val::I64(val) => {
            let jvalue = JValue::Long(*val);
            env.new_object(LONG, "(J)V", &[jvalue])
        }
        Val::I32(val) => {
            let jvalue = JValue::Int(*val);
            env.new_object(INTEGER, "(I)V", &[jvalue])
        }
        Val::F64(val) => {
            let jvalue = JValue::Double(f64::from_bits(*val));
            env.new_object(INTEGER, "(D)V", &[jvalue])
        }
        Val::F32(val) => {
            let jvalue = JValue::Float(f32::from_bits(*val));
            env.new_object(INTEGER, "(F)V", &[jvalue])
        }
        _ => return Err(anyhow!("Unsupported WASM type: {}", val.ty())),
    };

    obj.with_context(|| format!("failed to convert {:?} to java", val))
}
