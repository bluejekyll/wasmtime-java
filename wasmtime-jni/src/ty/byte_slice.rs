use std::ops::Deref;

use anyhow::{anyhow, ensure, Error};
use log::debug;
use wasmtime::{Store, Val, ValType};
pub use wasmtime_jni_exports::WasmSlice;

use crate::ty::{Abi, ComplexTy, ReturnAbi, WasmAlloc, WasmSliceWrapper};

// pub fn greet(name: &str) -> String {
//     format!("Hello, {}!", name)
// }

// #[allow(non_snake_case)]
// #[cfg_attr(
//     all(target_arch = "wasm32", not(target_os = "emscripten")),
//     export_name = "greet"
// )]
// #[allow(clippy::all)]
// pub extern "C" fn __wasm_bindgen_generated_greet(
//     arg0: <str as wasm_bindgen::convert::RefFromWasmAbi>::Abi,
// ) -> <String as wasm_bindgen::convert::ReturnWasmAbi>::Abi {
//     let _ret = {
//         let arg0 = unsafe { <str as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(arg0) };
//         let arg0 = &*arg0;
//         greet(arg0)
//     };
//     <String as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(_ret)
// }

// #[no_mangle]
// #[allow(non_snake_case)]
// #[doc(hidden)]
// #[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
// #[allow(clippy::all)]
// pub extern "C" fn __wbindgen_describe_greet() {
//     use wasm_bindgen::describe::*;

//     wasm_bindgen::__rt::link_mem_intrinsics();

//     inform(FUNCTION);
//     inform(0);
//     inform(1u32);

//     <&str as WasmDescribe>::describe();
//     <String as WasmDescribe>::describe();
// }

// #[allow(non_upper_case_globals)]
// #[cfg(target_arch = "wasm32")]
// #[link_section = "__wasm_bindgen_unstable"]
// #[doc(hidden)]
// #[allow(clippy::all)]
// pub static __WASM_BINDGEN_GENERATED_85872804fca1fa32: [u8; 107usize] = {
//     static _INCLUDED_FILES: &[&str] = &[];
//     *
//     b".\x00\x00\x00{\"schema_version\":\"0.2.68\",\"version\":\"0.2.68\"}5\x00\x00\x00\x01\x00\x00\x00\x01\x04name\x05greet\x01\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x18strings-841a3553a8e33c06\x00"
// };

#[derive(Debug)]
#[repr(transparent)]
pub struct ByteSlice([u8]);

impl ByteSlice {
    // pub fn new<'a>(bytes: &'a [u8]) -> &'a Self {
    //     unsafe { &*(bytes as *const [u8] as *const Self) }
    // }
}

impl Deref for ByteSlice {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ComplexTy for ByteSlice {
    type Abi = WasmSlice;

    #[inline]
    fn compatible_with_store(&self, _store: &Store) -> bool {
        true
    }
}

impl Abi for WasmSlice {
    fn push_arg_tys(args: &mut Vec<ValType>) {
        args.push(ValType::I32); // offset into memory
        args.push(ValType::I32); // length
    }

    fn store_to_args(self, args: &mut Vec<Val>) {
        let WasmSlice { ptr, len } = self;
        args.push(Val::from(ptr as i32));
        args.push(Val::from(len as i32));
    }

    fn load_from_args(mut args: impl Iterator<Item = Val>) -> Result<Self, anyhow::Error> {
        let ptr = args
            .next()
            .ok_or_else(|| anyhow!("missing ptr arg"))?
            .i32()
            .ok_or_else(|| anyhow!("ptr not i32"))?;

        let len = args
            .next()
            .ok_or_else(|| anyhow!("missing ptr arg"))?
            .i32()
            .ok_or_else(|| anyhow!("ptr not i32"))?;

        Ok(WasmSlice { ptr, len })
    }

    fn matches_arg_tys(mut tys: impl Iterator<Item = ValType>) -> anyhow::Result<()> {
        // offset
        let next = tys.next();
        ensure!(
            next == Some(ValType::I32),
            "Expected offset for memory: {:?}",
            next
        );

        // length
        let next = tys.next();
        ensure!(
            next == Some(ValType::I32),
            "Expected length for memory: {:?}",
            next
        );

        Ok(())
    }
}

impl ReturnAbi for WasmSlice {
    /// Place the necessary type signature in the type list
    #[allow(unused)]
    fn return_or_push_arg_tys(args: &mut Vec<ValType>) -> Option<ValType> {
        // For slice returns, we need a pointer to WasmSlice in the final parameter position
        args.push(ValType::I32);

        None
    }

    /// Place the values in the argument list, if there was an allocation, the pointer is returned
    #[allow(unused)]
    fn return_or_store_to_arg<'w>(
        args: &mut Vec<Val>,
        wasm_alloc: Option<&'w WasmAlloc>,
    ) -> Result<Option<WasmSliceWrapper<'w>>, Error> {
        // create a place in memory for the slice to be returned
        let slice = wasm_alloc
            .ok_or_else(|| anyhow!("WasmAlloc not supplied"))?
            .alloc::<Self>()?;

        args.push(Val::from(slice.ptr));
        Ok(Some(slice))
    }

    fn get_return_by_ref_arg(mut args: impl Iterator<Item = Val>) -> Option<i32> {
        args.next().as_ref().and_then(Val::i32)
    }

    /// Load from the returned value, or from the passed in pointer to the return by ref parameter
    fn return_or_load_or_from_args(
        _ret: Option<&Val>,
        mut ret_by_ref_ptr: Option<WasmSliceWrapper<'_>>,
        _wasm_alloc: Option<&WasmAlloc>,
    ) -> Result<Self, anyhow::Error> {
        let ptr = ret_by_ref_ptr
            .take()
            .ok_or_else(|| anyhow!("No pointer was supplied"))?;
        let wasm_slice = unsafe { ptr.obj_as_mut() };

        debug!("read {:?}", wasm_slice);
        Ok(*wasm_slice)
    }

    /// matches the arg tys
    fn matches_return_or_arg_tys(
        _ret: Option<ValType>,
        mut tys: impl Iterator<Item = ValType>,
    ) -> Result<(), Error> {
        let ty = tys.next();

        // is a pointer type
        ensure!(
            ty == Some(ValType::I32),
            "Expected ptr for return by ref: {:?}",
            ty
        );

        Ok(())
    }
}

// impl<'b> IntoAbi for &'b ByteSlice {
//     type Abi = WasmSlice;

//     fn into_abi<'a>(self) -> Self::Abi {
//         let ptr = self.0.as_ptr() as i32;
//         let len = self.len() as i32;

//         WasmSlice { ptr, len }
//     }
// }

// impl<'b> FromAbi<WasmSlice> for &'b ByteSlice {
//     unsafe fn from_abi<'a>(abi: WasmSlice) -> Self {
//         let WasmSlice { ptr, len } = abi;

//         let ptr = ptr as *const u8;
//         ByteSlice::new(slice::from_raw_parts(ptr as *const _, len as usize))
//     }
// }
