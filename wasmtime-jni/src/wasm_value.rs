use std::borrow::Cow;
use std::fmt;
use std::marker::PhantomData;
use std::slice;

use anyhow::{anyhow, Context, Error};
use jni::objects::{JByteBuffer, JClass, JObject, JString, JValue, ReleaseMode};
use jni::sys::jbyteArray;
use jni::JNIEnv;
use log::debug;
use wasmtime::{Val, ValType};

use crate::ty::{Abi, ReturnAbi, WasmAlloc, WasmSlice};

const CLASS: &str = "Ljava/lang/Class;";
const LONG: &str = "java/lang/Long";
const INTEGER: &str = "java/lang/Integer";
const DOUBLE: &str = "java/lang/Double";
const FLOAT: &str = "java/lang/Float";
const VOID: &str = "java/lang/Void";
const BYTE_ARRAY: &str = "[B";
const PRIMITIVE: &str = "TYPE";

const BYTE_BUFFER: &str = "java/nio/ByteBuffer";

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
    ByteArray,
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
            WasmTy::ByteBuffer | WasmTy::ByteArray => WasmSlice::push_arg_tys(args),
            WasmTy::ValType(ValType::I32) => i32::push_arg_tys(args),
            WasmTy::ValType(ValType::I64) => i64::push_arg_tys(args),
            WasmTy::ValType(ValType::F32) => f32::push_arg_tys(args),
            WasmTy::ValType(ValType::F64) => f64::push_arg_tys(args),
            WasmTy::ValType(_) => unimplemented!(),
        }
    }

    pub fn matches_arg_tys(&self, tys: impl Iterator<Item = ValType>) -> anyhow::Result<()> {
        match self {
            WasmTy::ByteBuffer | WasmTy::ByteArray => WasmSlice::matches_arg_tys(tys),
            WasmTy::ValType(ValType::I32) => i32::matches_arg_tys(tys),
            WasmTy::ValType(ValType::I64) => i64::matches_arg_tys(tys),
            WasmTy::ValType(ValType::F32) => f32::matches_arg_tys(tys),
            WasmTy::ValType(ValType::F64) => f64::matches_arg_tys(tys),
            WasmTy::ValType(_) => unimplemented!(),
        }
    }

    pub fn get_return_by_ref_arg(&self, args: impl Iterator<Item = Val>) -> Option<i32> {
        match self {
            WasmTy::ByteBuffer | WasmTy::ByteArray => WasmSlice::get_return_by_ref_arg(args),
            WasmTy::ValType(ValType::I32) => i32::get_return_by_ref_arg(args),
            WasmTy::ValType(ValType::I64) => i64::get_return_by_ref_arg(args),
            WasmTy::ValType(ValType::F32) => f32::get_return_by_ref_arg(args),
            WasmTy::ValType(ValType::F64) => f64::get_return_by_ref_arg(args),
            WasmTy::ValType(_) => unimplemented!(),
        }
    }

    pub(crate) unsafe fn load_from_args<'j>(
        &self,
        env: &JNIEnv<'j>,
        args: impl Iterator<Item = Val>,
        wasm_alloc: Option<&mut WasmAlloc>,
    ) -> Result<JObject<'j>, anyhow::Error> {
        match self {
            WasmTy::ByteBuffer => {
                IntoByteBuffer(WasmSlice::load_from_args(args)?).into_java(env, wasm_alloc)
            }
            WasmTy::ByteArray => {
                IntoByteArray(WasmSlice::load_from_args(args)?).into_java(env, wasm_alloc)
            }
            WasmTy::ValType(ValType::I32) => i32::load_from_args(args)?.into_java(env, wasm_alloc),
            WasmTy::ValType(ValType::I64) => i64::load_from_args(args)?.into_java(env, wasm_alloc),
            WasmTy::ValType(ValType::F32) => f32::load_from_args(args)?.into_java(env, wasm_alloc),
            WasmTy::ValType(ValType::F64) => f64::load_from_args(args)?.into_java(env, wasm_alloc),
            WasmTy::ValType(_) => unimplemented!(),
        }
    }

    pub(crate) fn return_or_push_arg_tys(&self, args: &mut Vec<ValType>) -> Option<ValType> {
        match self {
            WasmTy::ByteBuffer | WasmTy::ByteArray => WasmSlice::return_or_push_arg_tys(args),
            WasmTy::ValType(ValType::I32) => i32::return_or_push_arg_tys(args),
            WasmTy::ValType(ValType::I64) => i64::return_or_push_arg_tys(args),
            WasmTy::ValType(ValType::F32) => f32::return_or_push_arg_tys(args),
            WasmTy::ValType(ValType::F64) => f64::return_or_push_arg_tys(args),
            WasmTy::ValType(_) => unimplemented!(),
        }
    }

    /// Matches the return type or the arg tys
    #[allow(unused)]
    pub(crate) fn matches_return_or_arg_tys(
        &self,
        ret: Option<ValType>,
        arg_tys: impl Iterator<Item = ValType>,
    ) -> Result<(), anyhow::Error> {
        match self {
            WasmTy::ByteBuffer | WasmTy::ByteArray => {
                WasmSlice::matches_return_or_arg_tys(ret, arg_tys)
            }
            WasmTy::ValType(ValType::I32) => i32::matches_return_or_arg_tys(ret, arg_tys),
            WasmTy::ValType(ValType::I64) => i64::matches_return_or_arg_tys(ret, arg_tys),
            WasmTy::ValType(ValType::F32) => f32::matches_return_or_arg_tys(ret, arg_tys),
            WasmTy::ValType(ValType::F64) => f64::matches_return_or_arg_tys(ret, arg_tys),
            WasmTy::ValType(_) => unimplemented!(),
        }
    }

    /// Place the values in the argument list
    pub(crate) fn return_or_store_to_arg(
        self,
        args: &mut Vec<Val>,
        wasm_alloc: Option<&mut WasmAlloc>,
    ) -> Result<Option<i32>, Error> {
        match self {
            WasmTy::ByteBuffer | WasmTy::ByteArray => {
                WasmSlice::return_or_store_to_arg(args, wasm_alloc)
            }
            WasmTy::ValType(ValType::I32) => i32::return_or_store_to_arg(args, wasm_alloc),
            WasmTy::ValType(ValType::I64) => i64::return_or_store_to_arg(args, wasm_alloc),
            WasmTy::ValType(ValType::F32) => f32::return_or_store_to_arg(args, wasm_alloc),
            WasmTy::ValType(ValType::F64) => f64::return_or_store_to_arg(args, wasm_alloc),
            WasmTy::ValType(_) => unimplemented!(),
        }
    }
}

