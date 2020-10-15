use std::ops::Deref;
use std::slice;

use anyhow::{anyhow, ensure};
use wasmtime::{Val, ValType, WeakStore};

use crate::ty::{Abi, ComplexTy, FromAbi, IntoAbi};

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
    pub fn new<'a>(bytes: &'a [u8]) -> &'a Self {
        unsafe { &*(bytes as *const [u8] as *const Self) }
    }
}

impl Deref for ByteSlice {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct WasmSlice {
    pub ptr: u32,
    pub len: u32,
}

impl ComplexTy for ByteSlice {
    type Abi = WasmSlice;

    #[inline]
    fn compatible_with_store<'a>(&self, _store: WeakStore<'a>) -> bool {
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
            .ok_or_else(|| anyhow!("ptr not i32"))? as u32;

        let len = args
            .next()
            .ok_or_else(|| anyhow!("missing ptr arg"))?
            .i32()
            .ok_or_else(|| anyhow!("ptr not i32"))? as u32;

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

impl<'b> IntoAbi for &'b ByteSlice {
    type Abi = WasmSlice;

    fn into_abi<'a>(self) -> Self::Abi {
        let ptr = self.0.as_ptr() as u32;
        let len = self.len() as u32;

        WasmSlice { ptr, len }
    }
}

impl<'b> FromAbi<WasmSlice> for &'b ByteSlice {
    unsafe fn from_abi<'a>(abi: WasmSlice) -> Self {
        let WasmSlice { ptr, len } = abi;

        let ptr = ptr as *const u8;
        ByteSlice::new(slice::from_raw_parts(ptr as *const _, len as usize))
    }
}
