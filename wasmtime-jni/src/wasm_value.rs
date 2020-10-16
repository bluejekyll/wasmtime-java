use std::borrow::Cow;
use std::fmt;

use anyhow::{anyhow, ensure, Context, Error};
use jni::objects::{JByteBuffer, JClass, JObject, JString, JValue};
use jni::JNIEnv;
use log::debug;
use wasmtime::{Func, Memory, Val, ValType};

use crate::ty::{Abi, ByteSlice, ComplexTy, WasmSlice};

const CLASS: &str = "Ljava/lang/Class;";
const LONG: &str = "java/lang/Long";
const INTEGER: &str = "java/lang/Integer";
const DOUBLE: &str = "java/lang/Double";
const FLOAT: &str = "java/lang/Float";
const VOID: &str = "java/lang/Void";
const PRIMITIVE: &str = "TYPE";

const BYTE_BUFFER: &str = "java/nio/ByteBuffer";
const MEM_SEGMENT_SIZE: usize = 64 * 1024;

pub(crate) fn get_class_name_obj<'j>(env: &JNIEnv<'j>, obj: JObject<'j>) -> Result<String, Error> {
    get_class_name(env, env.get_object_class(obj)?)
}

pub(crate) fn get_class_name<'j>(env: &JNIEnv<'j>, clazz: JClass<'j>) -> Result<String, Error> {
    let name = env.call_method(clazz, "getCanonicalName", "()Ljava/lang/String;", &[])?;
    let name = name.l()?;
    let name = JString::from(name);
    let name = env.get_string(name)?;
    Ok(Cow::from(&name).to_string())
}

#[derive(Clone, Debug)]
pub(crate) enum WasmTy {
    ByteBuffer,
    ValType(ValType),
}

impl From<ValType> for WasmTy {
    fn from(val: ValType) -> Self {
        WasmTy::ValType(val)
    }
}

impl WasmTy {
    pub fn push_arg_tys(&self, args: &mut Vec<ValType>) {
        match self {
            WasmTy::ByteBuffer => <ByteSlice as ComplexTy>::Abi::push_arg_tys(args),
            WasmTy::ValType(ValType::I32) => i32::push_arg_tys(args),
            WasmTy::ValType(ValType::I64) => i64::push_arg_tys(args),
            WasmTy::ValType(ValType::F32) => f32::push_arg_tys(args),
            WasmTy::ValType(ValType::F64) => f64::push_arg_tys(args),
            WasmTy::ValType(_) => unimplemented!(),
        }
    }

    pub fn matches_arg_tys(&self, tys: impl Iterator<Item = ValType>) -> anyhow::Result<()> {
        match self {
            WasmTy::ByteBuffer => <ByteSlice as ComplexTy>::Abi::matches_arg_tys(tys),
            WasmTy::ValType(ValType::I32) => i32::matches_arg_tys(tys),
            WasmTy::ValType(ValType::I64) => i64::matches_arg_tys(tys),
            WasmTy::ValType(ValType::F32) => f32::matches_arg_tys(tys),
            WasmTy::ValType(ValType::F64) => f64::matches_arg_tys(tys),
            WasmTy::ValType(_) => unimplemented!(),
        }
    }

    pub(crate) unsafe fn load_from_args<'j>(
        &self,
        env: &JNIEnv<'j>,
        args: impl Iterator<Item = Val>,
        memory: Option<&Memory>,
    ) -> Result<JObject<'j>, anyhow::Error> {
        match self {
            WasmTy::ByteBuffer => {
                <ByteSlice as ComplexTy>::Abi::load_from_args(args)?.into_java(env, memory)
            }
            WasmTy::ValType(ValType::I32) => i32::load_from_args(args)?.into_java(env, memory),
            WasmTy::ValType(ValType::I64) => i64::load_from_args(args)?.into_java(env, memory),
            WasmTy::ValType(ValType::F32) => f32::load_from_args(args)?.into_java(env, memory),
            WasmTy::ValType(ValType::F64) => f64::load_from_args(args)?.into_java(env, memory),
            WasmTy::ValType(_) => unimplemented!(),
        }
    }
}

impl fmt::Display for WasmTy {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            WasmTy::ByteBuffer => write!(f, "ByteBuffer"),
            WasmTy::ValType(val) => val.fmt(f),
        }
    }
}

pub(crate) enum WasmVal<'j> {
    ByteBuffer(JByteBuffer<'j>),
    Val(Val),
}

impl<'j> fmt::Debug for WasmVal<'j> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            WasmVal::ByteBuffer(_) => write!(f, "ByteBuffer"),
            WasmVal::Val(val) => val.fmt(f),
        }
    }
}