impl fmt::Display for WasmTy {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            WasmTy::ByteBuffer => write!(f, "ByteBuffer"),
            WasmTy::ByteArray => write!(f, "byte[]"),
            WasmTy::ValType(val) => val.fmt(f),
        }
    }
}

pub(crate) enum WasmVal<'j> {
    ByteBuffer(JByteBuffer<'j>),
    ByteArray {
        jarray: jbyteArray,
        lifetime: PhantomData<&'j ()>,
    },
    Val(Val),
}

impl<'j> fmt::Debug for WasmVal<'j> {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            WasmVal::ByteBuffer(_) => write!(f, "ByteBuffer"),
            WasmVal::ByteArray { .. } => write!(f, "byte[]"),
            WasmVal::Val(val) => val.fmt(f),
        }
    }
}

impl<'j> WasmVal<'j> {
    fn from_byte_array(_env: &JNIEnv<'j>, jarray: jbyteArray) -> Self {
        WasmVal::ByteArray {
            jarray,
            lifetime: PhantomData,
        }
    }

    pub fn ty(&self) -> WasmTy {
        match self {
            WasmVal::ByteBuffer(_) => WasmTy::ByteBuffer,
            WasmVal::ByteArray { .. } => WasmTy::ByteArray,
            WasmVal::Val(val) => val.ty().into(),
        }
    }

    pub fn store_to_args(
        self,
        env: &JNIEnv<'j>,
        args: &mut Vec<Val>,
        wasm_alloc: Option<&mut WasmAlloc>,
    ) -> Result<(), anyhow::Error> {
        match self {
            WasmVal::ByteBuffer(buffer) => {
                let direct_bytes: &[u8] = env.get_direct_buffer_address(buffer)?;

                // the module might not have the memory exported
                let wasm_alloc = wasm_alloc
                    .ok_or_else(|| anyhow!("no memory or allocator supplied from module"))?;
                let wasm_slice = wasm_alloc.alloc_bytes(direct_bytes)?;
                wasm_slice.store_to_args(args);
            }
            WasmVal::ByteArray { jarray, .. } => {
                // This is should be safe, it's copied into while borrowed the WASM context.
                let len = env.get_array_length(jarray)?;
                let (jbytes, _is_copy) = env.get_byte_array_elements(jarray)?;
                let byte_array: &[u8] =
                    unsafe { slice::from_raw_parts(jbytes as *const u8, len as usize) };

                // the module might not have the memory exported
                let wasm_alloc = wasm_alloc
                    .ok_or_else(|| anyhow!("no memory or allocator supplied from module"))?;
                let wasm_slice = wasm_alloc.alloc_bytes(byte_array)?;

                // release the array reference, CopyBack is following the JNI guidelines
                env.release_byte_array_elements(
                    jarray,
                    unsafe { &mut *jbytes },
                    ReleaseMode::CopyBack,
                )
                .context("failed to release Java array elements")?;
                wasm_slice.store_to_args(args);
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

impl From<Val> for WasmVal<'_> {
    fn from(val: Val) -> Self {
        WasmVal::Val(val)
    }
}

trait IntoJavaObject {
    unsafe fn into_java<'j>(
        self,
        env: &JNIEnv<'j>,
        wasm_alloc: Option<&mut WasmAlloc>,
    ) -> Result<JObject<'j>, Error>;
}

struct IntoByteBuffer(WasmSlice);
impl IntoJavaObject for IntoByteBuffer {
    unsafe fn into_java<'j>(
        self,
        env: &JNIEnv<'j>,
        wasm_alloc: Option<&mut WasmAlloc>,
    ) -> Result<JObject<'j>, Error> {
        let wasm_alloc =
            wasm_alloc.ok_or_else(|| anyhow!("WasmAlloc is required for this return type"))?;
        let bytes = wasm_alloc.as_mut(self.0);

        debug!(
            "length of bytes for ByteBuffer: {} expected len: {}",
            bytes.len(),
            self.0.len
        );
        debug!("read bytes from wasm_slice: {:x?}", bytes);

        let buffer = env
            .new_direct_byte_buffer(bytes)
            .context("Failed to create new ByteBuffer")?;

        Ok(buffer.into())
    }
}

struct IntoByteArray(WasmSlice);
impl IntoJavaObject for IntoByteArray {
    unsafe fn into_java<'j>(
        self,
        env: &JNIEnv<'j>,
        wasm_alloc: Option<&mut WasmAlloc>,
    ) -> Result<JObject<'j>, Error> {
        let wasm_alloc =
            wasm_alloc.ok_or_else(|| anyhow!("WasmAlloc is required for this return type"))?;
        let bytes = wasm_alloc.as_mut(self.0);

        debug!(
            "length of bytes for ByteBuffer: {} expected len: {}",
            bytes.len(),
            self.0.len
        );
        debug!("read bytes from wasm_slice: {:x?}", bytes);

        let buffer = env
            .byte_array_from_slice(bytes)
            .context("Failed to create new byte[]")?;

        // free the wasm bytes...
        wasm_alloc.dealloc_bytes(self.0)?;
        Ok(buffer.into())
    }
}

impl IntoJavaObject for i64 {
    unsafe fn into_java<'j>(
        self,
        env: &JNIEnv<'j>,
        _wasm_alloc: Option<&mut WasmAlloc>,
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
        _wasm_alloc: Option<&mut WasmAlloc>,
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
        _wasm_alloc: Option<&mut WasmAlloc>,
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
        _wasm_alloc: Option<&mut WasmAlloc>,
    ) -> Result<JObject<'j>, Error> {
        let jvalue = JValue::Float(self);
        env.new_object(FLOAT, "(F)V", &[jvalue])
            .context("Failed to create new Float")
    }
}

pub(crate) fn from_java_class<'j>(
    env: &JNIEnv<'j>,
    clazz: JClass<'j>,
    for_return: bool,
) -> Result<Option<WasmTy>, Error> {
    if clazz.is_null() {
        return Ok(None); // FIXME: this should be an exception, right?
    }

    let longp: JClass = env.get_static_field(LONG, PRIMITIVE, CLASS)?.l()?.into();
    let intp: JClass = env.get_static_field(INTEGER, PRIMITIVE, CLASS)?.l()?.into();
    let doublep: JClass = env.get_static_field(DOUBLE, PRIMITIVE, CLASS)?.l()?.into();
    let floatp: JClass = env.get_static_field(FLOAT, PRIMITIVE, CLASS)?.l()?.into();
    let voidp: JClass = env.get_static_field(VOID, PRIMITIVE, CLASS)?.l()?.into();
    //let bytea: JClass = env.find_class("[B")?;

    let ty = match clazz {
        _ if env.is_assignable_from(clazz, LONG)? => ValType::I64,
        _ if env.is_assignable_from(clazz, longp)? => ValType::I64,
        _ if env.is_assignable_from(clazz, INTEGER)? => ValType::I32,
        _ if env.is_assignable_from(clazz, intp)? => ValType::I32,
        _ if env.is_assignable_from(clazz, DOUBLE)? => ValType::F64,
        _ if env.is_assignable_from(clazz, doublep)? => ValType::F64,
        _ if env.is_assignable_from(clazz, FLOAT)? => ValType::F32,
        _ if env.is_assignable_from(clazz, floatp)? => ValType::F32,
        _ if !for_return && env.is_assignable_from(clazz, BYTE_BUFFER)? => {
            // cant't return
            return Ok(Some(WasmTy::ByteBuffer));
        }
        _ if env.is_assignable_from(clazz, BYTE_ARRAY)? => return Ok(Some(WasmTy::ByteArray)),
        _ if env.is_assignable_from(clazz, VOID)? => return Ok(None),
        _ if env.is_assignable_from(clazz, voidp)? => return Ok(None),
        _ => {
            let name = get_class_name(env, clazz)?;
            if !for_return {
                return Err(anyhow!("Unsupported Java class as argument: {}", name));
            } else {
                return Err(anyhow!("Unsupported Java class as result: {}", name));
            }
        }
    };

    Ok(Some(ty.into()))
}