impl<'j> WasmVal<'j> {
    pub fn ty(&self) -> WasmTy {
        match self {
            WasmVal::ByteBuffer(_) => WasmTy::ByteBuffer,
            WasmVal::Val(val) => val.ty().into(),
        }
    }

    pub fn store_to_args(
        self,
        env: &JNIEnv<'j>,
        args: &mut Vec<Val>,
        memory: Option<&Memory>,
        allocator: Option<&Func>,
    ) -> Result<(), anyhow::Error> {
        match self {
            WasmVal::ByteBuffer(buffer) => {
                let direct_bytes: &[u8] = env.get_direct_buffer_address(buffer)?;

                // the module might not have the memory exported
                let memory = memory.ok_or_else(|| anyhow!("no memory supplied from module"))?;

                // the module didn't define __alloc?
                let allocator =
                    allocator.ok_or_else(|| anyhow!("no allocator supplied from module"))?;

                let mem_size = memory.size() as usize * MEM_SEGMENT_SIZE;
                ensure!(
                    mem_size > direct_bytes.len(),
                    "memory is {} need {} more",
                    mem_size,
                    direct_bytes.len()
                );

                // get target memor location and then copy into the function
                let data_len = direct_bytes.len();
                let offset = allocator
                    .call(&[Val::I32(data_len as i32)])?
                    .get(0)
                    .and_then(|v| v.i32())
                    .ok_or_else(|| anyhow!("i32 was not returned from the allocator"))?;

                debug!("data ptr: {}", offset);
                let mem_bytes =
                    unsafe { &mut memory.data_unchecked_mut()[offset as usize..][..data_len] };
                mem_bytes.copy_from_slice(direct_bytes);

                debug!(
                    "copied bytes into mem: {:x?}, mem_base: {:x?} mem_bytes: {:x?}",
                    mem_bytes,
                    memory.data_ptr(),
                    mem_bytes.as_ptr(),
                );

                let abi = WasmSlice {
                    ptr: offset,
                    len: mem_bytes.len() as i32,
                };

                abi.store_to_args(args);
            }
            WasmVal::Val(val @ Val::I32(_)) => val.unwrap_i32().store_to_args(args),
            WasmVal::Val(val @ Val::I64(_)) => val.unwrap_i64().store_to_args(args),
            WasmVal::Val(val @ Val::F32(_)) => val.unwrap_f32().store_to_args(args),
            WasmVal::Val(val @ Val::F64(_)) => val.unwrap_f64().store_to_args(args),
            WasmVal::Val(v) => unimplemented!("type not yet supported as an arg: {:?}", v),
        }

        Ok(())
    }
}

impl<'j> From<JByteBuffer<'j>> for WasmVal<'j> {
    fn from(bytes: JByteBuffer<'j>) -> Self {
        WasmVal::ByteBuffer(bytes)
    }
}

impl From<Val> for WasmVal<'static> {
    fn from(val: Val) -> Self {
        WasmVal::Val(val)
    }
}

trait IntoJavaObject {
    unsafe fn into_java<'j>(
        self,
        env: &JNIEnv<'j>,
        mem: Option<&Memory>,
    ) -> Result<JObject<'j>, Error>;
}

impl IntoJavaObject for <ByteSlice as ComplexTy>::Abi {
    unsafe fn into_java<'j>(
        self,
        env: &JNIEnv<'j>,
        mem: Option<&Memory>,
    ) -> Result<JObject<'j>, Error> {
        let bytes = &mut mem
            .ok_or_else(|| anyhow!("No memory provided for loading array"))?
            .data_unchecked_mut()[self.ptr as usize..][..self.len as usize];

        debug!("length of bytes for ByteBuffer: {}", bytes.len());
        debug!("read bytes: {}", String::from_utf8_lossy(bytes));

        let buffer = env
            .new_direct_byte_buffer(bytes)
            .context("Failed to create new ByteBuffer")?;

        Ok(buffer.into())
    }
}

impl IntoJavaObject for i64 {
    unsafe fn into_java<'j>(
        self,
        env: &JNIEnv<'j>,
        _mem: Option<&Memory>,
    ) -> Result<JObject<'j>, Error> {
        let jvalue = JValue::Long(self);
        env.new_object(LONG, "(J)V", &[jvalue])
            .context("Failed to create new Long")
    }
}