pub(crate) fn from_java<'j, 'b: 'j>(
    env: &'b JNIEnv<'j>,
    obj: JObject<'j>,
) -> Result<WasmVal<'j>, Error> {
    let bytea: JClass = env.find_class("[B")?;

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
        _ if env.is_instance_of(obj, bytea)? => Ok(WasmVal::from_byte_array(env, *obj)),
        _ => {
            let clazz = env.get_object_class(obj)?;
            let name = get_class_name(&env, clazz)?;
            Err(anyhow!("Unsupported Java object: {}", name))
        }
    }
}

pub(crate) fn from_jvalue<'j, 'b: 'j>(
    env: &'b JNIEnv<'j>,
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

// pub(crate) fn to_java<'j, 'w: 'j>(env: &JNIEnv<'j>, val: &'w Val) -> Result<JObject<'j>, Error> {
//     let obj = match val {
//         Val::I64(val) => {
//             let jvalue = JValue::Long(*val);
//             env.new_object(LONG, "(J)V", &[jvalue])
//         }
//         Val::I32(val) => {
//             let jvalue = JValue::Int(*val);
//             env.new_object(INTEGER, "(I)V", &[jvalue])
//         }
//         Val::F64(val) => {
//             let jvalue = JValue::Double(f64::from_bits(*val));
//             env.new_object(DOUBLE, "(D)V", &[jvalue])
//         }
//         Val::F32(val) => {
//             let jvalue = JValue::Float(f32::from_bits(*val));
//             env.new_object(FLOAT, "(F)V", &[jvalue])
//         }
//         _ => return Err(anyhow!("Unsupported WASM type: {}", val.ty())),
//     };

//     obj.with_context(|| format!("failed to convert {:?} to java", val))
// }

pub(crate) unsafe fn return_or_load_or_from_arg<'a>(
    env: &JNIEnv<'a>,
    ty: WasmTy,
    ret: Option<&Val>,
    ret_by_ref_ptr: Option<i32>,
    mut wasm_alloc: Option<&mut WasmAlloc>,
) -> Result<JObject<'a>, anyhow::Error> {
    match ty {
        WasmTy::ByteBuffer => {
            let wasm_slice = WasmSlice::return_or_load_or_from_args(
                ret,
                ret_by_ref_ptr,
                wasm_alloc.as_deref_mut(),
            )?;
            IntoByteBuffer(wasm_slice).into_java(env, wasm_alloc.as_deref_mut())
        }
        WasmTy::ByteArray => {
            let wasm_slice = WasmSlice::return_or_load_or_from_args(
                ret,
                ret_by_ref_ptr,
                wasm_alloc.as_deref_mut(),
            )?;
            IntoByteArray(wasm_slice).into_java(env, wasm_alloc.as_deref_mut())
        }
        WasmTy::ValType(ValType::I32) => {
            i32::return_or_load_or_from_args(ret, ret_by_ref_ptr, wasm_alloc.as_deref_mut())?
                .into_java(env, wasm_alloc.as_deref_mut())
        }
        WasmTy::ValType(ValType::I64) => {
            i64::return_or_load_or_from_args(ret, ret_by_ref_ptr, wasm_alloc.as_deref_mut())?
                .into_java(env, wasm_alloc.as_deref_mut())
        }
        WasmTy::ValType(ValType::F32) => {
            f32::return_or_load_or_from_args(ret, ret_by_ref_ptr, wasm_alloc.as_deref_mut())?
                .into_java(env, wasm_alloc.as_deref_mut())
        }
        WasmTy::ValType(ValType::F64) => {
            f64::return_or_load_or_from_args(ret, ret_by_ref_ptr, wasm_alloc.as_deref_mut())?
                .into_java(env, wasm_alloc.as_deref_mut())
        }
        WasmTy::ValType(_) => unimplemented!(),
    }
}