impl IntoJavaObject for i32 {
    unsafe fn into_java<'j>(
        self,
        env: &JNIEnv<'j>,
        _mem: Option<&Memory>,
    ) -> Result<JObject<'j>, Error> {
        let jvalue = JValue::Int(self);
        env.new_object(INTEGER, "(I)V", &[jvalue])
            .context("Failed to create new Integer")
    }
}

impl IntoJavaObject for f64 {
    unsafe fn into_java<'j>(
        self,
        env: &JNIEnv<'j>,
        _mem: Option<&Memory>,
    ) -> Result<JObject<'j>, Error> {
        let jvalue = JValue::Double(self);
        env.new_object(DOUBLE, "(D)V", &[jvalue])
            .context("Failed to create new Double")
    }
}

impl IntoJavaObject for f32 {
    unsafe fn into_java<'j>(
        self,
        env: &JNIEnv<'j>,
        _mem: Option<&Memory>,
    ) -> Result<JObject<'j>, Error> {
        let jvalue = JValue::Float(self);
        env.new_object(FLOAT, "(F)V", &[jvalue])
            .context("Failed to create new Float")
    }
}

pub(crate) fn from_java_class<'j>(
    env: &JNIEnv<'j>,
    clazz: JClass<'j>,
) -> Result<Option<WasmTy>, Error> {
    if clazz.is_null() {
        return Ok(None); // FIXME: this should be an exception, right?
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
        _ if env.is_assignable_from(clazz, BYTE_BUFFER)? => return Ok(Some(WasmTy::ByteBuffer)),
        _ if env.is_assignable_from(clazz, VOID)? => return Ok(None),
        _ if env.is_assignable_from(clazz, voidp)? => return Ok(None),
        _ => {
            let name = get_class_name(env, clazz)?;
            return Err(anyhow!("Unsupported Java class: {}", name));
        }
    };

    Ok(Some(ty.into()))
}

pub(crate) fn from_java<'j>(env: &JNIEnv<'j>, obj: JObject<'j>) -> Result<WasmVal<'j>, Error> {
    assert!(!obj.is_null(), "obj should not be null for conversion");
    match obj {
        _ if env.is_instance_of(obj, LONG)? => {
            let jvalue = env.call_method(obj, "longValue", "()J", &[])?;
            Ok(Val::I64(jvalue.j()?).into())
        }
        _ if env.is_instance_of(obj, INTEGER)? => {
            let jvalue = env.call_method(obj, "intValue", "()I", &[])?;
            Ok(Val::I32(jvalue.i()?).into())
        }
        _ if env.is_instance_of(obj, DOUBLE)? => {
            let jvalue = env.call_method(obj, "doubleValue", "()D", &[])?;
            Ok(Val::F64(jvalue.d()?.to_bits()).into())
        }
        _ if env.is_instance_of(obj, FLOAT)? => {
            let jvalue = env.call_method(obj, "floatValue", "()F", &[])?;
            Ok(Val::F32(jvalue.f()?.to_bits()).into())
        }
        _ if env.is_instance_of(obj, BYTE_BUFFER)? => {
            let buf = JByteBuffer::from(obj);
            Ok(WasmVal::from(buf))
        }
        _ => {
            let clazz = env.get_object_class(obj)?;
            let name = get_class_name(&env, clazz)?;
            Err(anyhow!("Unsupported Java object: {}", name))
        }
    }
}

pub(crate) fn from_jvalue<'j, 'b>(
    env: &JNIEnv<'j>,
    val: JValue<'j>,
) -> Result<Option<WasmVal<'j>>, Error> {
    let val = match val {
        JValue::Object(obj) => {
            if obj.is_null() {
                return Ok(None);
            } else {
                return from_java(env, obj).map(Some);
            }
        }
        JValue::Long(v) => Val::I64(v),
        JValue::Int(v) => Val::I32(v),
        JValue::Double(v) => Val::F64(f64::to_bits(v)),
        JValue::Float(v) => Val::F32(f32::to_bits(v)),
        _ => return Err(anyhow!("Unsuppored return type: {}", val.type_name())),
    };

    Ok(Some(val.into()))
}

pub(crate) fn to_java<'j, 'w: 'j>(env: &JNIEnv<'j>, val: &'w Val) -> Result<JObject<'j>, Error> {
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
            env.new_object(DOUBLE, "(D)V", &[jvalue])
        }
        Val::F32(val) => {
            let jvalue = JValue::Float(f32::from_bits(*val));
            env.new_object(FLOAT, "(F)V", &[jvalue])
        }
        _ => return Err(anyhow!("Unsupported WASM type: {}", val.ty())),
    };

    obj.with_context(|| format!("failed to convert {:?} to java", val))
}
